mod flags;
pub use self::flags::*;

use registers::*;

#[repr(C)]
pub struct FlashRegisters {
    pub access_control: Rw<u32>,
    pub key: Wo<u32>,
    pub option_key: Wo<u32>,
    pub status: Rw<u32>,
    pub control: Rw<u32>,
    pub option_control: Rw<u32>
}

extern {
    pub fn flash_get() -> &'static mut FlashRegisters;
}
