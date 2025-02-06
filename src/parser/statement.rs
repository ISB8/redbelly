use std::rc::Rc;

use crate::lexer::{Token, TokenType};

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
                let val = expr.evaluate(environment)?;
                environment.define(self.name.lexeme.clone(), val);
                Ok(())
            }
            None => {
                environment.define(self.name.lexeme.clone(), crate::lexer::TokenType::Nil);
                Ok(())
            }
        }
    }
}

pub struct BlockStatement {
    pub(super) statements: Vec<Rc<dyn Statement>>,
}

impl BlockStatement {
    pub fn new(statements: Vec<Rc<dyn Statement>>) -> Self {
        Self { statements }
    }
}

impl Statement for BlockStatement {
    fn execute(&self, environment: &mut Environment) -> Result<(), ParseError> {
        let mut inner_env = environment.clone().enclosed();

        for stmt in &self.statements {
            stmt.execute(&mut inner_env)?
        }

        let new_env = inner_env
            .release_enclosing()
            .expect("Never fails as inner_env always has enclosed val");
        environment.values = new_env.values;
        environment.enclosing = new_env.enclosing;

        Ok(())
    }
}

pub struct IfStatement {
    condition: Rc<dyn Expression>,
    then_branch: Rc<dyn Statement>,
    else_branch: Option<Rc<dyn Statement>>,
}

impl IfStatement {
    pub fn new(
        condition: Rc<dyn Expression>,
        then_branch: Rc<dyn Statement>,
        else_branch: Option<Rc<dyn Statement>>,
    ) -> Self {
        Self {
            condition,
            then_branch,
            else_branch,
        }
    }
}

impl Statement for IfStatement {
    fn execute(&self, environment: &mut Environment) -> Result<(), ParseError> {
        let result = self.condition.evaluate(environment)?;

        match result {
            TokenType::True => self.then_branch.execute(environment),
            TokenType::False => {
                if let Some(else_branch) = &self.else_branch {
                    else_branch.execute(environment)?;
                }
                Ok(())
            }
            // FIXME hacky fix is hacky
            _ => panic!("Hacky Fix for handling invalid if condition"),
        }
    }
}

pub struct WhileStatement {
    condition: Rc<dyn Expression>,
    body: Rc<dyn Statement>,
}

impl WhileStatement {
    pub fn new(condition: Rc<dyn Expression>, body: Rc<dyn Statement>) -> Self {
        Self { condition, body }
    }
}

impl Statement for WhileStatement {
    fn execute(&self, environment: &mut Environment) -> Result<(), ParseError> {
        while self.condition.evaluate(environment)? == TokenType::True {
            self.body.execute(environment)?;
        }

        Ok(())
    }
}
