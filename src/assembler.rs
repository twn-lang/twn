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
    pub word: usize,
}
impl Token {
    pub fn new(kind: TokenKind, line: usize, word: usize) -> Self {
        Self { kind, line, word }
    }
}

pub fn parse(input: String) -> Result<Vec<Token>, String> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut line: usize = 0;

    let split_tokens: Vec<&str> = input.split('\n').collect::<Vec<&str>>();

    for split_token in split_tokens.iter() {
        line += 1;
        let mut word: usize = 0;

        let code = split_token.split(';').next().unwrap_or("");

        for token in code.split_whitespace() {
            word += 1;

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
                    word,
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
                            word,
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
                    word,
                });

                continue;
            }

            if token.ends_with(':') {
                let token = token.strip_suffix(':').unwrap();
                tokens.push(Token {
                    kind: TokenKind::LabelDef(token.to_string()),
                    line,
                    word,
                });

                continue;
            }

            tokens.push(Token {
                kind: TokenKind::LabelRef(token.to_string()),
                line,
                word,
            });
        }
    }

    Ok(tokens)
}
