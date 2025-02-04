use std::collections::HashMap;

use crate::lexer::{Token, TokenType};

use super::ParseError;

pub struct Environment {
    values: HashMap<String, TokenType>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
        }
    }
    pub fn define(&mut self, name: String, value: TokenType) {
        self.values.insert(name, value);
    }

    pub fn get(&self, name: Token) -> Result<&TokenType, ParseError> {
        match self.values.get(&name.lexeme) {
            Some(val) => Ok(val),
            None => Err(ParseError::new("Undefined variable", &name)),
        }
    }
}
