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
        self.index += 1; /* skip lambda */
        let Some(Token::Identifyer(param)) =
            self.tokens.get(self.index).cloned()
        else {
            return Err("Expected parameter name".to_string());
        };
        self.index += 1;
        let Some(Token::Dot) = self.tokens.get(self.index) else {
            return Err("Expected dot".to_string());
        };
        self.index += 1;
        if let Some(Token::Lambda) = self.tokens.get(self.index) {
            /* special treatment for \x.\y.N
             * Without this, (\x.\y.N) M is parsed as (\x.(\y.N) M) */
            let inner_function = self.parse_function()?;
            return Ok(Expression::Function {
                param,
                body: Box::new(inner_function),
            });
        }
        let body = self.parse_application_chain()?;
        Ok(Expression::Function {
            param,
            body: Box::new(body),
        })
    }

    fn parse_application_chain(&mut self) -> Result<Expression, String> {
        let mut exprs = Vec::<Expression>::new();
        loop {
            let Some(token) = self.tokens.get(self.index).cloned() else {
                break;
            };
            match token {
                Token::Lambda => exprs.push(self.parse_function()?),
                Token::Identifyer(name) => {
                    self.index += 1;
                    exprs.push(Expression::Variable(name))
                }
                Token::LeftPar => {
                    self.index += 1; /* skip ( */
                    exprs.push(self.parse_application_chain()?);
                    self.index += 1; /* skip ) */
                }
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
