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
        ]
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
}
