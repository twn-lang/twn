use std::collections::HashMap;
use std::io::Write;
use std::path::Path;
use std::process::exit;

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

    let opcode_map: HashMap<&str, u8> = HashMap::from([
        ("PUSH", 0x01),
        ("POP", 0x02),
        ("ADD", 0x10),
        ("SUB", 0x11),
        ("MUL", 0x12),
        ("DIV", 0x13),
        ("MOD", 0x14),
        ("ADDI", 0x15),
        ("SUBI", 0x16),
        ("MULI", 0x17),
        ("DIVI", 0x18),
        ("MODI", 0x19),
        ("JZ", 0x20),
        ("JMZ", 0x21),
        ("STORE", 0x30),
        ("LOAD", 0x31),
        ("PRINT", 0x90),
        ("DUMP", 0x91),
        ("FIN", 0xFF),
    ]);

    let mut label_count: u8 = 0;
    let mut labels: HashMap<String, u8> = HashMap::new();

    let split_tokens: Vec<&str> = input.split('\n').collect::<Vec<&str>>();

    for split_token in split_tokens.iter() {
        for token in split_token.split_whitespace() {
            if token.starts_with(';') {
                break;
            }

            label_count += 1;
            if token.ends_with(':') {
                label_count -= 1;
                let label = token.strip_suffix(':').unwrap();

                labels.insert(label.to_string(), label_count);
            }
        }
    }

    for split_token in split_tokens.iter() {
        for token in split_token.split_whitespace() {
            if token.starts_with(';') {
                break;
            }

            label_count += 1;
            let token = token.to_uppercase();

            if let Some(opcode) = opcode_map.get(token.as_str()) {
                write!(output_file, "{:02X} ", opcode).expect("Failed write in file");
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
