use std::collections::HashMap;

use crate::opcode::OpCode;

#[derive(Debug, Clone)]
pub enum TokenKind {
    Op(OpCode),
    Int(u8),
    LabelDef(String),
    LabelRef(String),
}

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub line: usize,
}
impl Token {
    pub fn new(kind: TokenKind, line: usize) -> Self {
        Self { kind, line }
    }
}

pub fn parse(input: String) -> Result<Vec<Token>, String> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut line: usize = 0;

    let split_tokens: Vec<&str> = input.split('\n').collect::<Vec<&str>>();

    for split_token in split_tokens.iter() {
        line += 1;

        let code = split_token.split(';').next().unwrap_or("");

        for token in code.split_whitespace() {
            // ignore comment
            if token.starts_with(';') {
                break;
            }

            let token = token.to_uppercase();

            // case OpCode
            if let Some(opcode) = OpCode::from_str(token.as_str()) {
                tokens.push(Token {
                    kind: TokenKind::Op(opcode),
                    line,
                });

                continue;
            }

            // case number such as 0x...
            if token.starts_with("0X") {
                let token = token.strip_prefix("0X").unwrap();
                match u8::from_str_radix(token, 16) {
                    Ok(number) => {
                        tokens.push(Token {
                            kind: TokenKind::Int(number),
                            line,
                        });
                    }
                    Err(_) => {
                        return Err(format!("Line {}: Invalid hex number '{}'", line, token));
                    }
                };

                continue;
            }

            // case number such as 10(= radix 10)
            if let Ok(number) = u8::from_str_radix(token.as_str(), 10) {
                tokens.push(Token {
                    kind: TokenKind::Int(number),
                    line,
                });

                continue;
            }

            if token.ends_with(':') {
                let token = token.strip_suffix(':').unwrap();
                tokens.push(Token {
                    kind: TokenKind::LabelDef(token.to_string()),
                    line,
                });

                continue;
            }

            tokens.push(Token {
                kind: TokenKind::LabelRef(token.to_string()),
                line,
            });
        }
    }

    Ok(tokens)
}

pub fn resolve(tokens: Vec<Token>) -> Result<Vec<u8>, String> {
    let mut labels: HashMap<String, usize> = HashMap::new();
    let mut address: usize = 0;

    for token in &tokens {
        match &token.kind {
            TokenKind::LabelDef(label) => {
                labels.insert(label.clone(), address);
            }
            _ => address += 1,
        }
    }

    let mut binary: Vec<u8> = Vec::new();
    binary.extend_from_slice(&[0x00, b'T', b'W', b'N']);

    for token in &tokens {
        match &token.kind {
            TokenKind::LabelDef(_) => {}
            TokenKind::LabelRef(label) => match labels.get(label) {
                Some(value) => binary.push(*value as u8),
                None => return Err(format!("Line {}: Unknown label '{}'", token.line, label)),
            },
            TokenKind::Op(opcode) => {
                binary.push(*opcode as u8);
            }
            TokenKind::Int(number) => {
                binary.push(*number as u8);
            }
        }
    }

    Ok(binary)
}
