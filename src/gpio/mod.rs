use alloc::string::String;
use alloc::string::ToString;

use silica::peripheral::Peripheral;
use silica::peripheral::gpio::{Input as IInput, Output as IOutput};

use rcc;
use registers::*;

#[derive(Copy, Clone)]
pub enum AlternateFunction {
    AF0 = 0,
    AF1 = 1,
    AF2 = 2,
    AF3 = 3,
    AF4 = 4,
    AF5 = 5,
    AF6 = 6,
    AF7 = 7,
    AF8 = 8,
    AF9 = 9,
    AF10 = 10,
    AF11 = 11,
    AF12 = 12,
    AF13 = 13,
    AF14 = 14,
    AF15 = 15,
}

#[derive(Copy, Clone)]
pub enum OutputType {
    PushPull    = 0,
    OpenDrain   = 1
}

pub enum Mode {
    In,
    Out(OutputType, bool),
    AlternateFunction(AlternateFunction, OutputType),
    Analog
}

#[derive(Copy, Clone)]
pub enum Frequency {
    F2MHz   = 0,
    F20MHz  = 1,
    F50MHz  = 2,
    F100MHz = 3
}

#[derive(Copy, Clone)]
pub enum PullSide {
    None    = 0,
    Up      = 1,
    Down    = 2,
    Both    = 3
}

#[repr(C)]
pub struct PortRegisters {
    mode: Rw<u32>,
    output_type: Rw<u16>,
    reserved0: u16,
    output_speed: Rw<u32>,
    pu_pd: Rw<u32>,
    input_data: Ro<u16>,
    reserved1: u16,
    output_data: Rw<u16>,
    reserved2: u16,
    bit_set: Wo<u16>,
    bit_reset: Wo<u16>,
    lock: Rw<u16>,
    lock_key: Rw<u16>,
    alternate_function_low: Rw<u32>,
    alternate_function_high: Rw<u32>
}

pub struct PortPeripheral {
    pub base_address: *mut PortRegisters,
    pub clock: rcc::RCCPeripheral
}
unsafe impl Sync for PortPeripheral {}
impl Peripheral for PortPeripheral {
    fn init(&self) -> Result<(), String> {
        init_peripherals![&self.clock];

        Ok(())
    }
}
impl Drop for PortPeripheral {
    fn drop(&mut self) {
        unimplemented!();
    }
}

pub struct PinPeripheral<'a> {
    pub port: &'a PortPeripheral,
    pub pin: u32,
    pub mode: Mode,
    pub speed: Frequency,
    pub pull_side: PullSide,
}
unsafe impl<'a> Sync for PinPeripheral<'a> {}

impl<'a> Peripheral for PinPeripheral<'a> {
    fn init(&self) -> Result<(), String> {
        if 15 < self.pin {
            return Err("Invalid pin number".to_string())
        }

        if let Err(msg) = self.port.init() {
            return Err(msg)
        }

        let (mode, otype, af, state) = match self.mode {
            Mode::In => (0, 0, 0, false),
            Mode::Out(otype, state) => (1, otype as u16, 0, state),
            Mode::AlternateFunction(af, otype) => (2, otype as u16, af as u32, false),
            Mode::Analog => { (3, 0, 0, false) }
        };

        let onebit_mask = 1 << self.pin;
        let twobit_shift = self.pin * 2;
        let twobit_mask = 3 << twobit_shift;

        let mut af_shift = self.pin * 4;
        if af_shift >= 32 {
            af_shift -= 32;
        }
        let af_mask = 0xF << af_shift;

        unsafe {
            (*self.port.base_address).output_type.update(otype << self.pin, onebit_mask);
            (*self.port.base_address).output_speed.update((self.speed as u32) << twobit_shift, twobit_mask);
            (*self.port.base_address).pu_pd.update((self.pull_side as u32) << twobit_shift, twobit_mask);
            if state {
                (*self.port.base_address).output_data.update(1 << self.pin, onebit_mask);
            } else {
                (*self.port.base_address).output_data.update(0, onebit_mask);
            }
            if self.pin < 8 {
                (*self.port.base_address).alternate_function_low.update(af << af_shift, af_mask);
            } else {
                (*self.port.base_address).alternate_function_high.update(af << af_shift, af_mask);
            }
            (*self.port.base_address).mode.update(mode << twobit_shift, twobit_mask);
        }

        Ok(())
    }
}
impl<'a> Drop for PinPeripheral<'a> {
    fn drop(&mut self) {
        unimplemented!();
    }
}

pub struct In<'a> {
    periph: &'a PinPeripheral<'a>
}

impl<'a> IInput for In<'a> {
    fn read(&self) -> bool {
        let mask = 1 << self.periph.pin;
        unsafe {
            ((*self.periph.port.base_address).input_data.read() & mask) == mask
        }
    }
}

pub struct Out<'a> {
    periph: &'a PinPeripheral<'a>
}

impl<'a> Out<'a> {
    pub fn from(f: &'a PinPeripheral<'a>) -> Result<Out<'a>, String> {
        if let Err(msg) = f.init() {
            Err(msg)
        } else {
            Ok(Out { periph: f })
        }
    }
}

impl<'a> IOutput for Out<'a> {
    fn get_command(&self) -> bool {
        let mask = 1 << self.periph.pin;
        unsafe {
            ((*self.periph.port.base_address).output_data.read() & mask) == mask
        }
    }
    fn write(&mut self, command: bool) -> bool {
        let mask = 1 << self.periph.pin;
        unsafe {
            if command {
                (*self.periph.port.base_address).bit_set.write(mask);
            } else {
                (*self.periph.port.base_address).bit_reset.write(mask);
            }
            ((*self.periph.port.base_address).output_data.read() & mask) == mask
        }
    }
}
