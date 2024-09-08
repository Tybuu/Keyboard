//! This example test the RP Pico on board LED.
//!
//! It does not work with the RP Pico W board. See wifi_blinky.rs.

#![no_std]
#![no_main]

use core::sync::atomic::{AtomicBool, Ordering};

use defmt::info;
use embassy_executor::Spawner;
use embassy_futures::join::join3;
use embassy_rp::adc::{self, Adc, Channel, Config as AdcConfig};
use embassy_rp::gpio::{Pin, Pull};
use embassy_rp::{bind_interrupts, gpio, peripherals, usb};
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::mutex::Mutex;
use embassy_time::Timer;
use keyboard::descriptor::{BufferReport, KeyboardReportNKRO, MouseReport};
use keyboard::key_config::load_key_config;
use keyboard::keys::Keys;

use embassy_rp::usb::Driver;
use embassy_usb::class::hid::{HidReaderWriter, HidWriter, State};
use embassy_usb::{Builder, Config, Handler};
use gpio::{Level, Output};
use keyboard::report::Report;
use usbd_hid::descriptor::SerializedDescriptor;
use {defmt_rtt as _, panic_probe as _};

bind_interrupts!(struct Irqs {
    USBCTRL_IRQ => usb::InterruptHandler<peripherals::USB>;
    ADC_IRQ_FIFO => adc::InterruptHandler;
});

static MUX: Mutex<CriticalSectionRawMutex, [u8; 3]> = Mutex::new([0u8; 3]);

pub const NUM_KEYS: usize = 42;
#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    info!("Device Started!");
    let p = embassy_rp::init(Default::default());
    // Create the driver, from the HAL.
    let driver = Driver::new(p.USB, Irqs);

    // Create embassy-usb Config
    let mut config = Config::new(0xa55, 0xa55);
    config.manufacturer = Some("Tybeast Corp.");
    config.product = Some("Tybeast Ones (Left)");
    config.max_power = 500;
    config.max_packet_size_0 = 64;
    config.composite_with_iads = true;
    config.device_class = 0xef;
    config.device_sub_class = 0x02;
    config.device_protocol = 0x01;

    // Create embassy-usb DeviceBuilder using the driver and config.
    // It needs some buffers for building the descriptors.
    let mut config_descriptor = [0; 256];
    let mut bos_descriptor = [0; 256];
    let mut msos_descriptor = [0; 256];
    let mut control_buf = [0; 64];
    let mut device_handler = MyDeviceHandler::new();

    let mut key_state = State::new();
    let mut slave_state = State::new();
    let mut com_state = State::new();
    let mut mouse_state = State::new();

    let mut builder = Builder::new(
        driver,
        config,
        &mut config_descriptor,
        &mut bos_descriptor,
        &mut msos_descriptor,
        &mut control_buf,
    );

    builder.handler(&mut device_handler);

    // Create classes on the builder.
    let key_config = embassy_usb::class::hid::Config {
        report_descriptor: KeyboardReportNKRO::desc(),
        request_handler: None,
        poll_ms: 1,
        max_packet_size: 32,
    };
    let slave_config = embassy_usb::class::hid::Config {
        report_descriptor: BufferReport::desc(),
        request_handler: None,
        poll_ms: 1,
        max_packet_size: 64,
    };
    let com_config = embassy_usb::class::hid::Config {
        report_descriptor: BufferReport::desc(),
        request_handler: None,
        poll_ms: 60,
        max_packet_size: 64,
    };
    let mouse_config = embassy_usb::class::hid::Config {
        report_descriptor: MouseReport::desc(),
        request_handler: None,
        poll_ms: 1,
        max_packet_size: 5,
    };

    let mut key_writer = HidWriter::<_, 29>::new(&mut builder, &mut key_state, key_config);
    let slave_hid = HidReaderWriter::<_, 4, 1>::new(&mut builder, &mut slave_state, slave_config);
    let com_hid = HidReaderWriter::<_, 32, 32>::new(&mut builder, &mut com_state, com_config);
    let mut mouse_writer = HidWriter::<_, 5>::new(&mut builder, &mut mouse_state, mouse_config);

    let (mut s_reader, mut s_writer) = slave_hid.split();
    let (mut c_reader, mut c_writer) = com_hid.split();

    // Build the builder.
    let mut usb = builder.build();
    let usb_fut = usb.run();

    // Sel Pins
    let mut sel0 = Output::new(p.PIN_2, Level::Low);
    let mut sel1 = Output::new(p.PIN_1, Level::Low);
    let mut sel2 = Output::new(p.PIN_0, Level::Low);

    // Adc
    let mut adc = Adc::new(p.ADC, Irqs, AdcConfig::default());
    let mut a3 = Channel::new_pin(p.PIN_26, Pull::None);
    let mut a2 = Channel::new_pin(p.PIN_27, Pull::None);
    let mut a1 = Channel::new_pin(p.PIN_28, Pull::None);
    let mut a0 = Channel::new_pin(p.PIN_29, Pull::None);

    let mut order: [usize; NUM_KEYS / 2] = [
        7, 14, 2, 18, 5, 0, 3, 11, 6, 1, 9, 4, 15, 19, 10, 13, 17, 8, 12, 16, 20,
    ];
    find_order(&mut order);

    let mut keys = Keys::<NUM_KEYS>::default();
    load_key_config(&mut keys);

    let mut report = Report::default();

    // Main keyboard loop
    let usb_key_in = async {
        loop {
            let mut slave_keys = [0u8; 3];
            {
                let shared = MUX.lock().await;
                slave_keys = *shared;
            }
            let mut pos = 0;
            // Left Keyboard Scan
            for i in order {
                let chan = pos % 4;
                if chan == 0 {
                    change_sel(&mut sel0, &mut sel1, &mut sel2, pos / 4);
                    Timer::after_micros(2).await;
                }
                match chan {
                    0 => keys.update_buf(i, adc.read(&mut a0).await.unwrap()),
                    1 => keys.update_buf(i, adc.read(&mut a1).await.unwrap()),
                    2 => keys.update_buf(i, adc.read(&mut a2).await.unwrap()),
                    3 => keys.update_buf(i, adc.read(&mut a3).await.unwrap()),
                    _ => {}
                }
                pos += 1;
            }
            // Right Keyboard Scan
            for i in 0..21 {
                let a_idx = (i / 8) as usize;
                let b_idx = i % 8;
                let val = (slave_keys[a_idx] >> b_idx) & 1;
                keys.update_buf(i + 21, val as u16);
            }
            let (key_report, m_report) = report.generate_report(&mut keys);
            match key_report {
                Some(report) => {
                    key_writer.write_serialize(&report).await.unwrap();
                }
                None => {}
            }
            match m_report {
                Some(report) => {
                    mouse_writer.write_serialize(&report).await.unwrap();
                }
                None => {}
            }
        }
    };

    // Get slave keys from HID report
    let buffer_out = async {
        loop {
            let mut buf = [0u8; 32];
            s_reader.read(&mut buf).await.unwrap();
            if buf[0] == 5 {
                let mut shared = MUX.lock().await;
                (*shared)[0] = buf[1];
                (*shared)[1] = buf[2];
                (*shared)[2] = buf[3];
            }
        }
    };
    join3(usb_key_in, usb_fut, buffer_out).await;
}

struct MyDeviceHandler {
    configured: AtomicBool,
}

impl MyDeviceHandler {
    fn new() -> Self {
        MyDeviceHandler {
            configured: AtomicBool::new(false),
        }
    }
}

impl Handler for MyDeviceHandler {
    fn enabled(&mut self, enabled: bool) {
        self.configured.store(false, Ordering::Relaxed);
        if enabled {
            info!("Device enabled");
        } else {
            info!("Device disabled");
        }
    }

    fn reset(&mut self) {
        self.configured.store(false, Ordering::Relaxed);
        info!("Bus reset, the Vbus current limit is 100mA");
    }

    fn addressed(&mut self, addr: u8) {
        self.configured.store(false, Ordering::Relaxed);
        info!("USB address set to: {}", addr);
    }

    fn configured(&mut self, configured: bool) {
        self.configured.store(configured, Ordering::Relaxed);
        if configured {
            info!(
                "Device configured, it may now draw up to the configured current limit from Vbus."
            )
        } else {
            info!("Device is no longer configured, the Vbus current limit is 100mA.");
        }
    }
}

fn find_order(ary: &mut [usize]) {
    let mut new_ary = [0usize; NUM_KEYS / 2];
    for i in 0..ary.len() {
        for j in 0..ary.len() {
            if ary[j as usize] == i {
                new_ary[i as usize] = j;
            }
        }
    }
    ary.copy_from_slice(&new_ary);
}

/// Change the sel pins to represent the state represented in num
fn change_sel<P0: Pin, P1: Pin, P2: Pin>(
    sel0: &mut Output<P0>,
    sel1: &mut Output<P1>,
    sel2: &mut Output<P2>,
    num: u8,
) {
    match num {
        0 => {
            sel0.set_low();
            sel1.set_low();
            sel2.set_low();
        }
        1 => {
            sel0.set_high();
            sel1.set_low();
            sel2.set_low();
        }
        2 => {
            sel0.set_low();
            sel1.set_high();
            sel2.set_low();
        }
        3 => {
            sel0.set_high();
            sel1.set_high();
            sel2.set_low();
        }
        4 => {
            sel0.set_low();
            sel1.set_low();
            sel2.set_high();
        }
        5 => {
            sel0.set_high();
            sel1.set_low();
            sel2.set_high();
        }
        _ => {
            sel0.set_low();
            sel1.set_low();
            sel2.set_low();
        }
    }
}
