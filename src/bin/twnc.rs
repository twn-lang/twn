use std::io::Write;
use std::path::Path;
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

    let output_path = Path::new(input_file).with_extension("twnd");
    let mut output_file = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(output_path)
        .expect("Failed to open output file");

    let tokens = match assembler::parse(input) {
        Ok(t) => t,
        Err(msg) => {
            eprintln!("Parse Error -> {}", msg);
            exit(1);
        }
    };

    let binary = match assembler::resolve(tokens) {
        Ok(b) => b,
        Err(msg) => {
            eprintln!("Assemble Error -> {}", msg);
            exit(1);
        }
    };

    if let Err(e) = output_file.write_all(&binary) {
        eprintln!("Failed to write binary: {}", e);
        exit(1);
    }

    println!(
        "Successfully assembled to {}",
        Path::new(input_file).with_extension("twnd").display()
    );
}
