/// Special Function Registers, 0x00 to 0x3F
#[derive(Clone, Copy, PartialEq, Eq, Debug, strum::FromRepr, strum::Display, strum::EnumString)]
#[repr(u8)]
pub enum Sfr {
    #[strum(serialize = "PC", to_string = "SFR_PRG_COUNT")]
    ProgramCounter = 0x02,
    /// Status Register (SR)
    /// SR.TR (SB_EN_TOUT_RST): Timeout Reset Enable
    /// SR.SU (SB_STACK_USED): Stack Used
    /// SR.GY (SB_GP_BIT_Y): General Purpose Bit Y
    /// SR.Z (SB_FLAG_Z): Zero Flag
    /// SR.GX (SB_GP_BIT_X): General Purpose Bit X
    /// SR.C (SB_FLAG_C): Carry Flag
    #[strum(serialize = "SR", to_string = "SFR_STATUS_REG")]
    Status = 0x03,

    #[strum(serialize = "IP1", to_string = "SFR_INDIR_PORT")]
    IndirPort1 = 0x00,
    #[strum(serialize = "IP2", to_string = "SFR_INDIR_PORT2")]
    IndirPort2 = 0x01,
    /// Indirect Address Register 1, aka. F1
    #[strum(serialize = "IA1", to_string = "SFR_INDIR_ADDR")]
    IndirAddr1 = 0x04,
    /// Indirect Address Register 2, auto-incremented after each access
    #[strum(serialize = "IA2", to_string = "SFR_INDIR_ADDR2")]
    IndirAddr2 = 0x09,

    #[strum(serialize = "TMRCNT", to_string = "SFR_TMR0_COUNT")]
    TimerCount = 0x05,
    /// Timer Control Register (TMRCTL)
    /// TMRCTL.L1E (SB_EN_LEVEL1): Enable Level 1
    /// TMRCTL.L0E (SB_EN_LEVEL0): Enable Level 0
    /// TMRCTL.T0E (SB_TMR0_ENABLE): Timer 0 Enable
    /// TMRCTL.T0OE (SB_TMR0_OUT_EN): Timer 0 Output Enable
    /// TMRCTL.T0M (SB_TMR0_MODE): Timer 0 Mode
    /// TMRCTL.T0F2 (SB_TMR0_FREQ2): Timer 0 Frequency Bit 2
    /// TMRCTL.T0F1 (SB_TMR0_FREQ1): Timer 0 Frequency Bit 1
    /// TMRCTL.T0F0 (SB_TMR0_FREQ0): Timer 0 Frequency Bit 0
    #[strum(serialize = "TMRCTL", to_string = "SFR_TIMER_CTRL")]
    TimerCtrl = 0x06,
    #[strum(serialize = "TMRINIT", to_string = "SFR_TMR0_INIT")]
    TimerInit = 0x07,
    /// Encoding Bit Period Register, readable and writable by Host MCU
    /// Bit Cycle Register (BITCYC)
    /// BITCYC.TXO (SB_BIT_TX_O0): Bit Transmit Output 0
    /// BITCYC.C6 (SB_BIT_CYCLE_6): Bit Cycle 6
    /// BITCYC.C5 (SB_BIT_CYCLE_5): Bit Cycle 5
    /// BITCYC.C4 (SB_BIT_CYCLE_4): Bit Cycle 4
    /// BITCYC.C3 (SB_BIT_CYCLE_3): Bit Cycle 3
    /// BITCYC.C2 (SB_BIT_CYCLE_2): Bit Cycle 2
    /// BITCYC.C1 (SB_BIT_CYCLE_1): Bit Cycle 1
    /// BITCYC.C0 (SB_BIT_CYCLE_0): Bit Cycle 0
    #[strum(serialize = "BITCYC", to_string = "SFR_BIT_CYCLE")]
    BitCycle = 0x08,
    /// Port Direction Register (PDIR)
    /// PDIR.M3 (SB_PORT_MOD3): Port Mode 3
    /// PDIR.M2 (SB_PORT_MOD2): Port Mode 2
    /// PDIR.M1 (SB_PORT_MOD1): Port Mode 1
    /// PDIR.M0 (SB_PORT_MOD0): Port Mode 0
    /// PDIR.PU1 (SB_PORT_PU1): Port Pull-Up 1
    /// PDIR.PU0 (SB_PORT_PU0): Port Pull-Up 0
    /// PDIR.D1 (SB_PORT_DIR1): Port Direction 1
    /// PDIR.D0 (SB_PORT_DIR0): Port Direction 0
    #[strum(serialize = "PDIR", to_string = "SFR_PORT_DIR")]
    PortDir = 0x0A,
    /// Port I/O Register (PIO)
    /// PIO.INXOR (SB_PORT_IN_XOR): Port Input XOR
    /// PIO.RXI (SB_BIT_RX_I0): Bit Receive Input 0
    /// PIO.IN1 (SB_PORT_IN1): Port Input 1
    /// PIO.IN0 (SB_PORT_IN0): Port Input 0
    /// PIO.XOR1 (SB_PORT_XOR1): Port XOR 1
    /// PIO.XOR0 (SB_PORT_XOR0): Port XOR 0
    /// PIO.OUT1 (SB_PORT_OUT1): Port Output 1
    /// PIO.OUT0 (SB_PORT_OUT0): Port Output 0
    #[strum(serialize = "PIO", to_string = "SFR_PORT_IO")]
    PortIO = 0x0B,

    /// Bit Configuration Register (BITCFG)
    /// BITCFG.TXE (SB_BIT_TX_EN): Bit Transmit Enable
    /// BITCFG.CMOD (SB_BIT_CODE_MOD): Bit Code Mode
    /// BITCFG.EDGE (SB_PORT_IN_EDGE): Port Input Edge
    /// BITCFG.TAIL (SB_BIT_CYC_TAIL): Bit Cycle Tail
    /// BITCFG.CC6 (SB_BIT_CYC_CNT6): Bit Cycle Count 6
    /// BITCFG.CC5 (SB_BIT_CYC_CNT5): Bit Cycle Count 5
    /// BITCFG.CC4 (SB_BIT_CYC_CNT4): Bit Cycle Count 4
    /// BITCFG.CC3 (SB_BIT_CYC_CNT3): Bit Cycle Count 3
    #[strum(serialize = "BITCFG", to_string = "SFR_BIT_CONFIG")]
    BitConfig = 0x0C,
    /// System Configuration Register (SYSCFG)
    /// SYSCFG.INTQ (SB_INT_REQ): Interrupt Request
    /// SYSCFG.DSMR (SB_DATA_SW_MR): Data Switch Master to RAM
    /// SYSCFG.DMSR (SB_DATA_MW_SR): Data Master to Switch RAM
    /// SYSCFG.MCB4 (SB_MST_CFG_B4): Master Config Bit 4
    /// SYSCFG.MIO1 (SB_MST_IO_EN1): Master I/O Enable 1
    /// SYSCFG.MIO0 (SB_MST_IO_EN0): Master I/O Enable 0
    /// SYSCFG.MRST (SB_MST_RESET): Master Reset
    /// SYSCFG.MCLKG (SB_MST_CLK_GATE): Master Clock Gate
    #[strum(serialize = "SYSCFG", to_string = "SFR_SYS_CFG")]
    SysConf = 0x1C,
    #[strum(serialize = "CTLRD", to_string = "SFR_CTRL_RD")]
    CtrlRead = 0x1D,
    #[strum(serialize = "CTLWR", to_string = "SFR_CTRL_WR")]
    CtrlWrite = 0x1E,

    /// Data Exchange Register, aka. F2
    #[strum(serialize = "DEXCH", to_string = "SFR_DATA_EXCH")]
    DataExch = 0x1F,

    #[strum(serialize = "D0", to_string = "SFR_DATA_REG0")]
    Data0 = 0x20,
    #[strum(serialize = "D1", to_string = "SFR_DATA_REG1")]
    Data1 = 0x21,
    #[strum(serialize = "D2", to_string = "SFR_DATA_REG2")]
    Data2 = 0x22,
    #[strum(serialize = "D3", to_string = "SFR_DATA_REG3")]
    Data3 = 0x23,
    #[strum(serialize = "D4", to_string = "SFR_DATA_REG4")]
    Data4 = 0x24,
    #[strum(serialize = "D5", to_string = "SFR_DATA_REG5")]
    Data5 = 0x25,
    #[strum(serialize = "D6", to_string = "SFR_DATA_REG6")]
    Data6 = 0x26,
    #[strum(serialize = "D7", to_string = "SFR_DATA_REG7")]
    Data7 = 0x27,
    #[strum(serialize = "D8", to_string = "SFR_DATA_REG8")]
    Data8 = 0x28,
    #[strum(serialize = "D9", to_string = "SFR_DATA_REG9")]
    Data9 = 0x29,
    #[strum(serialize = "D10", to_string = "SFR_DATA_REG10")]
    Data10 = 0x2A,
    #[strum(serialize = "D11", to_string = "SFR_DATA_REG11")]
    Data11 = 0x2B,
    #[strum(serialize = "D12", to_string = "SFR_DATA_REG12")]
    Data12 = 0x2C,
    #[strum(serialize = "D13", to_string = "SFR_DATA_REG13")]
    Data13 = 0x2D,
    #[strum(serialize = "D14", to_string = "SFR_DATA_REG14")]
    Data14 = 0x2E,
    #[strum(serialize = "D15", to_string = "SFR_DATA_REG15")]
    Data15 = 0x2F,
    #[strum(serialize = "D16", to_string = "SFR_DATA_REG16")]
    Data16 = 0x30,
    #[strum(serialize = "D17", to_string = "SFR_DATA_REG17")]
    Data17 = 0x31,
    #[strum(serialize = "D18", to_string = "SFR_DATA_REG18")]
    Data18 = 0x32,
    #[strum(serialize = "D19", to_string = "SFR_DATA_REG19")]
    Data19 = 0x33,
    #[strum(serialize = "D20", to_string = "SFR_DATA_REG20")]
    Data20 = 0x34,
    #[strum(serialize = "D21", to_string = "SFR_DATA_REG21")]
    Data21 = 0x35,
    #[strum(serialize = "D22", to_string = "SFR_DATA_REG22")]
    Data22 = 0x36,
    #[strum(serialize = "D23", to_string = "SFR_DATA_REG23")]
    Data23 = 0x37,
    #[strum(serialize = "D24", to_string = "SFR_DATA_REG24")]
    Data24 = 0x38,
    #[strum(serialize = "D25", to_string = "SFR_DATA_REG25")]
    Data25 = 0x39,
    #[strum(serialize = "D26", to_string = "SFR_DATA_REG26")]
    Data26 = 0x3A,
    #[strum(serialize = "D27", to_string = "SFR_DATA_REG27")]
    Data27 = 0x3B,
    #[strum(serialize = "D28", to_string = "SFR_DATA_REG28")]
    Data28 = 0x3C,
    #[strum(serialize = "D29", to_string = "SFR_DATA_REG29")]
    Data29 = 0x3D,
    #[strum(serialize = "D30", to_string = "SFR_DATA_REG30")]
    Data30 = 0x3E,
    #[strum(serialize = "D31", to_string = "SFR_DATA_REG31")]
    Data31 = 0x3F,
}

impl TryFrom<u8> for Sfr {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::from_repr(value).ok_or(())
    }
}
