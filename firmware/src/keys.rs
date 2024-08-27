use core::{borrow::BorrowMut, ops::Range};

use embassy_time::{Duration, Instant};
use heapless::{FnvIndexSet, Vec};

use crate::codes::KeyCodes;

const DEFAULT_RELEASE_SCALE: f32 = 0.20;
const DEFAULT_ACTUATE_SCALE: f32 = 0.25;
const TOLERANCE_SCALE: f32 = 0.075;
const BUFFER_SIZE: u16 = 1;

pub const NUM_LAYERS: usize = 10;

// Makes hall effect switches act like a normal mechanical switch
#[derive(Copy, Clone, Default, Debug)]
struct DigitalPosition {
    buffer: [u32; BUFFER_SIZE as usize], // Take multiple readings to smooth out buffer
    buffer_pos: u16,
    release_point: u32,
    actuation_point: u32,
    lowest_point: u32,
    highest_point: u32,
    is_pressed: bool,
}

impl DigitalPosition {
    /// Creates a new [`DigitalPosition`].
    pub fn new(lowest_point: u32, highest_point: u32) -> Self {
        let dif = (highest_point - lowest_point) as f32;
        Self {
            buffer: [0; BUFFER_SIZE as usize],
            buffer_pos: 0,
            release_point: (highest_point - (DEFAULT_RELEASE_SCALE * dif) as u32),
            actuation_point: (highest_point - (DEFAULT_ACTUATE_SCALE * dif) as u32),
            is_pressed: false,
            lowest_point,
            highest_point,
        }
    }

    // is_pressed is set like a normal mechanical switch, where if the buf
    // is higher than the release point, is_pressed is false, and if
    // the buf is lower than the acutation point, is_pressed is true
    fn update_buf(&mut self, pos: u16) {
        self.buffer[self.buffer_pos as usize] = pos as u32;
        self.buffer_pos = (self.buffer_pos + 1) % BUFFER_SIZE;
        let mut sum = 0;
        for buf in self.buffer {
            sum += buf;
        }
        let avg = sum / BUFFER_SIZE as u32;
        if self.is_pressed && avg > self.release_point {
            self.is_pressed = false;
        } else if !self.is_pressed && avg < self.actuation_point {
            self.is_pressed = true;
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
        sum / BUFFER_SIZE
    }
}

#[derive(Copy, Clone, Default, Debug)]
struct WootingPosition {
    pub buffer: [u32; BUFFER_SIZE as usize],
    avg: u32,
    buffer_pos: u16,
    release_point: u32,
    actuation_point: u32,
    tolerance: u32,
    lowest_point: u32,
    highest_point: u32,
    is_pressed: bool,
    wooting: bool,
}

impl WootingPosition {
    pub fn new(lowest_point: u32, highest_point: u32) -> Self {
        let dif = (highest_point - lowest_point) as f32;
        Self {
            buffer: [0; BUFFER_SIZE as usize],
            avg: (highest_point - (DEFAULT_RELEASE_SCALE * dif) as u32),
            buffer_pos: 0,
            release_point: (highest_point - (DEFAULT_RELEASE_SCALE * dif) as u32),
            actuation_point: (highest_point - (DEFAULT_ACTUATE_SCALE * dif) as u32),
            tolerance: (dif * TOLERANCE_SCALE) as u32,
            lowest_point,
            highest_point,
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

    fn is_pressed(&self) -> bool {
        self.is_pressed
    }

    fn get_buf(&self) -> u16 {
        let mut sum = 0;
        for buf in self.buffer {
            sum += buf as u16;
        }
        sum / BUFFER_SIZE
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
}

/// Represents a layer scancode. Pos represents the layer
/// the scancode will switch to and toggle will repsent if
/// the layer stored in the code stays after the key is released
#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
pub struct Layer {
    pub pos: usize,
    pub toggle: bool,
}

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
pub enum ScanCodeBehavior {
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
}

#[derive(Copy, Clone, Debug)]
struct Key {
    pos: Position,
    codes: [ScanCodeBehavior; NUM_LAYERS],
    pub current_layer: Option<usize>,
}

impl Key {
    fn default() -> Self {
        Self {
            pos: Position::Wooting(WootingPosition::new(1100, 2000)),
            codes: [ScanCodeBehavior::Single(ScanCode::Letter(0)); NUM_LAYERS],
            current_layer: None,
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
        self.pos = Position::Digital(DigitalPosition::new(1400, 2000));
    }

    fn set_wooting(&mut self) {
        self.pos = Position::Wooting(WootingPosition::new(1400, 2000));
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
}

#[derive(Copy, Clone, Debug)]
pub struct Keys<const S: usize> {
    keys: [Key; S],
}

impl<const S: usize> Keys<S> {
    /// Returns a Keys struct
    pub fn default() -> Self {
        Self {
            keys: [Key::default(); S],
        }
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

    /// Sets the following indexed to be a toggle layer key for the passed in layer. Any none layer
    /// keys passed in will be set like in set_code
    pub fn set_toggle_layer(&mut self, layer_code: KeyCodes, index: usize, layer: usize) {
        self.keys[index].set_code(layer_code, true, layer);
    }

    /// All indexes stored within the range will set the respective keys as having slave positions
    pub fn set_slave(&mut self, range: Range<u8>) {
        for i in range {
            self.keys[i as usize].set_slave();
        }
    }

    /// Updates the indexed key with the provided reading
    pub fn update_buf(&mut self, index: usize, reading: u16) {
        self.keys[index].update_buf(reading);
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
        set: &mut FnvIndexSet<ScanCode, 64>,
    ) -> bool {
        match self.keys[index].codes[layer].borrow_mut() {
            ScanCodeBehavior::Single(code) => {
                if self.keys[index].pos.is_pressed() {
                    set.insert(*code).unwrap();
                    true
                } else {
                    false
                }
            }
            ScanCodeBehavior::Double(code0, code1) => {
                if self.keys[index].pos.is_pressed() {
                    set.insert(*code0).unwrap();
                    set.insert(*code1).unwrap();
                    true
                } else {
                    false
                }
            }
            ScanCodeBehavior::Triple(code0, code1, code2) => {
                if self.keys[index].pos.is_pressed() {
                    set.insert(*code0).unwrap();
                    set.insert(*code1).unwrap();
                    set.insert(*code2).unwrap();
                    true
                } else {
                    false
                }
            }
            ScanCodeBehavior::CombinedKey {
                other_index,
                normal_code,
                combined_code: other_key_code,
            } => {
                if self.keys[index].pos.is_pressed() {
                    if self.keys[*other_index].pos.is_pressed() {
                        set.insert(*other_key_code).unwrap();
                        true
                    } else {
                        set.insert(*normal_code).unwrap();
                        true
                    }
                } else {
                    false
                }
            }
            ScanCodeBehavior::IntervalPresses(val) => {
                if self.keys[index].pos.is_pressed() {
                    set.insert(val.get_code()).unwrap();
                    true
                } else {
                    val.starting_time = None;
                    false
                }
            }
        }
    }

    /// Returns the proper layer code from all the keys. Use this method
    /// to get the layer rather than get_pressed_code as calling get_pressed_code
    /// twice can lead to unintended behaviour for certain ScanCodeBehavior types
    fn get_layer_code(&mut self, index: usize, layer: usize) -> ScanCode {
        match self.keys[index].codes[layer] {
            ScanCodeBehavior::Single(code) => {
                if self.keys[index].pos.is_pressed() {
                    if let ScanCode::Layer(res) = code {
                        ScanCode::Layer(res)
                    } else {
                        ScanCode::None
                    }
                } else {
                    ScanCode::None
                }
            }
            ScanCodeBehavior::CombinedKey {
                other_index,
                normal_code,
                combined_code,
            } => {
                if self.keys[index].pos.is_pressed() {
                    if self.keys[other_index].pos.is_pressed() {
                        if let ScanCode::Layer(res) = combined_code {
                            ScanCode::Layer(res)
                        } else {
                            ScanCode::None
                        }
                    } else {
                        if let ScanCode::Layer(res) = normal_code {
                            ScanCode::Layer(res)
                        } else {
                            ScanCode::None
                        }
                    }
                } else {
                    ScanCode::None
                }
            }
            // Layer keys can only be a single code
            _ => ScanCode::None,
        }
    }

    /// Get the current layer value from all the keys. Toggle layers gets priority. Note that if this method
    /// was called earlier and returned a pressed scan value, it will use the previous layer
    /// rather than the provided layer. This allows keys to hold their values even when switching
    /// layers. When the key is released, it will start using the provided layer code
    pub fn get_layer(&mut self, layer: usize) -> Option<Layer> {
        let mut new_layer: Option<Layer> = None;
        for i in 0..S {
            let layer = match self.keys[i].current_layer {
                Some(num) => num,
                None => layer,
            };
            if let ScanCode::Layer(code) = self.get_layer_code(i, layer) {
                match new_layer {
                    Some(val) => {
                        if val.pos > code.pos || val.toggle {
                            new_layer = Some(code);
                            if val.toggle {
                                break;
                            }
                        }
                    }
                    None => new_layer = Some(code),
                };
                self.keys[i].current_layer = Some(layer);
            }
        }
        if let Some(code) = new_layer {
            if code.toggle {
                for key in &mut self.keys {
                    key.current_layer = None;
                }
            }
        }
        new_layer
    }

    /// Returns all the pressed scancodes in the Keys struct. Returns it through
    /// the passed in vector. This function won't return layer codes. That will be done
    /// through the get_layer method. The passed in vector should be empty.
    /// Note that if a key is held, it will ignore the passed in layer and use the
    /// previous layer it's holding
    pub fn get_keys(&mut self, layer: usize, set: &mut FnvIndexSet<ScanCode, 64>) {
        for i in 0..S {
            let layer = match self.keys[i].current_layer {
                Some(num) => num,
                None => layer,
            };
            if self.get_pressed_code(i, layer, set) {
                self.keys[i].current_layer = Some(layer);
            } else {
                self.keys[i].current_layer = None;
            }
        }
    }
}
