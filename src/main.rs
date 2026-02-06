use std::{io::{self, Read}, process::exit};

/*
 * OpCode
 * 0x01: PUSH
 * 0x10: ADD
 * 0x11: SUB
 * 0x12: MUL
 * 0x13: DIV
 * 0x20: JZ
 * 0x21: JMZ
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
    io::stdin().read_to_string(&mut input).expect("Input is empty");

    let tokens: Vec<u8> = input
        .split_whitespace()
        .map(|token| u8::from_str_radix(token, 16).expect("Included invalid token"))
        .collect::<Vec<u8>>();

    let mut pc: usize = 0;
    let mut stack: Vec<u8> = Vec::new();

    while pc < tokens.len() {
        let token = tokens[pc];

        match token {
            0x01 => {
                pc += 1;
                stack.push(tokens[pc]);
            }
            0x10 => {
                let b: u8 = stack.pop().unwrap_or_default();
                let a: u8 = stack.pop().unwrap_or_default();

                stack.push(a.saturating_add(b));
            }
            0x11 => {
                let b: u8 = stack.pop().unwrap_or_default();
                let a: u8 = stack.pop().unwrap_or_default();

                stack.push(a.saturating_sub(b));
            }
            0x12 => {
                let b: u8 = stack.pop().unwrap_or_default();
                let a: u8 = stack.pop().unwrap_or_default();

                stack.push(a.saturating_mul(b));
            }
            0x13 => {
                let b: u8 = stack.pop().unwrap_or_default();
                let a: u8 = stack.pop().unwrap_or_default();

                stack.push(a.saturating_div(b));
            }
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
            0x21 => {
                pc += 1;
                let dst = tokens[pc];
                pc = dst as usize;
                continue;
            }
            0x90 => {
                if let Some(value) = stack.last() {
                    println!("{}", value);
                } else {
                    irregular("Stack is empty");
                }
            }
            0x91 => {
                println!("{:?}", stack);
            }
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
