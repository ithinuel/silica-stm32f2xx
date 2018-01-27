#![feature(compiler_builtins_lib)]
#![feature(drop_types_in_const)]
#![feature(collections)]
#![feature(box_syntax)]
#![feature(const_fn)]
#![feature(linkage)]
#![feature(alloc)]
#![feature(asm)]

#![no_std]

extern crate silica;
#[macro_use]
extern crate alloc;
extern crate silica_cortexm3;
extern crate compiler_builtins;

#[macro_export]
macro_rules! init_peripherals {
    ( $( $x:expr ),* ) => {
        $(
            $x.init()?;
        )*
    }
}

#[macro_export]
macro_rules! init_optional_peripherals {
    ( $( $x:expr ),* ) => {
        $(
            if let Some(ref pin) = $x {
                init_peripherals![pin];
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

pub mod irq;

pub mod registers {
    pub use silica_cortexm3::{Ro, Rw, Wo};
}

pub struct AdvancedPeripheralBus {
    pub clock_prescaler: u32
}
