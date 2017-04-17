use collections::string::String;
use collections::string::ToString;

use rcc;
use Peripheral;
use IRQType;

pub struct DMARegisters {

}

pub struct DMAStreamRegisters {

}

pub enum Channel {
    Channel0 = 0,
    Channel1 = 1,
    Channel2 = 2,
    Channel3 = 3,
    Channel4 = 4,
    Channel5 = 5,
    Channel6 = 6,
    Channel7 = 7,
    Channel8 = 8
}

pub struct DMAPeripheral {
    pub base_address: *mut DMARegisters,
    pub isr_id: IRQType,
    pub clock: rcc::RCCPeripheral
}

pub struct DMAStreamPeripheral<'a> {
    pub dma: &'a DMAPeripheral,
    pub base_address: *mut DMAStreamRegisters,
    pub channel: Channel,
}
impl<'a> Peripheral for DMAStreamPeripheral<'a> {
    fn init(&self) -> Result<(), String> {


        Err("Not yet implemented".to_string())
    }
    fn deinit(&self) -> Result<(), String> {
        Err("Not yet implemented".to_string())
    }
}
