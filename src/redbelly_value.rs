use std::{fmt::Display, rc::Rc};

use crate::parser::environment::Environment;

#[derive(Debug, Clone, PartialEq)]
pub enum RedbellyValue {
    Number(f64),
    String(String),
    Identifier(IdentifierType),
    True,
    False,
    Nil,
}

impl Display for RedbellyValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RedbellyValue::Number(num) => write!(f, "{}", num),
            RedbellyValue::String(str) => write!(f, "{}", str),
            RedbellyValue::Identifier(identifier_type) => match identifier_type {
                IdentifierType::Variable => todo!(),
                IdentifierType::Callable => todo!(),
            },
            RedbellyValue::True => write!(f, "true"),
            RedbellyValue::False => write!(f, "false"),
            RedbellyValue::Nil => write!(f, "nil"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum IdentifierType {
    Variable,
    Callable,
}

pub(crate) trait RedbellyCallable {
    fn call(&self, environment: &mut Environment, args: Vec<RedbellyValue>) -> RedbellyValue;
    fn arity(&self) -> usize;
    fn arguments(&self) -> Vec<RedbellyValue>;
    fn print(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result;
}

pub struct RedbellyFunction {
    arity: usize,
}

impl RedbellyCallable for RedbellyFunction {
    fn call(&self, environment: &mut Environment, args: Vec<RedbellyValue>) -> RedbellyValue {
        todo!()
    }

    fn arity(&self) -> usize {
        todo!()
    }

    fn arguments(&self) -> Vec<RedbellyValue> {
        todo!()
    }
    fn print(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "function")
    }
}

impl Display for RedbellyFunction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.print(f)
    }
}

pub(crate) fn try_cast_to_redbelly_callable(
    _rb_val: &RedbellyValue,
) -> Option<Rc<dyn RedbellyCallable>> {
    todo!();
}
