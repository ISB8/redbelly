use std::collections::HashMap;

use crate::lexer::{Token, TokenType};

use super::ParseError;

#[derive(Clone)]
pub struct Environment {
    pub(crate) values: HashMap<String, TokenType>,
    pub(crate) enclosing: Box<Option<Environment>>,
}

impl Environment {
    pub fn new(enclosing: Option<Environment>) -> Self {
        Self {
            values: HashMap::new(),
            enclosing: Box::new(enclosing),
        }
    }
    pub fn define(&mut self, name: String, value: TokenType) {
        self.values.insert(name, value);
    }

    pub fn assign(&mut self, name: Token, value: TokenType) -> Option<ParseError> {
        use std::collections::hash_map::Entry;
        match self.values.entry(name.lexeme.clone()) {
            Entry::Occupied(mut occupied_entry) => {
                *occupied_entry.get_mut() = value;
                return None;
            }
            Entry::Vacant(_) => {
                if let Some(env) = &mut *self.enclosing {
                    return env.assign(name, value);
                }
            }
        }
        Some(ParseError::new(
            &format!("Undefined Variable {}", name.lexeme),
            &name,
        ))
    }

    pub fn get(&self, name: Token) -> Result<&TokenType, ParseError> {
        match self.values.get(&name.lexeme) {
            Some(val) => Ok(val),
            None => {
                if let Some(env) = &*self.enclosing {
                    return env.get(name);
                }
                Err(ParseError::new("Undefined variable", &name))
            }
        }
    }

    pub fn release_enclosing(self) -> Option<Environment> {
        *self.enclosing
    }

    pub fn enclosed(self) -> Environment {
        Environment::new(Some(self))
    }
}

#[cfg(test)]
mod tests {
    use crate::lexer::{Token, TokenType};

    use super::Environment;

    #[test]
    fn test_enclosing_environments() {
        let mut enclosing = Environment::new(None);
        enclosing.define("test".to_owned(), TokenType::Number(3.));
        let environment = enclosing.enclosed();

        let result = environment.get(Token::new(TokenType::Identifier, "test".to_owned(), 1));

        assert!(result.unwrap() == &TokenType::Number(3.));
    }

    #[test]
    fn test_shadowing() {
        let mut enclosing = Environment::new(None);
        enclosing.define("test".to_owned(), TokenType::Number(3.));
        let mut environment = enclosing.enclosed();

        environment.define("test".to_owned(), TokenType::Number(4.));

        let result = environment.get(Token::new(TokenType::Identifier, "test".to_owned(), 1));

        assert!(result.unwrap() == &TokenType::Number(4.));
    }
    #[test]
    fn test_nested_reassignment() {
        let mut enclosing = Environment::new(None);
        enclosing.define("test".to_owned(), TokenType::Number(3.));
        let mut environment = enclosing.enclosed();

        environment.assign(
            Token::new(TokenType::Identifier, "test".to_owned(), 1),
            TokenType::Number(4.),
        );

        let new_env = environment.release_enclosing().unwrap();
        let result = new_env.get(Token::new(TokenType::Identifier, "test".to_owned(), 1));

        assert!(result.unwrap() == &TokenType::Number(4.));
    }
}
