use alloc::string::String;
use silica::peripheral::Peripheral;

mod flags;
pub use self::flags::*;

use registers::*;

const APBAHB_PRESCALER_TABLE: [usize; 8] = [2, 4, 8, 16, 64, 128, 256, 512];

static mut SYSCLOCK: usize = 8_000_000;

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

extern {
    pub fn rcc_get() -> &'static mut RCCRegisters;
}

pub enum ClockSelection {
    HSE(u32),
    HSI
}

pub enum PLL {
    Off,
    On(u8, u8, u8, u8)
}

fn wait_for(status: &Fn() -> bool) -> bool {
    let mut ret = false;
    let mut timeout_cnt = 0x500;

    while (!ret) && (timeout_cnt != 0) {
        ret = status();
        timeout_cnt -= 1;
    }
    ret
}

// pll input
// sw
// rtcsel
// I2SSRC
// AHB presc
// APBx presc
// eth ptp clock
#[allow(non_snake_case)]
pub fn system_init(clksrc: ClockSelection, pll: PLL,
                   hpre: CFGR_HPrescaler,
                   apb1: CFGR_PPrescaler1,
                   apb2: CFGR_PPrescaler2) -> Result<u32, &'static str> {
    let mut sysclock = 16_000_000;
    let mut source = CFGR_SW_HSI;
    let mut source_status = CFGR_SWS_HSI;
    let mut pll_source = PLLCFGR_SRC_HSI;

    let rcc = unsafe { rcc_get() };

    // set hsi on
    rcc.control.update(CR_HSION, CR_HSION);

    // reset RCC config
    rcc.config.write(0);

    // reset HSEON, CSSON & PLLON
    rcc.control.update(0, CR_HSEON | CR_CSSON | CR_PLLON);

    rcc.pll_config.write(0x24003010); // 0x24003010 is its reset value

    rcc.control.update(0, CR_HSEBYP);

    rcc.clock_interrupt.write(0);

    let rdyflag = match clksrc {
        ClockSelection::HSI => {
            CR_HSIRDY
        }
        ClockSelection::HSE(clk) => {
            if (clk < 4_000_000) || (26_000_000 < clk ) {
                return Err("External clock speed out of range ([4; 26] MHz).");
            }

            sysclock = clk;
            source = CFGR_SW_HSE;
            source_status = CFGR_SWS_HSE;
            pll_source = PLLCFGR_SRC_HSE;
            rcc.control.update(CR_HSEON, CR_HSEON);
            CR_HSERDY
        }
    };

    let status = wait_for(&|| (rcc.control.read() & rdyflag) ==  rdyflag);
    if !status {
        return Err("Failed to initialize the clock.");
    }

    rcc.config.update(
        (hpre as u32) | (apb1 as u32) | (apb2 as u32),
        CFGR_HPRE_MASK | CFGR_PPRE1_MASK | CFGR_PPRE2_MASK
    );

    if let PLL::On(M, N, P, Q) = pll {
        let (M, N, P, Q)= (M as u32, N as u32, P as u32, Q as u32);
        source = CFGR_SW_PLL;
        source_status = CFGR_SWS_PLL;

        if (M < 2) || (N < 2) || (Q < 2) {
            return Err("M, N and Q must be > 2.");
        }

        if (P != 2) && (P != 4) && (P != 6) && (P != 8) {
            return Err("P must be 2, 4, 6 or 8.");
        }

        sysclock /= M;

        if (sysclock < 1_000_000) || (2_000_000 < sysclock ) {
            return Err("PLL input must be in [1; 2]MHz.");
        }

        sysclock *= N;

        if (sysclock < 192_000_000) || (432_000_000 < sysclock ) {
            return Err("VCO output must be in [192; 432]MHz.");
        }

        let usbotgclk = sysclock / Q;
        sysclock /= P;

        if 120_000_000 < sysclock {
            return Err("PLL output clock must not exceed 120MHz.");
        }
        if 48_000_000 < usbotgclk {
            return Err("USB OTG output clock must not exceed 48MHz.");
        }

        rcc.pll_config.write(
            M |
            (N << PLLCFGR_N_SHIFT) |
            (((P>>1) - 1) << PLLCFGR_P_SHIFT) |
            pll_source |
            (Q << PLLCFGR_Q_SHIFT)
        );
        rcc.control.update(CR_PLLON, CR_PLLON);

        // no time out here in the peripheral lib .. hm
        while (rcc.control.read() & CR_PLLRDY) != CR_PLLRDY {}
    }

    // latency should be selected according to power supply voltage & clock freq
    {
        use ::flash::*;
        let flash = unsafe { flash_get() } ;
        flash.access_control.update(
            ACR_ICEN | ACR_DCEN | ACR_PRFTEN | 3,
            ACR_ICEN | ACR_DCEN | ACR_PRFTEN | ACR_LATENCY_MASK
        );
    }

    rcc.config.update(0, CFGR_SW_MASK);
    rcc.config.update(source, CFGR_SW_MASK);

    // no time out here in the peripheral lib .. hm
    while (rcc.config.read() & CFGR_SWS_MASK) != source_status {}

    unsafe {
        SYSCLOCK = sysclock as usize;
    }
    Ok(sysclock)
}

pub struct RCCPeripheral {
    pub rcc: *mut RCCRegisters,
    pub clock: Clock
}
unsafe impl Sync for RCCPeripheral {}
impl RCCPeripheral {
    fn set_clock(&self, v: bool) {
        unsafe {
            let reg = (self.clock as u8) >> 5;
            let mask = 1 << ((self.clock as u8) & 0x1F);
            let bit = if v {mask} else {0};
            let rcc = &mut *self.rcc;
            match reg {
                0 => &mut rcc.ahb1_clock_enable,
                1 => &mut rcc.ahb2_clock_enable,
                2 => &mut rcc.ahb3_clock_enable,
                3 => &mut rcc.apb1_clock_enable,
                4 => &mut rcc.apb2_clock_enable,
                _ => { panic!("RCC: Unknown peripheral") }
            }.update(bit, mask);
        }
    }
    pub fn get_clock(&self) -> usize {
        unsafe {
            // critical_section_start
            let mut clock = SYSCLOCK;
            let config = (&mut *self.rcc).config.read();
            let hpre = (config >> 4) & 0xF;
            clock /= if hpre < 0b1000 {
                1
            } else {
                APBAHB_PRESCALER_TABLE[(hpre-0b1000) as usize]
            };

            let reg = (self.clock as u8) >> 5;
            let ppre = if reg == 3 {
                config >> 10
            } else if reg == 4 {
                config >> 13
            } else {
                panic!("RCC: Unknown peripheral")
            } & 0x7;

            clock /= if ppre < 0b100 {
                1
            } else {
                APBAHB_PRESCALER_TABLE[(ppre-0b100) as usize]
            };

            clock
            // critical_section_end
        }
    }
}
impl Peripheral for RCCPeripheral {
    fn init(&self) -> Result<(), String> {
        self.set_clock(true);
        Ok(())
    }
}
impl Drop for RCCPeripheral {
    fn drop(&mut self) {
        self.set_clock(false);
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
