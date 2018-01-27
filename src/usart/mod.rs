use alloc::string::String;
use alloc::string::ToString;
use alloc::Vec;

use alloc::arc::Arc;

use silica::peripheral::Peripheral;
use silica::peripheral::serial::{BitCount, Parity, StopBit, Serial as ISerial};
use silica::sync::mpsc::Sender;
use silica::io::{Read, Write, Receive};

use dma::{DMAStreamPeripheral, PeriphDescriptor, MemDescriptor, FifoMode, FlowControl, BurstSize};
use gpio::PinPeripheral;
use irq::IRQType;
use registers::*;
use rcc;

mod flags;
pub use self::flags::*;

#[repr(C)]
pub struct USARTRegisters {
    status: Rw<u16>,
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
        let mut cr3 = 0;
        let mut cr1 = 0;

        // setup GPIOs
        init_optional_peripherals![self.pin_tx, self.pin_rx]; // data lines
        init_optional_peripherals![self.pin_dtr, self.pin_dcd, self.pin_dsr, self.pin_ri]; // sw flow control
        init_optional_peripherals![self.pin_rts, self.pin_cts]; // hw flow control

        // enable clock (RCC)
        init_peripherals![&self.clock];

        if self.pin_cts.is_some() {
            cr3 |= CONTROL3_CTSE;
        }
        if self.pin_rts.is_some() {
            cr3 |= CONTROL3_RTSE;
        }

        // setup DMA rx
        init_optional_peripherals![self.dma_rx];
        if self.dma_rx.is_some() {
            cr3 |= CONTROL3_DMAR;
        }

        // setup DMA tx
        init_optional_peripherals![self.dma_tx];
        if self.dma_tx.is_some() {
            cr3 |= CONTROL3_DMAT;
        }

        if self.pin_tx.is_some() {
            cr1 |= CONTROL1_TE;
        }
        if self.pin_rx.is_some() {
            cr1 |= CONTROL1_RE;
        }

        unsafe {
            (*self.base_address).control3.write(cr3);
            (*self.base_address).control1.write(cr1);
        }

        Ok(())
    }
}

pub struct Serial<'a> {
    periph: &'a USARTPeripheral<'a>,
    rx_buf: Option<Arc<Vec<u8>>>
}
impl<'a> Serial<'a> {
    pub fn from(f: &'a USARTPeripheral<'a>) -> Result<Serial<'a>, String> {
        f.init()?;

        let rx_buf = if let Some(ref dma_rx) = f.dma_rx {
            let mut buf = Arc::new(vec![0u8; 512]);
            let from = unsafe {
                let periph: &u16 = &(&*f.base_address).data;
                PeriphDescriptor {
                    data: periph as *const u16 as *mut u8,
                    burst: None,
                    circular_mode: false,
                    fifo_mode: FifoMode::Direct,
                    flow_ctrl: FlowControl::Peripheral
                }
            };
            let to = MemDescriptor {
                data: buf.clone(),
                burst: Some(BurstSize::Byte)
            };
            dma_rx.periph_to_mem(from, to)?;
            Some(buf)
        } else {
            None
        };

        Ok(Serial { periph: f, rx_buf: rx_buf })
    }

    fn read_data(&self) -> u8 {
        unsafe { (*self.periph.base_address).data.read() as u8 }
    }
    fn read_status(&self) -> u16 {
        unsafe { (*self.periph.base_address).status.read() }
    }
    fn write_data(&self, d: u8) {
        unsafe { (*self.periph.base_address).data.write(d as u16) };
    }
}
impl<'a> ISerial for Serial<'a> {
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
    fn open(&mut self, baudrate:usize, word_len: BitCount, parity: Parity, stop_bit: StopBit) -> Result<(), String> {
        let mut cr1 = 0;
        let mut cr2 = 0;

        cr2 |= match stop_bit {
            StopBit::OneBit => Control2_StopBits::OneBit,
            StopBit::OneAndAHalfBit => Control2_StopBits::OneAndAHalfBit,
            StopBit::TwoBits => Control2_StopBits::TwoBits
        } as u16;

        cr1 |= match word_len {
            BitCount::SevenBits => return Err("7 bits word len is not supported by this usart".to_string()),
            BitCount::EightBits => 0,
            BitCount::NineBits => {
                if parity != Parity::None {
                    return Err("Nine bit word is not available when parity is on".to_string())
                }
                CONTROL1_NINEBITSWORD
            }
        };

        cr1 |= match parity {
            Parity::None => 0,
            Parity::Even => CONTROL1_PCE | CONTROL1_NINEBITSWORD,
            Parity::Odd => CONTROL1_PCE | CONTROL1_PS | CONTROL1_NINEBITSWORD,
            Parity::Mark => return Err("mark parity bit is not supported by this usart".to_string()),
            Parity::Space => return Err("space parity bit is not supported by this usart".to_string()),
        };

        let clk = self.periph.clock.get_clock();

        let mut usartdiv_int = clk / (16 * baudrate);
        let mut usartdiv_frac = (clk - (usartdiv_int * 16 * baudrate)) / baudrate;

        let over8 = usartdiv_int == 0;
        if over8 {
            cr1 |= CONTROL1_OVER8;
            usartdiv_frac *= 2;
            usartdiv_int += usartdiv_frac >> 3;
            usartdiv_frac &= 0x7;
        } else {
            usartdiv_frac &= 0xF;
        }
        if usartdiv_int == 0 {
            return Err("This baudrate is too high for this serial port.".to_string());
        }
        if usartdiv_int >= 4096 {
            return Err("This baudrate is too low for this serial port.".to_string());
        }

        //let frac = usartdiv_frac * if over8 {8} else {16};

        let mantissa: u16 = usartdiv_int as u16;
        let div: u16 = usartdiv_frac as u16;

        unsafe {
            (*self.periph.base_address).baud_rate.write((mantissa << 4) | div);
            (*self.periph.base_address).control1.update(cr1 | CONTROL1_UE,
                CONTROL1_OVER8 | CONTROL1_PCE | CONTROL1_PS | CONTROL1_NINEBITSWORD | CONTROL1_UE);
            (*self.periph.base_address).control2.update(cr2, CONTROL2_STOP_MASK);
        }
        Ok(())
    }
    fn close(&mut self) {
        unsafe {
            (*self.periph.base_address).control1.update(0, CONTROL1_UE);
        }
    }
}
impl<'a> Read for Serial<'a> {
    fn read(&mut self, dest: &mut [u8]) -> Result<usize, String> {
        if dest.len() < 1 {
            return Ok(0)
        }
        let status = self.read_status();
        let data = self.read_data();

        if (status & STATUS_RXNE) != STATUS_RXNE {
            Ok(0)
        } else if (status & STATUS_ORE) == STATUS_ORE {
            Err("Serial overrun error !".to_string())
        } else if (status & STATUS_PE) == STATUS_PE {
            self.read_data(); // clearing sequence
            Err("Parity error !".to_string())
        } else {
            dest[0] = data;
            Ok(1)
        }
    }
}
impl<'a> Write for Serial<'a> {
    fn write(&mut self, dest: &[u8]) -> Result<usize, String> {
        // CTS flag's not available for uart 4 and 5
        self.read_status();
        for b in dest {
            self.write_data(*b);
            while (self.read_status() & STATUS_TC) != STATUS_TC {}
        }
        Ok(dest.len())
    }
}
impl<'a> Drop for Serial<'a> {
    fn drop(&mut self) {
        self.close();
    }
}
impl<'a> Receive for Serial<'a> {
    fn on_recv(&mut self, s: Sender<()>) {
        let _ = s;
    }
}
