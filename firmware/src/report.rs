use embassy_time::Instant;
use heapless::{FnvIndexSet, Vec};

use crate::{
    descriptor::{KeyboardReportNKRO, MouseReport},
    keys::{Keys, ModCombo, ScanCode},
};

fn set_bit(num: &mut u8, bit: u8, pos: u8) {
    let mask = 1 << pos;
    if bit == 1 {
        *num |= mask
    } else {
        *num &= !mask
    }
}

enum State {
    Stick(u8),
    Pressed,
    None,
}

pub struct Report {
    key_report: KeyboardReportNKRO,
    mouse_report: MouseReport,
    current_layer: usize,
    reset_layer: usize,
    stick: State,
}

impl Report {
    pub fn default() -> Self {
        Self {
            key_report: KeyboardReportNKRO::default(),
            mouse_report: MouseReport::default(),
            current_layer: 0,
            reset_layer: 0,
            stick: State::None,
        }
    }

    /// Generates a report with the provided keys. Returns a option tuple
    /// where it returns a Some when a report need to be sent
    pub fn generate_report<const S: usize>(
        &mut self,
        keys: &mut Keys<S>,
    ) -> (Option<&KeyboardReportNKRO>, Option<&MouseReport>) {
        let mut new_layer = None;
        let mut pressed_keys = Vec::<ScanCode, 64>::new();
        let mut new_key_report = KeyboardReportNKRO::default();
        let mut new_mouse_report = MouseReport::default();
        let mut pressed = false;
        let mut stick = false;

        keys.get_keys(self.current_layer, &mut pressed_keys);
        for key in &pressed_keys {
            match key {
                ScanCode::Modifier(code) => {
                    let b_idx = code % 8;
                    set_bit(&mut new_key_report.modifier, 1, b_idx);
                }
                ScanCode::Letter(code) => {
                    let n_idx = (code / 8) as usize;
                    let b_idx = code % 8;
                    set_bit(&mut new_key_report.nkro_keycodes[n_idx], 1, b_idx);
                    pressed = true;
                }
                ScanCode::MouseButton(code) => {
                    let b_idx = code % 8;
                    set_bit(&mut new_mouse_report.buttons, 1, b_idx);
                }
                ScanCode::MouseX(code) => {
                    new_mouse_report.x += code;
                }
                ScanCode::MouseY(code) => {
                    new_mouse_report.y += code;
                }
                ScanCode::Scroll(code) => {
                    new_mouse_report.wheel += code;
                }
                ScanCode::Layer(layer) => match new_layer {
                    Some(_) => {
                        if layer.toggle {
                            new_layer = Some(layer);
                        }
                    }
                    None => {
                        new_layer = Some(layer);
                    }
                },
                ScanCode::Sticky => {
                    stick = true;
                }
                ScanCode::None => {}
            };
        }
        if stick {
            if pressed {
                match self.stick {
                    State::Stick(_) => {
                        self.stick = State::Pressed;
                    }
                    State::Pressed => {}
                    State::None => {
                        self.stick = State::Pressed;
                    }
                }
            } else {
                match self.stick {
                    State::Stick(_) => {
                        if new_key_report.modifier != 0 {
                            self.stick = State::Stick(new_key_report.modifier)
                        }
                    }
                    State::Pressed => {}
                    State::None => {
                        if new_key_report.modifier != 0 {
                            self.stick = State::Stick(new_key_report.modifier)
                        } else {
                            self.stick = State::None;
                        }
                    }
                }
            }
        } else {
            match self.stick {
                State::Stick(val) => {
                    if pressed {
                        new_key_report.modifier = val;
                        self.stick = State::None;
                    }
                }
                State::Pressed => {
                    self.stick = State::None;
                }
                State::None => {}
            }
        }

        match new_layer {
            Some(layer) => {
                if layer.toggle {
                    self.reset_layer = layer.pos;
                }
                self.current_layer = layer.pos;
            }
            None => {
                self.current_layer = self.reset_layer;
            }
        }
        let mut returned_report = (None, None);
        if self.key_report != new_key_report {
            self.key_report = new_key_report;
            returned_report.0 = Some(&self.key_report);
        }

        if self.mouse_report.buttons != new_mouse_report.buttons
            || new_mouse_report.x != 0
            || new_mouse_report.y != 0
            || new_mouse_report.wheel != 0
        {
            self.mouse_report = new_mouse_report;
            returned_report.1 = Some(&self.mouse_report);
        }
        returned_report
    }
}
