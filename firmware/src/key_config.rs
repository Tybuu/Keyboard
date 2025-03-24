use embassy_rp::rom_data::reset_to_usb_boot;
use embassy_time::Duration;

use crate::{codes::KeyCodes, keys::Keys};

const SCROLL_TIME: u64 = 500;
const MOUSE_POINTER_TIME: u64 = 5;

/// This function initalizes a Keys struct
pub fn load_key_config<const S: usize>(keys: &mut Keys<S>) {
    *keys = Keys::<S>::default();
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
    keys.set_code(KeyCodes::Layer1, 19, 0);
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
    keys.set_code(KeyCodes::Layer2, 40, 0);
    keys.set_code(KeyCodes::KeyboardLeftAlt, 41, 0);

    // Layer 1
    keys.set_code(KeyCodes::KeyboardTab, 0, 1);
    keys.set_double(KeyCodes::Keyboard4Dollar, KeyCodes::KeyboardLeftShift, 1, 1);
    keys.set_code(KeyCodes::KeyboardOpenBracketBrace, 2, 1);
    keys.set_double(
        KeyCodes::KeyboardOpenBracketBrace,
        KeyCodes::KeyboardLeftShift,
        3,
        1,
    );
    keys.set_double(
        KeyCodes::Keyboard9OpenParens,
        KeyCodes::KeyboardLeftShift,
        4,
        1,
    );
    keys.set_double(
        KeyCodes::KeyboardBacktickTilde,
        KeyCodes::KeyboardLeftShift,
        5,
        1,
    );

    keys.set_code(KeyCodes::KeyboardLeftControl, 6, 1);
    keys.set_code(KeyCodes::Keyboard1Exclamation, 7, 1);
    keys.set_code(KeyCodes::Keyboard2At, 8, 1);
    keys.set_code(KeyCodes::Keyboard3Hash, 9, 1);
    keys.set_code(KeyCodes::Keyboard4Dollar, 10, 1);
    keys.set_code(KeyCodes::Keyboard5Percent, 11, 1);

    keys.set_code(KeyCodes::KeyboardLeftShift, 12, 1);
    keys.set_double(
        KeyCodes::KeyboardEqualPlus,
        KeyCodes::KeyboardLeftShift,
        13,
        1,
    );
    keys.set_code(KeyCodes::KeyboardDashUnderscore, 14, 1);
    keys.set_double(
        KeyCodes::Keyboard8Asterisk,
        KeyCodes::KeyboardLeftShift,
        15,
        1,
    );
    keys.set_code(KeyCodes::KeyboardEqualPlus, 16, 1);
    keys.set_double(
        KeyCodes::Keyboard5Percent,
        KeyCodes::KeyboardLeftShift,
        17,
        1,
    );

    keys.set_code(KeyCodes::KeyboardLeftGUI, 18, 1);
    keys.set_code(KeyCodes::Layer1, 19, 1);
    keys.set_code(KeyCodes::KeyboardSpacebar, 20, 1);

    keys.set_double(KeyCodes::Keyboard6Caret, KeyCodes::KeyboardLeftShift, 21, 1);
    keys.set_double(
        KeyCodes::Keyboard0CloseParens,
        KeyCodes::KeyboardLeftShift,
        22,
        1,
    );
    keys.set_double(
        KeyCodes::KeyboardCloseBracketBrace,
        KeyCodes::KeyboardLeftShift,
        23,
        1,
    );
    keys.set_code(KeyCodes::KeyboardCloseBracketBrace, 24, 1);
    keys.set_code(KeyCodes::KeyboardBacktickTilde, 25, 1);
    keys.set_code(KeyCodes::KeyboardBackspace, 26, 1);

    keys.set_code(KeyCodes::Keyboard6Caret, 27, 1);
    keys.set_code(KeyCodes::Keyboard7Ampersand, 28, 1);
    keys.set_code(KeyCodes::Keyboard8Asterisk, 29, 1);
    keys.set_code(KeyCodes::Keyboard9OpenParens, 30, 1);
    keys.set_code(KeyCodes::Keyboard0CloseParens, 31, 1);

    keys.set_double(
        KeyCodes::KeyboardBackslashBar,
        KeyCodes::KeyboardLeftShift,
        33,
        1,
    );
    keys.set_code(KeyCodes::KeyboardBackslashBar, 34, 1);
    keys.set_double(
        KeyCodes::Keyboard7Ampersand,
        KeyCodes::KeyboardLeftShift,
        35,
        1,
    );
    keys.set_double(
        KeyCodes::KeyboardDashUnderscore,
        KeyCodes::KeyboardLeftShift,
        36,
        1,
    );
    keys.set_double(
        KeyCodes::Keyboard1Exclamation,
        KeyCodes::KeyboardLeftShift,
        37,
        1,
    );

    keys.set_code(KeyCodes::KeyboardEnter, 39, 1);
    keys.set_code(KeyCodes::Layer2, 40, 1);

    // Layer 2
    keys.set_code(KeyCodes::KeyboardF1, 1, 2);
    keys.set_code(KeyCodes::KeyboardF2, 2, 2);
    keys.set_code(KeyCodes::KeyboardF3, 3, 2);
    keys.set_code(KeyCodes::KeyboardF4, 4, 2);
    keys.set_code(KeyCodes::KeyboardF5, 5, 2);

    keys.set_code(KeyCodes::Layer1, 19, 2);

    keys.set_code(KeyCodes::KeyboardF6, 21, 2);
    keys.set_code(KeyCodes::KeyboardF7, 22, 2);
    keys.set_code(KeyCodes::KeyboardF8, 23, 2);
    keys.set_code(KeyCodes::KeyboardF9, 24, 2);
    keys.set_code(KeyCodes::KeyboardF10, 25, 2);
    keys.set_config(load_callum, 26, 2);

    keys.set_code(KeyCodes::KeyboardLeftArrow, 27, 2);
    keys.set_code(KeyCodes::KeyboardDownArrow, 28, 2);
    keys.set_code(KeyCodes::KeyboardUpArrow, 29, 2);
    keys.set_code(KeyCodes::KeyboardRightArrow, 30, 2);

    keys.set_toggle_layer(KeyCodes::Layer3, 38, 2);

    keys.set_code(KeyCodes::KeyboardEnter, 39, 2);
    keys.set_code(KeyCodes::Layer2, 40, 2);

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

    keys.set_code(KeyCodes::Layer4, 19, 4);

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
    keys.set_toggle_layer(KeyCodes::Layer0, 41, 3);

    keys.set_slave(21..42);

    keys.set_reverse(false, 0);
    keys.set_reverse(false, 6);
    keys.set_reverse(false, 12);
}

pub fn load_callum<const S: usize>(keys: &mut Keys<S>) {
    *keys = Keys::<S>::default();
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

    keys.set_code(KeyCodes::Layer4, 18, 0);
    keys.set_combined(KeyCodes::Layer1, KeyCodes::Layer3, 40, 19, 0);
    keys.set_code(KeyCodes::KeyboardSpacebar, 20, 0);

    keys.set_code(KeyCodes::KeyboardYy, 21, 0);
    keys.set_code(KeyCodes::KeyboardUu, 22, 0);
    keys.set_code(KeyCodes::KeyboardIi, 23, 0);
    keys.set_code(KeyCodes::KeyboardOo, 24, 0);
    keys.set_code(KeyCodes::KeyboardPp, 25, 0);

    keys.set_code(KeyCodes::KeyboardHh, 27, 0);
    keys.set_code(KeyCodes::KeyboardJj, 28, 0);
    keys.set_code(KeyCodes::KeyboardKk, 29, 0);
    keys.set_code(KeyCodes::KeyboardLl, 30, 0);
    keys.set_code(KeyCodes::KeyboardSemiColon, 31, 0);

    keys.set_code(KeyCodes::KeyboardNn, 33, 0);
    keys.set_code(KeyCodes::KeyboardMm, 34, 0);
    keys.set_code(KeyCodes::KeyboardCommaLess, 35, 0);
    keys.set_code(KeyCodes::KeyboardPeriodGreater, 36, 0);
    keys.set_code(KeyCodes::KeyboardSlashQuestion, 37, 0);

    keys.set_code(KeyCodes::KeyboardLeftShift, 39, 0);
    keys.set_combined(KeyCodes::Layer2, KeyCodes::Layer3, 19, 40, 0);
    keys.set_code(KeyCodes::KeyboardRightControl, 41, 0);

    // Layer 1
    keys.set_config(bios_config, 0, 1);
    keys.set_code(KeyCodes::KeyboardTab, 1, 1);
    // keys.set_code(KeyCodes::KeyboardWw, 2, 1);
    // keys.set_code(KeyCodes::KeyboardEe, 3, 1);
    // keys.set_code(KeyCodes::KeyboardRr, 4, 1);
    keys.set_code(KeyCodes::KeyboardVolumeUp, 5, 1);

    keys.set_code(KeyCodes::KeyboardLeftShift, 7, 1);
    keys.set_code(KeyCodes::KeyboardLeftControl, 8, 1);
    keys.set_code(KeyCodes::KeyboardLeftAlt, 9, 1);
    keys.set_code(KeyCodes::KeyboardLeftGUI, 10, 1);
    keys.set_code(KeyCodes::KeyboardVolumeDown, 11, 1);

    let func = |x: u64| -> u64 { ((10000 * x.pow(2)) / (x.pow(2) + 50000)) + 1000 };
    keys.set_interval(
        KeyCodes::MouseScrollDown,
        Duration::from_millis(SCROLL_TIME),
        func,
        13,
        1,
    );
    keys.set_interval(
        KeyCodes::MouseScrollUp,
        Duration::from_millis(SCROLL_TIME),
        func,
        14,
        1,
    );
    keys.set_code(KeyCodes::MouseLeftClick, 15, 1);
    keys.set_code(KeyCodes::MouseMiddleClick, 16, 1);
    keys.set_code(KeyCodes::MouseRightClick, 17, 1);

    keys.set_combined(KeyCodes::Layer1, KeyCodes::Layer3, 40, 19, 1);
    keys.set_code(KeyCodes::KeyboardSpacebar, 20, 1);

    keys.set_code(KeyCodes::KeyboardCapsLock, 21, 1);
    // keys.set_code(KeyCodes::KeyboardUu, 22, 1);
    // keys.set_code(KeyCodes::KeyboardIi, 23, 1);
    // keys.set_code(KeyCodes::KeyboardOo, 24, 1);
    keys.set_code(KeyCodes::KeyboardDelete, 25, 1);

    keys.set_code(KeyCodes::KeyboardLeftArrow, 27, 1);
    keys.set_code(KeyCodes::KeyboardDownArrow, 28, 1);
    keys.set_code(KeyCodes::KeyboardUpArrow, 29, 1);
    keys.set_code(KeyCodes::KeyboardRightArrow, 30, 1);
    keys.set_code(KeyCodes::KeyboardBackspace, 31, 1);

    // keys.set_code(KeyCodes::KeyboardNn, 33, 1);
    // keys.set_toggle_layer(KeyCodes::Layer4, 33, 1);
    keys.set_interval(
        KeyCodes::MouseNegativeX,
        Duration::from_millis(MOUSE_POINTER_TIME),
        func,
        33,
        1,
    );
    keys.set_interval(
        KeyCodes::MousePositiveY,
        Duration::from_millis(MOUSE_POINTER_TIME),
        func,
        34,
        1,
    );
    keys.set_interval(
        KeyCodes::MouseNegativeY,
        Duration::from_millis(MOUSE_POINTER_TIME),
        func,
        35,
        1,
    );
    keys.set_interval(
        KeyCodes::MousePositiveX,
        Duration::from_millis(MOUSE_POINTER_TIME),
        func,
        36,
        1,
    );
    keys.set_code(KeyCodes::KeyboardEnter, 37, 1);

    keys.set_code(KeyCodes::KeyboardLeftShift, 39, 1);
    keys.set_combined(KeyCodes::Layer2, KeyCodes::Layer3, 19, 40, 1);

    // Layer 2
    keys.set_code(KeyCodes::KeyboardEscape, 1, 2);
    keys.set_code(KeyCodes::KeyboardOpenBracketBrace, 2, 2);
    keys.set_double(
        KeyCodes::KeyboardOpenBracketBrace,
        KeyCodes::KeyboardLeftShift,
        3,
        2,
    );
    keys.set_double(
        KeyCodes::Keyboard9OpenParens,
        KeyCodes::KeyboardLeftShift,
        4,
        2,
    );
    keys.set_double(
        KeyCodes::KeyboardBacktickTilde,
        KeyCodes::KeyboardLeftShift,
        5,
        2,
    );

    keys.set_code(KeyCodes::KeyboardDashUnderscore, 7, 2);
    keys.set_double(
        KeyCodes::Keyboard8Asterisk,
        KeyCodes::KeyboardLeftShift,
        8,
        2,
    );
    keys.set_code(KeyCodes::KeyboardEqualPlus, 9, 2);
    keys.set_double(
        KeyCodes::KeyboardDashUnderscore,
        KeyCodes::KeyboardLeftShift,
        10,
        2,
    );
    keys.set_double(
        KeyCodes::Keyboard4Dollar,
        KeyCodes::KeyboardLeftShift,
        11,
        2,
    );

    keys.set_double(
        KeyCodes::KeyboardEqualPlus,
        KeyCodes::KeyboardLeftShift,
        13,
        2,
    );
    keys.set_double(
        KeyCodes::KeyboardBackslashBar,
        KeyCodes::KeyboardLeftShift,
        14,
        2,
    );
    keys.set_double(KeyCodes::Keyboard2At, KeyCodes::KeyboardLeftShift, 15, 2);
    keys.set_code(KeyCodes::KeyboardSingleDoubleQuote, 16, 2);
    keys.set_double(
        KeyCodes::Keyboard5Percent,
        KeyCodes::KeyboardLeftShift,
        17,
        2,
    );

    keys.set_combined(KeyCodes::Layer1, KeyCodes::Layer3, 40, 19, 2);
    keys.set_code(KeyCodes::KeyboardSpacebar, 20, 2);

    keys.set_double(KeyCodes::Keyboard6Caret, KeyCodes::KeyboardLeftShift, 21, 2);
    keys.set_double(
        KeyCodes::Keyboard0CloseParens,
        KeyCodes::KeyboardLeftShift,
        22,
        2,
    );
    keys.set_double(
        KeyCodes::KeyboardCloseBracketBrace,
        KeyCodes::KeyboardLeftShift,
        23,
        2,
    );
    keys.set_code(KeyCodes::KeyboardCloseBracketBrace, 24, 2);
    keys.set_code(KeyCodes::KeyboardBacktickTilde, 25, 2);

    keys.set_double(KeyCodes::Keyboard3Hash, KeyCodes::KeyboardLeftShift, 27, 2);
    keys.set_code(KeyCodes::KeyboardRightGUI, 28, 2);
    keys.set_code(KeyCodes::KeyboardRightAlt, 29, 2);
    keys.set_code(KeyCodes::KeyboardRightControl, 30, 2);
    keys.set_code(KeyCodes::KeyboardRightShift, 31, 2);

    // keys.set_code(KeyCodes::KeyboardBackslashBar, 33, 2);
    keys.set_code(KeyCodes::KeyboardBackslashBar, 34, 2);
    keys.set_double(
        KeyCodes::Keyboard7Ampersand,
        KeyCodes::KeyboardLeftShift,
        35,
        2,
    );
    keys.set_double(
        KeyCodes::KeyboardSingleDoubleQuote,
        KeyCodes::KeyboardLeftShift,
        36,
        2,
    );
    keys.set_double(
        KeyCodes::Keyboard1Exclamation,
        KeyCodes::KeyboardLeftShift,
        37,
        2,
    );

    keys.set_code(KeyCodes::KeyboardLeftShift, 39, 2);
    keys.set_combined(KeyCodes::Layer2, KeyCodes::Layer3, 19, 40, 2);

    // Layer 3
    keys.set_code(KeyCodes::Keyboard1Exclamation, 1, 3);
    keys.set_code(KeyCodes::Keyboard2At, 2, 3);
    keys.set_code(KeyCodes::Keyboard3Hash, 3, 3);
    keys.set_code(KeyCodes::Keyboard4Dollar, 4, 3);
    keys.set_code(KeyCodes::Keyboard5Percent, 5, 3);

    keys.set_code(KeyCodes::KeyboardLeftShift, 7, 3);
    keys.set_code(KeyCodes::KeyboardLeftControl, 8, 3);
    keys.set_code(KeyCodes::KeyboardLeftAlt, 9, 3);
    keys.set_code(KeyCodes::KeyboardLeftGUI, 10, 3);
    keys.set_code(KeyCodes::KeyboardF11, 11, 3);

    keys.set_code(KeyCodes::KeyboardF1, 13, 3);
    keys.set_code(KeyCodes::KeyboardF2, 14, 3);
    keys.set_code(KeyCodes::KeyboardF3, 15, 3);
    keys.set_code(KeyCodes::KeyboardF4, 16, 3);
    keys.set_code(KeyCodes::KeyboardF5, 17, 3);

    keys.set_combined(KeyCodes::Layer1, KeyCodes::Layer3, 40, 19, 3);
    keys.set_code(KeyCodes::KeyboardSpacebar, 20, 3);

    keys.set_code(KeyCodes::Keyboard6Caret, 21, 3);
    keys.set_code(KeyCodes::Keyboard7Ampersand, 22, 3);
    keys.set_code(KeyCodes::Keyboard8Asterisk, 23, 3);
    keys.set_code(KeyCodes::Keyboard9OpenParens, 24, 3);
    keys.set_code(KeyCodes::Keyboard0CloseParens, 25, 3);

    keys.set_code(KeyCodes::KeyboardF12, 27, 3);
    keys.set_code(KeyCodes::KeyboardRightGUI, 28, 3);
    keys.set_code(KeyCodes::KeyboardRightAlt, 29, 3);
    keys.set_code(KeyCodes::KeyboardRightControl, 30, 3);
    keys.set_code(KeyCodes::KeyboardRightShift, 31, 3);

    keys.set_code(KeyCodes::KeyboardF6, 33, 3);
    keys.set_code(KeyCodes::KeyboardF7, 34, 3);
    keys.set_code(KeyCodes::KeyboardF8, 35, 3);
    keys.set_code(KeyCodes::KeyboardF9, 36, 3);
    keys.set_code(KeyCodes::KeyboardF10, 37, 3);
    keys.set_config(load_key_config, 38, 3);

    keys.set_code(KeyCodes::KeyboardLeftShift, 39, 3);
    keys.set_combined(KeyCodes::Layer2, KeyCodes::Layer3, 19, 40, 3);

    keys.set_code(KeyCodes::KeyboardTab, 1, 4);

    keys.set_code(KeyCodes::KeyboardLeftGUI, 6, 4);
    keys.set_code(KeyCodes::Keyboard1Exclamation, 7, 4);
    keys.set_code(KeyCodes::Keyboard2At, 8, 4);
    keys.set_code(KeyCodes::Keyboard3Hash, 9, 4);
    keys.set_code(KeyCodes::Keyboard4Dollar, 10, 4);
    keys.set_code(KeyCodes::Keyboard5Percent, 11, 4);

    keys.set_code(KeyCodes::KeyboardLeftShift, 12, 4);

    keys.set_slave(21..42);
    keys.set_reverse(false, 0);
    keys.set_reverse(false, 6);
    keys.set_reverse(false, 12);
}

pub fn bios_config<const S: usize>(keys: &mut Keys<S>) {
    *keys = Keys::<S>::default();
    keys.set_code(KeyCodes::KeyboardEscape, 0, 0);
    keys.set_code(KeyCodes::KeyboardQq, 1, 0);
    keys.set_code(KeyCodes::KeyboardWw, 2, 0);
    keys.set_code(KeyCodes::KeyboardUpArrow, 3, 0);
    keys.set_code(KeyCodes::KeyboardRr, 4, 0);
    keys.set_code(KeyCodes::KeyboardTt, 5, 0);

    keys.set_code(KeyCodes::KeyboardAa, 7, 0);
    keys.set_code(KeyCodes::KeyboardLeftArrow, 8, 0);
    keys.set_code(KeyCodes::KeyboardDownArrow, 9, 0);
    keys.set_code(KeyCodes::KeyboardRightArrow, 10, 0);
    keys.set_code(KeyCodes::KeyboardGg, 11, 0);

    keys.set_config(load_callum, 13, 0);
    keys.set_function(
        || {
            reset_to_usb_boot(0, 0);
        },
        14,
        0,
    );
    keys.set_code(KeyCodes::KeyboardCc, 15, 0);
    keys.set_code(KeyCodes::KeyboardTab, 16, 0);
    keys.set_code(KeyCodes::KeyboardEnter, 17, 0);

    keys.set_code(KeyCodes::KeyboardBackspace, 18, 0);
    keys.set_code(KeyCodes::Layer1, 19, 0);
    keys.set_code(KeyCodes::KeyboardSpacebar, 20, 0);

    keys.set_code(KeyCodes::KeyboardF1, 1, 1);
    keys.set_code(KeyCodes::KeyboardF2, 2, 1);
    keys.set_code(KeyCodes::KeyboardF3, 3, 1);
    keys.set_code(KeyCodes::KeyboardF4, 4, 1);
    keys.set_code(KeyCodes::KeyboardF5, 5, 1);

    keys.set_code(KeyCodes::Layer1, 6, 1);
    keys.set_code(KeyCodes::KeyboardF6, 7, 1);
    keys.set_code(KeyCodes::KeyboardF7, 8, 1);
    keys.set_code(KeyCodes::KeyboardF8, 9, 1);
    keys.set_code(KeyCodes::KeyboardF9, 10, 1);
    keys.set_code(KeyCodes::KeyboardF10, 11, 1);

    keys.set_code(KeyCodes::KeyboardF11, 13, 1);
    keys.set_code(KeyCodes::KeyboardF12, 14, 1);

    // Layer 4

    keys.set_slave(21..42);
    keys.set_reverse(false, 0);
    keys.set_reverse(false, 6);
    keys.set_reverse(false, 12);
}

pub fn load_new_layout<const S: usize>(keys: &mut Keys<S>) {
    *keys = Keys::<S>::default();
    // Layer 0
    keys.set_code(KeyCodes::KeyboardEscape, 0, 0);
    keys.set_code(KeyCodes::KeyboardQq, 1, 0);
    keys.set_code(KeyCodes::KeyboardWw, 2, 0);
    keys.set_code(KeyCodes::KeyboardFf, 3, 0);
    keys.set_code(KeyCodes::KeyboardPp, 4, 0);
    keys.set_code(KeyCodes::KeyboardBb, 5, 0);

    keys.set_code(KeyCodes::KeyboardLeftControl, 6, 0);
    keys.set_code(KeyCodes::KeyboardAa, 7, 0);
    keys.set_code(KeyCodes::KeyboardRr, 8, 0);
    keys.set_code(KeyCodes::KeyboardSs, 9, 0);
    keys.set_code(KeyCodes::KeyboardTt, 10, 0);
    keys.set_code(KeyCodes::KeyboardGg, 11, 0);

    keys.set_code(KeyCodes::KeyboardLeftShift, 12, 0);
    keys.set_code(KeyCodes::KeyboardZz, 13, 0);
    keys.set_code(KeyCodes::KeyboardXx, 14, 0);
    keys.set_code(KeyCodes::KeyboardCc, 15, 0);
    keys.set_code(KeyCodes::KeyboardDd, 16, 0);
    keys.set_code(KeyCodes::KeyboardVv, 17, 0);

    keys.set_code(KeyCodes::Layer4, 18, 0);
    keys.set_combined(KeyCodes::Layer1, KeyCodes::Layer3, 40, 19, 0);
    keys.set_code(KeyCodes::KeyboardSpacebar, 20, 0);

    keys.set_code(KeyCodes::KeyboardJj, 21, 0);
    keys.set_code(KeyCodes::KeyboardLl, 22, 0);
    keys.set_code(KeyCodes::KeyboardUu, 23, 0);
    keys.set_code(KeyCodes::KeyboardYy, 24, 0);
    keys.set_code(KeyCodes::KeyboardSemiColon, 25, 0);

    keys.set_code(KeyCodes::KeyboardMm, 27, 0);
    keys.set_code(KeyCodes::KeyboardNn, 28, 0);
    keys.set_code(KeyCodes::KeyboardEe, 29, 0);
    keys.set_code(KeyCodes::KeyboardIi, 30, 0);
    keys.set_code(KeyCodes::KeyboardOo, 31, 0);

    keys.set_code(KeyCodes::KeyboardKk, 33, 0);
    keys.set_code(KeyCodes::KeyboardHh, 34, 0);
    keys.set_code(KeyCodes::KeyboardCommaLess, 35, 0);
    keys.set_code(KeyCodes::KeyboardPeriodGreater, 36, 0);
    keys.set_code(KeyCodes::KeyboardSlashQuestion, 37, 0);

    keys.set_code(KeyCodes::KeyboardLeftShift, 39, 0);
    keys.set_combined(KeyCodes::Layer2, KeyCodes::Layer3, 19, 40, 0);
    keys.set_code(KeyCodes::KeyboardRightControl, 41, 0);

    // Layer 1
    keys.set_config(bios_config, 0, 1);
    keys.set_code(KeyCodes::KeyboardTab, 1, 1);
    // keys.set_code(KeyCodes::KeyboardWw, 2, 1);
    // keys.set_code(KeyCodes::KeyboardEe, 3, 1);
    // keys.set_code(KeyCodes::KeyboardRr, 4, 1);
    keys.set_code(KeyCodes::KeyboardVolumeUp, 5, 1);

    keys.set_code(KeyCodes::KeyboardLeftShift, 7, 1);
    keys.set_code(KeyCodes::KeyboardLeftControl, 8, 1);
    keys.set_code(KeyCodes::KeyboardLeftAlt, 9, 1);
    keys.set_code(KeyCodes::KeyboardLeftGUI, 10, 1);
    keys.set_code(KeyCodes::KeyboardVolumeDown, 11, 1);

    let func = |x: u64| -> u64 { ((10000 * x.pow(2)) / (x.pow(2) + 50000)) + 1000 };
    keys.set_interval(
        KeyCodes::MouseScrollDown,
        Duration::from_millis(SCROLL_TIME),
        func,
        13,
        1,
    );
    keys.set_interval(
        KeyCodes::MouseScrollUp,
        Duration::from_millis(SCROLL_TIME),
        func,
        14,
        1,
    );
    keys.set_code(KeyCodes::MouseLeftClick, 15, 1);
    keys.set_code(KeyCodes::MouseMiddleClick, 16, 1);
    keys.set_code(KeyCodes::MouseRightClick, 17, 1);

    keys.set_combined(KeyCodes::Layer1, KeyCodes::Layer3, 40, 19, 1);
    keys.set_code(KeyCodes::KeyboardSpacebar, 20, 1);

    keys.set_code(KeyCodes::KeyboardCapsLock, 21, 1);
    // keys.set_code(KeyCodes::KeyboardUu, 22, 1);
    // keys.set_code(KeyCodes::KeyboardIi, 23, 1);
    // keys.set_code(KeyCodes::KeyboardOo, 24, 1);
    keys.set_code(KeyCodes::KeyboardDelete, 25, 1);

    keys.set_code(KeyCodes::KeyboardLeftArrow, 27, 1);
    keys.set_code(KeyCodes::KeyboardDownArrow, 28, 1);
    keys.set_code(KeyCodes::KeyboardUpArrow, 29, 1);
    keys.set_code(KeyCodes::KeyboardRightArrow, 30, 1);
    keys.set_code(KeyCodes::KeyboardBackspace, 31, 1);

    // keys.set_code(KeyCodes::KeyboardNn, 33, 1);
    // keys.set_toggle_layer(KeyCodes::Layer4, 33, 1);
    keys.set_interval(
        KeyCodes::MouseNegativeX,
        Duration::from_millis(MOUSE_POINTER_TIME),
        func,
        33,
        1,
    );
    keys.set_interval(
        KeyCodes::MousePositiveY,
        Duration::from_millis(MOUSE_POINTER_TIME),
        func,
        34,
        1,
    );
    keys.set_interval(
        KeyCodes::MouseNegativeY,
        Duration::from_millis(MOUSE_POINTER_TIME),
        func,
        35,
        1,
    );
    keys.set_interval(
        KeyCodes::MousePositiveX,
        Duration::from_millis(MOUSE_POINTER_TIME),
        func,
        36,
        1,
    );
    keys.set_code(KeyCodes::KeyboardEnter, 37, 1);

    keys.set_code(KeyCodes::KeyboardLeftShift, 39, 1);
    keys.set_combined(KeyCodes::Layer2, KeyCodes::Layer3, 19, 40, 1);

    // Layer 2
    keys.set_code(KeyCodes::KeyboardEscape, 1, 2);
    keys.set_code(KeyCodes::KeyboardOpenBracketBrace, 2, 2);
    keys.set_double(
        KeyCodes::KeyboardOpenBracketBrace,
        KeyCodes::KeyboardLeftShift,
        3,
        2,
    );
    keys.set_double(
        KeyCodes::Keyboard9OpenParens,
        KeyCodes::KeyboardLeftShift,
        4,
        2,
    );
    keys.set_double(
        KeyCodes::KeyboardBacktickTilde,
        KeyCodes::KeyboardLeftShift,
        5,
        2,
    );

    keys.set_code(KeyCodes::KeyboardDashUnderscore, 7, 2);
    keys.set_double(
        KeyCodes::Keyboard8Asterisk,
        KeyCodes::KeyboardLeftShift,
        8,
        2,
    );
    keys.set_code(KeyCodes::KeyboardEqualPlus, 9, 2);
    keys.set_double(
        KeyCodes::KeyboardDashUnderscore,
        KeyCodes::KeyboardLeftShift,
        10,
        2,
    );
    keys.set_double(
        KeyCodes::Keyboard4Dollar,
        KeyCodes::KeyboardLeftShift,
        11,
        2,
    );

    keys.set_double(
        KeyCodes::KeyboardEqualPlus,
        KeyCodes::KeyboardLeftShift,
        13,
        2,
    );
    keys.set_double(
        KeyCodes::KeyboardBackslashBar,
        KeyCodes::KeyboardLeftShift,
        14,
        2,
    );
    keys.set_double(KeyCodes::Keyboard2At, KeyCodes::KeyboardLeftShift, 15, 2);
    keys.set_code(KeyCodes::KeyboardSingleDoubleQuote, 16, 2);
    keys.set_double(
        KeyCodes::Keyboard5Percent,
        KeyCodes::KeyboardLeftShift,
        17,
        2,
    );

    keys.set_combined(KeyCodes::Layer1, KeyCodes::Layer3, 40, 19, 2);
    keys.set_code(KeyCodes::KeyboardSpacebar, 20, 2);

    keys.set_double(KeyCodes::Keyboard6Caret, KeyCodes::KeyboardLeftShift, 21, 2);
    keys.set_double(
        KeyCodes::Keyboard0CloseParens,
        KeyCodes::KeyboardLeftShift,
        22,
        2,
    );
    keys.set_double(
        KeyCodes::KeyboardCloseBracketBrace,
        KeyCodes::KeyboardLeftShift,
        23,
        2,
    );
    keys.set_code(KeyCodes::KeyboardCloseBracketBrace, 24, 2);
    keys.set_code(KeyCodes::KeyboardBacktickTilde, 25, 2);

    keys.set_double(KeyCodes::Keyboard3Hash, KeyCodes::KeyboardLeftShift, 27, 2);
    keys.set_code(KeyCodes::KeyboardRightGUI, 28, 2);
    keys.set_code(KeyCodes::KeyboardRightAlt, 29, 2);
    keys.set_code(KeyCodes::KeyboardRightControl, 30, 2);
    keys.set_code(KeyCodes::KeyboardRightShift, 31, 2);

    // keys.set_code(KeyCodes::KeyboardBackslashBar, 33, 2);
    keys.set_code(KeyCodes::KeyboardBackslashBar, 34, 2);
    keys.set_double(
        KeyCodes::Keyboard7Ampersand,
        KeyCodes::KeyboardLeftShift,
        35,
        2,
    );
    keys.set_double(
        KeyCodes::KeyboardSingleDoubleQuote,
        KeyCodes::KeyboardLeftShift,
        36,
        2,
    );
    keys.set_double(
        KeyCodes::Keyboard1Exclamation,
        KeyCodes::KeyboardLeftShift,
        37,
        2,
    );

    keys.set_code(KeyCodes::KeyboardLeftShift, 39, 2);
    keys.set_combined(KeyCodes::Layer2, KeyCodes::Layer3, 19, 40, 2);

    // Layer 3
    keys.set_code(KeyCodes::Keyboard1Exclamation, 1, 3);
    keys.set_code(KeyCodes::Keyboard2At, 2, 3);
    keys.set_code(KeyCodes::Keyboard3Hash, 3, 3);
    keys.set_code(KeyCodes::Keyboard4Dollar, 4, 3);
    keys.set_code(KeyCodes::Keyboard5Percent, 5, 3);

    keys.set_code(KeyCodes::KeyboardLeftShift, 7, 3);
    keys.set_code(KeyCodes::KeyboardLeftControl, 8, 3);
    keys.set_code(KeyCodes::KeyboardLeftAlt, 9, 3);
    keys.set_code(KeyCodes::KeyboardLeftGUI, 10, 3);
    keys.set_code(KeyCodes::KeyboardF11, 11, 3);

    keys.set_code(KeyCodes::KeyboardF1, 13, 3);
    keys.set_code(KeyCodes::KeyboardF2, 14, 3);
    keys.set_code(KeyCodes::KeyboardF3, 15, 3);
    keys.set_code(KeyCodes::KeyboardF4, 16, 3);
    keys.set_code(KeyCodes::KeyboardF5, 17, 3);

    keys.set_combined(KeyCodes::Layer1, KeyCodes::Layer3, 40, 19, 3);
    keys.set_code(KeyCodes::KeyboardSpacebar, 20, 3);

    keys.set_code(KeyCodes::Keyboard6Caret, 21, 3);
    keys.set_code(KeyCodes::Keyboard7Ampersand, 22, 3);
    keys.set_code(KeyCodes::Keyboard8Asterisk, 23, 3);
    keys.set_code(KeyCodes::Keyboard9OpenParens, 24, 3);
    keys.set_code(KeyCodes::Keyboard0CloseParens, 25, 3);

    keys.set_code(KeyCodes::KeyboardF12, 27, 3);
    keys.set_code(KeyCodes::KeyboardRightGUI, 28, 3);
    keys.set_code(KeyCodes::KeyboardRightAlt, 29, 3);
    keys.set_code(KeyCodes::KeyboardRightControl, 30, 3);
    keys.set_code(KeyCodes::KeyboardRightShift, 31, 3);

    keys.set_code(KeyCodes::KeyboardF6, 33, 3);
    keys.set_code(KeyCodes::KeyboardF7, 34, 3);
    keys.set_code(KeyCodes::KeyboardF8, 35, 3);
    keys.set_code(KeyCodes::KeyboardF9, 36, 3);
    keys.set_code(KeyCodes::KeyboardF10, 37, 3);
    keys.set_config(load_key_config, 38, 3);

    keys.set_code(KeyCodes::KeyboardLeftShift, 39, 3);
    keys.set_combined(KeyCodes::Layer2, KeyCodes::Layer3, 19, 40, 3);

    keys.set_code(KeyCodes::KeyboardTab, 1, 4);

    keys.set_code(KeyCodes::KeyboardLeftGUI, 6, 4);
    keys.set_code(KeyCodes::Keyboard1Exclamation, 7, 4);
    keys.set_code(KeyCodes::Keyboard2At, 8, 4);
    keys.set_code(KeyCodes::Keyboard3Hash, 9, 4);
    keys.set_code(KeyCodes::Keyboard4Dollar, 10, 4);
    keys.set_code(KeyCodes::Keyboard5Percent, 11, 4);

    keys.set_code(KeyCodes::KeyboardLeftShift, 12, 4);

    keys.set_slave(21..42);
    keys.set_reverse(false, 0);
    keys.set_reverse(false, 6);
    keys.set_reverse(false, 12);
}
