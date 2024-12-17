use register_macro::register;
use registers::BitField;
// Below are examples on how I will approach designing the register proc_macro
#[register(32, u32)]
struct UartTestControlReg {
    #[bits(0)]
    pub sw_rx_rst: bool,

    #[bits(1)]
    pub sw_tx_rst: bool,

    #[bits(2)]
    pub rx_en: bool,

    #[bits(3, default = true)]
    pub rx_dis: bool,

    #[bits(4)]
    pub tx_en: bool,

    #[bits(5, default = true)]
    pub tx_dis: bool,

    #[bits(6)]
    pub rst_to_cntr: bool,

    #[bits(7)]
    pub start_tx_brk: bool,

    #[bits(8)]
    pub stop_tx_brk: u8,
}

pub enum CharacterLength {
    EightBits = 0b00,
    SevenBits = 0b10,
    SixBits = 0b11,
}

pub enum Parity {
    EvenParity = 0b000,
    OddParity = 0b001,
    SpaceParity = 0b010,
    MarkParity = 0b011,
    NoParity = 0b100,
}

pub enum StopBits {
    OneBit = 0b00,
    OneAndHalfBits = 0b01,
    TwoBits = 0b10,
}

pub enum ChannelMode {
    Normal = 0b00,
    AutoEcho = 0b01,
    LocalLoopback = 0b10,
    RemoteLoopback = 0b11,
}

#[register(32, u32)]
struct UartTestModeReg {
    #[bits(0)]
    pub clk_src_sel: bool,

    #[bits(1:2, default = CharacterLength::EightBits, repr = u8)]
    pub char_len: CharacterLength,

    #[bits(3:5, default = Parity::NoParity, repr = u8)]
    pub parity: Parity,

    #[bits(6:7, default = StopBits::OneBit, repr = u8)]
    pub stop_bits: StopBits,

    #[bits(8:9, default = ChannelMode::Normal, repr = u8)]
    pub channel_mode: ChannelMode,
}
