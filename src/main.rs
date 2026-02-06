use std::{
    io::{self, Read},
    process::exit,
};

const MEMORY_SIZE: usize = 256;

/*
 * OpCode
 * 0x01: PUSH
 * 0x10: ADD
 * 0x11: SUB
 * 0x12: MUL
 * 0x13: DIV
 * 0x20: JZ
 * 0x21: JMZ
 * 0x30: STORE
 * 0x31: LOAD
 * 0x90: PRINT
 * 0x91: DUMP
 * 0xFF: FIN
 */

fn irregular(statement: &'static str) {
    eprintln!("{statement}");
    exit(1);
}

fn main() {
    let mut input: String = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .expect("Input is empty");

    let tokens: Vec<u8> = input
        .split_whitespace()
        .map(|token| u8::from_str_radix(token, 16).expect("Included invalid token"))
        .collect::<Vec<u8>>();

    let mut pc: usize = 0;
    let mut stack: Vec<u8> = Vec::new();
    let mut memory: Vec<Option<u8>> = vec![None; MEMORY_SIZE];

    while pc < tokens.len() {
        let token = tokens[pc];

        match token {
            // PUSH
            0x01 => {
                pc += 1;
                stack.push(tokens[pc]);
            }
            // ADD
            0x10 => {
                let b: u8 = stack.pop().unwrap_or_default();
                let a: u8 = stack.pop().unwrap_or_default();

                stack.push(a.saturating_add(b));
            }
            // SUB
            0x11 => {
                let b: u8 = stack.pop().unwrap_or_default();
                let a: u8 = stack.pop().unwrap_or_default();

                stack.push(a.saturating_sub(b));
            }
            // MUL
            0x12 => {
                let b: u8 = stack.pop().unwrap_or_default();
                let a: u8 = stack.pop().unwrap_or_default();

                stack.push(a.saturating_mul(b));
            }
            // DIV
            0x13 => {
                let b: u8 = stack.pop().unwrap_or_default();
                let a: u8 = stack.pop().unwrap_or_default();

                stack.push(a.saturating_div(b));
            }
            // JZ
            0x20 => {
                if let Some(&flg) = stack.last() {
                    pc += 1;
                    if flg == 0 {
                        let dst = tokens[pc];
                        pc = dst as usize;
                        continue;
                    }
                } else {
                    irregular("Not exist flag");
                }
            }
            // JMZ
            0x21 => {
                pc += 1;
                let dst = tokens[pc];
                pc = dst as usize;
                continue;
            }
            // STORE
            0x30 => {
                pc += 1;
                let mem_dst: usize = tokens[pc] as usize;

                if let Some(target) = stack.pop() {
                    memory[mem_dst] = Some(target);
                } else {
                    irregular("Stack is empty");
                }
            }
            // LOAD
            0x31 => {
                pc += 1;
                let mem_dst: usize = tokens[pc] as usize;

                if let Some(target) = memory[mem_dst] {
                    stack.push(target);
                } else {
                    irregular("Not exist in designated address");
                }
            }
            // PRINT
            0x90 => {
                if let Some(value) = stack.last() {
                    println!("{}", value);
                } else {
                    irregular("Stack is empty");
                }
            }
            // DUMP
            0x91 => {
                println!("{:?}", stack);
            }
            // FIN
            0xFF => {
                exit(0);
            }
            _ => {
                irregular("Include invalid OpCode");
            }
        }

        pc += 1;
    }
}
