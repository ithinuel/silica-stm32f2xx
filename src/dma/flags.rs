
pub mod interrupt {
    pub const FIFO_ERROR: u32 = 0b000001;
    pub const DIRECT_MODE_ERROR: u32 = 0b000100;
    pub const TRANSFERT_ERROR: u32 = 0b001000;
    pub const HALF_TRANSFERT: u32 = 0b010000;
    pub const TRANSFERT_COMPLETE: u32 = 0b100000;
    pub const SHIFT_FACTOR: u32 = 6;
}

pub mod stream {

    pub const CONFIG_CHANNEL_SELECTION_SHIFT: u32 = 25;
    pub const CONFIG_CHANNEL_SELECTION_MASK: u32 = 0x0E00_0000;
    pub const CONFIG_MEMORY_BURST_TRANSFERT_SHIFT: u32 = 23;
    pub const CONFIG_MEMORY_BURST_TRANSFERT_MASK: u32 = 0x0180_0000;
    pub const CONFIG_PERIPHERAL_BURST_TRANSFERT_SHIFT: u32 = 21;
    pub const CONFIG_PERIPHERAL_BURST_TRANSFERT_MASK: u32 = 0x0060_0000;
    #[allow(non_camel_case_types)]
    pub enum Config_BurstTransfert {
        Single = 0b00,
        Incr4 = 0b01,
        Incr8 = 0b10,
        Incr16 = 0b11
    }

    pub const CONFIG_CURRENT_TARGET_SHIFT: u32 = 19;
    pub const CONFIG_CURRENT_TARGET_MASK: u32 = 0x0008_0000;
    pub const CONFIG_DOUBLE_BUFFER_MODE: u32 = 0x0004_0000;
    pub const CONFIG_PRIORITY_LEVEL_MASK: u32 = 0x0003_0000;
    #[allow(non_camel_case_types)]
    pub enum Config_PriorityLevel {
        Low = 0x0000_0000,
        Medium = 0x0001_0000,
        High = 0x0002_0000,
        VeryHigh = 0x0003_0000
    }

    #[allow(non_camel_case_types)]
    pub enum Config_PeripheralIncrOffsetSize {
        LinkedToPSize = 0,
        FixedTo4Bytes = 0x0000_8000
    }
    pub const CONFIG_PERIPHERAL_INCREMENT_OFFSET_SIZE_MASK: u32 = 0x0000_8000;

    pub const CONFIG_MEMORY_DATA_SIZE_SHIFT: u32 = 13;
    pub const CONFIG_MEMORY_DATA_SIZE_MASK: u32 = 0x0000_6000;
    pub const CONFIG_PERIPHERAL_DATA_SIZE_SHIFT: u32 = 11;
    pub const CONFIG_PERIPHERAL_DATA_SIZE_MASK: u32 = 0x0000_1800;
    #[allow(non_camel_case_types)]
    pub enum Config_DataSize {
        Byte = 0b00,
        HalfWord = 0b01,
        Word = 0b10
    }

    pub const CONFIG_MEMORY_INCREMENT: u32 = 0x0000_0400;
    pub const CONFIG_PERIPHERAL_INCREMENT: u32 = 0x0000_0200;
    pub const CONFIG_CIRCULAR_MODE: u32 = 0x0000_0100;

    pub const CONFIG_TRANFERT_DIRECTION_MASK: u32 = 0x000000C0;
    #[derive(Clone)]
    #[allow(non_camel_case_types)]
    pub enum Config_TranfertDirection {
        PeripheralToMemory = 0x0000_0000,
        MemoryToPeripheral = 0x0000_0040,
        MemoryToMemory = 0x0000_0080,
    }

    pub const CONFIG_PERIPHERAL_FLOW_CONTROL: u32 = 0x0000_0020;
    pub const CONFIG_TRANFER_COMPLETE_INTERUPT_ENABLE: u32 = 0x0000_0010;
    pub const CONFIG_HALF_TRANFER_INTERUPT_ENABLE: u32 = 0x0000_0008;
    pub const CONFIG_TRANFER_ERROR_INTERUPT_ENABLE: u32 = 0x0000_0004;
    pub const CONFIG_DIRECT_MODE_INTERUPT_ENABLE: u32 = 0x0000_0002;
    pub const CONFIG_STREAM_ENABLE: u32 = 0x0000_0001;

    pub const FIFO_STATUS_MASK: u32 = 0x0000_0038;
    #[allow(non_camel_case_types)]
    pub enum Fifo_Status {
        Low = 0,
        Medium = 0x0000_0008,
        High = 0x0000_0010,
        VeryHigh = 0x0000_0018,
        Empty = 0x0000_0020,
        Full = 0x0000_0028
    }
    pub const FIFO_DIRECT_MODE_ENABLE: u32 = 0x0000_0004;
    pub const FIFO_THRESHOLD_SELECTION_MASK: u32 = 0x0000_0003;
    #[allow(non_camel_case_types)]
    pub enum FiFo_ThresholdSelection {
        OneQuarterFull = 0b00,
        HalfFull = 0b01,
        ThreeQuarterFull = 0b10,
        Full = 0b11
    }
}
