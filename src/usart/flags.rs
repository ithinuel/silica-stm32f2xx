/// Parity Error
pub const STATUS_PE:u16 = 0x0001;
/// Framing Error
pub const STATUS_FE:u16 = 0x0002;
/// Noise Error Flag
pub const STATUS_NE:u16 = 0x0004;
/// OverRun Error
pub const STATUS_ORE:u16 = 0x0008;
/// IDLE line detected
pub const STATUS_IDLE:u16 = 0x0010;
/// Read Data Register Not Empty
pub const STATUS_RXNE:u16 = 0x0020;
/// Transmission Complete
pub const STATUS_TC:u16 = 0x0040;
/// Transmit Data Register Empty
pub const STATUS_TXE:u16 = 0x0080;
/// LIN Break Detection Flag
pub const STATUS_LBD:u16 = 0x0100;
/// CTS Flag
pub const STATUS_CTS:u16 = 0x0200;

pub const DATA_REGISTER_MASK:u16 = 0x01FF;

/// Fraction of USARTDIV
pub const BAUDRATE_FRACTION_MASK:u16 = 0x000F;
/// Mantissa of USARTDIV
pub const BAUDRATE_MANTISSA_SHIFT:u16 = 4;
pub const BAUDRATE_MANTISSA_MASK:u16 = 0xFFF0;

/// Send Break
pub const CONTROL1_SBK:u16 = 0x0001;
/// Receiver wakeup
pub const CONTROL1_RWU:u16 = 0x0002;
/// Receiver Enable
pub const CONTROL1_RE:u16 = 0x0004;
/// Transmitter Enable
pub const CONTROL1_TE:u16 = 0x0008;
/// IDLE Interrupt Enable
pub const CONTROL1_IDLEIE:u16 = 0x0010;
/// RXNE Interrupt Enable
pub const CONTROL1_RXNEIE:u16 = 0x0020;
/// Transmission Complete Interrupt Enable
pub const CONTROL1_TCIE:u16 = 0x0040;
/// PE Interrupt Enable
pub const CONTROL1_TXEIE:u16 = 0x0080;
/// PE Interrupt Enable
pub const CONTROL1_PEIE:u16 = 0x0100;
/// Parity Selection
pub const CONTROL1_PS:u16 = 0x0200;
/// Parity Control Enable
pub const CONTROL1_PCE:u16 = 0x0400;
/// Wakeup method
pub const CONTROL1_WAKE:u16 = 0x0800;
/// Word length
pub const CONTROL1_NINEBITSWORD:u16 = 0x1000;
/// USART Enable
pub const CONTROL1_UE:u16 = 0x2000;
/// USART Oversampling by 8 enable
pub const CONTROL1_OVER8:u16 = 0x8000;

/// Address of the USART node
pub const CONTROL2_ADD:u16 = 0x000F;
/// LIN Break Detection Length
pub const CONTROL2_LBDL:u16 = 0x0020;
/// LIN Break Detection Interrupt Enable
pub const CONTROL2_LBDIE:u16 = 0x0040;
/// Last Bit Clock pulse
pub const CONTROL2_LBCL:u16 = 0x0100;
/// Clock Phase
pub const CONTROL2_CPHA:u16 = 0x0200;
/// Clock Polarity
pub const CONTROL2_CPOL:u16 = 0x0400;
/// Clock Enable
pub const CONTROL2_CLKEN:u16 = 0x0800;

/// STOP[1:0] bits (STOP bits)
pub const CONTROL2_STOP_MASK:u16 = 0x3000;
#[allow(non_camel_case_types)]
pub enum Control2_StopBits {
    OneBit = 0x0000,
    HalfBit = 0x1000,
    TwoBits = 0x2000,
    OneAndAHalfBit = 0x3000
}
/// LIN mode enable
pub const CONTROL2_LINEN:u16 = 0x4000;

/// Error Interrupt Enable
pub const CONTROL3_EIE:u16 = 0x0001;
/// IrDA mode Enable
pub const CONTROL3_IREN:u16 = 0x0002;
/// IrDA Low-Power
pub const CONTROL3_IRLP:u16 = 0x0004;
/// Half-Duplex Selection
pub const CONTROL3_HDSEL:u16 = 0x0008;
/// Smartcard NACK enable
pub const CONTROL3_NACK:u16 = 0x0010;
/// Smartcard mode enable
pub const CONTROL3_SCEN:u16 = 0x0020;
/// DMA Enable Receiver
pub const CONTROL3_DMAR:u16 = 0x0040;
/// DMA Enable Transmitter
pub const CONTROL3_DMAT:u16 = 0x0080;
/// RTS Enable
pub const CONTROL3_RTSE:u16 = 0x0100;
/// CTS Enable
pub const CONTROL3_CTSE:u16 = 0x0200;
/// CTS Interrupt Enable
pub const CONTROL3_CTSIE:u16 = 0x0400;
/// USART One bit method enable
pub const CONTROL3_ONEBIT:u16 = 0x0800;
