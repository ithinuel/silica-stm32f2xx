use collections::string::String;

use registers::*;
use Peripheral;

const APBAHB_PRESCALER_TABLE: [u8; 16] = [0, 0, 0, 0, 1, 2, 3, 4, 1, 2, 3, 4, 6, 7, 8, 9];

extern {
    pub fn rcc_get() -> *mut RCCRegisters;
}
static mut HSI_CLOCK: usize = 0;
static mut HSE_CLOCK: usize = 0;

// _ = reserved
// r = ro
// w = rw
// W = wo
// control  ____rwrw____wwrwrrrrrrrrwwwww_rw
// pllcfgr  ____wwww_w____ww_wwwwwwwwwwwwwww
// cfgr     wwwwwwwwwwwwwwwwwwwwww__wwwwrrww
// cir      ________w_wwwwww__wwwwwwr_rrrrrr
// ahb1rstr __w___w__ww________w___wwwwwwwww
// ahb2rstr ________________________wwww___w
// ahb3rstr _______________________________w
// apb1rstr __ww_ww_wwwwwww_ww__w__wwwwwwwww
// apb2rstr _____________www_w_ww__w__ww__ww
// ahb1enr  _wwwwww__ww__w_____w___wwwwwwwww
// ahb2enr  ________________________wwww___w
// ahb3enr  _______________________________w
// apb1enr  __ww_ww_wwwwwww_ww__w__wwwwwwwww
// apb2enr  _____________www_w_wwwww__ww__ww
// ahb1lpenr_wwwwww__ww__wwww__w___wwwwwwwww
// ahb2lpenr________________________wwww___w
// ahb3lpenr_______________________________w
// apb1lpenr__wwwww_wwwwwww_ww__w__wwwwwwwww
// apb2lpenr_____________www_w_wwwww__ww__ww
// bdcr     _______________ww_____ww_____wrw
// cs       rrrrrrrw______________________rw
// sscgr    ww__wwwwwwwwwwwwwwwwwwwwwwwwwwww
// plli2scfg_www_____________wwwwwwwww______

#[repr(C)]
pub struct RCCRegisters {
    control: Rw<u32>,
    pll_config: Rw<u32>,
    config: Rw<u32>,
    clock_interrupt: Rw<u32>,
    ahb1_reset: Rw<u32>,
    ahb2_reset: Rw<u32>,
    ahb3_reset: Rw<u32>,
    reserved0: u32,
    apb1_reset: Rw<u32>,
    apb2_reset: Rw<u32>,
    reserved1: u32,
    reserved2: u32,
    ahb1_clock_enable: Rw<u32>,
    ahb2_clock_enable: Rw<u32>,
    ahb3_clock_enable: Rw<u32>,
    reserved3: u32,
    apb1_clock_enable: Rw<u32>,
    apb2_clock_enable: Rw<u32>,
    reserved4: u32,
    reserved5: u32,
    ahb1_lowpower_clock: Rw<u32>,
    ahb2_lowpower_clock: Rw<u32>,
    ahb3_lowpower_clock: Rw<u32>,
    reserved6: u32,
    apb1_lowpower_clock: Rw<u32>,
    apb2_lowpower_clock: Rw<u32>,
    reserved7: u32,
    reserved8: u32,
    backup_domain_control: Rw<u32>,
    control_and_status: Rw<u32>,
    reserved9: u32,
    reserved10: u32,
    spread_spectrum: Rw<u32>,
    pll_i2s_cfg: Rw<u32>
}

pub struct RCCPeripheral {
    pub rcc: *mut RCCRegisters,
    pub clock: Clock
}
impl RCCPeripheral {
    pub fn set_clocks(hse: usize, hsi: usize) {
        unsafe {
            HSE_CLOCK = hse;
            HSI_CLOCK = hsi;
        }
    }
}
impl RCCPeripheral {
    fn set_clock_enable(&self, reg: u8, bit: u32, mask: u32) {
        unsafe {
            let rcc = &mut *self.rcc;
            match reg {
                0 => rcc.ahb1_clock_enable.update(bit, mask),
                1 => rcc.ahb2_clock_enable.update(bit, mask),
                2 => rcc.ahb3_clock_enable.update(bit, mask),
                3 => rcc.apb1_clock_enable.update(bit, mask),
                4 => rcc.apb2_clock_enable.update(bit, mask),
                _ => {}
            }
        }
    }
    pub fn get_clock(&self) -> usize {
        let (hse, hsi) = unsafe {
            (HSE_CLOCK, HSI_CLOCK)
        };
        let cfgr = unsafe { (*self.rcc).config.read() } as usize;
        let pllcfgr = unsafe { (*self.rcc).pll_config.read() } as usize;

        let sysclock = match cfgr & 0xC {
            0 => hsi,
            4 => hse,
            8 => {
                let source: usize = if (pllcfgr >> 22) == 1 {
                    hse
                } else {
                    hsi
                };
                let pllp = (((pllcfgr >> 16) & 0x03) + 1) * 2;
                let plln = (pllcfgr >> 6) & 0x1FF;
                let pllm = pllcfgr & 0x3F;
                ((source / pllm) * (plln)) / pllp
            }
            _ => hsi
        };

        // get bus from peripheral
        let hpre = (cfgr >> 4) & 0x0F;
        let hclk = sysclock << APBAHB_PRESCALER_TABLE[hpre];

        match (self.clock as u8) >> 5 {
            3 => {
                let ppre = ((cfgr >> 10)) & 0x07;
                hclk << APBAHB_PRESCALER_TABLE[ppre]
            }
            4 => {
                let ppre = ((cfgr >> 13)) & 0x07;
                hclk << APBAHB_PRESCALER_TABLE[ppre]
            }
            _ => hclk
        }
    }
}
impl Peripheral for RCCPeripheral {
    fn init(&self) -> Result<(), String> {
        let reg = (self.clock as u8) >> 5;
        let mask = (1 << (self.clock as u8)) & 0x1F;

        self.set_clock_enable(reg, mask, mask);

        Ok(())
    }

    fn deinit(&self) -> Result<(), String> {
        let reg = (self.clock as u8) >> 5;
        let mask = (1 << (self.clock as u8)) & 0x1F;

        self.set_clock_enable(reg, 0, mask);

        Ok(())
    }
}

pub enum Reset {

}

#[derive(Copy, Clone)]
pub enum Clock {
    TIM11 = 0x92,
    TIM10 = 0x91,
    TIM9 = 0x90,
    SYSCFG = 0x8E,
    SPI1 = 0x8C,
    SDIO = 0x8B,
    ADC3 = 0x8A,
    ADC2 = 0x89,
    ADC1 = 0x88,
    USART6 = 0x85,
    USART1 = 0x84,
    TIM8 = 0x81,
    TIM1 = 0x80,
    DAC = 0x7D,
    PWR = 0x7C,
    CAN2 = 0x7A,
    CAN1 = 0x79,
    I2C3 = 0x77,
    I2C2 = 0x76,
    I2C1 = 0x75,
    UART5 = 0x74,
    UART4 = 0x73,
    USART3 = 0x72,
    USART2 = 0x71,
    SPI3 = 0x6F,
    SPI2 = 0x6E,
    WWDG = 0x6B,
    TIM14 = 0x68,
    TIM13 = 0x67,
    TIM12 = 0x66,
    TIM7 = 0x65,
    TIM6 = 0x64,
    TIM5 = 0x63,
    TIM4 = 0x62,
    TIM3 = 0x61,
    TIM2 = 0x60,
    FSMC = 0x40,
    OTGFS = 0x27,
    RNG = 0x26,
    HASH = 0x25,
    CRYP = 0x24,
    DCMI = 0x20,
    OTGHSULPI = 0x1E,
    OTGHS = 0x1D,
    ETHMACPTP = 0x1C,
    ETHMACRX = 0x1B,
    ETHMACTX = 0x1A,
    ETHMAC = 0x19,
    DMA2 = 0x16,
    DMA1 = 0x15,
    BKPSRAM = 0x12,
    CRC = 0x0C,
    GPIOI = 0x08,
    GPIOH = 0x07,
    GPIOG = 0x06,
    GPIOF = 0x05,
    GPIOE = 0x04,
    GPIOD = 0x03,
    GPIOC = 0x02,
    GPIOB = 0x01,
    GPIOA = 0x00
}
pub enum LowPowerLock {

}
