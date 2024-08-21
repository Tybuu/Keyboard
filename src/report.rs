use heapless::FnvIndexSet;

use crate::{
    descriptor::{KeyboardReportNKRO, MouseReport},
    keys::{Keys, ScanCode},
};
pub const NUM_KEYS: usize = 42;

fn set_bit(num: &mut u8, bit: u8, pos: u8) {
    let mask = 1 << pos;
    if bit == 1 {
        *num |= mask
    } else {
        *num &= !mask
    }
}

pub struct Report {
    key_report: KeyboardReportNKRO,
    mouse_report: MouseReport,
    current_layer: usize,
    reset_layer: usize,
}

impl Report {
    pub fn default() -> Self {
        Self {
            key_report: KeyboardReportNKRO::default(),
            mouse_report: MouseReport::default(),
            current_layer: 0,
            reset_layer: 0,
        }
    }

    /// Generates a report with the provided keys. Returns a option tuple
    /// where it returns a Some when a report need to be sent
    pub fn generate_report(
        &mut self,
        keys: &mut Keys<NUM_KEYS>,
    ) -> (Option<KeyboardReportNKRO>, Option<MouseReport>) {
        match keys.get_layer(self.current_layer) {
            Some(layer) => {
                if layer.toggle {
                    self.reset_layer = layer.pos;
                }
                self.current_layer = layer.pos
            }
            None => self.current_layer = self.reset_layer,
        }

        let mut pressed_keys = FnvIndexSet::<ScanCode, 64>::new();

        let mut new_key_report = KeyboardReportNKRO::default();
        let mut new_mouse_report = MouseReport::default();

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
                _ => {}
            };
        }
        let mut returned_report = (None, None);
        if self.key_report != new_key_report {
            self.key_report = new_key_report;
            returned_report.0 = Some(self.key_report);
        }
        // Second bool condtion is needed as the mouse report is relative.
        // If a key is held, we need to constantly send reports to represent
        // that state to the host
        if self.mouse_report != new_mouse_report || new_mouse_report != MouseReport::default() {
            self.mouse_report = new_mouse_report;
            returned_report.1 = Some(new_mouse_report);
        }
        returned_report
    }
}
