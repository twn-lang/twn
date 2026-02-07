use std::process::exit;

use twn::utils::irregular;

const MEMORY_SIZE: usize = 256;

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
            // POP
            0x02 => {
                if stack.is_empty() {
                    irregular("Stack is empty", token);
                }
                stack.pop().unwrap();
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
            // MOD
            0x14 => {
                pc += 1;
                let b: u8 = stack.pop().unwrap_or_default();
                let a: u8 = stack.pop().unwrap_or_default();

                stack.push(a % b);
            }
            // ADDI
            0x15 => {
                pc += 1;
                let b: u8 = tokens[pc];
                let a: u8 = stack.pop().unwrap_or_default();

                stack.push(a.saturating_add(b));
            }
            // SUBI
            0x16 => {
                pc += 1;
                let b: u8 = tokens[pc];
                let a: u8 = stack.pop().unwrap_or_default();

                stack.push(a.saturating_sub(b));
            }
            // MULI
            0x17 => {
                pc += 1;
                let b: u8 = tokens[pc];
                let a: u8 = stack.pop().unwrap_or_default();

                stack.push(a.saturating_mul(b));
            }
            // DIVI
            0x18 => {
                pc += 1;
                let b: u8 = tokens[pc];
                let a: u8 = stack.pop().unwrap_or_default();

                stack.push(a.saturating_div(b));
            }
            // MODI
            0x19 => {
                pc += 1;
                let b: u8 = tokens[pc];
                let a: u8 = stack.pop().unwrap_or_default();

                stack.push(a % b);
            }
            // JZ
            0x20 => {
                if let Some(flg) = stack.pop() {
                    pc += 1;
                    if flg == 0 {
                        let dst = tokens[pc];
                        pc = dst as usize;
                        continue;
                    }
                } else {
                    irregular("Not exist flag", token);
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
                    irregular("Stack is empty", token);
                }
            }
            // LOAD
            0x31 => {
                pc += 1;
                let mem_dst: usize = tokens[pc] as usize;

                if let Some(target) = memory[mem_dst] {
                    stack.push(target);
                } else {
                    irregular("Not exist in designated address", token);
                }
            }
            // PRINT
            0x90 => {
                if let Some(value) = stack.pop() {
                    println!("{}", value);
                } else {
                    irregular("Stack is empty", token);
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
                irregular("Include invalid OpCode", token);
            }
        }

        pc += 1;
    }
}
