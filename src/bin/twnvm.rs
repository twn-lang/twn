use std::process::exit;

use twn::*;

const MEMORY_SIZE: usize = 256;
const STACK_SIZE: usize = 256;
const CALL_SIZE: usize = 256;
const BYTE_SIZE: u8 = 1;

#[derive(Debug)]
enum VmError {
    StackUnderflow,             // POPしようとしたがスタックが空
    StackOverflow,              // スタックが上限を超えた
    CallUnderflow,              // POPしようとしたがコールスタックが空
    CallOverflow,               // コールスタックが上限を超えた
    DivisionByZero,             // 0で割ろうとした
    InvalidOpcode(u8),          // 知らない命令が来た
    InvalidMemoryAccess(usize), // メモリ範囲外にアクセスした
    UninitializedMemory(usize), // まだ値の入っていないメモリにアクセスした
    UnexpectedEof,              // 命令の途中でファイルが終わった
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
            Self::UnexpectedEof => write!(f, "Unexpected EOF"),
        }
    }
}

struct VM {
    pub pc: usize,
    stack: Vec<u8>,
    memory: Vec<Option<u8>>,
    call: Vec<usize>,
    tokens: Vec<u8>,
}
impl VM {
    fn new(tokens: Vec<u8>) -> Self {
        Self {
            pc: 0,
            stack: Vec::new(),
            memory: vec![None; MEMORY_SIZE],
            call: Vec::new(),
            tokens,
        }
    }

    fn next_byte(&mut self) -> Result<u8, VmError> {
        if self.tokens.len() - 1 <= self.pc {
            return Err(VmError::UnexpectedEof);
        }

        self.pc += 1 * (BYTE_SIZE as usize);

        Ok(self.tokens[self.pc])
    }

    fn push_stack(&mut self, content: u8) -> Result<(), VmError> {
        if STACK_SIZE <= self.stack.len() {
            return Err(VmError::StackOverflow);
        }

        self.stack.push(content);

        Ok(())
    }

    fn pop_stack(&mut self) -> Result<u8, VmError> {
        if self.stack.is_empty() {
            return Err(VmError::StackUnderflow);
        }

        Ok(self.stack.pop().unwrap())
    }

    fn store_memory(&mut self, content: u8, dst: usize) -> Result<(), VmError> {
        if MEMORY_SIZE <= dst {
            return Err(VmError::InvalidMemoryAccess(dst));
        }

        self.memory[dst] = Some(content);

        Ok(())
    }

    fn load_memory(&self, dst: usize) -> Result<Option<u8>, VmError> {
        if MEMORY_SIZE <= dst {
            return Err(VmError::InvalidMemoryAccess(dst));
        }

        match self.memory[dst] {
            Some(content) => return Ok(Some(content)),
            None => return Err(VmError::UninitializedMemory(dst)),
        }
    }

    fn push_call(&mut self, content: usize) -> Result<(), VmError> {
        if CALL_SIZE <= self.call.len() {
            return Err(VmError::CallOverflow);
        }

        self.call.push(content);

        Ok(())
    }

    fn pop_call(&mut self) -> Result<usize, VmError> {
        if self.call.is_empty() {
            return Err(VmError::CallUnderflow);
        }

        Ok(self.call.pop().unwrap())
    }

    fn run(&mut self) -> Result<(), VmError> {
        while self.pc < self.tokens.len() {
            let token = self.tokens[self.pc];

            if let Some(opcode) = OpCode::from_u8(token) {
                match opcode {
                    OpCode::Push => {
                        let val = self.next_byte()?;
                        self.push_stack(val)?;
                    }
                    OpCode::Pop => {
                        self.pop_stack()?;
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
                    OpCode::Print => {
                        let value = self.pop_stack()?;
                        println!("{value}");
                    }
                    OpCode::Dump => {
                        println!("STACK : {:?}", self.stack);
                        let rle = self
                            .memory
                            .chunk_by(|&a, &b| a == b)
                            .map(|chunk| (chunk[0], chunk.len()))
                            .collect::<Vec<(Option<u8>, usize)>>();
                        println!("MEMORY: {:?}", rle);
                        println!();
                    }
                    OpCode::Fin => {
                        exit(0);
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

fn main() {
    let args = std::env::args().collect::<Vec<String>>();

    if args.len() < 2 {
        eprintln!("Usage: {} <FILE.twnd>", args[0]);
        exit(1);
    }

    let input = std::fs::read_to_string(&args[1]).expect("Input is empty");

    let tokens: Vec<u8> = input
        .split_whitespace()
        .map(|token| u8::from_str_radix(token, 16).expect("Included invalid token"))
        .collect::<Vec<u8>>();

    let mut vm: VM = VM::new(tokens);

    if let Err(e) = vm.run() {
        eprintln!("Error: {} (at address 0x{:02X})", e, vm.pc);
        std::process::exit(1);
    }
}
