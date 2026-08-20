use std::{iter::Peekable, str::Chars};

#[derive(Debug, Clone)]
pub enum Token {
    Lambda, // \
    Identifyer(String),
    Dot,      // .
    LeftPar,  // (
    RightPar, // )
}

pub struct TokenStream<'a> {
    input: Peekable<Chars<'a>>,
}

impl<'a> TokenStream<'a> {
    pub fn new(input: Chars<'a>) -> Self {
        Self {
            input: input.peekable(),
        }
    }

    // Very large amount of extraordinary chars are passed LOL
    fn is_identifiable(c: char) -> bool {
        c != '\\' && c != '.' && c != '(' && c != ')' && !c.is_whitespace()
    }

    pub fn next(&mut self) -> Option<Token> {
        let chr;
        // skip whitespaces
        loop {
            let Some(c) = self.input.next() else {
                return None;
            };
            if !c.is_whitespace() {
                chr = c;
                break;
            }
        }
        match chr {
            '\0' => unreachable!(), // The caller dares not ever to send me a null character!
            '\\' => return Some(Token::Lambda),
            '.' => return Some(Token::Dot),
            '(' => return Some(Token::LeftPar),
            ')' => return Some(Token::RightPar),
            _ => {}
        }

        let mut str = String::new();
        str.push(chr);
        loop {
            let Some(c) = self.input.peek() else {
                break;
            };
            let c = c.clone();
            if !Self::is_identifiable(c) {
                break;
            }
            str.push(c);
            self.input.next();
        }
        Some(Token::Identifyer(str))
    }

    pub fn collect(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        while let Some(token) = self.next() {
            tokens.push(token);
        }
        tokens
    }
}
