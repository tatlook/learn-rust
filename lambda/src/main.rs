use std::{
    collections::HashMap,
    io::{BufRead, BufReader, Cursor, Write},
};

use crate::{lex::Token, parser::Expression};

mod eval;
mod lex;
mod parser;

static STANDARD_LIBERARY: &str = r"
set Y       \g. (\x. g (x x)) (\x. g (x x))

set true    \x.\y. x
set false   \x.\y. y
set not     \b.\x.\y. b y x

set pair    \a.\b.\s. s a b
set fst     \p. p true
set snd     \p. p false

set succ    \n.\f.\x. f (n f x)
set add     \m.\n.\f.\x. m f (n f x)
set mul     \m.\n.\f. m (n f)
set is0     \n. n (\x. false) true
set 0       \f.\x. x
set 1       succ 0
set 2       succ 1
) it may be too annoying if we pre-define too much numbers

set pred    \n. fst (n (\p. pair (snd p) (succ (snd p))) (pair 0 0))

) TODO: add more functions
";

fn help() {
    println!(
        "This is a simple lambda calculus interpreter.
eval, e, n or nf <expression>    reduct, normal order
 cn or whnf <expression>         reduct, call-by-name
set or s <variable> <expression> define a variable
cat or c <variable>              look at its value (unevaluated)
       ls                        list all defined variables
       std                       load the standard liberary
       help                      show this message
       exit                      exit the interpreter"
    );
}

fn get_expression(stream: &mut lex::TokenStream) -> Option<parser::Expression> {
    let tokens = stream.collect();
    let parser = parser::Parser::new(tokens);
    match parser.parse() {
        Ok(expr) => Some(expr),
        Err(e) => {
            eprintln!("Error parsing expression: {}", e);
            None
        }
    }
}

fn interpret(
    input: impl BufRead,
    mut hint: impl Write,
    variables: &mut HashMap<String, Expression>,
) {
    let mut lines = input.lines();
    loop {
        write!(hint, ">>> ").unwrap(); /* Let's steal it from Python */
        hint.flush().unwrap();

        let Some(str) = lines.next() else { break };
        let str = str.unwrap();
        let mut stream = lex::TokenStream::new(str.chars());

        let Some(Token::Identifyer(command)) = stream.next() else {
            /* lines begin with anything but identifyer is a comment */
            continue;
        };
        match command.as_str() {
            "exit" => break,
            "help" => {
                help();
                continue;
            }
            "std" => {
                interpret(
                    Cursor::new(String::from(STANDARD_LIBERARY)),
                    &mut std::io::sink(),
                    variables,
                );
            }
            "ls" => {
                for (name, value) in variables.iter() {
                    println!("{} = {}", name, value)
                }
            }
            "eval" | "e" | "n" | "nf" => {
                let Some(expr) = get_expression(&mut stream) else {
                    continue;
                };
                let mut engine = eval::Engine::new();
                let expr = engine.put_variables(expr, &variables);
                let expr = engine.reduct_normal_order(expr);
                println!("{}", expr);
            }
            "cn" | "whnf" => {
                let Some(expr) = get_expression(&mut stream) else {
                    continue;
                };
                let mut engine = eval::Engine::new();
                let expr = engine.put_variables(expr, &variables);
                let expr = engine.reduct_call_by_name(expr);
                println!("{}", expr);
            }
            "set" | "s" => {
                let Some(Token::Identifyer(name)) = stream.next() else {
                    eprintln!("set: Expected variable name");
                    continue;
                };
                let Some(expr) = get_expression(&mut stream) else {
                    continue;
                };
                let mut engine = eval::Engine::new();
                let expr = engine.put_variables(expr, &variables);
                writeln!(hint, "{} = {}", name, expr).unwrap();
                variables.insert(name, expr);
            }
            "cat" | "c" => {
                let Some(Token::Identifyer(name)) = stream.next() else {
                    eprintln!("cat: Expected variable name");
                    continue;
                };
                if let Some(expr) = variables.get(&name) {
                    println!("{} = {}", name, expr)
                } else {
                    println!("{} =", name);
                }
            }
            command => {
                eprintln!("Unknown command: {}", command);
                continue;
            }
        }
    }
}

fn main() {
    println!(
        "No copyright (CC0) 2026 Youzhe Zhen
type `help` for helps, `exit` to exit"
    );
    interpret(
        BufReader::new(std::io::stdin()),
        std::io::stdout(),
        &mut HashMap::<String, Expression>::new(),
    );
}
