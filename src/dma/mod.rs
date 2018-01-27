use alloc::Vec;
use alloc::arc::Arc;
use alloc::string::String;

use silica::peripheral::Peripheral;

use rcc;
use irq::IRQType;
use registers::*;

mod flags;
pub use self::flags::*;
pub use self::flags::stream::Config_PriorityLevel as Priority;

pub struct DMAStreamRegisters {
    pub config: Rw<u32>,
    pub number_of_data: Rw<u32>,
    pub peripheral_addr: Rw<u32>,
    pub memory_0_addr: Rw<u32>,
    pub memory_1_addr: Rw<u32>,
    pub fifo_control: Rw<u32>
}

pub struct DMARegisters {
    pub interrupt_status_low: Ro<u32>,
    pub interrupt_status_high: Ro<u32>,
    pub interrupt_flag_clear_low: Wo<u32>,
    pub interrupt_flag_clear_high: Wo<u32>,
    pub streams: [DMAStreamRegisters; 8]
}

pub enum Channel {
    Channel0 = 0,
    Channel1 = 1,
    Channel2 = 2,
    Channel3 = 3,
    Channel4 = 4,
    Channel5 = 5,
    Channel6 = 6,
    Channel7 = 7
}
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Stream {
    Stream0 = 0,
    Stream1 = 1,
    Stream2 = 2,
    Stream3 = 3,
    Stream4 = 4,
    Stream5 = 5,
    Stream6 = 6,
    Stream7 = 7,
}

pub struct DMAPeripheral {
    pub base_address: *mut DMARegisters,
    pub clock: rcc::RCCPeripheral
}
unsafe impl Sync for DMAPeripheral {}
impl Peripheral for DMAPeripheral {
    fn init(&self) -> Result<(), String> {
        init_peripherals![self.clock];
        Ok(())
    }
}

pub enum DataSize {
    Byte = 1,
    HalfWord = 2,
    Word = 4
}
pub enum BurstSize {
    Byte = 1,
    Word = 4,
    DoubleWord = 8,
    QuadWord = 16
}

pub enum IncOffset {
    PSize,
    Word
}

pub enum FifoThreshold {
    OneQuarterFull,
    HalfFull,
    ThreeQuarterFull,
    Full
}

pub enum FifoMode {
    Direct,
    Fifo(FifoThreshold)
}

pub enum FlowControl {
    Peripheral,
    DMA
}

pub struct PeriphDescriptor<T: DMAAble> {
    pub data: *mut T,
    pub burst: Option<(BurstSize, IncOffset)>,
    pub circular_mode: bool,
    pub fifo_mode: FifoMode,
    pub flow_ctrl: FlowControl
}

pub struct MemDescriptor<T: DMAAble> {
    pub data: Arc<Vec<T>>,
    pub burst: Option<BurstSize>
}

pub struct DMAStreamPeripheral<'a> {
    pub dma: &'a DMAPeripheral,
    pub isr_id: IRQType,
    pub stream: Stream,
    pub channel: Channel,
    pub priority: Priority
}

pub trait DMAAble {
    fn datasize(&self) -> DataSize;
}

macro_rules! impl_DMAAble {
    ($id:ident, $datasize:expr) => {
        impl DMAAble for $id {
            fn datasize(&self) -> DataSize {
                $datasize
            }
        }
    }
}
impl_DMAAble!(u32, DataSize::Word);
impl_DMAAble!(u16, DataSize::HalfWord);
impl_DMAAble!(u8, DataSize::Byte);
impl_DMAAble!(i32, DataSize::Word);
impl_DMAAble!(i16, DataSize::HalfWord);
impl_DMAAble!(i8, DataSize::Byte);

unsafe impl<'a> Sync for DMAStreamPeripheral<'a> {}
impl<'a> DMAStreamPeripheral<'a> {
    fn registers(&self) -> &DMAStreamRegisters {
        unsafe { &(&*self.dma.base_address).streams[self.stream as usize] }
    }
    fn registers_mut(&self) -> &mut DMAStreamRegisters {
        unsafe { &mut (&mut *self.dma.base_address).streams[self.stream as usize] }
    }
    fn setup<T, U>(&self, dir: stream::Config_TranfertDirection,
        from: (*const T, Option<BurstSize>, Option<IncOffset>),
        to: (*const U, Option<BurstSize>, Option<IncOffset>)) -> Result<(), String> {

        let mut config = (self.stream as u32) << stream::CONFIG_CHANNEL_SELECTION_SHIFT | (dir.clone() as u32);

        config |= match dir {
            stream::Config_TranfertDirection::PeripheralToMemory => {0},
            stream::Config_TranfertDirection::MemoryToPeripheral => {0},
            stream::Config_TranfertDirection::MemoryToMemory => {0}
        };

        self.registers_mut().config.update(
            stream::CONFIG_STREAM_ENABLE,
            stream::CONFIG_CHANNEL_SELECTION_MASK |
            stream::CONFIG_MEMORY_BURST_TRANSFERT_MASK |
            stream::CONFIG_PERIPHERAL_BURST_TRANSFERT_MASK |
            stream::CONFIG_CURRENT_TARGET_MASK |
            stream::CONFIG_DOUBLE_BUFFER_MODE |
            stream::CONFIG_PRIORITY_LEVEL_MASK |
            stream::CONFIG_PERIPHERAL_INCREMENT_OFFSET_SIZE_MASK |
            stream::CONFIG_MEMORY_DATA_SIZE_MASK |
            stream::CONFIG_PERIPHERAL_DATA_SIZE_MASK |
            stream::CONFIG_MEMORY_INCREMENT |
            stream::CONFIG_PERIPHERAL_INCREMENT |
            stream::CONFIG_CIRCULAR_MODE |
            stream::CONFIG_TRANFERT_DIRECTION_MASK |
            stream::CONFIG_PERIPHERAL_FLOW_CONTROL |
            stream::CONFIG_TRANFER_COMPLETE_INTERUPT_ENABLE |
            stream::CONFIG_HALF_TRANFER_INTERUPT_ENABLE |
            stream::CONFIG_TRANFER_ERROR_INTERUPT_ENABLE |
            stream::CONFIG_DIRECT_MODE_INTERUPT_ENABLE |
            stream::CONFIG_STREAM_ENABLE
        );

        Ok(())
    }

    pub fn mem_to_periph<T, U>(&self, from: MemDescriptor<T>, to: PeriphDescriptor<U>) -> Result<(), String>
        where T: DMAAble, U: DMAAble {
    /*    let (pburst, pinc) = if let Some((burst, inc)) = from.burst {
            (Some(burst), Some(inc))
        } else {
            (None, None)
        };
        let (mburst, minc) = if let Some(burst, inc) = to.burst {
            (Some(burst), Some(inc))
        } else {
            (None, None)
        };

        self.setup(stream::Config_TranfertDirection::MemoryToPeripheral,
            (Arc::into_raw(from.data), mburst, minc),
            (to.data, pburst, pinc));

*/
        unimplemented!()
    }
    pub fn periph_to_mem<T, U>(&self, from: PeriphDescriptor<U>, to: MemDescriptor<T>) -> Result<(), String>
        where T: DMAAble, U: DMAAble {
    /*    let (pburst, pinc) = if let Some((burst, inc)) = from.burst {
            (Some(burst), Some(inc))
        } else {
            (None, None)
        };
        let (mburst, minc) = if let Some(burst, inc) = to.burst {
            (Some(burst), Some(inc))
        } else {
            (None, None)
        };

        self.setup(stream::Config_TranfertDirection::PeripheralToMemory,
            (from.data, pburst, pinc),
            (Arc::into_raw(to.data), mburst, minc));

        // never forget to Arc::from_raw(ptr) or this will lead to a memory leak
*/
        unimplemented!()
    }
    pub fn mem_to_mem<T, U>(&self, from: Arc<Vec<T>>, to: Arc<Vec<T>>, burst: Option<BurstSize>) -> Result<(), String>
        where T: DMAAble, U: DMAAble {

        unimplemented!()
    }
    pub fn start(&self) -> Result<(), String> {
        self.registers_mut().config.update(stream::CONFIG_STREAM_ENABLE, stream::CONFIG_STREAM_ENABLE);
        Ok(())
    }
    pub fn is_enabled(&self) -> bool {
        (self.registers().config.read() & stream::CONFIG_STREAM_ENABLE) == stream::CONFIG_STREAM_ENABLE
    }
    pub fn ndata(&self) -> Result<usize, String> {
        Ok(self.registers().number_of_data.read() as usize)
    }
    pub fn stop(&self) -> Result<(), String> {
        self.registers_mut().config.update(0, stream::CONFIG_STREAM_ENABLE);
        Ok(())
    }
}
impl<'a> Peripheral for DMAStreamPeripheral<'a> {
    fn init(&self) -> Result<(), String> {
        init_peripherals![self.dma];

        Ok(())
    }
}
impl<'a> Drop for DMAStreamPeripheral<'a> {
    fn drop(&mut self) {
        unimplemented!()
    }
}
