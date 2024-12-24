use core::{borrow::BorrowMut, ops::Range};

use embassy_time::{Duration, Instant};
use heapless::Vec;

use crate::codes::KeyCodes;

const DEFAULT_RELEASE_SCALE: f32 = 0.30;
const DEFAULT_ACTUATE_SCALE: f32 = 0.35;
const TOLERANCE_SCALE: f32 = 0.1;
const BUFFER_SIZE: usize = 1;
const HOLD_TIME: Duration = Duration::from_millis(150);

const NUM_COMB: usize = 4;
const HOLD_DURATION: Duration = Duration::from_millis(50);

pub const NUM_LAYERS: usize = 10;

pub const DEFAULT_HIGH: u32 = 1700;
pub const DEFAULT_LOW: u32 = 1400;

// Makes hall effect switches act like a normal mechanical switch
#[derive(Copy, Clone, Default, Debug)]
struct DigitalPosition {
    buffer: [u32; BUFFER_SIZE as usize], // Take multiple readings to smooth out buffer
    buffer_pos: usize,
    release_point: u32,
    actuation_point: u32,
    lowest_point: u32,
    highest_point: u32,
    is_pressed: bool,
}

impl DigitalPosition {
    /// Creates a new [`DigitalPosition`].
    pub const fn default() -> Self {
        let dif = (DEFAULT_HIGH - DEFAULT_LOW) as f32;
        Self {
            buffer: [0; BUFFER_SIZE as usize],
            buffer_pos: 0,
            release_point: (DEFAULT_HIGH - (DEFAULT_RELEASE_SCALE * dif) as u32),
            actuation_point: (DEFAULT_HIGH - (DEFAULT_ACTUATE_SCALE * dif) as u32),
            is_pressed: false,
            lowest_point: DEFAULT_LOW,
            highest_point: DEFAULT_HIGH,
        }
    }

    // is_pressed is set like a normal mechanical switch, where if the buf
    // is higher than the release point, is_pressed is false, and if
    // the buf is lower than the acutation point, is_pressed is true
    fn update_buf(&mut self, pos: u16) {
        self.buffer[self.buffer_pos] = pos as u32;
        self.buffer_pos = (self.buffer_pos + 1) % BUFFER_SIZE;
        let mut sum = 0;
        for buf in self.buffer {
            sum += buf;
        }
        let avg = sum / BUFFER_SIZE as u32;
        self.calibrate(avg);
        if avg <= self.actuation_point {
            self.is_pressed = true;
        } else if avg > self.release_point {
            self.is_pressed = false;
        }
    }

    fn is_pressed(&self) -> bool {
        self.is_pressed
    }

    fn get_buf(&self) -> u16 {
        let mut sum = 0;
        for buf in self.buffer {
            sum += buf as u16;
        }
        sum / BUFFER_SIZE as u16
    }

    // Keep calling this function with adc readings
    // until it returns true to calibrate keys
    fn setup(&mut self, reading: u16) -> bool {
        if self.buffer[0] == 0 || self.buffer_pos != 0 {
            self.buffer[self.buffer_pos] = reading as u32;
            self.buffer_pos = (self.buffer_pos + 1) % BUFFER_SIZE as usize;
            false
        } else {
            let mut buf = 0;
            for num in self.buffer {
                buf += num;
            }
            let avg = buf / BUFFER_SIZE as u32;
            self.calibrate(avg);
            true
        }
    }

    fn calibrate(&mut self, buf: u32) {
        let mut changed = false;
        if self.highest_point < buf {
            self.highest_point = buf;
            changed = true;
        } else if self.lowest_point > buf {
            self.lowest_point = buf;
            changed = true;
        }

        if changed {
            let dif = (self.highest_point - self.lowest_point) as f32;
            self.release_point = self.highest_point - (DEFAULT_RELEASE_SCALE * dif) as u32;
            self.actuation_point = self.highest_point - (DEFAULT_ACTUATE_SCALE * dif) as u32;
        }
    }
}

#[derive(Copy, Clone, Default, Debug)]
struct WootingPosition {
    pub buffer: [u32; BUFFER_SIZE as usize],
    avg: u32,
    buffer_pos: usize,
    release_point: u32,
    actuation_point: u32,
    tolerance: u32,
    lowest_point: u32,
    highest_point: u32,
    is_pressed: bool,
    wooting: bool,
}

impl WootingPosition {
    pub const fn default() -> Self {
        let dif = (DEFAULT_HIGH - DEFAULT_LOW) as f32;
        Self {
            buffer: [0; BUFFER_SIZE as usize],
            avg: 0,
            buffer_pos: 0,
            release_point: (DEFAULT_HIGH - (DEFAULT_RELEASE_SCALE * dif) as u32),
            actuation_point: (DEFAULT_HIGH - (DEFAULT_ACTUATE_SCALE * dif) as u32),
            tolerance: (dif * TOLERANCE_SCALE) as u32,
            lowest_point: DEFAULT_LOW,
            highest_point: DEFAULT_HIGH,
            is_pressed: false,
            wooting: false,
        }
    }

    fn update_buf(&mut self, pos: u16) {
        self.buffer[self.buffer_pos as usize] = pos as u32;
        self.buffer_pos = (self.buffer_pos + 1) % BUFFER_SIZE;
        let mut sum = 0;
        for buf in self.buffer {
            sum += buf;
        }
        let avg = sum / BUFFER_SIZE as u32;
        if avg > self.release_point {
            self.avg = avg;
            self.wooting = false;
            self.is_pressed = false;
            self.calibrate(avg);
        } else if avg < self.lowest_point + 200 {
            self.avg = avg;
            self.wooting = true;
            self.is_pressed = true;
            self.calibrate(avg);
        } else if avg < self.avg - self.tolerance || (avg <= self.actuation_point && !self.wooting)
        {
            self.avg = avg;
            self.wooting = true;
            self.is_pressed = true;
        } else if avg > self.avg + self.tolerance {
            self.avg = avg;
            self.is_pressed = false;
        }
    }

    fn calibrate(&mut self, buf: u32) {
        let mut changed = false;
        if self.highest_point < buf {
            self.highest_point = buf;
            changed = true;
        } else if self.lowest_point > buf {
            self.lowest_point = buf;
            changed = true;
        }

        if changed {
            let dif = (self.highest_point - self.lowest_point) as f32;
            self.release_point = self.highest_point - (DEFAULT_RELEASE_SCALE * dif) as u32;
            self.actuation_point = self.highest_point - (DEFAULT_ACTUATE_SCALE * dif) as u32;
            self.tolerance = (dif as f32 * TOLERANCE_SCALE) as u32;
        }
    }

    fn setup(&mut self, reading: u16) -> bool {
        if self.buffer[0] == 0 || self.buffer_pos != 0 {
            self.buffer[self.buffer_pos] = reading as u32;
            self.buffer_pos = (self.buffer_pos + 1) % BUFFER_SIZE as usize;
            false
        } else {
            let mut buf = 0;
            for num in self.buffer {
                buf += num;
            }
            let avg = buf / BUFFER_SIZE as u32;
            self.calibrate(avg);
            true
        }
    }

    fn is_pressed(&self) -> bool {
        self.is_pressed
    }

    fn get_buf(&self) -> u16 {
        let mut sum = 0;
        for buf in self.buffer {
            sum += buf as u16;
        }
        sum / BUFFER_SIZE as u16
    }
}

#[derive(Copy, Clone, Debug)]
enum Position {
    Digital(DigitalPosition),
    Wooting(WootingPosition),
    Slave(u8),
}

impl Position {
    /// Returns the pressed status of the position
    fn is_pressed(&self) -> bool {
        match self {
            Position::Digital(pos) => pos.is_pressed(),
            Position::Wooting(pos) => pos.is_pressed(),
            Position::Slave(pos) => *pos == 1,
        }
    }

    /// Updates the buf of the key. Updating the buf will also update
    /// the value returned from the is_pressed function
    fn update_buf(&mut self, buf: u16) {
        match self {
            Position::Digital(pos) => pos.update_buf(buf),
            Position::Wooting(pos) => pos.update_buf(buf),
            Position::Slave(pos) => *pos = buf as u8,
        }
    }

    /// Returns the average buf of position
    fn get_buf(&self) -> u16 {
        match self {
            Position::Digital(pos) => pos.get_buf(),
            Position::Wooting(pos) => pos.get_buf(),
            Position::Slave(pos) => *pos as u16,
        }
    }

    fn setup(&mut self, buf: u16) -> bool {
        match self {
            Position::Slave(_) => true,
            Position::Digital(pos) => pos.setup(buf),
            Position::Wooting(pos) => pos.setup(buf),
        }
    }

    fn get_highest(&self) -> u32 {
        match self {
            Position::Digital(pos) => pos.highest_point,
            Position::Wooting(pos) => pos.highest_point,
            _ => 0,
        }
    }

    fn get_lowest(&self) -> u32 {
        match self {
            Position::Digital(pos) => pos.lowest_point,
            Position::Wooting(pos) => pos.lowest_point,
            _ => 0,
        }
    }
}

/// Represents a layer scancode. Pos represents the layer
/// the scancode will switch to and toggle will repsent if
/// the layer stored in the code stays after the key is released
#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
pub struct Layer {
    pub pos: usize,
    pub toggle: bool,
}

/// Sends the scan code in intervals which is determined by the passed in delay
/// and passed in equation.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct IntervalPresses {
    code: ScanCode,
    starting_time: Option<Instant>,
    last_pressed_time: Instant,
    org_delay: Duration,
    current_delay: Duration,
    acc_eq: fn(elasped: u64) -> u64,
}
impl IntervalPresses {
    pub fn new(val: ScanCode, delay: Duration, acc_eq: fn(u64) -> u64) -> Self {
        Self {
            code: val,
            starting_time: None,
            last_pressed_time: Instant::now(),
            org_delay: delay,
            current_delay: Duration::default(),
            acc_eq,
        }
    }
    fn get_code(&mut self) -> ScanCode {
        if let Some(time) = self.starting_time {
            if self.last_pressed_time.elapsed() > self.current_delay {
                self.last_pressed_time = Instant::now();
                let val = (self.acc_eq)(time.elapsed().as_millis());
                self.current_delay = Duration::from_millis(self.org_delay.as_micros() / val);
                self.code
            } else {
                ScanCode::None
            }
        } else {
            self.starting_time = Some(Instant::now());
            self.last_pressed_time = Instant::now();
            self.current_delay = self.org_delay;
            self.code
        }
    }
}

/// Sends a different ScanCode determined by how long a key is held for.
#[derive(Copy, Clone, Debug)]
pub struct ModTap {
    press_code: ScanCode,
    hold_code: ScanCode,
    start_time: Option<Instant>,
    held: bool,
}

impl ModTap {
    fn new(press_code: ScanCode, hold_code: ScanCode) -> ModTap {
        Self {
            press_code,
            hold_code,
            start_time: None,
            held: false,
        }
    }

    fn get_code(&mut self, pressed: bool) -> ModTapResult {
        if pressed {
            if let Some(time) = self.start_time {
                if time.elapsed() > HOLD_TIME {
                    self.held = true;
                    ModTapResult::Pressed(self.hold_code)
                } else {
                    ModTapResult::Holding
                }
            } else {
                self.held = false;
                self.start_time = Some(Instant::now());
                ModTapResult::Holding
            }
        } else {
            if self.start_time.is_some() {
                self.start_time = None;
                if !self.held {
                    ModTapResult::Pressed(self.press_code)
                } else {
                    ModTapResult::None
                }
            } else {
                ModTapResult::None
            }
        }
    }

    // This methods returns the ModTapResult without consuming the press code
    fn get_layer(&mut self, pressed: bool) -> ModTapResult {
        if pressed {
            if let Some(time) = self.start_time {
                if time.elapsed() > HOLD_TIME {
                    self.held = true;
                    ModTapResult::Pressed(self.hold_code)
                } else {
                    ModTapResult::Holding
                }
            } else {
                self.held = false;
                self.start_time = Some(Instant::now());
                ModTapResult::Holding
            }
        } else {
            if self.start_time.is_some() {
                if !self.held {
                    ModTapResult::Pressed(self.press_code)
                } else {
                    ModTapResult::None
                }
            } else {
                ModTapResult::None
            }
        }
    }
}

enum ModTapResult {
    Pressed(ScanCode),
    Holding,
    None,
}

#[derive(Copy, Clone, Debug)]
enum ModComboState {
    Combo(usize),
    Normal,
    Holding,
    None,
}

#[derive(Copy, Clone, Debug)]
pub struct ModCombo {
    tap_code: ScanCode,
    hold_code: [ScanCode; NUM_COMB],
    other_index: [Option<usize>; NUM_COMB],
    start_time: Instant,
    combo: ModComboState,
}

impl ModCombo {
    fn new(
        tap_code: ScanCode,
        hold_code: [ScanCode; NUM_COMB],
        other_index: [Option<usize>; NUM_COMB],
    ) -> Self {
        Self {
            tap_code,
            hold_code,
            other_index,
            start_time: Instant::now(),
            combo: ModComboState::None,
        }
    }

    fn get_code(
        &mut self,
        pressed: bool,
        other_pressed: ModComboState,
        other_index: usize,
    ) -> Option<ScanCode> {
        if pressed {
            match self.combo {
                ModComboState::Combo(num) => Some(self.hold_code[num]),
                ModComboState::Normal => Some(self.tap_code),
                ModComboState::Holding => {
                    if self.start_time.elapsed() >= HOLD_DURATION {
                        self.combo = ModComboState::Normal;
                        return Some(self.tap_code);
                    }
                    match other_pressed {
                        ModComboState::Normal | ModComboState::Combo(_) => {
                            self.combo = ModComboState::Normal;
                            Some(self.tap_code)
                        }
                        ModComboState::Holding => {
                            self.combo = ModComboState::Combo(other_index);
                            Some(self.hold_code[other_index])
                        }
                        ModComboState::None => None,
                    }
                }
                ModComboState::None => {
                    self.combo = ModComboState::Holding;
                    self.start_time = Instant::now();
                    None
                }
            }
        } else {
            match self.combo {
                ModComboState::Holding => {
                    self.combo = ModComboState::None;
                    Some(self.tap_code)
                }
                _ => {
                    self.combo = ModComboState::None;
                    None
                }
            }
        }
    }
}

/// Represents all the different types of scancodes.
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum ScanCode {
    Letter(u8),
    Modifier(u8),
    MouseButton(u8),
    MouseX(i8),
    MouseY(i8),
    Layer(Layer),
    Scroll(i8),
    None,
}

/// Wrapper around ScanCode to allow different fuctionalites when pressed
/// such as sending multiple keys
#[derive(Copy, Clone, Debug)]
pub enum ScanCodeBehavior<const S: usize> {
    Single(ScanCode),
    Double(ScanCode, ScanCode),
    Triple(ScanCode, ScanCode, ScanCode),
    // Return a different key code depending on the other indexed key press status
    CombinedKey {
        other_index: usize,
        normal_code: ScanCode,
        combined_code: ScanCode,
    },
    IntervalPresses(IntervalPresses),
    ModTap(ModTap),
    ModCombo(ModCombo),
    Config(fn(&mut Keys<S>)),
    Function(fn()),
}

#[derive(Copy, Clone, Debug)]
struct Key<const S: usize> {
    pos: Position,
    codes: [ScanCodeBehavior<S>; NUM_LAYERS],
    pub current_layer: Option<usize>,
    reverse: bool,
}

impl<const S: usize> Key<S> {
    const fn default() -> Self {
        Self {
            pos: Position::Digital(DigitalPosition::default()),
            codes: [ScanCodeBehavior::Single(ScanCode::Letter(0)); NUM_LAYERS],
            current_layer: None,
            reverse: true,
        }
    }

    fn set_code(&mut self, code: KeyCodes, toggle: bool, layer: usize) {
        self.codes[layer] = match code.get_scan_code() {
            ScanCode::Layer(mut l) => {
                l.toggle = toggle;
                ScanCodeBehavior::Single(ScanCode::Layer(l))
            }
            rest => ScanCodeBehavior::Single(rest),
        }
    }

    fn set_digital(&mut self) {
        self.pos = Position::Digital(DigitalPosition::default());
    }

    fn set_wooting(&mut self) {
        self.pos = Position::Wooting(WootingPosition::default());
    }

    fn set_slave(&mut self) {
        self.pos = Position::Slave(0);
    }

    fn update_buf(&mut self, buf: u16) {
        self.pos.update_buf(buf);
    }

    /// Pushes the scan code into the provided index set depending on the Key's position
    pub fn get_buf(&self) -> u16 {
        self.pos.get_buf()
    }

    fn get_hold_state(&mut self, layer: usize) -> ModComboState {
        match self.codes[layer] {
            ScanCodeBehavior::ModCombo(val) => val.combo,
            _ => ModComboState::None,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Keys<const S: usize> {
    keys: [Key<S>; S],
}

enum PressResult {
    Pressed,
    Function,
    None,
}
impl<const S: usize> Keys<S> {
    /// Returns a Keys struct
    pub const fn default() -> Self {
        Self {
            keys: [Key::default(); S],
        }
    }

    pub fn get_pressed(&self, index: usize) -> bool {
        self.keys[index].pos.is_pressed()
    }

    pub fn get_highest(&self, index: usize) -> u32 {
        self.keys[index].pos.get_highest()
    }

    pub fn get_lowest(&self, index: usize) -> u32 {
        self.keys[index].pos.get_lowest()
    }

    // flips reading if key reverse state is true
    fn get_reading(&mut self, index: usize, reading: u16) -> u16 {
        if self.keys[index].reverse {
            4095 - reading
        } else {
            reading
        }
    }

    pub fn setup(&mut self, index: usize, buf: u16) -> bool {
        let reading = self.get_reading(index, buf);
        self.keys[index].pos.setup(reading)
    }

    /// Sets the code on the passed in layer on the indexed key. Returns
    /// an err on invalid index or invalid layer
    pub fn set_code(&mut self, code: KeyCodes, index: usize, layer: usize) {
        self.keys[index].set_code(code, false, layer);
    }

    /// Sets the indexed key to be a double key. A double key sends two keycodes rather than one
    pub fn set_double(&mut self, code0: KeyCodes, code1: KeyCodes, index: usize, layer: usize) {
        self.keys[index].codes[layer] =
            ScanCodeBehavior::Double(code0.get_scan_code(), code1.get_scan_code());
    }

    /// Sets the indexed key to be a combined key. other_index is the other indexed key that needs
    /// to be held for the comb_code to be activated
    pub fn set_combined(
        &mut self,
        norm_code: KeyCodes,
        comb_code: KeyCodes,
        other_index: usize,
        index: usize,
        layer: usize,
    ) {
        self.keys[index].codes[layer] = ScanCodeBehavior::CombinedKey {
            other_index,
            normal_code: norm_code.get_scan_code(),
            combined_code: comb_code.get_scan_code(),
        }
    }

    /// Sets the indexed key to be an interval key. An interval key sends a press every dur. The
    /// passed in function represent the rate the dur should change
    pub fn set_interval(
        &mut self,
        code: KeyCodes,
        dur: Duration,
        f: fn(u64) -> u64,
        index: usize,
        layer: usize,
    ) {
        self.keys[index].codes[layer] =
            ScanCodeBehavior::IntervalPresses(IntervalPresses::new(code.get_scan_code(), dur, f))
    }

    pub fn set_modtap(&mut self, p_code: KeyCodes, h_code: KeyCodes, index: usize, layer: usize) {
        self.keys[index].codes[layer] =
            ScanCodeBehavior::ModTap(ModTap::new(p_code.get_scan_code(), h_code.get_scan_code()));
    }

    pub fn set_modcomb(
        &mut self,
        p_code: KeyCodes,
        h_code: [Option<KeyCodes>; NUM_COMB],
        other_index: [Option<usize>; NUM_COMB],
        index: usize,
        layer: usize,
    ) {
        let mut h_codes = [ScanCode::None; NUM_COMB];
        for i in 0..NUM_COMB {
            match h_code[i] {
                Some(code) => h_codes[i] = code.get_scan_code(),
                None => {}
            }
        }
        self.keys[index].codes[layer] =
            ScanCodeBehavior::ModCombo(ModCombo::new(p_code.get_scan_code(), h_codes, other_index));
    }

    /// Sets the following indexed to be a toggle layer key for the passed in layer. Any none layer
    /// keys passed in will be set like in set_code
    pub fn set_toggle_layer(&mut self, layer_code: KeyCodes, index: usize, layer: usize) {
        match layer_code.get_scan_code() {
            ScanCode::Layer(_) => {}
            _ => {
                panic!("bruh")
            }
        }
        self.keys[index].set_code(layer_code, true, layer);
    }

    /// All indexes stored within the range will set the respective keys as having slave positions
    pub fn set_slave(&mut self, range: Range<u8>) {
        for i in range {
            self.keys[i as usize].set_slave();
            self.keys[i as usize].reverse = false;
        }
    }

    pub fn set_reverse(&mut self, val: bool, index: usize) {
        self.keys[index].reverse = val;
    }

    pub fn set_config(&mut self, f: fn(&mut Keys<S>), index: usize, layer: usize) {
        self.keys[index].codes[layer] = ScanCodeBehavior::Config(f);
    }

    pub fn set_function(&mut self, f: fn(), index: usize, layer: usize) {
        self.keys[index].codes[layer] = ScanCodeBehavior::Function(f);
    }

    /// Updates the indexed key with the provided reading
    pub fn update_buf(&mut self, index: usize, reading: u16) {
        let res = self.get_reading(index, reading);
        self.keys[index].update_buf(res);
    }

    /// Gets the average buf of the indexed key
    pub fn get_buf(&self, index: usize) -> u16 {
        self.keys[index].get_buf()
    }

    /// Returns the indexes of all the keys that are pressed to the vec
    pub fn is_pressed(&self, vec: &mut Vec<usize, S>) {
        for i in 0..S {
            if self.keys[i].pos.is_pressed() {
                vec.push(i).unwrap();
            }
        }
    }

    /// Pushes the resulting ScanResult onto the provided vec depending on the indexed key's
    /// position. Returns true if a key was pushed into the provided index set
    fn get_pressed_code(
        &mut self,
        index: usize,
        layer: usize,
        set: &mut Vec<ScanCode, 64>,
    ) -> PressResult {
        let pressed = self.keys[index].pos.is_pressed();
        let mut other_index = 0;
        let mut unpressed = false;
        let mut broke = false;
        let other_pressed = ModComboState::None;
        // let other_pressed = if let ScanCodeBehavior::ModCombo(val) = self.keys[index].codes[layer] {
        //     let mut res = ModComboState::None;
        //     'outer: for i in 0..NUM_COMB {
        //         let other = match val.other_index[i] {
        //             Some(num) => num,
        //             None => break 'outer,
        //         };
        //         match self.keys[other].codes[layer] {
        //             ScanCodeBehavior::ModCombo(val_o) => match val_o.combo {
        //                 ModComboState::Combo(num) => {
        //                     res = val_o.combo;
        //                     if val_o.other_index[num].unwrap() == index {
        //                         res = ModComboState::Holding;
        //                         other_index = i;
        //                         broke = true;
        //                         break 'outer;
        //                     }
        //                 }
        //                 ModComboState::Normal => {
        //                     res = ModComboState::Normal;
        //                 }
        //                 ModComboState::Holding => {
        //                     res = ModComboState::Holding;
        //                     other_index = i;
        //                     broke = true;
        //                     break 'outer;
        //                 }
        //                 ModComboState::None => {
        //                     unpressed = true;
        //                 }
        //             },
        //             _ => {}
        //         };
        //     }
        //     if unpressed && !broke {
        //         ModComboState::None
        //     } else {
        //         res
        //     }
        // } else {
        //     ModComboState::None
        // };
        match self.keys[index].codes[layer].borrow_mut() {
            ScanCodeBehavior::Single(code) => {
                if pressed {
                    set.push(*code).unwrap();
                    PressResult::Pressed
                } else {
                    PressResult::None
                }
            }
            ScanCodeBehavior::Double(code0, code1) => {
                if pressed {
                    set.push(*code0).unwrap();
                    set.push(*code1).unwrap();
                    PressResult::Pressed
                } else {
                    PressResult::None
                }
            }
            ScanCodeBehavior::Triple(code0, code1, code2) => {
                if pressed {
                    set.push(*code0).unwrap();
                    set.push(*code1).unwrap();
                    set.push(*code2).unwrap();
                    PressResult::Pressed
                } else {
                    PressResult::None
                }
            }
            ScanCodeBehavior::CombinedKey {
                other_index,
                normal_code,
                combined_code: other_key_code,
            } => {
                if pressed {
                    if self.keys[*other_index].pos.is_pressed() {
                        set.push(*other_key_code).unwrap();
                        PressResult::Pressed
                    } else {
                        set.push(*normal_code).unwrap();
                        PressResult::Pressed
                    }
                } else {
                    PressResult::None
                }
            }
            ScanCodeBehavior::IntervalPresses(val) => {
                if pressed {
                    set.push(val.get_code()).unwrap();
                    PressResult::Pressed
                } else {
                    val.starting_time = None;
                    PressResult::None
                }
            }
            ScanCodeBehavior::ModTap(val) => {
                match val.get_code(self.keys[index].pos.is_pressed()) {
                    ModTapResult::Pressed(code) => {
                        set.push(code).unwrap();
                        PressResult::Pressed
                    }
                    ModTapResult::Holding => {
                        // Held keys need to stay in the same layer so we'll have to return PresResult::Pressed
                        PressResult::Pressed
                    }
                    ModTapResult::None => PressResult::None,
                }
            }
            ScanCodeBehavior::ModCombo(val) => {
                let pressed = pressed;
                match val.get_code(pressed, other_pressed, other_index) {
                    Some(code) => {
                        set.push(code).unwrap();
                        PressResult::Pressed
                    }
                    None => PressResult::None,
                }
            }
            ScanCodeBehavior::Config(f) => {
                if pressed {
                    f(self);
                    PressResult::Function
                } else {
                    PressResult::None
                }
            }
            ScanCodeBehavior::Function(f) => {
                if pressed {
                    f();
                    PressResult::Function
                } else {
                    PressResult::None
                }
            }
        }
    }

    /// Returns all the pressed scancodes in the Keys struct. Returns it through
    /// the passed in vector. This function won't return layer codes. That will be done
    /// through the get_layer method. The passed in vector should be empty.
    /// Note that if a key is held, it will ignore the passed in layer and use the
    /// previous layer it's holding
    pub fn get_keys(&mut self, layer: usize, set: &mut Vec<ScanCode, 64>) {
        for i in 0..S {
            let layer = match self.keys[i].current_layer {
                Some(num) => num,
                None => layer,
            };
            match self.get_pressed_code(i, layer, set) {
                PressResult::Function => {
                    set.clear();
                    break;
                }
                PressResult::Pressed => {
                    self.keys[i].current_layer = Some(layer);
                }
                PressResult::None => {
                    self.keys[i].current_layer = None;
                }
            }
        }
    }
}
