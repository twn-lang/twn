use std::process::exit;

use twn::assembler;

fn main() {
    let args = std::env::args().collect::<Vec<String>>();

    if args.len() < 2 {
        eprintln!("Usage: {} <FILE.twn>", args[0]);
        exit(1);
    }

    let input_file = &args[1];
    let input = std::fs::read_to_string(input_file).expect("Input is empty");

    let tokens = match assembler::parse(input) {
        Ok(t) => t,
        Err(msg) => {
            eprintln!("Parse Error -> {}", msg);
            exit(1);
        }
    };

    println!("Successfully parsed {} tokens!", tokens.len());
    println!("{:#?}", tokens);
}
