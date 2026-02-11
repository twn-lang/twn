/*
 * OpCode
 * 0x00: SYSCALL
 * 0x01: PUSH
 * 0x02: POP
 * 0x03: DUP
 * 0x04: SWAP
 * 0x10: ADD
 * 0x11: SUB
 * 0x12: MUL
 * 0x13: DIV
 * 0x14: MOD
 * 0x15: ADDI
 * 0x16: SUBI
 * 0x17: MULI
 * 0x18: DIVI
 * 0x19: MODI
 * 0x1A: EQ
 * 0x1B: NEQ
 * 0x1C: LT
 * 0x1D: LE
 * 0x1E: GT
 * 0x1F: GE
 * 0x20: JZ
 * 0x21: JMZ
 * 0x30: STORE
 * 0x31: LOAD
 * 0x32: STOREI
 * 0x33: LOADI
 * 0x40: CALL
 * 0x41: RET
 * 0xFF: FIN
 */

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum OpCode {
    SysCall = 0x00,
    Push = 0x01,
    Pop = 0x02,
    Dup = 0x03,
    Swap = 0x04,
    Add = 0x10,
    Sub = 0x11,
    Mul = 0x12,
    Div = 0x13,
    Mod = 0x14,
    AddI = 0x15,
    SubI = 0x16,
    MulI = 0x17,
    DivI = 0x18,
    ModI = 0x19,
    Eq = 0x1A,
    Neq = 0x1B,
    Lt = 0x1C,
    Le = 0x1D,
    Gt = 0x1E,
    Ge = 0x1F,
    Jz = 0x20,
    Jmz = 0x21,
    Store = 0x30,
    Load = 0x31,
    StoreI = 0x32,
    LoadI = 0x33,
    Call = 0x40,
    Ret = 0x41,
    Fin = 0xFF,
}

impl OpCode {
    pub fn from_u8(n: u8) -> Option<Self> {
        match n {
            0x00 => Some(Self::SysCall),
            0x01 => Some(Self::Push),
            0x02 => Some(Self::Pop),
            0x03 => Some(Self::Dup),
            0x04 => Some(Self::Swap),
            0x10 => Some(Self::Add),
            0x11 => Some(Self::Sub),
            0x12 => Some(Self::Mul),
            0x13 => Some(Self::Div),
            0x14 => Some(Self::Mod),
            0x15 => Some(Self::AddI),
            0x16 => Some(Self::SubI),
            0x17 => Some(Self::MulI),
            0x18 => Some(Self::DivI),
            0x19 => Some(Self::ModI),
            0x1A => Some(Self::Eq),
            0x1B => Some(Self::Neq),
            0x1C => Some(Self::Lt),
            0x1D => Some(Self::Le),
            0x1E => Some(Self::Gt),
            0x1F => Some(Self::Ge),
            0x20 => Some(Self::Jz),
            0x21 => Some(Self::Jmz),
            0x30 => Some(Self::Store),
            0x31 => Some(Self::Load),
            0x32 => Some(Self::StoreI),
            0x33 => Some(Self::LoadI),
            0x40 => Some(Self::Call),
            0x41 => Some(Self::Ret),
            0xFF => Some(Self::Fin),
            _ => None,
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_uppercase().as_str() {
            "SYSCALL" => Some(Self::SysCall),
            "PUSH" => Some(Self::Push),
            "POP" => Some(Self::Pop),
            "DUP" => Some(Self::Dup),
            "SWAP" => Some(Self::Swap),
            "ADD" => Some(Self::Add),
            "SUB" => Some(Self::Sub),
            "MUL" => Some(Self::Mul),
            "DIV" => Some(Self::Div),
            "MOD" => Some(Self::Mod),
            "ADDI" => Some(Self::AddI),
            "SUBI" => Some(Self::SubI),
            "MULI" => Some(Self::MulI),
            "DIVI" => Some(Self::DivI),
            "MODI" => Some(Self::ModI),
            "EQ" => Some(Self::Eq),
            "NEQ" => Some(Self::Neq),
            "LT" => Some(Self::Lt),
            "LE" => Some(Self::Le),
            "Gt" => Some(Self::Gt),
            "Ge" => Some(Self::Ge),
            "JZ" => Some(Self::Jz),
            "JMZ" => Some(Self::Jmz),
            "STORE" => Some(Self::Store),
            "LOAD" => Some(Self::Load),
            "STOREI" => Some(Self::StoreI),
            "LOADI" => Some(Self::LoadI),
            "CALL" => Some(Self::Call),
            "RET" => Some(Self::Ret),
            "FIN" => Some(Self::Fin),
            _ => None,
        }
    }
}
