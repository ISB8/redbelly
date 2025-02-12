use std::rc::Rc;

use crate::{
    lexer::Token,
    redbelly_value::{RedbellyFunction, RedbellyValue},
};

use super::{
    environment::Environment, expression::Expression, RedbellyRuntimeException, RuntimeError,
};

pub trait Statement {
    fn execute(&self, environment: &mut Environment) -> Result<(), RedbellyRuntimeException>;
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
    fn execute(&self, environment: &mut Environment) -> Result<(), RedbellyRuntimeException> {
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
    fn execute(&self, environment: &mut Environment) -> Result<(), RedbellyRuntimeException> {
        match &self.initializer {
            Some(expr) => {
                let val = expr.evaluate(environment)?;
                environment.define(self.name.lexeme.clone(), val);
                Ok(())
            }
            None => {
                environment.define(self.name.lexeme.clone(), RedbellyValue::Nil);
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
    fn execute(&self, environment: &mut Environment) -> Result<(), RedbellyRuntimeException> {
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
    fn execute(&self, environment: &mut Environment) -> Result<(), RedbellyRuntimeException> {
        let result = self.condition.evaluate(environment)?;

        match result {
            RedbellyValue::True => self.then_branch.execute(environment),
            RedbellyValue::False => {
                if let Some(else_branch) = &self.else_branch {
                    else_branch.execute(environment)?;
                }
                Ok(())
            }
            _ => Err(RedbellyRuntimeException::Error(RuntimeError::new(
                "Condition within if statement was not True or False",
            ))),
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
    fn execute(&self, environment: &mut Environment) -> Result<(), RedbellyRuntimeException> {
        while self.condition.evaluate(environment)? == RedbellyValue::True {
            self.body.execute(environment)?;
        }

        Ok(())
    }
}

#[derive(Clone)]
pub struct FunctionStatement {
    pub name: Token,
    pub parameters: Vec<Token>,
    pub body: Rc<dyn Statement>,
}

impl FunctionStatement {
    pub fn new(name: Token, parameters: Vec<Token>, body: Rc<dyn Statement>) -> Self {
        Self {
            name,
            parameters,
            body,
        }
    }
}

impl Statement for FunctionStatement {
    fn execute(
        &self,
        environment: &mut Environment,
    ) -> std::result::Result<(), RedbellyRuntimeException> {
        let function = RedbellyFunction::new(self.to_owned());
        environment.define(
            self.name.lexeme.clone(),
            RedbellyValue::Callable(Rc::from(function)),
        );
        Ok(())
    }
}

pub struct ReturnStatement {
    value: Rc<dyn Expression>,
}

impl ReturnStatement {
    pub fn new(value: Rc<dyn Expression>) -> Self {
        Self { value }
    }
}

impl Statement for ReturnStatement {
    fn execute(&self, environment: &mut Environment) -> Result<(), RedbellyRuntimeException> {
        Err(RedbellyRuntimeException::Return(
            self.value.evaluate(environment)?,
        ))
    }
}

impl From<RuntimeError> for RedbellyRuntimeException {
    fn from(value: RuntimeError) -> Self {
        RedbellyRuntimeException::Error(value)
    }
}
