use std::{
    fmt::{Debug, Display},
    rc::Rc,
};

use crate::parser::statement::FunctionStatement;
use crate::parser::{environment::Environment, RuntimeError};

#[derive(Clone, Debug)]
pub enum RedbellyValue {
    Number(f64),
    String(String),
    True,
    False,
    Nil,
    Callable(Rc<dyn RedbellyCallable>),
}

impl RedbellyValue {
    pub fn try_cast_to_f64(&self) -> Option<f64> {
        match self {
            &RedbellyValue::Number(num) => Some(num),
            _ => None,
        }
    }

    pub fn try_cast_to_string(&self) -> Option<String> {
        match self {
            RedbellyValue::String(str) => Some(str.clone()),
            _ => None,
        }
    }
}

impl Display for RedbellyValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RedbellyValue::Number(num) => write!(f, "{}", num),
            RedbellyValue::String(str) => write!(f, "{}", str),
            RedbellyValue::True => write!(f, "true"),
            RedbellyValue::False => write!(f, "false"),
            RedbellyValue::Nil => write!(f, "nil"),
            RedbellyValue::Callable(func) => write!(f, "{}", func),
        }
    }
}

impl PartialEq for RedbellyValue {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Number(l0), Self::Number(r0)) => l0 == r0,
            (Self::String(l0), Self::String(r0)) => l0 == r0,
            (Self::Callable(_l0), Self::Callable(_r0)) => false,
            _ => core::mem::discriminant(self) == core::mem::discriminant(other),
        }
    }
}

pub trait RedbellyCallable {
    fn arity(&self) -> usize;
    fn call(
        &self,
        environment: &mut Environment,
        args: Vec<RedbellyValue>,
    ) -> Result<RedbellyValue, RuntimeError>;
    fn to_string(&self) -> String;
}

impl Display for dyn RedbellyCallable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl Debug for dyn RedbellyCallable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

pub struct RedbellyFunction {
    arity: usize,
    declaration: FunctionStatement,
}

impl RedbellyFunction {
    pub fn new(declaration: FunctionStatement) -> Self {
        Self {
            arity: declaration.parameters.len(),
            declaration,
        }
    }
}

impl RedbellyCallable for RedbellyFunction {
    fn arity(&self) -> usize {
        self.arity
    }

    fn call(
        &self,
        environment: &mut Environment,
        args: Vec<RedbellyValue>,
    ) -> Result<RedbellyValue, RuntimeError> {
        let mut environment = environment.clone().enclosed();
        for (index, paramenter) in self.declaration.parameters.clone().into_iter().enumerate() {
            environment.define(
                paramenter.lexeme,
                args.get(index)
                    .expect("Index should never be invalid")
                    .clone(),
            );
        }

        self.declaration.body.execute(&mut environment).unwrap();

        Ok(RedbellyValue::Nil)
    }

    fn to_string(&self) -> String {
        format!("<fn {}>", self.declaration.name.lexeme)
    }
}
