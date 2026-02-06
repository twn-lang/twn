use std::{io, process::exit};

/*
 * OpCode
 * 0x01: PUSH
 * 0x11: ADD
 * 0x12: SUB
 * 0x13: MUL
 * 0x14: DIV
 * 0x99: PRINT
 */

fn main() {
    let mut input: String = String::new();
    io::stdin().read_line(&mut input).expect("Input is empty");

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
            0x11 => {
                let b: u8 = stack.pop().unwrap_or_default();
                let a: u8 = stack.pop().unwrap_or_default();

                stack.push(a.saturating_add(b));
            }
            0x12 => {
                let b: u8 = stack.pop().unwrap_or_default();
                let a: u8 = stack.pop().unwrap_or_default();

                stack.push(a.saturating_sub(b));
            }
            0x13 => {
                let b: u8 = stack.pop().unwrap_or_default();
                let a: u8 = stack.pop().unwrap_or_default();

                stack.push(a.saturating_mul(b));
            }
            0x14 => {
                let b: u8 = stack.pop().unwrap_or_default();
                let a: u8 = stack.pop().unwrap_or_default();

                stack.push(a.saturating_div(b));
            }
            0x99 => {
                if let Some(value) = stack.last() {
                    println!("{}", value);
                } else {
                    eprintln!("Stack is empty");
                    exit(1);
                }
            }
            _ => {
                eprintln!("Included invalid OpCode");
                exit(1);
            }
        }

        pc += 1;
    }
}
