#![allow(dead_code)]
use crate::{lexer::Token, stdlib::StdLib};

#[derive(Debug, PartialEq, Clone)]
pub enum Type {
    I8,
    I16,
    I32,
    I64,
    U8,
    U16,
    U32,
    U64,
    F32,
    F64,
    Bool,
    String,
    Dynamic,
    // Complex types
    Unique(Box<Type>),             // ~T
    Shared(Box<Type>),             // @T
    Vec(Box<Type>),                // Vec<T>
    HashMap(Box<Type>, Box<Type>), // HashMap<K,V>
}

#[derive(Debug, PartialEq, Clone)]
pub enum Attribute {
    Weak,
    Sync,
    Own,
    Actor,
}

#[derive(Debug, PartialEq, Clone)]
pub enum AstNode {
    // Literals
    Integer(i32),
    Float(f64),
    String(String),
    Boolean(bool),

    // Variables
    Identifier(String),
    VariableDecl {
        name: String,
        type_annotation: Option<Type>,
        initializer: Option<Box<AstNode>>,
        ownership: Option<Ownership>,
    },

    // Functions
    FunctionDecl {
        name: String,
        params: Vec<(String, Type)>,
        return_type: Option<Type>,
        body: Box<AstNode>,
        attributes: Vec<Attribute>,
        is_async: bool,
    },
    FunctionCall {
        name: String,
        args: Vec<AstNode>,
    },

    // Types and Ownership
    TypeAnnotation(Type),
    Ownership(Ownership),

    // Control Flow
    Block(Vec<AstNode>),
    IfExpr {
        condition: Box<AstNode>,
        then_branch: Box<AstNode>,
        else_branch: Option<Box<AstNode>>,
    },
    WhileLoop {
        condition: Box<AstNode>,
        body: Box<AstNode>,
    },

    // Operations
    BinaryOp {
        left: Box<AstNode>,
        operator: Operator,
        right: Box<AstNode>,
    },
    UnaryOp {
        operator: UnaryOperator,
        operand: Box<AstNode>,
    },
    CompoundAssign {
        operator: Operator,
        target: Box<AstNode>,
        value: Box<AstNode>,
    },

    // Concurrency
    ChannelCreate,
    Send {
        channel: Box<AstNode>,
        value: Box<AstNode>,
    },
    Receive {
        channel: Box<AstNode>,
    },
    Await {
        expression: Box<AstNode>,
    },
}

#[derive(Debug, PartialEq, Clone)]
pub enum Ownership {
    Unique, // ~
    Shared, // @
    Weak,   // #weak
}

#[derive(Debug, PartialEq, Clone)]
pub enum Operator {
    Assign,
    Add,
    SelfAdd,
    Inc,
    Sub,
    SelfSub,
    Dec,
    Mul,
    Div,
    Eq,
    NotEq,
    Lt,
    Gt,
    LtEq,
    GtEq,
    And,
    Or,
    Mod,
}

#[derive(Debug, PartialEq, Clone)]
pub enum UnaryOperator {
    Not,
    Neg,
    Inc,
    Dec,
}

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, current: 0 }
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.current)
    }

    fn advance(&mut self) -> Option<Token> {
        let token = self.peek().cloned();
        self.current += 1;
        token
    }

    fn expect(&mut self, expected: Token) -> Result<(), String> {
        if self.peek() == Some(&expected) {
            self.advance();
            Ok(())
        } else {
            Err(format!("Expected {:?}, got {:?}", expected, self.peek()))
        }
    }

    pub fn parse(&mut self) -> Result<Vec<AstNode>, String> {
        let mut statements = Vec::new();
        while self.peek().is_some() {
            statements.push(self.parse_statement()?);
        }
        Ok(statements)
    }

    fn parse_statement(&mut self) -> Result<AstNode, String> {
        match self.peek() {
            Some(Token::Let) => self.parse_variable_declaration(),
            Some(Token::Func) => self.parse_function_declaration(),
            Some(Token::If) => self.parse_if_statement(),
            Some(Token::While) => self.parse_while_statement(),
            Some(Token::Identifier(_)) => {
                let expr = self.parse_expression()?;
                if self.peek() == Some(&Token::Semicolon) {
                    self.advance();
                }
                Ok(expr)
            }
            _ => self.parse_expression(),
        }
    }

    fn parse_variable_declaration(&mut self) -> Result<AstNode, String> {
        self.advance(); // consume 'let'
        let name = match self.advance() {
            Some(Token::Identifier(name)) => name,
            _ => return Err("Expected identifier after 'let'".to_string()),
        };

        let type_annotation = if self.peek() == Some(&Token::Colon) {
            self.advance(); // consume ':'
            Some(self.parse_type()?)
        } else {
            None
        };

        let initializer = if self.peek() == Some(&Token::Assign) {
            self.advance(); // consume '='
            Some(Box::new(self.parse_expression()?))
        } else {
            None
        };

        // self.expect(Token::Semicolon)?;

        // Ok(AstNode::VariableDecl {
        //     name,
        //     type_annotation,
        //     initializer,
        //     ownership: None, // Handle ownership later
        // })

        println!("Current token before semicolon check: {:?}", self.peek());

        match self.peek() {
            Some(Token::Semicolon) => {
                self.advance(); // Consume semicolon
                Ok(AstNode::VariableDecl {
                    name,
                    type_annotation,
                    initializer,
                    ownership: None,
                })
            }
            other => Err(format!(
                "Expected semicolon after variable declaration, got {:?}",
                other
            )),
        }
    }

    fn parse_type(&mut self) -> Result<Type, String> {
        match self.advance() {
            Some(Token::Tilde) => Ok(Type::Unique(Box::new(self.parse_type()?))),
            Some(Token::At) => Ok(Type::Shared(Box::new(self.parse_type()?))),
            Some(token) => match token {
                Token::TypeI8 => Ok(Type::I8),
                Token::TypeI16 => Ok(Type::I16),
                Token::TypeI32 => Ok(Type::I32),
                Token::TypeI64 => Ok(Type::I64),
                Token::TypeU8 => Ok(Type::U8),
                Token::TypeU16 => Ok(Type::U16),
                Token::TypeU32 => Ok(Type::U32),
                Token::TypeU64 => Ok(Type::U64),
                Token::TypeF32 => Ok(Type::F32),
                Token::TypeF64 => Ok(Type::F64),
                Token::TypeBool => Ok(Type::Bool),
                Token::TypeString => Ok(Type::String),
                Token::TypeDyn => Ok(Type::Dynamic),
                _ => Err(format!("Unexpected type token: {:?}", token)),
            },
            None => Err("Unexpected end of input while parsing type".to_string()),
        }
    }

    fn parse_function_declaration(&mut self) -> Result<AstNode, String> {
        self.advance(); // consume 'func'

        let mut attributes = Vec::new();
        let mut is_async = false;

        while let Some(token) = self.peek() {
            match token {
                Token::WeakAttr => {
                    self.advance();
                    attributes.push(Attribute::Weak);
                }
                Token::SyncAttr => {
                    self.advance();
                    attributes.push(Attribute::Sync);
                }
                Token::OwnAttr => {
                    self.advance();
                    attributes.push(Attribute::Own);
                }
                Token::ActorAttr => {
                    self.advance();
                    attributes.push(Attribute::Actor);
                }
                Token::Async => {
                    self.advance();
                    is_async = true;
                }
                _ => break,
            }
        }

        let name = match self.advance() {
            Some(Token::Identifier(name)) => name,
            _ => return Err("Expected function name".to_string()),
        };

        self.expect(Token::LParen)?;
        let mut params = Vec::new();
        while self.peek() != Some(&Token::RParen) {
            if !params.is_empty() {
                self.expect(Token::Comma)?;
            }

            let param_name = match self.advance() {
                Some(Token::Identifier(name)) => name,
                _ => return Err("Expected parameter name".to_string()),
            };

            self.expect(Token::Colon)?;
            let param_type = self.parse_type()?;
            params.push((param_name, param_type));
        }
        self.expect(Token::RParen)?;

        let return_type = if self.peek() == Some(&Token::Arrow) {
            self.advance(); // consume '->'
            Some(self.parse_type()?)
        } else {
            None
        };

        let body = self.parse_block()?;

        Ok(AstNode::FunctionDecl {
            name,
            params,
            return_type,
            body: Box::new(body),
            attributes,
            is_async,
        })
    }

    fn parse_if_statement(&mut self) -> Result<AstNode, String> {
        self.advance(); // consume 'if'

        let condition = self.parse_expression()?;

        let then_branch = self.parse_block()?;

        let else_branch = if self.peek() == Some(&Token::Else) {
            self.advance(); // consume 'else'
            if self.peek() == Some(&Token::If) {
                Some(Box::new(self.parse_if_statement()?))
            } else {
                Some(Box::new(self.parse_block()?))
            }
        } else {
            None
        };

        Ok(AstNode::IfExpr {
            condition: Box::new(condition),
            then_branch: Box::new(then_branch),
            else_branch,
        })
    }

    fn parse_while_statement(&mut self) -> Result<AstNode, String> {
        self.advance(); // consume 'while'

        // Parse condition
        let condition = self.parse_expression()?;

        // Parse body
        let body = self.parse_block()?;

        Ok(AstNode::WhileLoop {
            condition: Box::new(condition),
            body: Box::new(body),
        })
    }

    fn parse_expression(&mut self) -> Result<AstNode, String> {
        let expr = self.parse_logical_or()?;

        // Handle assignment-like operators
        match self.peek() {
            Some(Token::Assign) => {
                self.advance();
                let value = self.parse_expression()?;
                if self.peek() == Some(&Token::Semicolon) {
                    self.advance();
                }
                Ok(AstNode::CompoundAssign {
                    operator: Operator::Assign,
                    target: Box::new(expr),
                    value: Box::new(value),
                })
            }
            Some(Token::PlusEq) => {
                self.advance();
                let value = self.parse_expression()?;
                Ok(AstNode::CompoundAssign {
                    operator: Operator::SelfAdd,
                    target: Box::new(expr),
                    value: Box::new(value),
                })
            }
            Some(Token::PlusPlus) => {
                self.advance();
                Ok(AstNode::UnaryOp {
                    operator: UnaryOperator::Inc,
                    operand: Box::new(expr),
                })
            }
            Some(Token::MinusEq) => {
                self.advance();
                let value = self.parse_expression()?;
                Ok(AstNode::CompoundAssign {
                    operator: Operator::SelfSub,
                    target: Box::new(expr),
                    value: Box::new(value),
                })
            }
            Some(Token::MinusMinus) => {
                self.advance();
                Ok(AstNode::UnaryOp {
                    operator: UnaryOperator::Dec,
                    operand: Box::new(expr),
                })
            }
            _ => Ok(expr),
        }
    }

    fn parse_logical_or(&mut self) -> Result<AstNode, String> {
        let mut left = self.parse_logical_and()?;

        while self.peek() == Some(&Token::Or) {
            self.advance();
            let right = self.parse_logical_and()?;
            left = AstNode::BinaryOp {
                left: Box::new(left),
                operator: Operator::Or,
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    fn parse_logical_and(&mut self) -> Result<AstNode, String> {
        let mut left = self.parse_equality()?;

        while self.peek() == Some(&Token::And) {
            self.advance();
            let right = self.parse_equality()?;
            left = AstNode::BinaryOp {
                left: Box::new(left),
                operator: Operator::And,
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    fn parse_equality(&mut self) -> Result<AstNode, String> {
        let mut left = self.parse_comparison()?;

        while let Some(token) = self.peek() {
            let operator = match token {
                Token::Eq => Operator::Eq,
                Token::NotEq => Operator::NotEq,
                _ => break,
            };
            self.advance();
            let right = self.parse_comparison()?;
            left = AstNode::BinaryOp {
                left: Box::new(left),
                operator,
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    fn parse_comparison(&mut self) -> Result<AstNode, String> {
        let mut left = self.parse_term()?;

        while let Some(token) = self.peek() {
            let operator = match token {
                Token::Lt => Operator::Lt,
                Token::Gt => Operator::Gt,
                Token::LtEq => Operator::LtEq,
                Token::GtEq => Operator::GtEq,
                _ => break,
            };
            self.advance();
            let right = self.parse_term()?;
            left = AstNode::BinaryOp {
                left: Box::new(left),
                operator,
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    fn parse_term(&mut self) -> Result<AstNode, String> {
        let mut left = self.parse_factor()?;

        while let Some(token) = self.peek() {
            let operator = match token {
                Token::Plus => Operator::Add,
                Token::Minus => Operator::Sub,
                _ => break,
            };
            self.advance();
            let right = self.parse_factor()?;
            left = AstNode::BinaryOp {
                left: Box::new(left),
                operator,
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    fn parse_factor(&mut self) -> Result<AstNode, String> {
        let mut left = self.parse_unary()?;

        while let Some(token) = self.peek() {
            let operator = match token {
                Token::Multiply => Operator::Mul,
                Token::Divide => Operator::Div,
                Token::Modulus => Operator::Mod,
                _ => break,
            };
            self.advance();
            let right = self.parse_unary()?;
            left = AstNode::BinaryOp {
                left: Box::new(left),
                operator,
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    fn parse_unary(&mut self) -> Result<AstNode, String> {
        match self.peek() {
            Some(Token::Minus) => {
                self.advance();
                let operand = self.parse_unary()?;
                Ok(AstNode::UnaryOp {
                    operator: UnaryOperator::Neg,
                    operand: Box::new(operand),
                })
            }
            Some(Token::Not) => {
                self.advance();
                let operand = self.parse_unary()?;
                Ok(AstNode::UnaryOp {
                    operator: UnaryOperator::Not,
                    operand: Box::new(operand),
                })
            }
            _ => self.parse_primary(),
        }
    }

    fn parse_primary(&mut self) -> Result<AstNode, String> {
        let current_token = match self.peek().cloned() {
            Some(token) => token,
            None => return Err("Unexpected end of input".to_string()),
        };

        match current_token {
            Token::Integer(_) => {
                if let Some(Token::Integer(value)) = self.advance() {
                    Ok(AstNode::Integer(value))
                } else {
                    Err("Expected integer".to_string())
                }
            }
            Token::Float(_) => {
                if let Some(Token::Float(value)) = self.advance() {
                    Ok(AstNode::Float(value))
                } else {
                    Err("Expected float".to_string())
                }
            }
            Token::String(_) => {
                if let Some(Token::String(value)) = self.advance() {
                    Ok(AstNode::String(value))
                } else {
                    Err("Expected string".to_string())
                }
            }
            Token::Bool(_) => {
                if let Some(Token::Bool(value)) = self.advance() {
                    Ok(AstNode::Boolean(value))
                } else {
                    Err("Expected boolean".to_string())
                }
            }

            Token::Identifier(name) => {
                self.advance(); // consume identifier
                if self.peek() == Some(&Token::LParen) || StdLib::is_builtin(&name) {
                    // Handle function call for both user-defined and built-in functions
                    self.advance(); // consume '('
                    let mut arguments = Vec::new();
                    while self.peek() != Some(&Token::RParen) {
                        if !arguments.is_empty() {
                            self.expect(Token::Comma)?;
                        }
                        arguments.push(self.parse_expression()?);
                    }
                    self.expect(Token::RParen)?;

                    // // Add semicolon handling
                    // if self.peek() == Some(&Token::Semicolon) {
                    //     self.advance(); // Consume semicolon
                    // }

                    Ok(AstNode::FunctionCall {
                        name,
                        args: arguments,
                    })
                } else {
                    Ok(AstNode::Identifier(name))
                }
            }
            Token::LParen => {
                self.advance(); // consume '('
                let expr = self.parse_expression()?;
                self.expect(Token::RParen)?;
                Ok(expr)
            }
            token => {
                self.advance();
                Err(format!(
                    "Unexpected token in primary expression: {:?}",
                    token
                ))
            }
        }
    }

    fn parse_block(&mut self) -> Result<AstNode, String> {
        self.expect(Token::LBrace)?;
        let mut statements = Vec::new();

        while self.peek() != Some(&Token::RBrace) {
            statements.push(self.parse_statement()?);
        }

        self.expect(Token::RBrace)?;
        Ok(AstNode::Block(statements))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Lexer;

    #[test]
    fn test_parse_function() {
        let input = "func add(x: i32, y: i32) -> i32 { x + y }";
        let mut lexer = Lexer::new(input.to_string());
        let mut tokens = Vec::new();

        // Collect tokens until we hit EOF
        loop {
            let token = lexer.next_token();
            tokens.push(token.clone());
            if matches!(token, Token::Eof) {
                break;
            }
        }

        let mut parser = Parser::new(tokens);
        let result = parser.parse_function_declaration();
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_complex_function() {
        let input = "func process(data: ~i32) -> bool {
            if data > 0 {
                true
            } else {
                false
            }
        }";

        let mut lexer = Lexer::new(input.to_string());
        let mut tokens = Vec::new();

        loop {
            let token = lexer.next_token();
            tokens.push(token.clone());
            if matches!(token, Token::Eof) {
                break;
            }
        }

        let mut parser = Parser::new(tokens);
        let result = parser.parse_function_declaration();
        assert!(result.is_ok());

        // if let Ok(AstNode::FunctionDecl {
        //     attributes,
        //     is_async,
        //     ..
        // }) = result
        // {
        //     assert!(attributes.contains(&Attribute::Sync));
        //     assert!(is_async);
        // } else {
        //     panic!("Expected function declaration");
        // }
    }
}
