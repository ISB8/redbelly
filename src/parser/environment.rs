use std::{
    collections::HashMap,
    fmt::Display,
    io::{stdin, stdout, Write},
    rc::Rc,
    time::UNIX_EPOCH,
};

use crate::{
    lexer::Token,
    redbelly_value::{RedbellyCallable, RedbellyValue},
};

use super::RuntimeError;

#[derive(Clone)]
pub struct Environment {
    pub(crate) values: HashMap<String, RedbellyValue>,
    pub(crate) enclosing: Box<Option<Environment>>,
}

impl Environment {
    pub fn new(enclosing: Option<Environment>) -> Self {
        Self {
            values: HashMap::new(),
            enclosing: Box::new(enclosing),
        }
    }
    pub fn define(&mut self, name: String, value: RedbellyValue) {
        self.values.insert(name, value);
    }

    pub fn assign(&mut self, name: Token, value: RedbellyValue) -> Option<RuntimeError> {
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
        Some(RuntimeError::from_token(
            &format!("Undefined Variable {}", name.lexeme),
            &name,
        ))
    }

    pub fn get(&self, name: Token) -> Result<&RedbellyValue, RuntimeError> {
        match self.values.get(&name.lexeme) {
            Some(val) => Ok(val),
            None => {
                if let Some(env) = &*self.enclosing {
                    return env.get(name);
                }
                Err(RuntimeError::from_token("Undefined variable", &name))
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

#[derive(Clone, PartialEq, Debug)]
struct RedbellyGlobalFunction {
    arity: usize,
    call: fn(environment: &mut Environment, args: Vec<RedbellyValue>) -> RedbellyValue,
    to_string: fn() -> String,
}

impl RedbellyGlobalFunction {
    fn new(
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
}

impl RedbellyCallable for RedbellyGlobalFunction {
    fn arity(&self) -> usize {
        self.arity
    }

    fn call(
        &self,
        environment: &mut Environment,
        args: Vec<RedbellyValue>,
    ) -> Result<RedbellyValue, RuntimeError> {
        Ok((self.call)(environment, args))
    }

    fn to_string(&self) -> String {
        (self.to_string)()
    }
}

impl Display for RedbellyGlobalFunction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", (self.to_string)())
    }
}

pub fn redbelly_globals() -> Environment {
    let mut globals = Environment::new(None);
    {
        let call = |_environment: &mut Environment, _args: Vec<RedbellyValue>| -> RedbellyValue {
            RedbellyValue::Number(
                std::time::SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .expect("Error Acquiring system time")
                    .as_secs() as f64,
            )
        };
        globals.define(
            "clock".to_owned(),
            RedbellyValue::Callable(Rc::from(RedbellyGlobalFunction::new(0, call, || {
                "<native fn>".to_owned()
            }))),
        );
    }
    {
        let call = |_environment: &mut Environment, args: Vec<RedbellyValue>| -> RedbellyValue {
            for arg in args {
                print!("{}", arg);
            }
            RedbellyValue::Nil
        };
        globals.define(
            "print".to_owned(),
            RedbellyValue::Callable(Rc::from(RedbellyGlobalFunction::new(1, call, || {
                "<native fn>".to_owned()
            }))),
        );
    }
    {
        let call = |_environment: &mut Environment, args: Vec<RedbellyValue>| -> RedbellyValue {
            for arg in args {
                print!("{}", arg);
            }
            println!();
            RedbellyValue::Nil
        };
        globals.define(
            "println".to_owned(),
            RedbellyValue::Callable(Rc::from(RedbellyGlobalFunction::new(1, call, || {
                "<native fn>".to_owned()
            }))),
        );
    }
    {
        let call = |_environment: &mut Environment, _args: Vec<RedbellyValue>| -> RedbellyValue {
            let mut input = String::new();
            let _ = stdout().flush();
            let _ = stdin().read_line(&mut input);
            let input = input.trim_end().to_owned();
            RedbellyValue::String(input)
        };
        globals.define(
            "input".to_owned(),
            RedbellyValue::Callable(Rc::from(RedbellyGlobalFunction::new(0, call, || {
                "<native fn>".to_owned()
            }))),
        );
    }
    {
        let call = |_environment: &mut Environment, args: Vec<RedbellyValue>| -> RedbellyValue {
            std::process::exit(args.first().unwrap().try_cast_to_f64().unwrap_or(1.) as i32)
        };
        globals.define(
            "exit".to_owned(),
            RedbellyValue::Callable(Rc::from(RedbellyGlobalFunction::new(1, call, || {
                "<native fn>".to_owned()
            }))),
        );
    }
    {
        let call = |_environment: &mut Environment, args: Vec<RedbellyValue>| -> RedbellyValue {
            match &args[0] {
                RedbellyValue::String(str) => match str.parse::<f64>() {
                    Ok(num) => RedbellyValue::Number(num),
                    Err(_) => RedbellyValue::Nil,
                },
                _ => RedbellyValue::Nil,
            }
        };
        globals.define(
            "str_to_num".to_owned(),
            RedbellyValue::Callable(Rc::from(RedbellyGlobalFunction::new(1, call, || {
                "<native fn>".to_owned()
            }))),
        );
    }
    {
        let call = |_environment: &mut Environment, args: Vec<RedbellyValue>| -> RedbellyValue {
            match &args[0] {
                RedbellyValue::Number(num) => RedbellyValue::Number(num.round()),
                _ => RedbellyValue::Nil,
            }
        };
        globals.define(
            "floor".to_owned(),
            RedbellyValue::Callable(Rc::from(RedbellyGlobalFunction::new(1, call, || {
                "<native fn>".to_owned()
            }))),
        );
    }
    {
        let call = |_environment: &mut Environment, args: Vec<RedbellyValue>| -> RedbellyValue {
            match &args[0] {
                RedbellyValue::Number(num) => RedbellyValue::Number(num.abs()),
                _ => RedbellyValue::Nil,
            }
        };
        globals.define(
            "abs".to_owned(),
            RedbellyValue::Callable(Rc::from(RedbellyGlobalFunction::new(1, call, || {
                "<native fn>".to_owned()
            }))),
        );
    }
    globals
}

#[cfg(test)]
mod tests {
    use crate::{lexer::Token, lexer::TokenType, redbelly_value::RedbellyValue};

    use super::Environment;

    #[test]
    fn test_enclosing_environments() {
        let mut enclosing = Environment::new(None);
        enclosing.define("test".to_owned(), RedbellyValue::Number(3.));
        let environment = enclosing.enclosed();

        let result = environment.get(Token::new(TokenType::Identifier, "test".to_owned(), 1));

        assert!(result.unwrap() == &RedbellyValue::Number(3.));
    }

    #[test]
    fn test_shadowing() {
        let mut enclosing = Environment::new(None);
        enclosing.define("test".to_owned(), RedbellyValue::Number(3.));
        let mut environment = enclosing.enclosed();

        environment.define("test".to_owned(), RedbellyValue::Number(4.));

        let result = environment.get(Token::new(TokenType::Identifier, "test".to_owned(), 1));

        assert!(result.unwrap() == &RedbellyValue::Number(4.));
    }
    #[test]
    fn test_nested_reassignment() {
        let mut enclosing = Environment::new(None);
        enclosing.define("test".to_owned(), RedbellyValue::Number(3.));
        let mut environment = enclosing.enclosed();

        environment.assign(
            Token::new(TokenType::Identifier, "test".to_owned(), 1),
            RedbellyValue::Number(4.),
        );

        let new_env = environment.release_enclosing().unwrap();
        let result = new_env.get(Token::new(TokenType::Identifier, "test".to_owned(), 1));

        assert!(result.unwrap() == &RedbellyValue::Number(4.));
    }
}
