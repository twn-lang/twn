use std::process::{Command, exit};

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();

    if args.is_empty() {
        eprintln!("Usage: twn <FILE_BASE_NAME>");
        exit(1);
    }

    let base_name = &args[0];
    let source_file = format!("{}.twn", base_name);
    let binary_file = format!("{}.twnd", base_name);

    // 1. twnc (アセンブラ) の実行
    println!("[twn] Assembling {}...", source_file);
    let status_twnc = Command::new("twnc").arg(&source_file).status();

    match status_twnc {
        Ok(status) => {
            if !status.success() {
                // コンパイルエラーなどで終了した場合。
                // twncがエラーを出しているはずなので、そのまま終了コードを引き継いで終了する
                exit(status.code().unwrap_or(1));
            }
        }
        Err(_) => {
            // コマンド自体が見つからない場合
            eprintln!("Error: Failed to execute 'twnc'. Make sure it is installed or in PATH.");
            exit(1);
        }
    }

    // 2. twnvm (VM) の実行
    println!("[twn] Running VM with {}...", binary_file);
    let status_twnvm = Command::new("twnvm").arg(&binary_file).status();

    match status_twnvm {
        Ok(status) => {
            if !status.success() {
                // VMが実行時エラー (UnexpectedEofなど) で終了した場合。
                // 既にVMがエラーメッセージを出力しているので、静かに終了する
                exit(status.code().unwrap_or(1));
            }
        }
        Err(_) => {
            // コマンド自体が見つからない場合
            eprintln!("Error: Failed to execute 'twnvm'. Make sure it is installed or in PATH.");
            exit(1);
        }
    }
}
