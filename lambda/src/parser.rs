use crate::lex::Token;

use std::fmt::Display;

#[derive(Debug, Clone)]
pub enum Expression {
    Variable(String),
    Function {
        param: String,
        body: Box<Expression>,
    },
    Application {
        function: Box<Expression>,
        arg: Box<Expression>,
    },
}

impl Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expression::Variable(name) => write!(f, "{}", name),
            Expression::Function { param, body } => {
                write!(f, "(\\{}. {})", param, body)
            }
            Expression::Application { function, arg } => {
                write!(f, "({} {})", function, arg)
            }
        }
    }
}

pub struct Parser {
    tokens: Vec<Token>,
    index: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            index: 0 as usize,
        }
    }

    fn parse_function(&mut self) -> Result<Expression, String> {
        let Some(Token::Identifyer(name)) =
            self.tokens.get(self.index).cloned()
        else {
            return Err("Expected funtion name".to_string());
        };
        self.index += 1;
        let Some(Token::Dot) = self.tokens.get(self.index) else {
            return Err("Expected dot".to_string());
        };
        self.index += 1;
        let body = self.parse_application_chain()?;
        Ok(Expression::Function {
            param: name,
            body: Box::new(body),
        })
    }

    fn parse_application_chain(&mut self) -> Result<Expression, String> {
        let mut exprs = Vec::<Expression>::new();
        loop {
            let Some(token) = self.tokens.get(self.index).cloned() else {
                break;
            };
            self.index += 1;
            match token {
                Token::Lambda => exprs.push(self.parse_function()?),
                Token::Identifyer(name) => {
                    exprs.push(Expression::Variable(name))
                }
                Token::LeftPar => exprs.push(self.parse_application_chain()?),
                Token::RightPar => break,
                Token::Dot => return Err("Unexpected dot".to_string()),
            }
        }
        /* Reassemble the list into a tree left-associativily
         * Like from a b c d e to (((a b) c) d) e */
        let mut exprs = exprs.into_iter();
        let Some(mut left) = exprs.next() else {
            return Err("Empty list".to_string());
        };
        for expr in exprs {
            left = Expression::Application {
                function: Box::new(left),
                arg: Box::new(expr),
            }
        }
        Ok(left)
    }

    pub fn parse(mut self) -> Result<Expression, String> {
        return self.parse_application_chain();
    }
}
