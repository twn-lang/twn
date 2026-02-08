use std::process::exit;

pub fn irregular(statement: &str, opcode: u8) {
    eprintln!("{statement} (opcode: {opcode:02X})");
    exit(1);
}

/*
 * OpCode
 * 0x01: PUSH
 * 0x02: POP
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
 * 0x20: JZ
 * 0x21: JMZ
 * 0x30: STORE
 * 0x31: LOAD
 * 0x90: PRINT
 * 0x91: DUMP
 * 0xFF: FIN
 */

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum OpCode {
    Push = 0x01,
    Pop = 0x02,
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
    Jz = 0x20,
    Jmz = 0x21,
    Store = 0x30,
    Load = 0x31,
    StoreI = 0x32,
    LoadI = 0x33,
    Print = 0x90,
    Dump = 0x91,
    Fin = 0xFF,
}

impl OpCode {
    pub fn from_u8(n: u8) -> Option<Self> {
        match n {
            0x01 => Some(Self::Push),
            0x02 => Some(Self::Pop),
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
            0x20 => Some(Self::Jz),
            0x21 => Some(Self::Jmz),
            0x30 => Some(Self::Store),
            0x31 => Some(Self::Load),
            0x32 => Some(Self::StoreI),
            0x33 => Some(Self::LoadI),
            0x90 => Some(Self::Print),
            0x91 => Some(Self::Dump),
            0xFF => Some(Self::Fin),
            _ => None,
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_uppercase().as_str() {
            "PUSH" => Some(Self::Push),
            "POP" => Some(Self::Pop),
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
            "JZ" => Some(Self::Jz),
            "JMZ" => Some(Self::Jmz),
            "STORE" => Some(Self::Store),
            "LOAD" => Some(Self::Load),
            "STOREI" => Some(Self::StoreI),
            "LOADI" => Some(Self::LoadI),
            "PRINT" => Some(Self::Print),
            "DUMP" => Some(Self::Dump),
            "FIN" => Some(Self::Fin),
            _ => None,
        }
    }
}
