use std::io::{stdin, stdout};
use std::process::exit;

use twn::vm::VM;

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

    let mut vm = VM::new(tokens, stdin().lock(), stdout().lock());

    if let Err(e) = vm.run() {
        eprintln!("Error: {} (at address 0x{:02X})", e, vm.pc);
        std::process::exit(1);
    }

    std::process::exit(vm.exit_code as i32);
}
