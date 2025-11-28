#![allow(clippy::unusual_byte_groupings)]

pub mod ast;
pub mod parser;
pub mod regs;
pub mod types;

use types::*;

/// OpCode of RISC8B, eMCU
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OpCode {
    /// NOP
    Nop,
    /// CLRWDT
    ClearWatchDog,
    /// SLEEP
    /// SLEEPX k2
    Sleep(U2),
    /*
    WB_DATA_SW_MR_0     EQU   0
    WB_BIT_CYC_TAIL_1   EQU   1
    WB_PORT_I0_FALL     EQU   2
    WB_PORT_I0_RISE     EQU   3
    WB_DATA_MW_SR_1     EQU   4
    WB_PORT_XOR1_1      EQU   5
    WB_PORT_XOR0_0      EQU   6
    WB_PORT_XOR0_1      EQU   7
     */
    /// WAITB b
    WaitB(WaitBit),
    // no WAITRD, WAITWR, WAITSPI
    // no idea of how to use  WRCODE, EXEC
    /// RDCODE k2
    // ROM_CODE(k2)->{SFR,A}
    ReadCode(U2),

    /// PUSHAS
    ///
    /// Push all
    PushA,
    /// POPAS
    ///
    /// Pop from TOS
    PopA,

    /// PUSHA2
    PushIndirAddr2,
    /// POPA2
    PopIndirAddr2,

    /// RET
    // 00000000 001100xx
    Return,
    /// RETZ        1->Z
    // 00000000 001101xx
    ReturnOk,
    /// RETIE - return interrupt
    // 00000000 001110xx
    ReturnInt,
    /// RETL k
    // 00100000 kkkkkkkk
    ReturnImm(u8),
    /// RETLN k
    // 0->Z
    // 00100001 kkkkkkkk
    ReturnErrImm(u8),

    /// CLRA
    // 00000000 000001xx
    ClearA,
    /// CLR f
    // 00000001 ffffffff
    Clear(Reg<u8>),

    /// MOVA F      A -> F
    /// Move A to F
    // 0001000F FFFFFFFF
    MoveA(Reg<U9>),

    /// MOV F       F -> f (Z)
    /// MOV F,A     F -> A (Z)
    // 000d001 FFFFFFFFF
    Move(Reg<U9>, Dest),

    /// INC f       f+1 -> f (Z)
    /// INC f,A     f+1 -> A (Z)
    // 000d0100 ffffffff
    Inc(Reg<u8>, Dest),
    // 000d0101 ffffffff
    Dec(Reg<u8>, Dest),

    /// INCSZ f,d
    // 000d0110
    IncAndSkipIfZero(Reg<u8>, Dest),
    /// DECSZ f,d
    // 000d0111
    DecAndSkipIfZero(Reg<u8>, Dest),

    /// SWAP f,d
    // 000d1000
    SwapHalfBytes(Reg<u8>, Dest),

    /// AND f,d
    // A & f -> A (Z)
    // 000d1001
    And(Reg<u8>, Dest),
    /// IOR f,d
    // 000d1010
    Or(Reg<u8>, Dest),
    // 000d1011
    Xor(Reg<u8>, Dest),
    // 000d1100
    Add(Reg<u8>, Dest),
    // SUB f,d
    // f-A->d
    // 000d1101
    Sub(Reg<u8>, Dest),

    /// RCL f,d
    // 000d1110
    RotateLeftWithCarry(Reg<u8>, Dest),
    /// RCR f,d
    // 000d1111
    RotateRightWithCarry(Reg<u8>, Dest),

    /// MOVIP k9
    // k9->{INDIR_RAM_PAGE, INDIR_RAM SFR_INDIR_ADDR}
    // 0010001k kkkkkkkk
    MoveImmToIndirAddr1(U9),
    /// MOVIA k10
    // k10->SFR_INDIR_ADDR2
    // 001001kk kkkkkkkk
    MoveImmToIndirAddr2(U10),

    // Fast moves
    // Fast1 SFR_PORT_DIR
    // Fast2 SFR_PORT_IO
    // P1 @SFR_INDIR_ADDR
    // P2 @SFR_INDIR_ADDR2
    /// MOVA1F
    /// SFR_PORT_DIR
    // 00100011 kkkkkkkk
    MoveImmToPortDir(u8),
    /// MOVA2F
    /// SFR_PORT_IO
    // 00100101 kkkkkkkk
    MoveImmToPortIo(u8),
    /// MOVA1P
    // 00100111 kkkkkkkk
    MoveImmToP1(u8),
    /// MOVA2P
    // 00100110 kkkkkkkk
    MoveImmToP2(u8),

    /// MOVL k
    /// k -> A
    // 00101000 kkkkkkkk
    MoveImm(u8),
    /// ADDL
    // 00101001 kkkkkkkk
    AndImm(u8),
    /// ORL
    // 00101010 kkkkkkkk
    OrImm(u8),
    /// XORL
    // 00101011 kkkkkkkk
    XorImm(u8),
    /// ADDL
    // 00101100 kkkkkkkk
    AddImm(u8),
    /// SUBL
    // 00101101 kkkkkkkk
    SubImm(u8),
    /// CMPLN
    // 00101110 kkkkkkkk
    CompareImmNegate(u8),
    /// CMPL
    // 00101111 kkkkkkkk
    CompareImm(u8),

    // bit op
    /// BC
    // 01000bbb ffffffff
    BitClear(Reg<u8>, U3),
    /// BS
    // 01001bbb, ffffffff
    BitSet(Reg<u8>, U3),

    /// BTSC
    // 01010bbb, ffffffff
    BitTestSkipIfClear(Reg<u8>, U3),
    /// BTSS
    // 01011bbb ffffffff
    BitTestSkipIfSet(Reg<u8>, U3),

    /// BCTC
    // BI_C_XOR_IN0        EQU   0
    // BI_BIT_RX_I0        EQU   1
    // BI_PORT_IN0         EQU   2
    // BI_PORT_IN1         EQU   3
    BitToC(BitInC),

    // BP1F, BP2F, BG1F, BG2F
    // F1: SFR_INDIR_ADDR
    // F2: SFR_DATA_EXCH
    /// BP1F
    // 1#bf[b]->bit[a]
    // 00000000 100aabbb
    // SFR_INDIR_ADDR
    BitOut1(BitOut, U3),
    /// BP2F
    // 2#bf[b]->bit[a]
    // 00000000 101aabbb
    // SFR_DATA_EXCH
    BitOut2(BitOut, U3),
    /// BG1F
    // bit[a]->1#bf[b]
    // 00000000 110aabbb
    // SFR_INDIR_ADDR
    BitIn1(BitIn, U3),
    /// BG2F
    // bit[a]->2#bf[b]
    // 00000000 111aabbb
    // SFR_DATA_EXCH
    BitIn2(BitIn, U3),

    /// JMP
    // 0110kkkk kkkkkkkk
    Jump(Label<U12>),
    /// CALL
    Call(Label<U12>),

    /// JNZ
    JumpIfNotZero(Label<U10>),
    /// JZ
    JumpIfZero(Label<U10>),
    /// JNC
    /// Jump if C==0
    JumpIfNotCarry(Label<U10>),
    /// JC
    /// Jump if C==1
    JumpIfCarry(Label<U10>),
    /// CMPZ K7,k
    // JEQ
    /// jump to u8 if U7==A
    // 1KKKKKKK kkkkkkkk
    JumpIfEqual(U7, Label<u8>),

    Unknown(u16),
}

impl OpCode {
    pub fn to_word(&self) -> u16 {
        use OpCode::*;
        match self {
            Nop => 0x0000,
            ClearWatchDog => 0x0008,
            Sleep(k2) => 0x000c | k2.0 as u16,
            WaitB(b) => 0x0010 | b.0 .0 as u16,
            ReadCode(k2) => 0x0018 | k2.0 as u16,
            PushA => 0x0020,
            PopA => 0x0024,
            PushIndirAddr2 => 0x0028,
            PopIndirAddr2 => 0x002c,
            Return => 0x0030,
            ReturnOk => 0x0034,
            ReturnInt => 0x0038,
            ReturnImm(k) => 0x2000 | *k as u16,
            ReturnErrImm(k) => 0x2100 | *k as u16,
            ClearA => 0x0004,
            Clear(f) => 0x0100 | f.0 as u16,
            MoveA(f) => 0x1000 | f.0 .0,
            Move(f, d) => 0x0200 | f.0 .0 | d.to_inst_part(),
            Inc(f, d) => 0x0400 | f.0 as u16 | d.to_inst_part(),
            Dec(f, d) => 0x0500 | f.0 as u16 | d.to_inst_part(),
            IncAndSkipIfZero(f, d) => 0x0600 | f.0 as u16 | d.to_inst_part(),
            DecAndSkipIfZero(f, d) => 0x0700 | f.0 as u16 | d.to_inst_part(),
            SwapHalfBytes(f, d) => 0x0800 | f.0 as u16 | d.to_inst_part(),
            And(f, d) => 0x0900 | f.0 as u16 | d.to_inst_part(),
            Or(f, d) => 0x0a00 | f.0 as u16 | d.to_inst_part(),
            Xor(f, d) => 0x0b00 | f.0 as u16 | d.to_inst_part(),
            Add(f, d) => 0x0c00 | f.0 as u16 | d.to_inst_part(),
            Sub(f, d) => 0x0d00 | f.0 as u16 | d.to_inst_part(),
            RotateLeftWithCarry(f, d) => 0x0e00 | f.0 as u16 | d.to_inst_part(),
            RotateRightWithCarry(f, d) => 0x0f00 | f.0 as u16 | d.to_inst_part(),
            MoveImmToIndirAddr1(k) => 0x2200 | k.0,
            MoveImmToIndirAddr2(k) => 0x2400 | k.0,
            MoveImmToPortDir(k) => 0x2300 | *k as u16,
            MoveImmToPortIo(k) => 0x2500 | *k as u16,
            MoveImmToP1(k) => 0x2700 | *k as u16,
            MoveImmToP2(k) => 0x2600 | *k as u16,
            MoveImm(k) => 0x2800 | *k as u16,
            AndImm(k) => 0x2900 | *k as u16,
            OrImm(k) => 0x2a00 | *k as u16,
            XorImm(k) => 0x2b00 | *k as u16,
            AddImm(k) => 0x2c00 | *k as u16,
            SubImm(k) => 0x2d00 | *k as u16,
            CompareImmNegate(k) => 0x2e00 | *k as u16,
            CompareImm(k) => 0x2f00 | *k as u16,
            BitClear(f, b) => 0x4000 | b.to_inst_part() | f.0 as u16,
            BitSet(f, b) => 0x4800 | b.to_inst_part() | f.0 as u16,
            BitTestSkipIfClear(f, b) => 0x5000 | b.to_inst_part() | f.0 as u16,
            BitTestSkipIfSet(f, b) => 0x5800 | b.to_inst_part() | f.0 as u16,
            BitToC(a) => 0x001c | a.to_inst_part(),
            BitOut1(a, b) => 0x0080 | a.to_inst_part() | b.0 as u16,
            BitOut2(a, b) => 0x00a0 | a.to_inst_part() | b.0 as u16,
            BitIn1(a, b) => 0x00c0 | a.to_inst_part() | b.0 as u16,
            BitIn2(a, b) => 0x00e0 | a.to_inst_part() | b.0 as u16,
            Jump(k) => 0x6000 | k.0 .0,
            Call(k) => 0x7000 | k.0 .0,
            JumpIfNotZero(k) => 0x3000 | k.0 .0,
            JumpIfZero(k) => 0x3400 | k.0 .0,
            JumpIfNotCarry(k) => 0x3800 | k.0 .0,
            JumpIfCarry(k) => 0x3c00 | k.0 .0,
            JumpIfEqual(k1, k2) => 0x8000 | ((k1.0 as u16) << 8) | k2.0 as u16,
            Unknown(op) => *op,
        }
    }

    pub fn to_wch_risc8b_asm(&self) -> String {
        use OpCode::*;
        match self {
            Nop => "NOP".to_string(),
            ClearWatchDog => "CLRWDT".to_string(),
            Sleep(k2) => format!("SLEEPX {k2}"),
            WaitB(b) => format!("WAITB {b}"),
            PushA => "PUSHA".to_string(),
            PopA => "POPA".to_string(),
            ReadCode(k2) => format!("RCODE {k2}"),
            Return => "RET".to_string(),
            ReturnOk => "RETZ".to_string(),
            ReturnInt => "RETIE".to_string(),
            ReturnImm(k) => format!("RETL {k}"),
            ReturnErrImm(k) => format!("RETLN {k}"),
            ClearA => "CLRA".to_string(),
            Clear(f) => format!("CLR {f}\t; 0x00->{f}, 1->Z"),
            MoveA(f) => format!("MOVA {f}\t; A->{f}"),
            Move(f, d) => format!("MOV {f}, {d}\t; {f}->{d}"),
            Inc(f, d) => format!("INC {f}, {d}\t; {f}+1->{d}"),
            Dec(f, d) => format!("DEC {f}, {d}\t; {f}-1->{d}"),
            IncAndSkipIfZero(f, d) => {
                format!("INCSZ {f}, {d}\t; {f}+1->{d}, skip if Z")
            }
            DecAndSkipIfZero(f, d) => {
                format!("DECSZ {f}, {d}\t; {f}-1->{d}, skip if Z")
            }
            SwapHalfBytes(f, d) => {
                format!("SWAP {f}, {d}\t; {f}[3:0]<=>{f}[7:4] -> {d}")
            }
            And(f, d) => format!("AND {f}, {d}\t; {f}&A->{d}"),
            Or(f, d) => format!("IOR {f}, {d}\t; {f}|A->{d}"),
            Xor(f, d) => format!("XOR {f}, {d}\t; {f}^A->{d}"),
            Add(f, d) => format!("ADD {f}, {d}\t; {f}+A->{d}"),
            Sub(f, d) => format!("SUB {f}, {d}\t; {f}-A->{d}"),
            RotateLeftWithCarry(f, d) => {
                format!("RCL {f}, {d}\t; {{{f},C}}<<1->{d},{f}[7]->C")
            }
            RotateRightWithCarry(f, d) => {
                format!("RCR {f}, {d}\t; {{{f},C}}>>1->{d},{f}[0]->C")
            }

            MoveImmToIndirAddr1(k) => format!("MOVIP {k}\t; {k}->SFR_INDIR_ADDR"),
            MoveImmToIndirAddr2(k) => format!("MOVIA {k}\t; {k}->SFR_INDIR_ADDR2"),
            MoveImmToPortDir(k) => format!("MOVA1F {k}\t; {k}->SFR_PORT_DIR"),
            MoveImmToPortIo(k) => format!("MOVA2F {k}\t; {k}->SFR_PORT_IO"),
            MoveImmToP2(k) => format!("MOVA2P {k}\t; {k}->@SFR_INDIR_ADDR2"),
            MoveImmToP1(k) => format!("MOVA1P {k}\t; {k}->@SFR_INDIR_ADDR"),

            MoveImm(k) => format!("MOVL {k}\t; {k}->A"),
            AndImm(k) => format!("ANDL {k}\t; {k}&A->A"),
            OrImm(k) => format!("IORL {k}\t; {k}|A->A"),
            XorImm(k) => format!("XORL {k}\t; {k}^A->A"),
            AddImm(k) => format!("ADDL {k}\t; {k}+A->A"),
            SubImm(k) => format!("SUBL {k}\t; A-{k}->A"),
            CompareImmNegate(k) => format!("CMPLN {k}\t; {k}+A -> Z,C"),
            CompareImm(k) => format!("CMPL {k}\t; {k}-A -> Z,C"),
            BitClear(f, b) => format!("BC {f}, {b}\t; 0->{f}[{b}]"),
            BitSet(f, b) => format!("BS {f}, {b}\t; 1->{f}[{b}]"),
            BitTestSkipIfClear(f, b) => format!("BTSC {f}, {b}\t; skip if {f}[{b}]==0"),
            BitTestSkipIfSet(f, b) => format!("BTSS {f}, {b}\t; skip if {f}[{b}]==1"),
            BitToC(a) => format!("BCTC {a}\t; {a}->C"),
            BitOut1(a, b) => {
                format!("BP1F {a}, {b}\t; SFR_INDIR_ADDR[{b}]->{a}")
            }
            BitOut2(a, b) => {
                format!("BP2F {a}, {b}\t; SFR_DATA_EXCH[{b}]->{a}")
            }
            BitIn1(a, b) => format!("BG1F {a}, {b}\t; {a}->SFR_INDIR_ADDR[{b}]"),
            BitIn2(a, b) => format!("BG2F {a}, {b}\t; {a}->SFR_DATA_EXCH[{b}]"),

            // jumps
            Jump(k12) => format!("JMP {k12}"),
            Call(k12) => format!("CALL {k12}"),
            JumpIfNotZero(k) => format!("JNZ {k}"),
            JumpIfZero(k) => format!("JZ {k}"),
            JumpIfNotCarry(k) => format!("JNC {k}"),
            JumpIfCarry(k10) => format!("JC {k10}"),
            JumpIfEqual(k7, label) => {
                format!("CMPZ {k7}, {label}\t; {label}->PC[7:0] if A=={k7}")
            }
            PushIndirAddr2 => "PUSHA2".to_string(),
            PopIndirAddr2 => "POPA2".to_string(),
            Unknown(op) => format!("DW {op:#04x}"),
        }
    }

    pub fn from_word(word: u16) -> OpCode {
        use OpCode::*;
        let k = (word & 0xFF) as u8;
        match (word >> 8) as u8 {
            0x00 if k & 0b111111_00 == 0x00 => Nop,
            0x00 if k & 0b111111_00 == 0b000010_00 => ClearWatchDog,
            0x00 if k & 0b111111_00 == 0b00001100 => Sleep(k.into()),
            0x00 if k & 0b111111_00 == 0b001000_00 => PushA,
            0x00 if k & 0b111111_00 == 0b001001_00 => PopA,
            0x00 if k & 0b111111_00 == 0b001010_00 => PushIndirAddr2,
            0x00 if k & 0b111111_00 == 0b001011_00 => PopIndirAddr2,

            0x00 if k & 0b111111_00 == 0b001100_00 => Return,
            0x00 if k & 0b111111_00 == 0b001101_00 => ReturnOk,
            0x00 if k & 0b111111_00 == 0b001110_00 => ReturnInt,

            0x00 if k & 0b11111000 == 0b00010_000 => WaitB(WaitBit(k.into())),
            0x00 if k & 0b111111_00 == 0b000001_00 => ClearA,

            0x00 if k & 0b111111_00 == 0b000111_00 => BitToC(BitInC(k.into())),
            0x00 if k & 0b111_00000 == 0b100_00_000 => BitOut1(BitOut((k >> 3).into()), k.into()),
            0x00 if k & 0b111_00000 == 0b101_00_000 => BitOut2(BitOut((k >> 3).into()), k.into()),
            0x00 if k & 0b111_00000 == 0b110_00_000 => BitIn1(BitIn((k >> 3).into()), k.into()),
            0x00 if k & 0b111_00000 == 0b111_00_000 => BitIn2(BitIn((k >> 3).into()), k.into()),
            0x00 if k & 0b111111_00 == 0b000110_00 => ReadCode(k.into()),

            0x00 if k == 0b00010100 => unimplemented!("WAITWR is not implemented"),

            0b000000_1 => Clear(Reg(k)),

            // immediate byte op
            0b00101000 => MoveImm(k),
            0b00101001 => AndImm(k),
            0b00101010 => OrImm(k),
            0b00101011 => XorImm(k),
            0b00101100 => AddImm(k),
            0b00101101 => SubImm(k),
            0b00101110 => CompareImmNegate(k),
            0b00101111 => CompareImm(k),

            0b00100000 => ReturnImm(k),
            0b00100001 => ReturnErrImm(k),

            // byte op
            0b00100011 => MoveImmToPortDir(k),
            0b00100101 => MoveImmToPortIo(k),
            0b00100110 => MoveImmToP2(k),
            0b00100111 => MoveImmToP2(k),

            x if x & 0b1111111_0 == 0b0010001_0 => MoveImmToIndirAddr1(word.into()),
            x if x & 0b111111_00 == 0b001001_00 => MoveImmToIndirAddr2(word.into()),

            x if x & 0b1111111_0 == 0b0001000_0 => MoveA(Reg(word.into())),
            x if x & 0b111_0_111_0 == 0b000_0_001_0 => {
                Move(Reg(word.into()), Dest::from(x & 0b000_1_000_0 != 0))
            }

            x if x & 0b111_0_1111 == 0b000_0_0100 => Inc(Reg(k), Dest::from(x & 0b000_1_0000 != 0)),
            x if x & 0b111_0_1111 == 0b000_0_0101 => Dec(Reg(k), Dest::from(x & 0b000_1_0000 != 0)),
            x if x & 0b111_0_1111 == 0b000_0_0110 => {
                IncAndSkipIfZero(Reg(k), Dest::from(x & 0b000_1_0000 != 0))
            }
            x if x & 0b111_0_1111 == 0b000_0_0111 => {
                DecAndSkipIfZero(Reg(k), Dest::from(x & 0b000_1_0000 != 0))
            }
            x if x & 0b111_0_1111 == 0b000_0_1000 => {
                SwapHalfBytes(Reg(k), Dest::from(x & 0b000_1_0000 != 0))
            }
            x if x & 0b111_0_1111 == 0b000_0_1001 => And(Reg(k), Dest::from(x & 0b000_1_0000 != 0)),
            x if x & 0b111_0_1111 == 0b000_0_1010 => Or(Reg(k), Dest::from(x & 0b000_1_0000 != 0)),
            x if x & 0b111_0_1111 == 0b000_0_1011 => Xor(Reg(k), Dest::from(x & 0b000_1_0000 != 0)),
            x if x & 0b111_0_1111 == 0b000_0_1100 => Add(Reg(k), Dest::from(x & 0b000_1_0000 != 0)),
            x if x & 0b111_0_1111 == 0b000_0_1101 => Sub(Reg(k), Dest::from(x & 0b000_1_0000 != 0)),
            x if x & 0b111_0_1111 == 0b000_0_1110 => {
                RotateLeftWithCarry(Reg(k), Dest::from(x & 0b000_1_0000 != 0))
            }
            x if x & 0b111_0_1111 == 0b000_0_1111 => {
                RotateRightWithCarry(Reg(k), Dest::from(x & 0b000_1_0000 != 0))
            }

            x if x & 0b1111_0000 == 0b0110_0000 => {
                let label: Label<U12> = Label(word.into());
                Jump(label)
            }
            x if x & 0b1111_0000 == 0b0111_0000 => Call(Label(word.into())),
            x if x & 0b111111_00 == 0b001100_00 => JumpIfNotZero(Label(word.into())),
            x if x & 0b111111_00 == 0b001101_00 => JumpIfZero(Label(word.into())),
            x if x & 0b111111_00 == 0b001110_00 => JumpIfNotCarry(Label(word.into())),
            x if x & 0b111111_00 == 0b001111_00 => JumpIfCarry(Label(word.into())),
            x if x & 0b1_0000000 == 0b1_0000000 => JumpIfEqual(U7::from(x), Label(k)),

            // bit op
            x if x & 0b11111_000 == 0b01000_000 => BitClear(Reg(k), x.into()),
            x if x & 0b11111_000 == 0b01001_000 => BitSet(Reg(k), x.into()),
            x if x & 0b11111_000 == 0b01010_000 => BitTestSkipIfClear(Reg(k), x.into()),
            x if x & 0b11111_000 == 0b01011_000 => BitTestSkipIfSet(Reg(k), x.into()),

            _ => unimplemented!("Unknown opcode: {word:#04x} {word:#016b}"),
        }
    }
}
