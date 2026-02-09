use std::collections::HashMap;
use std::io::Write;
use std::path::Path;
use std::process::exit;

use twn::OpCode;

fn main() {
    let args = std::env::args().collect::<Vec<String>>();

    if args.len() < 2 {
        eprintln!("Usage: {} <FILE.twn>", args[0]);
        exit(1);
    }

    let input_file = &args[1];
    let input = std::fs::read_to_string(input_file).expect("Input is empty");
    let output = Path::new(input_file).with_extension("twnd");
    let mut output_file = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(output)
        .expect("Failed to open or create the file");

    let mut label_count: u8 = 0;
    let mut labels: HashMap<String, u8> = HashMap::new();

    let split_tokens: Vec<&str> = input.split('\n').collect::<Vec<&str>>();

    for split_token in split_tokens.iter() {
        for token in split_token.split_whitespace() {
            if token.starts_with(';') {
                break;
            }

            if token.ends_with(':') {
                let label = token.strip_suffix(':').unwrap();

                labels.insert(label.to_string(), label_count);
                continue;
            }
            label_count += 1;
        }
    }

    for split_token in split_tokens.iter() {
        for token in split_token.split_whitespace() {
            if token.starts_with(';') {
                break;
            }

            let token = token.to_uppercase();

            if token.starts_with("0X") {
                let token = token.strip_prefix("0X").unwrap();
                write!(
                    output_file,
                    "{:02X} ",
                    u8::from_str_radix(token, 16).unwrap()
                )
                .expect("Failed write in file");

                continue;
            }

            if let Some(opcode) = OpCode::from_str(token.as_str()) {
                write!(output_file, "{:02X} ", opcode as u8).expect("Failed write in file");
                continue;
            }

            if let Ok(number) = u8::from_str_radix(token.as_str(), 10) {
                write!(output_file, "{:02X} ", number).expect("Failed write in file");
                continue;
            }

            if token.ends_with(':') {
                continue;
            }

            if !labels.contains_key(token.as_str()) {
                labels.insert(token.to_string(), label_count);
            }

            write!(output_file, "{:02X} ", labels.get(token.as_str()).unwrap())
                .expect("Failed write in file");
        }
        writeln!(output_file).expect("Failed write in file");
    }
}
