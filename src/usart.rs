use core::intrinsics;
use collections::string::String;
use collections::borrow::ToOwned;

use silica::peripheral::serial::{BitCount, Parity, StopBit, Serial as ISerial};
use silica::sync::mpsc::Sender;
use silica::io::{Read, Write, Receive, Error};

use rcc;
use IRQType;
use AdvancedPeripheralBus;
use Peripheral;
use registers::*;
use dma::DMAStreamPeripheral;
use gpio::PinPeripheral;

#[repr(C)]
pub struct USARTRegisters {
    status: Ro<u16>,
    reserved0: u16,
    data: Rw<u16>,
    reserved1: u16,
    baud_rate: Rw<u16>,
    reserved2: u16,
    control1: Rw<u16>,
    reserved3: u16,
    control2: Rw<u16>,
    reserved4: u16,
    control3: Rw<u16>,
    reserved5: u16,
    gtpr_psc: Rw<u8>,
    gtpr_gt: Rw<u8>,
    reserved6: u16
}

pub struct USARTPeripheral<'a> {
    pub base_address: *mut USARTRegisters,
    pub clock: rcc::RCCPeripheral,
    pub isr_id: IRQType,
    pub dma_rx: Option<&'a DMAStreamPeripheral<'a>>,
    pub dma_tx: Option<&'a DMAStreamPeripheral<'a>>,

    pub pin_tx: Option<&'a PinPeripheral<'a>>,  // output
    pub pin_rx: Option<&'a PinPeripheral<'a>>,  // input
    pub pin_dtr: Option<&'a PinPeripheral<'a>>, // output: data terminal ready
    pub pin_dcd: Option<&'a PinPeripheral<'a>>, // input: data carier detect
    pub pin_dsr: Option<&'a PinPeripheral<'a>>, // input: data set ready
    pub pin_ri: Option<&'a PinPeripheral<'a>>,  // input: ring indicator
    pub pin_rts: Option<&'a PinPeripheral<'a>>, // output: request to send
    pub pin_cts: Option<&'a PinPeripheral<'a>>, // input: clear to send
}
unsafe impl<'a> Sync for USARTPeripheral<'a> {}

impl<'a> Peripheral for USARTPeripheral<'a> {
    fn init(&self) -> Result<(), String> {
        let mut cr3 = unsafe { (*self.base_address).control3.read() };
        let mut cr1 = unsafe { (*self.base_address).control1.read() };

        // setup GPIOs
        init_peripheral![self.pin_tx, self.pin_rx]; // data lines
        init_peripheral![self.pin_dtr, self.pin_dcd, self.pin_dsr, self.pin_ri]; // sw flow control
        init_peripheral![self.pin_rts, self.pin_cts]; // hw flow control
        if let Some(_) = self.pin_cts {
            cr3 |= 0x200;
        }
        if let Some(_) = self.pin_rts {
            cr3 |= 0x100;
        }
        if let Some(_) = self.pin_tx {
            cr1 |= 0x08;
        }
        if let Some(_) = self.pin_rx {
            cr1 |= 0x04;
        }

        // enable clock (RCC)
        init_peripheral![Some(&self.clock)];

        unsafe {
            (*self.base_address).control3.write(cr3);
            (*self.base_address).control1.write(cr1);
        }

        Ok(())
    }

    fn deinit(&self) -> Result<(), String> {
        Err("Not yet implemented".to_owned())
    }
}

pub struct Serial<'a> {
    periph: &'a USARTPeripheral<'a>
}
impl<'a> Serial<'a> {
    pub fn from(f: &'a USARTPeripheral<'a>) -> Serial<'a> {
        Serial {
            periph: f
        }
    }
}
impl<'a> ISerial for Serial<'a> {
    #[allow(unused_variables)]
    fn setup(&mut self, baudrate:usize, word_len: BitCount, parity: Parity, stop_bit: StopBit) -> Result<(), String> {
        let mut cr3 = unsafe { (*self.periph.base_address).control3.read() };
        let mut cr1 = unsafe { (*self.periph.base_address).control1.read() };
        init_peripheral![Some(&self.periph)];

        let clk = self.periph.clock.get_clock() as f64;

        let mut usartdiv = clk / (16_f64 * (baudrate as f64));
        let over8 = usartdiv < 1_f64;
        if over8 {
            cr1 |= 0x8000;
            usartdiv *= 2_f64;
        }
        if usartdiv < 1_f64 {
            return Err("This baudrate is too high for this serial port.".to_owned());
        }
        if usartdiv >= 4096_f64 {
            return Err("This baudrate is too low for this serial port.".to_owned());
        }

        let mantissa_f64 = unsafe { intrinsics::truncf64(usartdiv) };
        let frac_f64 = (usartdiv - mantissa_f64) * if over8 {8_f64} else {16_f64};

        let mantissa: u16 = mantissa_f64 as u16;
        let div: u16 = unsafe { intrinsics::roundf64(frac_f64) as u16 };

        // setup DMA rx
        init_peripheral![self.periph.dma_rx];
        if let Some(_) = self.periph.dma_rx {
            cr3 |= 0x80;
        }

        // setup DMA tx
        init_peripheral![self.periph.dma_tx];
        if let Some(_) = self.periph.dma_tx {
            cr3 |= 0x40;
        }

        unsafe {
            (*self.periph.base_address).baud_rate.write((mantissa << 4) | div);
            (*self.periph.base_address).control3.write(cr3);
            (*self.periph.base_address).control1.write(cr1);
        }

        Ok(())
    }
    fn baudrate(&self) -> usize {
        // baud = fck / (8 * (2 - over8) * usartDiv)
        // baud * usartDiv = fck / (8*(2-over8))
        // usartDiv * (2-over8) = fck / (8*baud)

        // 60MHz/7

        /*
        let apbfreq = periph.apb.frequency();
        let fraction_divisor = (2 - (periph.base_address.control1.load() >> 15)) * 8;
        let (mantissa, fraction) = {
            let brr = periph.base_address.brr.load();
            (brr >> 4, brr & 0xF)
        };
        // (apbfreq*16) < 4GHz
        // apbfreq < (4GHz/16)
        // apbfreq < (1GHz/4)
        // apbfreq < 250MHz

        (apbfreq*fraction_divisor) / (mantissa*fraction_divisor + fraction)
        */
        0
    }
    fn open(&mut self) -> Result<(), String> {
        Ok(())
    }
    fn close(&mut self) {
    }
}
impl<'a> Read for Serial<'a> {
    fn read(&mut self, dest: &mut [u8]) -> Result<usize, Error> {

        Ok(0usize)
    }
}
impl<'a> Write for Serial<'a> {
    fn write(&mut self, dest: &[u8]) -> Result<usize, Error> {
        Ok(0usize)
    }
}
impl<'a> Drop for Serial<'a> {
    fn drop(&mut self) {
        self.close();
    }
}
impl<'a> Receive for Serial<'a> {
    fn on_recv(&mut self, s: Sender<()>) {

    }
}
