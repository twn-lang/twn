use std::io::{Read, Write};

use crate::opcode::OpCode;

const MEMORY_SIZE: usize = 256;
const STACK_SIZE: usize = 256;
const CALL_SIZE: usize = 256;
const BYTE_SIZE: u8 = 1;

#[derive(Debug)]
pub enum SysError {
    InvalidCharacter,
}
impl std::fmt::Display for SysError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidCharacter => write!(f, "Invalid character"),
        }
    }
}

#[derive(Debug)]
pub enum VmError {
    StackUnderflow,             // POPしようとしたがスタックが空
    StackOverflow,              // スタックが上限を超えた
    CallUnderflow,              // POPしようとしたがコールスタックが空
    CallOverflow,               // コールスタックが上限を超えた
    DivisionByZero,             // 0で割ろうとした
    InvalidOpcode(u8),          // 知らない命令が来た
    InvalidMemoryAccess(usize), // メモリ範囲外にアクセスした
    UninitializedMemory(usize), // まだ値の入っていないメモリにアクセスした
    UnexpectedSysCall(u8),      // 知らないシステムコールが来た
    UnexpectedEof,              // 命令の途中でファイルが終わった

    SysError(SysError), // SysCallでエラーが発生した
}
impl std::fmt::Display for VmError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::StackUnderflow => write!(f, "Stack underflow"),
            Self::StackOverflow => write!(f, "Stack Overflow"),
            Self::CallUnderflow => write!(f, "Call underflow"),
            Self::CallOverflow => write!(f, "Call Overflow"),
            Self::DivisionByZero => write!(f, "Division by zero"),
            Self::InvalidOpcode(opcode) => write!(f, "Invalid Opcode: {:02X}", opcode),
            Self::InvalidMemoryAccess(dst) => write!(f, "Invalid memory access: {:02X}", dst),
            Self::UninitializedMemory(dst) => write!(f, "Not exist designated memory: {:02X}", dst),
            Self::UnexpectedSysCall(n) => write!(f, "Unexpected SysCall: {:02X}", n),
            Self::UnexpectedEof => write!(f, "Unexpected EOF"),

            Self::SysError(e) => write!(f, "SysCall Error: {}", e),
        }
    }
}

pub struct VM<R: Read, W: Write> {
    pub pc: usize,
    pub stack: Vec<u8>,
    pub memory: Vec<Option<u8>>,
    pub call: Vec<usize>,
    pub tokens: Vec<u8>,
    pub halted: bool,
    pub exit_code: u8,

    pub in_port: R,
    pub out_port: W,
}
impl<R: Read, W: Write> VM<R, W> {
    pub fn new(mut tokens: Vec<u8>, in_port: R, out_port: W) -> Self {
        if tokens.len() >= "\0TWN".len() && &tokens[0..4] == [0x00, b'T', b'W', b'N'] {
            tokens.drain(0..4);
        } else {
            panic!("Invalid file format: Magic number not found");
        }

        Self {
            pc: 0,
            stack: Vec::new(),
            memory: vec![None; MEMORY_SIZE],
            call: Vec::new(),
            tokens,
            halted: false,
            exit_code: 0u8,

            in_port,
            out_port,
        }
    }

    pub fn next_byte(&mut self) -> Result<u8, VmError> {
        if self.tokens.len() - 1 <= self.pc {
            return Err(VmError::UnexpectedEof);
        }

        self.pc += 1 * (BYTE_SIZE as usize);

        Ok(self.tokens[self.pc])
    }

    pub fn push_stack(&mut self, content: u8) -> Result<(), VmError> {
        if STACK_SIZE <= self.stack.len() {
            return Err(VmError::StackOverflow);
        }

        self.stack.push(content);

        Ok(())
    }

    pub fn pop_stack(&mut self) -> Result<u8, VmError> {
        if self.stack.is_empty() {
            return Err(VmError::StackUnderflow);
        }

        Ok(self.stack.pop().unwrap())
    }

    pub fn store_memory(&mut self, content: u8, dst: usize) -> Result<(), VmError> {
        if MEMORY_SIZE <= dst {
            return Err(VmError::InvalidMemoryAccess(dst));
        }

        self.memory[dst] = Some(content);

        Ok(())
    }

    pub fn load_memory(&self, dst: usize) -> Result<Option<u8>, VmError> {
        if MEMORY_SIZE <= dst {
            return Err(VmError::InvalidMemoryAccess(dst));
        }

        match self.memory[dst] {
            Some(content) => return Ok(Some(content)),
            None => return Err(VmError::UninitializedMemory(dst)),
        }
    }

    pub fn push_call(&mut self, content: usize) -> Result<(), VmError> {
        if CALL_SIZE <= self.call.len() {
            return Err(VmError::CallOverflow);
        }

        self.call.push(content);

        Ok(())
    }

    pub fn pop_call(&mut self) -> Result<usize, VmError> {
        if self.call.is_empty() {
            return Err(VmError::CallUnderflow);
        }

        Ok(self.call.pop().unwrap())
    }

    pub fn sys_read(&mut self) -> Result<(), VmError> {
        let mut buffer = [0u8; 1];

        match self.in_port.read(&mut buffer) {
            Ok(0) => {
                self.push_stack(0)?;
            }
            Ok(_) => {
                self.push_stack(buffer[0])?;
            }
            Err(_) => {
                return Err(VmError::SysError(SysError::InvalidCharacter));
            }
        }
        Ok(())
    }

    pub fn sys_print(&mut self) -> Result<(), VmError> {
        let target = self.pop_stack()?;

        write!(self.out_port, "{}", target as char)
            .map_err(|_| VmError::SysError(SysError::InvalidCharacter))?;

        Ok(())
    }

    pub fn sys_dump(&self) {
        eprintln!("=== VM STATE ===");
        eprintln!("STACK : {:?}", self.stack);
        let memory = self
            .memory
            .chunk_by(|a, b| a == b)
            .map(|chunk| (chunk[0], chunk.len()))
            .collect::<Vec<(Option<u8>, usize)>>();
        eprintln!("MEMORY: {:?}", memory);
        eprintln!("==================");
    }

    pub fn sys_exit(&mut self) -> Result<(), VmError> {
        let code = self.pop_stack()?;
        self.exit_code = code;
        self.halted = true;

        Ok(())
    }

    pub fn run(&mut self) -> Result<(), VmError> {
        while !self.halted && (self.pc < self.tokens.len()) {
            let token = self.tokens[self.pc];

            if let Some(opcode) = OpCode::from_u8(token) {
                match opcode {
                    OpCode::SysCall => {
                        let n = self.pop_stack()?;
                        match n {
                            0 => self.sys_read()?,
                            1 => self.sys_print()?,
                            2 => self.sys_dump(),
                            3 => self.sys_exit()?,
                            _ => return Err(VmError::UnexpectedSysCall(n)),
                        };
                    }
                    OpCode::Push => {
                        let val = self.next_byte()?;
                        self.push_stack(val)?;
                    }
                    OpCode::Pop => {
                        self.pop_stack()?;
                    }
                    OpCode::Dup => {
                        let a = self.pop_stack()?;
                        self.push_stack(a)?;
                        self.push_stack(a)?;
                    }
                    OpCode::Swap => {
                        let a = self.pop_stack()?;
                        let b = self.pop_stack()?;

                        self.push_stack(a)?;
                        self.push_stack(b)?;
                    }
                    OpCode::Add => {
                        let b: u8 = self.pop_stack()?;
                        let a: u8 = self.pop_stack()?;

                        self.push_stack(a.saturating_add(b))?;
                    }
                    OpCode::Sub => {
                        let b: u8 = self.pop_stack()?;
                        let a: u8 = self.pop_stack()?;

                        self.push_stack(a.saturating_sub(b))?;
                    }
                    OpCode::Mul => {
                        let b: u8 = self.pop_stack()?;
                        let a: u8 = self.pop_stack()?;

                        self.push_stack(a.saturating_mul(b))?;
                    }
                    OpCode::Div => {
                        let b: u8 = self.pop_stack()?;
                        let a: u8 = self.pop_stack()?;

                        if b == 0 {
                            return Err(VmError::DivisionByZero);
                        }

                        self.push_stack(a.saturating_div(b))?;
                    }
                    OpCode::Mod => {
                        let b: u8 = self.pop_stack()?;
                        let a: u8 = self.pop_stack()?;

                        self.push_stack(a % b)?;
                    }
                    OpCode::AddI => {
                        let b: u8 = self.next_byte()?;
                        let a: u8 = self.pop_stack()?;

                        self.push_stack(a.saturating_add(b))?;
                    }
                    OpCode::SubI => {
                        let b: u8 = self.next_byte()?;
                        let a: u8 = self.pop_stack()?;

                        self.push_stack(a.saturating_sub(b))?;
                    }
                    OpCode::MulI => {
                        let b: u8 = self.next_byte()?;
                        let a: u8 = self.pop_stack()?;

                        self.push_stack(a.saturating_mul(b))?;
                    }
                    OpCode::DivI => {
                        let b: u8 = self.next_byte()?;
                        let a: u8 = self.pop_stack()?;

                        if b == 0 {
                            return Err(VmError::DivisionByZero);
                        }

                        self.push_stack(a.saturating_div(b))?;
                    }
                    OpCode::ModI => {
                        let b: u8 = self.next_byte()?;
                        let a: u8 = self.pop_stack()?;

                        self.push_stack(a % b)?;
                    }
                    OpCode::Eq => {
                        let b: u8 = self.pop_stack()?;
                        let a: u8 = self.pop_stack()?;

                        self.push_stack(if a == b { 0 } else { 1 })?;
                    }
                    OpCode::Neq => {
                        let b: u8 = self.pop_stack()?;
                        let a: u8 = self.pop_stack()?;

                        self.push_stack(if a != b { 0 } else { 1 })?;
                    }
                    OpCode::Lt => {
                        let b: u8 = self.pop_stack()?;
                        let a: u8 = self.pop_stack()?;

                        self.push_stack(if a < b { 0 } else { 1 })?;
                    }
                    OpCode::Le => {
                        let b: u8 = self.pop_stack()?;
                        let a: u8 = self.pop_stack()?;

                        self.push_stack(if a <= b { 0 } else { 1 })?;
                    }
                    OpCode::Gt => {
                        let b: u8 = self.pop_stack()?;
                        let a: u8 = self.pop_stack()?;

                        self.push_stack(if a > b { 0 } else { 1 })?;
                    }
                    OpCode::Ge => {
                        let b: u8 = self.pop_stack()?;
                        let a: u8 = self.pop_stack()?;

                        self.push_stack(if a >= b { 0 } else { 1 })?;
                    }
                    OpCode::Jz => {
                        let flg = self.pop_stack()?;
                        let dst = self.next_byte()?;
                        if flg == 0 {
                            self.pc = dst as usize;
                            continue;
                        }
                    }
                    OpCode::Jmz => {
                        let dst = self.next_byte()?;
                        self.pc = dst as usize;
                        continue;
                    }
                    OpCode::Store => {
                        let mem_dst = self.pop_stack()? as usize;
                        let target = self.pop_stack()?;
                        self.store_memory(target, mem_dst)?;
                    }
                    OpCode::Load => {
                        let mem_dst = self.pop_stack()? as usize;
                        let target = self.load_memory(mem_dst)?.unwrap();
                        self.push_stack(target)?;
                    }
                    OpCode::StoreI => {
                        let mem_dst = self.next_byte()? as usize;
                        let target = self.pop_stack()?;
                        self.store_memory(target, mem_dst)?;
                    }
                    OpCode::LoadI => {
                        let mem_dst = self.next_byte()? as usize;
                        let target = self.load_memory(mem_dst)?.unwrap();
                        self.push_stack(target)?;
                    }
                    OpCode::Call => {
                        let dst = self.next_byte()? as usize;
                        self.push_call(self.pc)?;
                        self.pc = dst;

                        continue;
                    }
                    OpCode::Ret => {
                        let dst = self.pop_call()?;
                        self.pc = dst;
                    }
                    OpCode::Fin => {
                        self.halted = true;
                        self.exit_code = 0;
                    }
                }
            } else {
                return Err(VmError::InvalidOpcode(token));
            }

            self.pc += 1;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::opcode::OpCode;

    fn run_vm(tokens: Vec<u8>) -> VM<std::io::Empty, std::io::Sink> {
        let mut vm = VM::new(tokens, std::io::empty(), std::io::sink());
        vm.run().unwrap();
        vm
    }

    #[test]
    fn test_add() {
        let code = vec![
            OpCode::Push as u8,
            10,
            OpCode::Push as u8,
            20,
            OpCode::Add as u8,
        ];
        let mut vm = run_vm(code);
        assert_eq!(vm.stack.pop(), Some(30));
    }

    #[test]
    fn test_stack_underflow() {
        // 空のスタックからPOPしようとする
        let code = vec![OpCode::Pop as u8];
        let mut vm = VM::new(code, std::io::empty(), std::io::sink());

        // エラーになるべき
        match vm.run() {
            Err(VmError::StackUnderflow) => (), // OK
            _ => panic!("Expected StackUnderflow error"),
        }
    }

    #[test]
    fn test_jz_jump() {
        // 条件ジャンプのテスト
        // PUSH 0, JZ 0x06, PUSH 1(Skip), FIN, PUSH 2(Target), FIN
        let code = vec![
            OpCode::Push as u8,
            0,
            OpCode::Jz as u8,
            0x07,
            OpCode::Push as u8,
            1, // ここは実行されないはず
            OpCode::Fin as u8,
            OpCode::Push as u8,
            2, // ここに飛んでくるはず
            OpCode::Fin as u8,
        ];
        let mut vm = run_vm(code);

        // PUSH 2 だけが実行されているはず
        assert_eq!(vm.stack.pop(), Some(2));
    }

    #[test]
    fn test_div_by_zero() {
        // 10 / 0
        let code = vec![
            OpCode::Push as u8,
            10,
            OpCode::Push as u8,
            0,
            OpCode::Div as u8,
        ];
        let mut vm = VM::new(code, std::io::empty(), std::io::sink());

        match vm.run() {
            Err(VmError::DivisionByZero) => (), // OK
            _ => panic!("Expected DivisionByZero error"),
        }
    }
}
