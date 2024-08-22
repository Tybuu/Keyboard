//! This example test the RP Pico on board LED.
//!
//! It does not work with the RP Pico W board. See wifi_blinky.rs.

#![no_std]
#![no_main]

use core::sync::atomic::{AtomicBool, Ordering};

use defmt::info;
use embassy_executor::Spawner;
use embassy_futures::join::{join, join3, join_array};
use embassy_rp::adc::{self, Adc, Channel, Config as AdcConfig};
use embassy_rp::gpio::{AnyPin, Pin, Pull};
use embassy_rp::{bind_interrupts, gpio, peripherals, usb};
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::mutex::Mutex;
use embassy_time::{Duration, Timer};
use keyboard::codes::KeyCodes;
use keyboard::descriptor::{BufferReport, KeyboardReportNKRO, MouseReport};
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

const SCROLL_TIME: u64 = 500;
const MOUSE_POINTER_TIME: u64 = 10;

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

    // Layer 0
    keys.set_code(KeyCodes::KeyboardEscape, 0, 0);
    keys.set_code(KeyCodes::KeyboardQq, 1, 0);
    keys.set_code(KeyCodes::KeyboardWw, 2, 0);
    keys.set_code(KeyCodes::KeyboardEe, 3, 0);
    keys.set_code(KeyCodes::KeyboardRr, 4, 0);
    keys.set_code(KeyCodes::KeyboardTt, 5, 0);

    keys.set_code(KeyCodes::KeyboardLeftControl, 6, 0);
    keys.set_code(KeyCodes::KeyboardAa, 7, 0);
    keys.set_code(KeyCodes::KeyboardSs, 8, 0);
    keys.set_code(KeyCodes::KeyboardDd, 9, 0);
    keys.set_code(KeyCodes::KeyboardFf, 10, 0);
    keys.set_code(KeyCodes::KeyboardGg, 11, 0);

    keys.set_code(KeyCodes::KeyboardLeftShift, 12, 0);
    keys.set_code(KeyCodes::KeyboardZz, 13, 0);
    keys.set_code(KeyCodes::KeyboardXx, 14, 0);
    keys.set_code(KeyCodes::KeyboardCc, 15, 0);
    keys.set_code(KeyCodes::KeyboardVv, 16, 0);
    keys.set_code(KeyCodes::KeyboardBb, 17, 0);

    keys.set_code(KeyCodes::KeyboardLeftGUI, 18, 0);
    keys.set_combined(KeyCodes::Layer1, KeyCodes::Layer6, 40, 19, 0);
    keys.set_code(KeyCodes::KeyboardSpacebar, 20, 0);

    keys.set_code(KeyCodes::KeyboardYy, 21, 0);
    keys.set_code(KeyCodes::KeyboardUu, 22, 0);
    keys.set_code(KeyCodes::KeyboardIi, 23, 0);
    keys.set_code(KeyCodes::KeyboardOo, 24, 0);
    keys.set_code(KeyCodes::KeyboardPp, 25, 0);
    keys.set_code(KeyCodes::KeyboardBackspace, 26, 0);

    keys.set_code(KeyCodes::KeyboardHh, 27, 0);
    keys.set_code(KeyCodes::KeyboardJj, 28, 0);
    keys.set_code(KeyCodes::KeyboardKk, 29, 0);
    keys.set_code(KeyCodes::KeyboardLl, 30, 0);
    keys.set_code(KeyCodes::KeyboardSemiColon, 31, 0);
    keys.set_code(KeyCodes::KeyboardSingleDoubleQuote, 32, 0);

    keys.set_code(KeyCodes::KeyboardNn, 33, 0);
    keys.set_code(KeyCodes::KeyboardMm, 34, 0);
    keys.set_code(KeyCodes::KeyboardCommaLess, 35, 0);
    keys.set_code(KeyCodes::KeyboardPeriodGreater, 36, 0);
    keys.set_code(KeyCodes::KeyboardSlashQuestion, 37, 0);
    keys.set_code(KeyCodes::KeyboardRightShift, 38, 0);

    keys.set_code(KeyCodes::KeyboardEnter, 39, 0);
    keys.set_combined(KeyCodes::Layer2, KeyCodes::Layer6, 19, 40, 0);
    keys.set_code(KeyCodes::KeyboardLeftAlt, 41, 0);

    // Layer 1
    keys.set_code(KeyCodes::KeyboardTab, 0, 1);
    keys.set_code(KeyCodes::KeyboardBacktickTilde, 1, 1);
    keys.set_double(
        KeyCodes::KeyboardOpenBracketBrace,
        KeyCodes::KeyboardLeftShift,
        1,
        1,
    );

    keys.set_code(KeyCodes::KeyboardLeftControl, 6, 1);
    keys.set_code(KeyCodes::Keyboard1Exclamation, 7, 1);
    keys.set_code(KeyCodes::Keyboard2At, 8, 1);
    keys.set_code(KeyCodes::Keyboard3Hash, 9, 1);
    keys.set_code(KeyCodes::Keyboard4Dollar, 10, 1);
    keys.set_code(KeyCodes::Keyboard5Percent, 11, 1);

    keys.set_code(KeyCodes::KeyboardLeftShift, 12, 1);

    keys.set_code(KeyCodes::KeyboardLeftGUI, 18, 1);
    keys.set_combined(KeyCodes::Layer1, KeyCodes::Layer6, 40, 19, 1);
    keys.set_code(KeyCodes::KeyboardSpacebar, 20, 1);

    keys.set_code(KeyCodes::KeyboardDashUnderscore, 21, 1);
    keys.set_code(KeyCodes::KeyboardEqualPlus, 22, 1);
    keys.set_code(KeyCodes::KeyboardOpenBracketBrace, 23, 1);
    keys.set_code(KeyCodes::KeyboardCloseBracketBrace, 24, 1);
    keys.set_code(KeyCodes::KeyboardBackslashBar, 25, 1);
    keys.set_code(KeyCodes::KeyboardBackspace, 26, 1);

    keys.set_code(KeyCodes::Keyboard6Caret, 27, 1);
    keys.set_code(KeyCodes::Keyboard7Ampersand, 28, 1);
    keys.set_code(KeyCodes::Keyboard8Asterisk, 29, 1);
    keys.set_code(KeyCodes::Keyboard9OpenParens, 30, 1);
    keys.set_code(KeyCodes::Keyboard0CloseParens, 31, 1);

    keys.set_combined(KeyCodes::Layer2, KeyCodes::Layer6, 19, 40, 1);

    // Layer 2
    keys.set_code(KeyCodes::KeyboardF1, 1, 2);
    keys.set_code(KeyCodes::KeyboardF2, 2, 2);
    keys.set_code(KeyCodes::KeyboardF3, 3, 2);
    keys.set_code(KeyCodes::KeyboardF4, 4, 2);
    keys.set_code(KeyCodes::KeyboardF5, 5, 2);

    keys.set_combined(KeyCodes::Layer1, KeyCodes::Layer6, 40, 19, 2);

    keys.set_code(KeyCodes::KeyboardF6, 21, 2);
    keys.set_code(KeyCodes::KeyboardF7, 22, 2);
    keys.set_code(KeyCodes::KeyboardF8, 23, 2);
    keys.set_code(KeyCodes::KeyboardF9, 24, 2);
    keys.set_code(KeyCodes::KeyboardF10, 25, 2);

    keys.set_code(KeyCodes::KeyboardLeftArrow, 27, 2);
    keys.set_code(KeyCodes::KeyboardDownArrow, 28, 2);
    keys.set_code(KeyCodes::KeyboardUpArrow, 29, 2);
    keys.set_code(KeyCodes::KeyboardRightArrow, 30, 2);

    keys.set_toggle_layer(KeyCodes::Layer3, 38, 2);
    keys.set_combined(KeyCodes::Layer2, KeyCodes::Layer6, 19, 40, 2);

    // Layer 3
    keys.set_code(KeyCodes::KeyboardEscape, 0, 3);
    keys.set_code(KeyCodes::KeyboardQq, 1, 3);
    keys.set_code(KeyCodes::KeyboardWw, 2, 3);
    keys.set_code(KeyCodes::KeyboardEe, 3, 3);
    keys.set_code(KeyCodes::KeyboardRr, 4, 3);
    keys.set_code(KeyCodes::KeyboardTt, 5, 3);

    keys.set_code(KeyCodes::KeyboardLeftControl, 6, 3);
    keys.set_code(KeyCodes::KeyboardAa, 7, 3);
    keys.set_code(KeyCodes::KeyboardSs, 8, 3);
    keys.set_code(KeyCodes::KeyboardDd, 9, 3);
    keys.set_code(KeyCodes::KeyboardFf, 10, 3);
    keys.set_code(KeyCodes::KeyboardGg, 11, 3);

    keys.set_code(KeyCodes::KeyboardLeftShift, 12, 3);
    keys.set_code(KeyCodes::KeyboardZz, 13, 3);
    keys.set_code(KeyCodes::KeyboardXx, 14, 3);
    keys.set_code(KeyCodes::KeyboardCc, 15, 3);
    keys.set_code(KeyCodes::KeyboardVv, 16, 3);
    keys.set_code(KeyCodes::KeyboardBb, 17, 3);

    keys.set_code(KeyCodes::KeyboardLeftGUI, 18, 3);
    keys.set_code(KeyCodes::Layer4, 19, 3);
    keys.set_code(KeyCodes::KeyboardSpacebar, 20, 3);

    let func = |x: u64| -> u64 { ((10000 * x.pow(2)) / (x.pow(2) + 50000)) + 1000 };
    keys.set_interval(
        KeyCodes::MouseScrollUp,
        Duration::from_millis(SCROLL_TIME),
        func,
        21,
        3,
    );
    keys.set_interval(
        KeyCodes::MouseNegativeY,
        Duration::from_millis(MOUSE_POINTER_TIME),
        func,
        23,
        3,
    );

    keys.set_interval(
        KeyCodes::MouseScrollDown,
        Duration::from_millis(SCROLL_TIME),
        func,
        27,
        3,
    );
    keys.set_interval(
        KeyCodes::MouseNegativeX,
        Duration::from_millis(MOUSE_POINTER_TIME),
        func,
        28,
        3,
    );
    keys.set_interval(
        KeyCodes::MousePositiveY,
        Duration::from_millis(MOUSE_POINTER_TIME),
        func,
        29,
        3,
    );
    keys.set_interval(
        KeyCodes::MousePositiveX,
        Duration::from_millis(MOUSE_POINTER_TIME),
        func,
        30,
        3,
    );

    keys.set_code(KeyCodes::MouseLeftClick, 39, 3);
    keys.set_code(KeyCodes::MouseRightClick, 40, 3);

    // Layer 4
    keys.set_code(KeyCodes::KeyboardLeftControl, 6, 4);
    keys.set_code(KeyCodes::Keyboard1Exclamation, 7, 4);
    keys.set_code(KeyCodes::Keyboard2At, 8, 4);
    keys.set_code(KeyCodes::Keyboard3Hash, 9, 4);
    keys.set_code(KeyCodes::Keyboard4Dollar, 10, 4);
    keys.set_code(KeyCodes::Keyboard5Percent, 11, 4);

    keys.set_interval(
        KeyCodes::MouseScrollUp,
        Duration::from_millis(SCROLL_TIME),
        func,
        21,
        4,
    );
    keys.set_interval(
        KeyCodes::MouseNegativeY,
        Duration::from_millis(MOUSE_POINTER_TIME),
        func,
        23,
        4,
    );

    keys.set_interval(
        KeyCodes::MouseScrollDown,
        Duration::from_millis(SCROLL_TIME),
        func,
        27,
        4,
    );
    keys.set_interval(
        KeyCodes::MouseNegativeX,
        Duration::from_millis(MOUSE_POINTER_TIME),
        func,
        28,
        4,
    );
    keys.set_interval(
        KeyCodes::MousePositiveY,
        Duration::from_millis(SCROLL_TIME),
        func,
        29,
        4,
    );
    keys.set_interval(
        KeyCodes::MousePositiveX,
        Duration::from_millis(MOUSE_POINTER_TIME),
        func,
        30,
        4,
    );
    keys.set_code(KeyCodes::MouseMiddleClick, 31, 4);

    keys.set_toggle_layer(KeyCodes::Layer0, 38, 4);
    keys.set_code(KeyCodes::MouseLeftClick, 39, 4);
    keys.set_code(KeyCodes::MouseRightClick, 40, 4);

    keys.set_code(KeyCodes::KeyboardAa, 0, 6);
    keys.set_slave(21..42);

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
    let mut new_ary = [0usize; 21 as usize];
    for i in 0..ary.len() {
        for j in 0..ary.len() {
            if ary[j as usize] == i {
                new_ary[i as usize] = j;
            }
        }
    }
    ary.copy_from_slice(&new_ary);
}

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
