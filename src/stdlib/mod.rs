use log::info;

use crate::interpreter::Value;
use std::io::{self, Write};

pub struct StdLib;

impl StdLib {
    pub fn get_builtin_functions() -> Vec<&'static str> {
        vec![
            "println",
            "print",
            "to_string",
            "to_int",
            "to_float",
            "to_bool",
            "read_file",
            "write_file",
        ]
    }

    pub fn handle_builtin_function(name: &str, args: Vec<Value>) -> Result<Value, String> {
        match name {
            "println" => StdLib::println(args),
            "print" => StdLib::print(args),
            "to_string" => StdLib::to_string(args),
            "to_int" => StdLib::to_int(args),
            "to_float" => StdLib::to_float(args),
            "to_bool" => StdLib::to_bool(args),
            "read_file" => StdLib::read_file(args),
            "write_file" => StdLib::write_file(args),
            _ => Err(format!("Unknown built-in function: {}", name)),
        }
    }

    pub fn is_builtin(name: &str) -> bool {
        Self::get_builtin_functions().contains(&name)
    }

    pub fn print(args: Vec<Value>) -> Result<Value, String> {
        if args.len() != 1 {
            return Err("print expects exactly one argument".to_string());
        }

        let output = match &args[0] {
            Value::String(s) => s.clone(),
            Value::Integer(i) => i.to_string(),
            Value::Float(f) => f.to_string(),
            Value::Boolean(b) => b.to_string(),
            _ => return Err("Unsupported type for print".to_string()),
        };

        print!("{}", output);
        io::stdout().flush().map_err(|e| e.to_string())?;
        Ok(Value::Unit)
    }

    pub fn println(args: Vec<Value>) -> Result<Value, String> {
        if args.len() != 1 {
            return Err("println expects exactly one argument".to_string());
        }

        let output = match &args[0] {
            Value::String(s) => s.clone(),
            Value::Integer(i) => i.to_string(),
            Value::Float(f) => f.to_string(),
            Value::Boolean(b) => b.to_string(),
            _ => return Err("Unsupported type for println".to_string()),
        };

        println!("{}", output);
        Ok(Value::Unit)
    }

    pub fn to_string(args: Vec<Value>) -> Result<Value, String> {
        if args.len() != 1 {
            return Err("to_string expects exactly one argument".to_string());
        }

        let result = match &args[0] {
            Value::Integer(i) => i.to_string(),
            Value::Float(f) => f.to_string(),
            Value::Boolean(b) => b.to_string(),
            Value::String(s) => s.clone(),
            _ => return Err("Cannot convert value to string".to_string()),
        };

        Ok(Value::String(result))
    }

    pub fn to_int(args: Vec<Value>) -> Result<Value, String> {
        if args.len() != 1 {
            return Err("to_int expects exactly one argument".to_string());
        }

        let result = match &args[0] {
            Value::String(s) => s
                .parse::<i32>()
                .map_err(|_| "Failed to parse string as integer".to_string())?,
            Value::Float(f) => *f as i32,
            Value::Integer(i) => *i,
            _ => return Err("Cannot convert value to integer".to_string()),
        };

        Ok(Value::Integer(result))
    }

    pub fn to_float(args: Vec<Value>) -> Result<Value, String> {
        if args.len() != 1 {
            return Err("to_float expects exactly one argument".to_string());
        }

        let result = match &args[0] {
            Value::String(s) => s
                .parse::<f64>()
                .map_err(|_| "Failed to parse string as float".to_string())?,
            Value::Integer(i) => *i as f64,
            Value::Float(f) => *f,
            _ => return Err("Cannot convert value to float".to_string()),
        };

        Ok(Value::Float(result))
    }

    pub fn to_bool(args: Vec<Value>) -> Result<Value, String> {
        if args.len() != 1 {
            return Err("to_bool expects exactly one argument".to_string());
        }

        let result = match &args[0] {
            Value::String(s) => s
                .parse::<bool>()
                .map_err(|_| "Failed to parse string as boolean".to_string())?,
            Value::Integer(i) => *i != 0,
            Value::Boolean(b) => *b,
            _ => return Err("Cannot convert value to boolean".to_string()),
        };

        Ok(Value::Boolean(result))
    }

    pub fn read_file(args: Vec<Value>) -> Result<Value, String> {
        if args.len() != 1 {
            return Err("read_file expects exactly one argument".to_string());
        }

        let filename = match &args[0] {
            Value::String(s) => s,
            _ => return Err("read_file expects a string argument".to_string()),
        };

        info!("Reading from file: {}", filename);

        let contents = std::fs::read_to_string(filename).map_err(|e| e.to_string())?;
        Ok(Value::String(contents))
    }

    pub fn write_file(args: Vec<Value>) -> Result<Value, String> {
        if args.len() != 2 {
            return Err("write_file expects exactly two arguments".to_string());
        }

        let filename = match &args[0] {
            Value::String(s) => s,
            _ => return Err("write_file expects a string as the first argument".to_string()),
        };

        let contents = match &args[1] {
            Value::String(s) => s,
            _ => return Err("write_file expects a string as the second argument".to_string()),
        };

        info!("Writing to file: {}", filename);

        std::fs::write(filename, contents).map_err(|e| e.to_string())?;
        Ok(Value::Unit)
    }
}
