use std::process::{Command, exit};

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();

    if args.is_empty() {
        eprintln!("Usage: twn <FILE_BASE_NAME>"); // cargo run --bin などの記述を消す
        exit(1);
    }

    let base_name = &args[0];
    let source_file = format!("{}.twn", base_name);
    let binary_file = format!("{}.twnd", base_name);

    // 1. twnc を直接実行
    // ※ twnc.exe がPATHの通った場所にあるか、カレントディレクトリにある必要があります
    println!("[twn] Assembling {}...", source_file);
    let status_twnc = Command::new("twnc").arg(&source_file).status(); // expectは外して、Resultで受けてエラーハンドリングするのがベター

    match status_twnc {
        Ok(status) if status.success() => {}
        _ => {
            eprintln!("Error: Failed to execute 'twnc'. Make sure it is installed or in PATH.");
            exit(1);
        }
    }

    // 2. twnvm を直接実行
    println!("[twn] Running VM with {}...", binary_file);
    let status_twnvm = Command::new("twnvm").arg(&binary_file).status();

    match status_twnvm {
        Ok(status) if status.success() => {}
        _ => {
            eprintln!("Error: Failed to execute 'twnvm'. Make sure it is installed or in PATH.");
            exit(1);
        }
    }
}
