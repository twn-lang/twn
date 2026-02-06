use std::process::exit;

pub fn irregular(statement: &'static str, opcode: u8) {
    eprintln!("{statement} (opcode: {opcode:02X})");
    exit(1);
}
