use std::io::Read;

#[derive(Debug, Clone)]
pub enum Token {
    Lambda, // \
    Identifyer(String),
    Dot, //
    LeftPar, // (
    RightPar,  // )
}

pub struct TokenStream<R: Read> {
    input: R,
    current_char: Option<char>,
}

impl<R: Read> TokenStream<R> {
    pub fn new(input: R) -> Self {
        Self {
            input,
            current_char: None,
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.current_char {
            if c.is_whitespace() {
                self.next_char();
            } else {
                break;
            }
        }
    }

    fn next_char(&mut self) {
        let mut c = [0; 1];
        if let Ok(_) = self.input.read(&mut c) {
            self.current_char = Some(c[0] as char);
        } else {
            self.current_char = None;
        }
    }

    fn is_identifiable(c: char) -> bool {
        c != '\\' && c != '.' && c != '(' && c != ')' && !c.is_whitespace()
    }

    pub fn next(&mut self) -> Option<Token> {
        if self.current_char == None {
            self.next_char()
        }
        self.skip_whitespace();
        let c = self.current_char?;
        match c {
            '\0' => { // '\0' is termination of stdin (Ctrl+D in UNIX, Ctrl+Z in Windows)
                self.current_char = None;
                return None;
            }
            '\\' | '.' | '(' | ')' => {
                self.current_char = None;
                match c {
                    '\\' => return Some(Token::Lambda),
                    '.' => return Some(Token::Dot),
                    '(' => return Some(Token::LeftPar),
                    ')' => return Some(Token::RightPar),
                    _ => unreachable!(),
                }
            }
            _ => {}
        }
        let mut str = String::new();
        while let Some(c) = self.current_char {
            if Self::is_identifiable(c) {
                str.push(c);
                self.next_char();
            } else {
                break;
            }
        }
        Some(Token::Identifyer(str))
    }
}
