use log::info;
use rand::Rng as _;

use crate::interpreter::Value;
use std::{
    collections::HashMap,
    io::{self, Write},
};

pub struct StdLib;

impl StdLib {
    pub fn get_builtin_functions() -> Vec<&'static str> {
        vec![
            // Type conversion functions
            "to_string",
            "to_int",
            "to_float",
            "to_bool",
            // IO functions
            "file_exists",
            "create_dir",
            "list_dir",
            "remove_file",
            "read_file",
            "write_file",
            "input",
            "raw_input",
            "println",
            "print",
            // String functions
            "split",
            "trim",
            "contains",
            "replace",
            // Math functions
            "abs",
            "max",
            "min",
            "sqrt",
            "pow",
            // Random functions
            "random",
            "random_range",
            "random_choice",
            // Collections functions
            "new_vector",
            "push",
            "pop",
            "set",
            "new_hashmap",
            "insert",
            "get",
        ]
    }

    pub fn handle_builtin_function(name: &str, args: Vec<Value>) -> Result<Value, String> {
        match name {
            // Type conversion functions
            "to_string" => StdLib::to_string(args),
            "to_int" => StdLib::to_int(args),
            "to_float" => StdLib::to_float(args),
            "to_bool" => StdLib::to_bool(args),
            // Math functions
            "file_exists" => StdLib::file_exists(args),
            "create_dir" => StdLib::create_dir(args),
            "list_dir" => StdLib::list_dir(args),
            "remove_file" => StdLib::remove_file(args),
            "read_file" => StdLib::read_file(args),
            "write_file" => StdLib::write_file(args),
            "input" => StdLib::input(),
            "raw_input" => StdLib::raw_input(),
            "println" => StdLib::println(args),
            "print" => StdLib::print(args),
            // String functions
            "split" => StdLib::split(args),
            "trim" => StdLib::trim(args),
            "contains" => StdLib::contains(args),
            "replace" => StdLib::replace(args),
            // Math functions
            "abs" => StdLib::abs(args),
            "max" => StdLib::max(args),
            "min" => StdLib::min(args),
            "sqrt" => StdLib::sqrt(args),
            "pow" => StdLib::pow(args),
            // Random functions
            "random" => Ok(StdLib::random()),
            "random_range" => StdLib::random_range(args),
            "random_choice" => StdLib::random_choice(args),
            // Collections functions
            "new_vector" => StdLib::vec_new(args),
            "push" => StdLib::vec_push(args),
            "pop" => StdLib::vec_pop(args),
            "set" => StdLib::vec_set(args),
            "new_hashmap" => StdLib::hashmap_new(args),
            "insert" => StdLib::hashmap_insert(args),
            "get" => StdLib::hashmap_get(args),
            _ => Err(format!("Unknown built-in function: {}", name)),
        }
    }

    pub fn is_builtin(name: &str) -> bool {
        Self::get_builtin_functions().contains(&name)
    }

    // Type conversion functions
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

    // IO functions
    pub fn file_exists(args: Vec<Value>) -> Result<Value, String> {
        if args.len() != 1 {
            return Err("file_exists expects exactly one argument".to_string());
        }

        let filename = match &args[0] {
            Value::String(s) => s,
            _ => return Err("file_exists expects a string argument".to_string()),
        };

        Ok(Value::Boolean(std::path::Path::new(filename).exists()))
    }

    pub fn create_dir(args: Vec<Value>) -> Result<Value, String> {
        if args.len() != 1 {
            return Err("create_file expects exactly one argument".to_string());
        }

        let dirname = match &args[0] {
            Value::String(s) => s,
            _ => return Err("create_dir expects a string argument".to_string()),
        };

        std::fs::create_dir(dirname).map_err(|e| e.to_string())?;
        Ok(Value::Unit)
    }

    pub fn list_dir(args: Vec<Value>) -> Result<Value, String> {
        if args.len() != 1 {
            return Err("list_dir expects exactly one argument".to_string());
        }

        let dirname = match &args[0] {
            Value::String(s) => s,
            _ => return Err("list_dir expects a string argument".to_string()),
        };

        let entries = std::fs::read_dir(dirname).map_err(|e| e.to_string())?;
        let mut result = Vec::new();

        for entry in entries {
            let entry = entry.map_err(|e| e.to_string())?;
            result.push(Value::String(
                entry.file_name().to_string_lossy().to_string(),
            ));
        }

        Ok(Value::Vector(result))
    }

    pub fn remove_file(args: Vec<Value>) -> Result<Value, String> {
        if args.len() != 1 {
            return Err("remove_file expects exactly one argument".to_string());
        }

        let filename = match &args[0] {
            Value::String(s) => s,
            _ => return Err("remove_file expects a string argument".to_string()),
        };

        std::fs::remove_file(filename).map_err(|e| e.to_string())?;
        Ok(Value::Unit)
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

    pub fn input() -> Result<Value, String> {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .map_err(|e| e.to_string())?;
        Ok(Value::String(input.trim().to_string()))
    }

    pub fn raw_input() -> Result<Value, String> {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .map_err(|e| e.to_string())?;
        Ok(Value::String(input))
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

    // String functions
    pub fn split(args: Vec<Value>) -> Result<Value, String> {
        if args.len() != 2 {
            return Err("split expects exactly two arguments".to_string());
        }

        let string = match &args[0] {
            Value::String(s) => s,
            _ => return Err("split expects a string as the first argument".to_string()),
        };

        let delimiter = match &args[1] {
            Value::String(s) => s,
            _ => return Err("split expects a string as the second argument".to_string()),
        };

        let parts: Vec<Value> = string
            .split(delimiter)
            .map(|s| Value::String(s.to_string()))
            .collect();

        Ok(Value::Vector(parts))
    }

    pub fn trim(args: Vec<Value>) -> Result<Value, String> {
        if args.len() != 1 {
            return Err("trim expects exactly one argument".to_string());
        }

        let string = match &args[0] {
            Value::String(s) => s,
            _ => return Err("trim expects a string as the first argument".to_string()),
        };

        Ok(Value::String(string.trim().to_string()))
    }

    pub fn contains(args: Vec<Value>) -> Result<Value, String> {
        if args.len() != 2 {
            return Err("contains expects exactly two arguments".to_string());
        }

        let string = match &args[0] {
            Value::String(s) => s,
            _ => return Err("contains expects a string as the first argument".to_string()),
        };

        let substring = match &args[1] {
            Value::String(s) => s,
            _ => return Err("contains expects a string as the second argument".to_string()),
        };

        Ok(Value::Boolean(string.contains(substring)))
    }

    pub fn replace(args: Vec<Value>) -> Result<Value, String> {
        if args.len() != 3 {
            return Err("replace expects exactly three arguments".to_string());
        }

        let string = match &args[0] {
            Value::String(s) => s,
            _ => return Err("replace expects a string as the first argument".to_string()),
        };

        let old = match &args[1] {
            Value::String(s) => s,
            _ => return Err("replace expects a string as the second argument".to_string()),
        };

        let new = match &args[2] {
            Value::String(s) => s,
            _ => return Err("replace expects a string as the third argument".to_string()),
        };

        Ok(Value::String(string.replace(old, new)))
    }

    // Math functions
    pub fn abs(args: Vec<Value>) -> Result<Value, String> {
        if args.len() != 1 {
            return Err("abs expects exactly one argument".to_string());
        }

        let result = match &args[0] {
            Value::Integer(i) => i.abs().into(),
            Value::Float(f) => f.abs(),
            _ => return Err("Cannot take absolute value of non-numeric value".to_string()),
        };

        Ok(Value::Float(result))
    }

    pub fn max(args: Vec<Value>) -> Result<Value, String> {
        if args.len() != 2 {
            return Err("max expects exactly two arguments".to_string());
        }

        let a = match &args[0] {
            Value::Integer(i) => *i as f64,
            Value::Float(f) => *f,
            _ => return Err("max expects numeric arguments".to_string()),
        };

        let b = match &args[1] {
            Value::Integer(i) => *i as f64,
            Value::Float(f) => *f,
            _ => return Err("max expects numeric arguments".to_string()),
        };

        Ok(Value::Float(a.max(b)))
    }

    pub fn min(args: Vec<Value>) -> Result<Value, String> {
        if args.len() != 2 {
            return Err("min expects exactly two arguments".to_string());
        }

        let a = match &args[0] {
            Value::Integer(i) => *i as f64,
            Value::Float(f) => *f,
            _ => return Err("min expects numeric arguments".to_string()),
        };

        let b = match &args[1] {
            Value::Integer(i) => *i as f64,
            Value::Float(f) => *f,
            _ => return Err("min expects numeric arguments".to_string()),
        };

        Ok(Value::Float(a.min(b)))
    }

    pub fn sqrt(args: Vec<Value>) -> Result<Value, String> {
        if args.len() != 1 {
            return Err("sqrt expects exactly one argument".to_string());
        }

        let result = match &args[0] {
            Value::Integer(i) => (*i as f64).sqrt(),
            Value::Float(f) => f.sqrt(),
            _ => return Err("Cannot take square root of non-numeric value".to_string()),
        };

        Ok(Value::Float(result))
    }

    pub fn pow(args: Vec<Value>) -> Result<Value, String> {
        if args.len() != 2 {
            return Err("pow expects exactly two arguments".to_string());
        }

        let base = match &args[0] {
            Value::Integer(i) => *i as f64,
            Value::Float(f) => *f,
            _ => return Err("pow expects numeric arguments".to_string()),
        };

        let exponent = match &args[1] {
            Value::Integer(i) => *i as f64,
            Value::Float(f) => *f,
            _ => return Err("pow expects numeric arguments".to_string()),
        };

        Ok(Value::Float(base.powf(exponent)))
    }

    // Random functions
    pub fn random() -> Value {
        Value::Float(rand::random::<f64>())
    }

    pub fn random_range(args: Vec<Value>) -> Result<Value, String> {
        if args.len() != 2 {
            return Err("random_range expects exactly two arguments".to_string());
        }

        let start = match &args[0] {
            Value::Integer(i) => *i as f64,
            Value::Float(f) => *f,
            _ => return Err("random_range expects numeric arguments".to_string()),
        };

        let end = match &args[1] {
            Value::Integer(i) => *i as f64,
            Value::Float(f) => *f,
            _ => return Err("random_range expects numeric arguments".to_string()),
        };

        Ok(Value::Float(rand::thread_rng().gen_range(start..end)))
    }

    pub fn random_choice(args: Vec<Value>) -> Result<Value, String> {
        if args.len() != 1 {
            return Err("random_choice expects exactly one argument".to_string());
        }

        let array = match &args[0] {
            Value::Vector(a) => a,
            _ => return Err("random_choice expects an array argument".to_string()),
        };

        let index = rand::thread_rng().gen_range(0..array.len());
        Ok(array[index].clone())
    }

    // Collections functions
    pub fn vec_new(_args: Vec<Value>) -> Result<Value, String> {
        Ok(Value::Vector(Vec::new()))
    }

    pub fn vec_set(args: Vec<Value>) -> Result<Value, String> {
        if args.len() != 3 {
            return Err("set expects three arguments: vector, index, and value".to_string());
        }

        match &args[0] {
            Value::SharedRef(rc) => {
                let mut value = rc.borrow_mut();
                if let Value::Vector(vec) = &mut *value {
                    let index = match &args[1] {
                        Value::Integer(i) => *i as usize,
                        _ => return Err("Index must be an integer".to_string()),
                    };
                    vec[index] = args[2].clone();
                    Ok(Value::Unit)
                } else {
                    Err("First argument must be a vector".to_string())
                }
            }
            Value::Vector(vec) => {
                let mut new_vec = vec.clone();
                let index = match &args[1] {
                    Value::Integer(i) => *i as usize,
                    _ => return Err("Index must be an integer".to_string()),
                };
                new_vec[index] = args[2].clone();
                Ok(Value::Vector(new_vec))
            }
            _ => Err("First argument must be a vector".to_string()),
        }
    }

    pub fn vec_push(args: Vec<Value>) -> Result<Value, String> {
        if args.len() != 2 {
            return Err("push expects two arguments: vector and value".to_string());
        }

        match &args[0] {
            Value::SharedRef(rc) => {
                let mut value = rc.borrow_mut();
                if let Value::Vector(vec) = &mut *value {
                    vec.push(args[1].clone());
                    Ok(Value::Unit)
                } else {
                    Err("First argument must be a vector".to_string())
                }
            }
            Value::Vector(vec) => {
                let mut new_vec = vec.clone();
                new_vec.push(args[1].clone());
                Ok(Value::Vector(new_vec))
            }
            _ => Err("First argument must be a vector".to_string()),
        }
    }

    pub fn vec_pop(args: Vec<Value>) -> Result<Value, String> {
        if args.len() != 1 {
            return Err("pop expects one argument: vector".to_string());
        }

        match &args[0] {
            Value::SharedRef(rc) => {
                let mut value = rc.borrow_mut();
                if let Value::Vector(vec) = &mut *value {
                    vec.pop().ok_or("Vector is empty".to_string())
                } else {
                    Err("Argument must be a vector".to_string())
                }
            }
            Value::Vector(vec) => {
                let mut new_vec = vec.clone();
                new_vec.pop().ok_or("Vector is empty".to_string())
            }
            _ => Err("Argument must be a vector".to_string()),
        }
    }

    pub fn hashmap_new(_args: Vec<Value>) -> Result<Value, String> {
        Ok(Value::HashMap(HashMap::new()))
    }

    pub fn hashmap_insert(args: Vec<Value>) -> Result<Value, String> {
        if args.len() != 3 {
            return Err("insert expects three arguments: hashmap, key, and value".to_string());
        }

        match &args[0] {
            Value::SharedRef(rc) => {
                let mut value = rc.borrow_mut();
                if let Value::HashMap(map) = &mut *value {
                    let key = match &args[1] {
                        Value::String(s) => s.clone(),
                        _ => return Err("Key must be a string".to_string()),
                    };
                    map.insert(key, args[2].clone());
                    Ok(Value::Unit)
                } else {
                    Err("First argument must be a hashmap".to_string())
                }
            }
            Value::HashMap(map) => {
                let mut new_map = map.clone();
                let key = match &args[1] {
                    Value::String(s) => s.clone(),
                    _ => return Err("Key must be a string".to_string()),
                };
                new_map.insert(key, args[2].clone());
                Ok(Value::HashMap(new_map))
            }
            _ => Err("First argument must be a hashmap".to_string()),
        }
    }

    pub fn hashmap_get(args: Vec<Value>) -> Result<Value, String> {
        if args.len() != 2 {
            return Err("get expects two arguments: hashmap and key".to_string());
        }

        match &args[0] {
            Value::SharedRef(rc) => {
                let value = rc.borrow();
                match &*value {
                    Value::HashMap(map) => {
                        let key = match &args[1] {
                            Value::String(s) => s,
                            _ => return Err("Key must be a string".to_string()),
                        };
                        map.get(key)
                            .cloned()
                            .ok_or("Key not found in hashmap".to_string())
                    }
                    _ => Err("First argument must be a hashmap".to_string()),
                }
            }
            Value::HashMap(map) => {
                let key = match &args[1] {
                    Value::String(s) => s,
                    _ => return Err("Key must be a string".to_string()),
                };
                map.get(key)
                    .cloned()
                    .ok_or("Key not found in hashmap".to_string())
            }
            _ => Err("First argument must be a hashmap".to_string()),
        }
    }
}
