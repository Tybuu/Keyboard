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
use embassy_rp::peripherals::USB;
use embassy_rp::{bind_interrupts, gpio, peripherals, usb};
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::mutex::Mutex;
use embassy_time::{Instant, Timer};
use keyboard::descriptor::{BufferReport, KeyboardReportNKRO, MouseReport};
use keyboard::key_config::{load_callum, load_key_config, load_trial};
use keyboard::keys::Keys;

use embassy_rp::usb::Driver;
use embassy_usb::class::hid::{HidReaderWriter, HidWriter, State};
use embassy_usb::{Builder, Config, Handler};
use gpio::{Level, Output};
use keyboard::report::Report;
use log::logger;
use usbd_hid::descriptor::SerializedDescriptor;
use {defmt_rtt as _, panic_probe as _};

bind_interrupts!(struct Irqs {
    USBCTRL_IRQ => usb::InterruptHandler<peripherals::USB>;
    ADC_IRQ_FIFO => adc::InterruptHandler;
});

#[embassy_executor::task]
async fn logger_task(driver: Driver<'static, USB>) {
    embassy_usb_logger::run!(1024, log::LevelFilter::Info, driver);
}

static MUX: Mutex<CriticalSectionRawMutex, [u8; 3]> = Mutex::new([0u8; 3]);

pub const NUM_KEYS: usize = 42;
#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    info!("Device Started!");
    let p = embassy_rp::init(Default::default());
    // Create the driver, from the HAL.
    let driver = Driver::new(p.USB, Irqs);

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
    load_callum(&mut keys);

    let mut report = Report::default();

    // Main keyboard loop
    _spawner.spawn(logger_task(driver)).unwrap();
    loop {
        let mut slave_keys = [0u8; 3];
        // {
        //     let shared = MUX.lock().await;
        //     slave_keys = *shared;
        // }
        let mut pos = 0;
        // Left Keyboard Scan
        for i in order {
            // Equivalent to pos % 4
            let chan = pos & 0b11;
            if chan == 0 {
                // equivalent to pos / 4
                change_sel(&mut sel0, &mut sel1, &mut sel2, pos >> 2);
            }
            match chan {
                0 => keys.update_buf(i, 4095 - adc.read(&mut a0).await.unwrap()),
                1 => keys.update_buf(i, 4095 - adc.read(&mut a1).await.unwrap()),
                2 => keys.update_buf(i, 4095 - adc.read(&mut a2).await.unwrap()),
                3 => keys.update_buf(i, 4095 - adc.read(&mut a3).await.unwrap()),
                _ => {}
            }
            pos += 1;
        }
        // Right Keyboard Scan
        for i in 0..21 {
            // equivalent to i / 8
            let a_idx = (i >> 3) as usize;
            // equivalent to i % 8
            let b_idx = i & 0b111;
            let val = (slave_keys[a_idx] >> b_idx) & 1;
            keys.update_buf(i + 21, val as u16);
        }
        let (key_report, m_report) = report.generate_report(&mut keys);
        log::info!(
            "[{}, {}, {}, {}, {}, {}",
            keys.get_buf(0),
            keys.get_buf(1),
            keys.get_buf(2),
            keys.get_buf(3),
            keys.get_buf(4),
            keys.get_buf(5),
        );

        log::info!(
            "{}, {}, {}, {}, {}, {}",
            keys.get_buf(6),
            keys.get_buf(7),
            keys.get_buf(8),
            keys.get_buf(9),
            keys.get_buf(10),
            keys.get_buf(11),
        );

        log::info!(
            "{}, {}, {}, {}, {}, {}",
            keys.get_buf(12),
            keys.get_buf(13),
            keys.get_buf(14),
            keys.get_buf(15),
            keys.get_buf(16),
            keys.get_buf(17),
        );

        log::info!(
            "{}, {}, {}",
            keys.get_buf(18),
            keys.get_buf(19),
            keys.get_buf(20),
        );
        Timer::after_millis(20).await;
    }
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
