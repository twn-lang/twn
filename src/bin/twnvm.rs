use std::process::exit;

use twn::*;

const MEMORY_SIZE: usize = 256;
const BYTE_SIZE: u8 = 1;

#[derive(Debug)]
enum VmError {
    StackUnderflow,             // POPしようとしたがスタックが空
    StackOverflow,              // スタックが上限を超えた
    DivisionByZero,             // 0で割ろうとした
    InvalidOpcode(u8),          // 知らない命令が来た
    InvalidMemoryAccess(usize), // メモリ範囲外にアクセスした
    UnexpectedEof,              // 命令の途中でファイルが終わった
    UnknownLabel(String),       // 未定義ラベル
}
impl std::fmt::Display for VmError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::StackUnderflow => write!(f, "Stack underflow"),
            Self::StackOverflow => write!(f, "Stack Overflow"),
            Self::DivisionByZero => write!(f, "Division by zero"),
            Self::InvalidOpcode(opcode) => write!(f, "Invalid Opcode: {:02X}", opcode),
            Self::InvalidMemoryAccess(dst) => write!(f, "Invalid memory access: {:02X}", dst),
            Self::UnexpectedEof => write!(f, "Unexpected EOF"),
            Self::UnknownLabel(label) => write!(f, "Unknown lable: {label}"),
            _ => write!(f, "{:?}", self),
        }
    }
}

struct VM {
    pc: usize,
    stack: Vec<u8>,
    memory: Vec<Option<u8>>,
    tokens: Vec<u8>,
}
impl VM {
    fn new(tokens: Vec<u8>) -> Self {
        Self {
            pc: 0,
            stack: Vec::new(),
            memory: vec![None; MEMORY_SIZE],
            tokens,
        }
    }

    fn run(&mut self) {
        while self.pc < self.tokens.len() {
            let token = self.tokens[self.pc];

            if let Some(opcode) = OpCode::from_u8(token) {
                match opcode {
                    OpCode::Push => {
                        self.pc += 1;
                        self.stack.push(self.tokens[self.pc]);
                    }
                    OpCode::Pop => {
                        if self.stack.is_empty() {
                            irregular("self.Stack is empty", token);
                        }
                        self.stack.pop().unwrap();
                    }
                    OpCode::Add => {
                        let b: u8 = self.stack.pop().unwrap_or_default();
                        let a: u8 = self.stack.pop().unwrap_or_default();

                        self.stack.push(a.saturating_add(b));
                    }
                    OpCode::Sub => {
                        let b: u8 = self.stack.pop().unwrap_or_default();
                        let a: u8 = self.stack.pop().unwrap_or_default();

                        self.stack.push(a.saturating_sub(b));
                    }
                    OpCode::Mul => {
                        let b: u8 = self.stack.pop().unwrap_or_default();
                        let a: u8 = self.stack.pop().unwrap_or_default();

                        self.stack.push(a.saturating_mul(b));
                    }
                    OpCode::Div => {
                        let b: u8 = self.stack.pop().unwrap_or_default();
                        let a: u8 = self.stack.pop().unwrap_or_default();

                        self.stack.push(a.saturating_div(b));
                    }
                    OpCode::Mod => {
                        self.pc += 1;
                        let b: u8 = self.stack.pop().unwrap_or_default();
                        let a: u8 = self.stack.pop().unwrap_or_default();

                        self.stack.push(a % b);
                    }
                    OpCode::AddI => {
                        self.pc += 1;
                        let b: u8 = self.tokens[self.pc];
                        let a: u8 = self.stack.pop().unwrap_or_default();

                        self.stack.push(a.saturating_add(b));
                    }
                    OpCode::SubI => {
                        self.pc += 1;
                        let b: u8 = self.tokens[self.pc];
                        let a: u8 = self.stack.pop().unwrap_or_default();

                        self.stack.push(a.saturating_sub(b));
                    }
                    OpCode::MulI => {
                        self.pc += 1;
                        let b: u8 = self.tokens[self.pc];
                        let a: u8 = self.stack.pop().unwrap_or_default();

                        self.stack.push(a.saturating_mul(b));
                    }
                    OpCode::DivI => {
                        self.pc += 1;
                        let b: u8 = self.tokens[self.pc];
                        let a: u8 = self.stack.pop().unwrap_or_default();

                        self.stack.push(a.saturating_div(b));
                    }
                    OpCode::ModI => {
                        self.pc += 1;
                        let b: u8 = self.tokens[self.pc];
                        let a: u8 = self.stack.pop().unwrap_or_default();

                        self.stack.push(a % b);
                    }
                    OpCode::Jz => {
                        if let Some(flg) = self.stack.pop() {
                            self.pc += 1;
                            if flg == 0 {
                                let dst = self.tokens[self.pc];
                                self.pc = dst as usize;
                                continue;
                            }
                        } else {
                            irregular("Not exist flag", token);
                        }
                    }
                    OpCode::Jmz => {
                        self.pc += 1;
                        let dst = self.tokens[self.pc];
                        self.pc = dst as usize;
                        continue;
                    }
                    OpCode::Store => {
                        self.pc += 1;
                        let mem_dst: usize = self.tokens[self.pc] as usize;

                        if let Some(target) = self.stack.pop() {
                            self.memory[mem_dst] = Some(target);
                        } else {
                            irregular("self.Stack is empty", token);
                        }
                    }
                    OpCode::Load => {
                        self.pc += 1;
                        let mem_dst: usize = self.tokens[self.pc] as usize;

                        if let Some(target) = self.memory[mem_dst] {
                            self.stack.push(target);
                        } else {
                            irregular("Not exist in designated address", token);
                        }
                    }
                    OpCode::Print => {
                        if let Some(value) = self.stack.pop() {
                            println!("{}", value);
                        } else {
                            irregular("self.Stack is empty", token);
                        }
                    }
                    OpCode::Dump => {
                        println!("{:?}", self.stack);
                    }
                    OpCode::Fin => {
                        exit(0);
                    }
                }
            } else {
                irregular("Include invalid OpCode", token);
            }

            self.pc += 1;
        }
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
    vm.run();
}
