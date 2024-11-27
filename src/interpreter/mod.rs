// src/interpreter/mod.rs

use crate::parser::{AstNode, Operator, Type, UnaryOperator};
use crate::stdlib::StdLib;
use std::collections::HashMap;

// Values that can exist during runtime
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Integer(i32),
    Float(f64),
    String(String),
    Boolean(bool),
    Unit,             // For functions that don't return a value
    Reference(usize), // For heap allocated values
    Function {
        params: Vec<(String, Type)>,
        body: Box<AstNode>,
        closure: Environment,
    },
}

// Environment to store variables and their values
#[derive(Debug, Clone, PartialEq)]
pub struct Environment {
    values: HashMap<String, Value>,
    parent: Option<Box<Environment>>,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            values: HashMap::new(),
            parent: None,
        }
    }

    pub fn with_parent(parent: Environment) -> Self {
        Environment {
            values: HashMap::new(),
            parent: Some(Box::new(parent)),
        }
    }

    pub fn define(&mut self, name: String, value: Value) {
        self.values.insert(name, value);
    }

    pub fn get(&self, name: &str) -> Option<Value> {
        match self.values.get(name) {
            Some(value) => Some(value.clone()),
            None => self.parent.as_ref().and_then(|parent| parent.get(name)),
        }
    }
}

// Memory management for heap allocated values
pub struct Heap {
    objects: Vec<Value>,
}

impl Heap {
    pub fn new() -> Self {
        Heap {
            objects: Vec::new(),
        }
    }

    pub fn allocate(&mut self, value: Value) -> usize {
        let address = self.objects.len();
        self.objects.push(value);
        address
    }

    pub fn get(&self, address: usize) -> Option<&Value> {
        self.objects.get(address)
    }
}

pub struct Interpreter {
    environment: Environment,
    heap: Heap,
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            environment: Environment::new(),
            heap: Heap::new(),
        }
    }

    pub fn interpret(&mut self, node: AstNode) -> Result<Value, String> {
        match node {
            AstNode::Integer(n) => Ok(Value::Integer(n)),
            AstNode::Float(f) => Ok(Value::Float(f)),
            AstNode::String(s) => Ok(Value::String(s)),
            AstNode::Boolean(b) => Ok(Value::Boolean(b)),

            AstNode::VariableDecl {
                name, initializer, ..
            } => {
                let value = match initializer {
                    Some(expr) => self.interpret(*expr)?,
                    None => Value::Unit,
                };
                self.environment.define(name, value.clone());
                Ok(value)
            }

            AstNode::Identifier(name) => self
                .environment
                .get(&name)
                .ok_or(format!("Undefined variable: {}", name)),

            AstNode::BinaryOp {
                left,
                operator,
                right,
            } => {
                let left_val = self.interpret(*left)?;
                let right_val = self.interpret(*right)?;
                self.evaluate_binary_op(operator, left_val, right_val)
            }

            AstNode::UnaryOp { operator, operand } => {
                let val = self.interpret(*operand)?;
                self.evaluate_unary_op(operator, val)
            }

            AstNode::Block(statements) => {
                let mut result = Value::Unit;
                for stmt in statements {
                    result = self.interpret(stmt)?;
                }
                Ok(result)
            }

            AstNode::IfExpr {
                condition,
                then_branch,
                else_branch,
            } => {
                let cond_val = self.interpret(*condition)?;
                match cond_val {
                    Value::Boolean(true) => self.interpret(*then_branch),
                    Value::Boolean(false) => {
                        if let Some(else_branch) = else_branch {
                            self.interpret(*else_branch)
                        } else {
                            Ok(Value::Unit)
                        }
                    }
                    _ => Err("Condition must be a boolean".to_string()),
                }
            }

            AstNode::WhileLoop { condition, body } => {
                loop {
                    let cond_val = self.interpret(*condition.clone())?;
                    match cond_val {
                        Value::Boolean(true) => {
                            self.interpret(*body.clone())?;
                        }
                        Value::Boolean(false) => break,
                        _ => return Err("Condition must be a boolean".to_string()),
                    }
                }
                Ok(Value::Unit)
            }

            AstNode::FunctionDecl {
                name, params, body, ..
            } => {
                let func_value = Value::Function {
                    params,
                    body: body.clone(),
                    closure: self.environment.clone(),
                };
                self.environment.define(name.clone(), func_value.clone());

                if name == "main" {
                    return self.call_user_function(vec![], body, vec![], self.environment.clone());
                }

                Ok(func_value)
            }

            AstNode::FunctionCall { name, args } => {
                // Evaluate all arguments first
                let evaluated_args = args
                    .into_iter()
                    .map(|arg| self.interpret(arg))
                    .collect::<Result<Vec<_>, _>>()?;

                // Handle the function call
                if let Some(func) = self.environment.get(&name) {
                    match func {
                        Value::Function {
                            params,
                            body,
                            closure,
                        } => {
                            // Handle user-defined functions
                            self.call_user_function(params, body, evaluated_args, closure)
                        }
                        _ => {
                            // Handle built-in functions
                            self.handle_builtin_function(&name, evaluated_args)
                        }
                    }
                } else {
                    // If not in environment, try as builtin
                    self.handle_builtin_function(&name, evaluated_args)
                }
            }

            // Handle unique ownership (~)
            AstNode::Ownership(ownership) => {
                // Implementation for ownership handling
                Ok(Value::Unit)
            }

            _ => Err(format!("Unimplemented node type: {:?}", node)),
        }
    }

    fn handle_builtin_function(&mut self, name: &str, args: Vec<Value>) -> Result<Value, String> {
        match name {
            "println" => StdLib::println(args),
            "print" => StdLib::print(args),
            "to_string" => StdLib::to_string(args),
            "to_int" => StdLib::to_int(args),
            "to_float" => StdLib::to_float(args),
            "to_bool" => StdLib::to_bool(args),
            _ => Err(format!("Unknown built-in function: {}", name)),
        }
    }

    fn call_user_function(
        &mut self,
        params: Vec<(String, Type)>,
        body: Box<AstNode>,
        args: Vec<Value>,
        closure: Environment,
    ) -> Result<Value, String> {
        // Verify argument count matches parameter count
        if args.len() != params.len() {
            return Err(format!(
                "Function expected {} arguments but got {}",
                params.len(),
                args.len()
            ));
        }

        // Create new environment with closure as parent
        let mut func_env = Environment::with_parent(closure);

        // Bind arguments to parameters
        for ((name, _type), value) in params.into_iter().zip(args) {
            func_env.define(name, value);
        }

        // Execute function body with new environment
        let previous_env = std::mem::replace(&mut self.environment, func_env);
        let result = self.interpret(*body);
        self.environment = previous_env;

        result
    }

    fn evaluate_binary_op(
        &mut self,
        operator: Operator,
        left: Value,
        right: Value,
    ) -> Result<Value, String> {
        match (operator, left, right) {
            (Operator::Add, Value::Integer(a), Value::Integer(b)) => Ok(Value::Integer(a + b)),
            (Operator::Sub, Value::Integer(a), Value::Integer(b)) => Ok(Value::Integer(a - b)),
            (Operator::Mul, Value::Integer(a), Value::Integer(b)) => Ok(Value::Integer(a * b)),
            (Operator::Div, Value::Integer(a), Value::Integer(b)) => {
                if b == 0 {
                    Err("Division by zero".to_string())
                } else {
                    Ok(Value::Integer(a / b))
                }
            }
            (Operator::Eq, Value::Integer(a), Value::Integer(b)) => Ok(Value::Boolean(a == b)),
            (Operator::NotEq, Value::Integer(a), Value::Integer(b)) => Ok(Value::Boolean(a != b)),
            // Add more cases for other types and operators
            _ => Err(format!("Invalid operator for types")),
        }
    }

    fn evaluate_unary_op(
        &mut self,
        operator: UnaryOperator,
        operand: Value,
    ) -> Result<Value, String> {
        match (operator, operand) {
            (UnaryOperator::Neg, Value::Integer(n)) => Ok(Value::Integer(-n)),
            (UnaryOperator::Not, Value::Boolean(b)) => Ok(Value::Boolean(!b)),
            _ => Err(format!("Invalid unary operator for type")),
        }
    }
}

// Add tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::AstNode;

    #[test]
    fn test_basic_arithmetic() {
        let mut interpreter = Interpreter::new();
        let ast = AstNode::BinaryOp {
            left: Box::new(AstNode::Integer(5)),
            operator: Operator::Add,
            right: Box::new(AstNode::Integer(3)),
        };

        assert_eq!(interpreter.interpret(ast).unwrap(), Value::Integer(8));
    }
}
