use interpreter::Value;
use lexer::Token;
use log::{error, info};
use std::fs;
use std::io::{self, BufRead, Write};
mod interpreter;
mod lexer;
mod parser;

use crate::interpreter::Interpreter;
use crate::lexer::Lexer;
use crate::parser::Parser;

fn execute_file(path: &str) -> Result<(), String> {
    info!("Executing file: {}", path);
    let content = fs::read_to_string(path).map_err(|e| format!("Failed to read file: {}", e))?;
    let mut interpreter = Interpreter::new();
    execute_code(&content, &mut interpreter)
}

fn execute_code(source: &str, interpreter: &mut Interpreter) -> Result<(), String> {
    // Create lexer
    let mut lexer = Lexer::new(source.to_string());
    let mut tokens = Vec::new();

    // const MAX_TOKENS: usize = 1000;

    // Collect tokens
    // while tokens.len() < MAX_TOKENS {
    loop {
        let token = lexer.next_token();
        match token {
            Token::EOF => break,
            Token::Invalid(c) => return Err(format!("Invalid character: {}", c)),
            _ => tokens.push(token),
        }
    }

    // if tokens.len() >= MAX_TOKENS {
    //     return Err("Maximum token limit exceeded".to_string());
    // }

    // Parse tokens
    let mut parser = Parser::new(tokens);
    let ast = parser.parse()?;

    // Interpret AST
    for node in ast {
        match interpreter.interpret(node) {
            Ok(value) => {
                if !matches!(value, Value::Unit) {
                    println!("=> {:?}", value);
                }
            }
            Err(e) => {
                error!("Execution error: {}", e);
                return Err(e);
            }
        }
    }

    Ok(())
}

fn run_repl() -> io::Result<()> {
    let mut interpreter = Interpreter::new();

    loop {
        print!("aki> ");
        io::stdout().flush()?;

        // Read input
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(n) => {
                // Check if we got any input
                if n == 0 {
                    // EOF reached
                    println!("\nGoodbye!");
                    break;
                }

                let trimmed = input.trim();
                if trimmed.is_empty() {
                    continue;
                }

                if trimmed == "exit" || trimmed == "quit" {
                    println!("Goodbye!");
                    break;
                }

                println!("Processing input: {}", trimmed); // Debug print
                match execute_code(trimmed, &mut interpreter) {
                    Ok(_) => (),
                    Err(e) => eprintln!("Error: {}", e),
                }
            }
            Err(error) => {
                eprintln!("Error reading input: {}", error);
                break;
            }
        }
    }
    Ok(())
}

fn main() {
    env_logger::init();
    info!("Animikiikode interpreter starting...");
    println!("Animikiikode v{}", env!("CARGO_PKG_VERSION"));

    let args: Vec<String> = std::env::args().collect();
    match args.len() {
        1 => {
            if let Err(e) = run_repl() {
                error!("REPL error: {}", e);
                std::process::exit(1);
            }
        }
        2 => {
            let file_path = &args[1];
            if let Err(e) = execute_file(file_path) {
                error!("Execution error: {}", e);
                std::process::exit(1);
            }
        }
        _ => {
            println!("Usage: aki [script.aki]");
            std::process::exit(1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_execution() {
        let mut interpreter = Interpreter::new();
        let result = execute_code("let x: i32 = 5 + 3;", &mut interpreter);
        assert!(result.is_ok());
    }

    #[test]
    fn test_function_execution() {
        let mut interpreter = Interpreter::new();
        let code = r#"
            func add(x: i32, y: i32) -> i32 {
                x + y
            }
            let result = add(5, 3);
        "#;
        let result = execute_code(code, &mut interpreter);
        assert!(result.is_ok());
    }

    #[test]
    fn test_invalid_syntax() {
        let mut interpreter = Interpreter::new();
        let result = execute_code("let x: i32 = ;", &mut interpreter);
        assert!(result.is_err());
    }
}
