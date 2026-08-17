use crate::lex;
use crate::lex::Token;

use std::io::Read;

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

pub struct Parser<R: Read> {
    stream: lex::TokenStream<R>,
}

impl<R: Read> Parser<R> {
    pub fn new(stream: lex::TokenStream<R>) -> Self {
        Self { stream }
    }

    // lambda is already consumed, so we expect an identifier, a dot, and then an expression
    fn parse_function(&mut self) -> Result<Expression, String> {
        let token = self
            .stream
            .next()
            .ok_or("Unexpected end of input, expected identifier")?;
        let Token::Identifyer(param) = token else {
            return Err(format!("Expected identifier, found {:?}", token));
        };
        let token = self
            .stream
            .next()
            .ok_or("Unexpected end of input, expected dot")?;
        let Token::Dot = token else {
            return Err(format!("Expected '.', found {:?}", token));
        };
        let body = self.parse_expression()?;
        Ok(Expression::Function {
            param,
            body: Box::new(body),
        })
    }

    fn parse_expression(&mut self) -> Result<Expression, String> {
        let Some(Token::LeftPar) = self.stream.next() else {
            return Err("Expected '('".to_string());
        };
        if let Some(exp) = self.parse_before_right_par()? {
            Ok(exp)
        } else {
            Err("Empty expression".to_string())
        }
    }

    fn parse_before_right_par(&mut self) -> Result<Option<Expression>, String> {
        let token = self.stream.next().ok_or("Unexpected end of input")?;
        match token {
            Token::RightPar => return Ok(None),
            Token::LeftPar => {
                let func = self.parse_before_right_par()?;
                if let None = func {
                    return Ok(None);
                }
                let arg = self.parse_before_right_par()?;
                if let None = arg {
                    return Ok(func);
                }
                return Ok(Some(Expression::Application {
                    function: Box::new(func.unwrap()),
                    arg: Box::new(arg.unwrap()),
                }));
            }
            Token::Lambda => {
                let func = self.parse_function()?;
                let arg = self.parse_before_right_par()?;
                if let None = arg {
                    return Ok(Some(func));
                }
                return Ok(Some(Expression::Application {
                    function: Box::new(func),
                    arg: Box::new(arg.unwrap()),
                }));
            }
            Token::Identifyer(name) => {
                let arg = self.parse_before_right_par()?;
                if let None = arg {
                    return Ok(Some(Expression::Variable(name)));
                }
                return Ok(Some(Expression::Application {
                    function: Box::new(Expression::Variable(name)),
                    arg: Box::new(arg.unwrap()),
                }));
            }
            token => return Err(format!("Unexpected token: {:?}", token)),
        }
    }

    pub fn parse(&mut self) -> Result<Expression, String> {
        return self.parse_expression();
    }
}
