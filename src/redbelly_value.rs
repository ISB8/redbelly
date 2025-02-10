use std::fmt::Display;

use crate::parser::environment::Environment;

#[derive(Clone, PartialEq, Debug)]
pub enum RedbellyValue {
    Number(f64),
    String(String),
    True,
    False,
    Nil,
    Callable(RedbellyCallable),
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

#[derive(Clone, PartialEq, Debug)]
pub struct RedbellyCallable {
    arity: usize,
    call: fn(environment: &mut Environment, args: Vec<RedbellyValue>) -> RedbellyValue,
    to_string: fn() -> String,
}

impl RedbellyCallable {
    pub fn new(
        arity: usize,
        call: fn(environment: &mut Environment, args: Vec<RedbellyValue>) -> RedbellyValue,
        to_string: fn() -> String,
    ) -> Self {
        Self {
            arity,
            call,
            to_string,
        }
    }
    pub fn arity(&self) -> usize {
        self.arity
    }

    pub fn call(&self, environment: &mut Environment, args: Vec<RedbellyValue>) -> RedbellyValue {
        (self.call)(environment, args)
    }
}

impl Display for RedbellyCallable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", (self.to_string)())
    }
}
