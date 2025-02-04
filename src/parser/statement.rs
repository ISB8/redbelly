use std::rc::Rc;

use crate::lexer::Token;

use super::{environment::Environment, expression::Expression, ParseError};

pub trait Statement {
    fn execute(&self, environment: &mut Environment) -> Result<(), ParseError>;
}

pub struct PrintStatement {
    expr: Rc<dyn Expression>,
}

impl PrintStatement {
    pub fn new(expr: Rc<dyn Expression>) -> Self {
        Self { expr }
    }
}

impl Statement for PrintStatement {
    fn execute(&self, environment: &mut Environment) -> Result<(), ParseError> {
        let _ = environment;
        let val = self.expr.evaluate(environment)?;
        println!("{}", val);
        Ok(())
    }
}

pub struct ExpressionStatement {
    expr: Rc<dyn Expression>,
}

impl ExpressionStatement {
    pub fn new(expr: Rc<dyn Expression>) -> Self {
        Self { expr }
    }
}

impl Statement for ExpressionStatement {
    fn execute(&self, environment: &mut Environment) -> Result<(), ParseError> {
        let _ = environment;
        self.expr.evaluate(environment)?;
        Ok(())
    }
}

pub struct VariableStatement {
    name: Token,
    initializer: Option<Rc<dyn Expression>>,
}

impl VariableStatement {
    pub fn new(name: Token, initializer: Option<Rc<dyn Expression>>) -> Self {
        Self { name, initializer }
    }
}

impl Statement for VariableStatement {
    fn execute(&self, environment: &mut Environment) -> Result<(), ParseError> {
        match &self.initializer {
            Some(expr) => {
                environment.define(self.name.lexeme.clone(), expr.evaluate(environment)?);
                Ok(())
            }
            None => {
                environment.define(self.name.lexeme.clone(), crate::lexer::TokenType::Nil);
                Ok(())
            }
        }
    }
}
