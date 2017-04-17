#![feature(compiler_builtins_lib)]
#![feature(drop_types_in_const)]
#![feature(core_intrinsics)]
#![feature(collections)]
#![feature(const_fn)]
#![feature(linkage)]
#![feature(asm)]

#![no_std]

#![macro_use]

extern crate silica;
extern crate collections;
extern crate silica_cortexm3;
extern crate compiler_builtins;

macro_rules! init_peripheral {
    ( $( $x:expr ),* ) => {
        $(
            if let Some(ref pin) = $x {
                if let Err(msg) = pin.init() {
                    return Err(msg)
                }
            }
        )*
    }
}

/// GPIO control module
pub mod gpio;
/// DMA control module
pub mod dma;
/// USART (and UART) control module
pub mod usart;
/// Timer control module
pub mod timer;
/// RCC control module
pub mod rcc;
/// Flash control module
pub mod flash;


use silica_cortexm3::{Exceptions, Handler};
use collections::string::String;

extern "C" {
    static idata_from: usize;
    static idata_to: usize;
    static idata_size: usize;
    static bss_start: usize;
    static bss_size: usize;
}

pub mod registers {
    pub use silica_cortexm3::{Ro, Rw, Wo};
}

pub struct AdvancedPeripheralBus {
    pub clock_prescaler: u32
}

/// Peripheral trait
pub trait Peripheral {
    fn init(&self) -> Result<(), String>;
    fn deinit(&self) -> Result<(), String>;
}
extern "Rust" {
    fn main();
}

pub unsafe extern "C" fn start() -> ! {
    // initialize bss
    let _bss_start = &bss_start as *const usize as *mut u8;
    let _bss_size = &bss_size as *const usize as usize;
    core::intrinsics::write_bytes(_bss_start, 0, _bss_size);

    // initialize idata
    let _idata_from = &idata_from as *const usize as *const u8;
    let _idata_to = &idata_to as *const usize as *mut u8;
    let _idata_size = &idata_size as *const usize as usize;
    core::intrinsics::copy(_idata_from, _idata_to, _idata_size);

    // system init
    main();
    silica_cortexm3::ppb::scb::system_reset();
}

#[cfg(target_arch = "arm")]
pub unsafe extern "C" fn default_handler()  {
    let isr_id: u32;
    asm!("mrs $0, ipsr": "=r"(isr_id));
}

pub unsafe extern "C" fn hf_handler() {
}
pub unsafe extern "C" fn pendsv() {
}
pub unsafe extern "C" fn systick() {
}

#[allow(non_camel_case_types)]
pub enum IRQType {
    WWDG            = 0,
    PVD             = 1,
    TAMP_STAMP      = 2,
    RTC_WKUP        = 3,
    FLASH           = 4,
    RCC             = 5,
    EXTI0           = 6,
    EXTI1           = 7,
    EXTI2           = 8,
    EXTI3           = 9,
    EXTI4           = 10,
    DMA1_Stream0    = 11,
    DMA1_Stream1    = 12,
    DMA1_Stream2    = 13,
    DMA1_Stream3    = 14,
    DMA1_Stream4    = 15,
    DMA1_Stream5    = 16,
    DMA1_Stream6    = 17,
    ADC             = 18,
    CAN1_TX         = 19,
    CAN1_RX0        = 20,
    CAN1_RX1        = 21,
    CAN1_SCE        = 22,
    EXTI9_5         = 23,
    TIM1_BRK_TIM9   = 24,
    TIM1_UP_TIM10   = 25,
    TIM1_TRG_COM_TIM11 = 26,
    TIM1_CC         = 27,
    TIM2            = 28,
    TIM3            = 29,
    TIM4            = 30,
    I2C1_EV         = 31,
    I2C1_ER         = 32,
    I2C2_EV         = 33,
    I2C2_ER         = 34,
    SPI1            = 35,
    SPI2            = 36,
    USART1          = 37,
    USART2          = 38,
    USART3          = 39,
    EXTI15_10       = 40,
    RTC_Alarm       = 41,
    OTG_FS_WKUP     = 42,
    TIM8_BRK_TIM12  = 43,
    TIM8_UP_TIM13   = 44,
    TIM8_TRG_COM_TIM14 = 45,
    TIM8_CC         = 46,
    DMA1_Stream7    = 47,
    FSMC            = 48,
    SDIO            = 49,
    TIM5            = 50,
    SPI3            = 51,
    UART4           = 52,
    UART5           = 53,
    TIM6_DAC        = 54,
    TIM7            = 55,
    DMA2_Stream0    = 56,
    DMA2_Stream1    = 57,
    DMA2_Stream2    = 58,
    DMA2_Stream3    = 59,
    DMA2_Stream4    = 60,
    ETH             = 61,
    ETH_WKUP        = 62,
    CAN2_TX         = 63,
    CAN2_RX0        = 64,
    CAN2_RX1        = 65,
    CAN2_SCE        = 66,
    OTG_FS          = 67,
    DMA2_Stream5    = 68,
    DMA2_Stream6    = 69,
    DMA2_Stream7    = 70,
    USART6          = 71,
    I2C3_EV         = 72,
    I2C3_ER         = 73,
    OTG_HS_EP1_OUT  = 74,
    OTG_HS_EP1_IN   = 75,
    OTG_HS_WKUP     = 76,
    OTG_HS          = 77,
    DCMI            = 78,
    CRYP            = 79,
    HASH_RNG        = 80,
}

#[no_mangle]
#[linkage = "external"]
#[link_section = ".text.exceptions"]
pub static EXCEPTIONS: Exceptions = Exceptions {
    reset: start,  // RESET
    nmi: default_handler,   // NMI
    hard_fault: hf_handler,   // Hardfault
    mem_manage: default_handler,   // MemManage
    bus_fault: default_handler,   // BusFault
    usage_fault: default_handler,   // UsageFault
    reserved1: [0; 4],
    sv_call: default_handler,   // SVCall
    debug_monitor: default_handler,   // Debug Monitor
    reserved2: 0,
    pendsv: pendsv,   // PendSV
    systick: systick,   // Systick
};

#[no_mangle]
#[linkage = "external"]
#[link_section = ".text.isr"]
pub static ISRVEC: [Handler;60] = [
    default_handler,   // wwdg
    default_handler,   // PVD
    default_handler,   // TAMPER
    default_handler,   // RTC
    default_handler,   // FLASH
    default_handler,   // RCC
    default_handler,   // EXTI0
    default_handler,   // EXTI1
    default_handler,   // EXTI2
    default_handler,   // EXTI3
    default_handler,   // EXTI4
    default_handler,   // DMA1_Channel1
    default_handler,   // DMA1_Channel2
    default_handler,   // DMA1_Channel3
    default_handler,   // DMA1_Channel4
    default_handler,   // DMA1_Channel5
    default_handler,   // DMA1_Channel6
    default_handler,   // DMA1_Channel7
    default_handler,   // ADC1_2
    default_handler,   // USB_HP_CAN_TX
    default_handler,   // USN_LP_CAN_RX0
    default_handler,   // CAN_RX1
    default_handler,   // CAN_SCE
    default_handler,   // EXTI9_5
    default_handler,   // TIM1_BRK
    default_handler,   // TIM1_UP
    default_handler,   // TIM1_TRG_COM
    default_handler,   // TIM1_CC
    default_handler,   // TIM2
    default_handler,   // TIM3
    default_handler,   // TIM4
    default_handler,   // I2C1_EV
    default_handler,   // I2C1_ER
    default_handler,   // I2C2_EV
    default_handler,   // I2C2_ER
    default_handler,   // SPI1
    default_handler,   // SPI2
    default_handler,   // USART1
    default_handler,   // USART2
    default_handler,   // USART3
    default_handler,   // EXTI15_10
    default_handler,   // RTCAlarm
    default_handler,   // USBWakeUp
    default_handler,   // TIIM8_BRK
    default_handler,   // TIIM8_UP
    default_handler,   // TIIM8_TRG_COM
    default_handler,   // TIIM8_CC
    default_handler,   // ADC3
    default_handler,   // FSMC
    default_handler,   // SDIO
    default_handler,   // TIM5
    default_handler,   // SPI3
    default_handler,   // UART4
    default_handler,   // UART5
    default_handler,   // TIM6
    default_handler,   // TIM7
    default_handler,   // DMA1_Channel1
    default_handler,   // DMA1_Channel2
    default_handler,   // DMA1_Channel3
    default_handler,   // DMA1_Channel4_5
];
