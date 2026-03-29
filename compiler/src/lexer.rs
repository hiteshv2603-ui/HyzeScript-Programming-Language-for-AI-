// HyzeScript/compiler/src/lexer.rs

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Let,
    Ident(String),
    Equals,
    Comma,
    LBrace,
    RBrace,
    LParen,
    RParen,
    LBracket,
    RBracket,
    Dot,
    Colon,
    LAngle,          // <
    RAngle,          // >
    Semi,
    Number(f64),
    String(String),
    Comment(String),

    // Keywords
    Tensor,
    Nn,
    Model,
    Backward,
    Print,
    IpuAnnotation,   // #[ipu]
}

pub fn tokenize(source: &str) -> Result<Vec<Token>, String> {
    let mut tokens = Vec::new();
    let mut chars = source.chars().peekable();

    while let Some(c) = chars.next() {
        match c {
            ' ' | '\t' | '\n' => continue,

            '0'..='9' => {
                let mut num_str = String::new();
                num_str.push(c);
                while let Some(&next) = chars.peek() {
                    if next.is_ascii_digit() || next == '.' {
                        num_str.push(chars.next().unwrap());
                    } else {
                        break;
                    }
                }
                let num = num_str.parse::<f64>().map_err(|_| "invalid number")?;
                tokens.push(Token::Number(num));
            }

            '"' => {
                let mut s = String::new();
                while let Some(ch) = chars.next() {
                    if ch == '"' {
                        break;
                    }
                    s.push(ch);
                }
                tokens.push(Token::String(s));
            }

            ',' => tokens.push(Token::Comma),
            '.' => tokens.push(Token::Dot),
            ':' => tokens.push(Token::Colon),
            '<' => tokens.push(Token::LAngle),
            '>' => tokens.push(Token::RAngle),
            ';' => tokens.push(Token::Semi),
            '(' => tokens.push(Token::LParen),
            ')' => tokens.push(Token::RParen),
            '[' => tokens.push(Token::LBracket),
            ']' => tokens.push(Token::RBracket),
            '{' => tokens.push(Token::LBrace),
            '}' => tokens.push(Token::RBrace),

            '#' => {
                if let Some('[') = chars.peek() {
                    chars.next();
                    if let Some('i') = chars.peek() {
                        chars.next();
                        if let Some('p') = chars.peek() {
                            chars.next();
                            if let Some('u') = chars.peek() {
                                chars.next();
                                if let Some(']') = chars.peek() {
                                    chars.next();
                                    tokens.push(Token::IpuAnnotation);
                                } else {
                                    return Err("'#[ipu]' annotation malformed".to_string());
                                }
                            }
                        }
                    }
                }
            }

            '=' => tokens.push(Token::Equals),

            c if c.is_ascii_alphabetic() => {
                let mut ident = String::new();
                ident.push(c);
                while let Some(&next) = chars.peek() {
                    if next.is_ascii_alphanumeric() || next == '_' {
                        ident.push(chars.next().unwrap());
                    } else {
                        break;
                    }
                }
                let token = match ident.as_str() {
                    "let" => Token::Let,
                    "tensor" => Token::Tensor,
                    "nn" => Token::Ident("nn".to_string()),
                    "backward" => Token::Backward,
                    "print" => Token::Print,
                    s => Token::Ident(s.to_string()),
                };
                tokens.push(token);
            }

            _ => return Err(format!("unexpected char '{}'", c)),
        }
    }

    Ok(tokens)
}
