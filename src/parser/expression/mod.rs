use std::{any::Any, rc::Rc};

use crate::{
    lexer::{Token, TokenType},
    redbelly_value::RedbellyValue,
};

use super::{environment::Environment, ParseError};
pub(crate) trait Expression {
    fn evaluate(&self, environment: &mut Environment) -> Result<RedbellyValue, ParseError>;
    fn to_any(&self) -> &dyn Any;
}

pub struct Literal {
    value: RedbellyValue,
}

impl Literal {
    pub fn new(value: RedbellyValue) -> Self {
        Self { value }
    }
}

impl Expression for Literal {
    fn evaluate(&self, environment: &mut Environment) -> Result<RedbellyValue, ParseError> {
        let _ = environment;
        Ok(self.value.clone())
    }

    fn to_any(&self) -> &dyn Any {
        self
    }
}
pub struct Grouping {
    expr: Rc<dyn Expression>,
}

impl Grouping {
    pub fn new(expr: Rc<dyn Expression>) -> Self {
        Self { expr }
    }
}

impl Expression for Grouping {
    fn evaluate(&self, environment: &mut Environment) -> Result<RedbellyValue, ParseError> {
        self.expr.evaluate(environment)
    }
    fn to_any(&self) -> &dyn Any {
        self
    }
}

pub struct Unary {
    operator: Token,
    right: Rc<dyn Expression>,
}

impl Unary {
    pub fn new(operator: Token, right: Rc<dyn Expression>) -> Self {
        Self { operator, right }
    }
}

impl Expression for Unary {
    fn evaluate(&self, environment: &mut Environment) -> Result<RedbellyValue, ParseError> {
        let expr = self.right.evaluate(environment)?;

        match self.operator.token_type {
            TokenType::Minus => {
                if let Some(num) = try_cast_to_f64(&expr) {
                    return Ok(RedbellyValue::Number(-num));
                } else {
                    return Err(ParseError::new("\"-\" not used on number", &self.operator));
                }
            }
            // No truthy values, might cause problems later
            // Returns the reverse of the expr
            TokenType::Bang => match expr {
                RedbellyValue::False => return Ok(RedbellyValue::True),
                RedbellyValue::True => return Ok(RedbellyValue::False),
                _ => {
                    return Err(ParseError::new(
                        "\"!\" used on non bool value",
                        &self.operator,
                    ))
                }
            },
            _ => (),
        }

        Err(ParseError::new("Invalid Operator", &self.operator))
    }
    fn to_any(&self) -> &dyn Any {
        self
    }
}

pub struct Binary {
    left: Rc<dyn Expression>,
    operator: Token,
    right: Rc<dyn Expression>,
}

impl Binary {
    pub fn new(left: Rc<dyn Expression>, operator: Token, right: Rc<dyn Expression>) -> Self {
        Self {
            left,
            operator,
            right,
        }
    }
}

impl Expression for Binary {
    fn evaluate(&self, environment: &mut Environment) -> Result<RedbellyValue, ParseError> {
        let left = self.left.evaluate(environment)?;
        let right = self.right.evaluate(environment)?;

        match self.operator.token_type {
            // Arithmetic Operators
            TokenType::Minus => {
                if let (Some(left_num), Some(right_num)) =
                    (try_cast_to_f64(&left), try_cast_to_f64(&right))
                {
                    Ok(RedbellyValue::Number(left_num - right_num))
                } else {
                    Err(ParseError::new("\"-\" not used on number", &self.operator))
                }
            }
            TokenType::Slash => {
                if let (Some(left_num), Some(right_num)) =
                    (try_cast_to_f64(&left), try_cast_to_f64(&right))
                {
                    Ok(RedbellyValue::Number(left_num / right_num))
                } else {
                    Err(ParseError::new("\"/\" not used on number", &self.operator))
                }
            }
            TokenType::Star => {
                if let (Some(left_num), Some(right_num)) =
                    (try_cast_to_f64(&left), try_cast_to_f64(&right))
                {
                    Ok(RedbellyValue::Number(left_num * right_num))
                } else {
                    Err(ParseError::new("\"*\" not used on number", &self.operator))
                }
            }
            TokenType::Plus => {
                if let (Some(left_str), Some(right_num)) =
                    (try_cast_to_f64(&left), try_cast_to_f64(&right))
                {
                    Ok(RedbellyValue::Number(left_str + right_num))
                } else if let (Some(left_str), Some(right_str)) =
                    (try_cast_to_string(&left), try_cast_to_string(&right))
                {
                    return Ok(RedbellyValue::String(left_str + &right_str));
                } else {
                    return Err(ParseError::new(
                        "\"+\" not used on number or string",
                        &self.operator,
                    ));
                }
            }
            // Comparison Operators
            TokenType::Greater => {
                if let (Some(left_num), Some(right_num)) =
                    (try_cast_to_f64(&left), try_cast_to_f64(&right))
                {
                    match left_num > right_num {
                        true => Ok(RedbellyValue::True),
                        false => Ok(RedbellyValue::False),
                    }
                } else {
                    Err(ParseError::new(
                        "Operator not used on number",
                        &self.operator,
                    ))
                }
            }
            TokenType::GreaterEqual => {
                if let (Some(left_num), Some(right_num)) =
                    (try_cast_to_f64(&left), try_cast_to_f64(&right))
                {
                    match left_num >= right_num {
                        true => Ok(RedbellyValue::True),
                        false => Ok(RedbellyValue::False),
                    }
                } else {
                    Err(ParseError::new(
                        "Operator not used on number",
                        &self.operator,
                    ))
                }
            }
            TokenType::Less => {
                if let (Some(left_num), Some(right_num)) =
                    (try_cast_to_f64(&left), try_cast_to_f64(&right))
                {
                    match left_num < right_num {
                        true => Ok(RedbellyValue::True),
                        false => Ok(RedbellyValue::False),
                    }
                } else {
                    Err(ParseError::new(
                        "Operator not used on number",
                        &self.operator,
                    ))
                }
            }
            TokenType::LessEqual => {
                if let (Some(left_num), Some(right_num)) =
                    (try_cast_to_f64(&left), try_cast_to_f64(&right))
                {
                    match left_num <= right_num {
                        true => Ok(RedbellyValue::True),
                        false => Ok(RedbellyValue::False),
                    }
                } else {
                    Err(ParseError::new(
                        "Operator not used on number",
                        &self.operator,
                    ))
                }
            }
            // Equality Operators
            TokenType::BangEqual => match right != left {
                true => Ok(RedbellyValue::True),
                false => Ok(RedbellyValue::False),
            },
            TokenType::EqualEqual => match right == left {
                true => Ok(RedbellyValue::True),
                false => Ok(RedbellyValue::False),
            },
            _ => Err(ParseError::new("Unknown Operator", &self.operator)),
        }
    }
    fn to_any(&self) -> &dyn Any {
        self
    }
}

pub struct CallExpression {
    callee: Rc<dyn Expression>,
    parentheses: Token,
    arguments: Vec<Rc<dyn Expression>>,
}

impl CallExpression {
    pub fn new(
        callee: Rc<dyn Expression>,
        parentheses: Token,
        arguments: Vec<Rc<dyn Expression>>,
    ) -> Self {
        Self {
            callee,
            parentheses,
            arguments,
        }
    }
}

impl Expression for CallExpression {
    fn evaluate(&self, environment: &mut Environment) -> Result<RedbellyValue, ParseError> {
        let mut args = vec![];
        for arg in &self.arguments {
            args.push(arg.evaluate(environment)?);
        }

        if let RedbellyValue::Callable(func) = self.callee.evaluate(environment)? {
            if args.len() != func.arity() {
                return Err(ParseError::new(
                    &format!(
                        "Expected {} arguments but found {}",
                        func.arity(),
                        args.len()
                    ),
                    &self.parentheses,
                ));
            }
            Ok(func.call(environment, args))
        } else {
            Err(ParseError::new(
                "Can only call functions",
                &self.parentheses,
            ))
        }
    }

    fn to_any(&self) -> &dyn Any {
        self
    }
}
pub struct VariableExpression {
    pub name: Token,
}

impl VariableExpression {
    pub fn new(name: Token) -> Self {
        Self { name }
    }
}

impl Expression for VariableExpression {
    fn evaluate(&self, environment: &mut Environment) -> Result<RedbellyValue, ParseError> {
        Ok(environment.get(self.name.clone())?.clone())
    }
    fn to_any(&self) -> &dyn Any {
        self
    }
}

pub struct AssignmentExpression {
    name: Token,
    value: Rc<dyn Expression>,
}

impl AssignmentExpression {
    pub fn new(name: Token, value: Rc<dyn Expression>) -> Self {
        Self { name, value }
    }
}

impl Expression for AssignmentExpression {
    fn evaluate(&self, environment: &mut Environment) -> Result<RedbellyValue, ParseError> {
        let value = self.value.evaluate(environment)?;
        match environment.assign(self.name.clone(), value.clone()) {
            Some(error) => Err(error),
            None => Ok(value),
        }
    }

    fn to_any(&self) -> &dyn Any {
        self
    }
}

pub struct Logical {
    left: Rc<dyn Expression>,
    operator: Token,
    right: Rc<dyn Expression>,
}

impl Logical {
    pub fn new(left: Rc<dyn Expression>, operator: Token, right: Rc<dyn Expression>) -> Self {
        Self {
            left,
            operator,
            right,
        }
    }
}

impl Expression for Logical {
    fn evaluate(&self, environment: &mut Environment) -> Result<RedbellyValue, ParseError> {
        let left = self.left.evaluate(environment)?;

        // Handle non bool values
        if left != RedbellyValue::True && left != RedbellyValue::False {
            return Err(ParseError::new(
                "Logical Operator used on non bool value",
                &self.operator,
            ));
        }

        if self.operator.token_type == TokenType::Or {
            if left == RedbellyValue::True {
                return Ok(left);
            }
        } else if left == RedbellyValue::False {
            return Ok(left);
        }

        let right = self.right.evaluate(environment)?;

        if right != RedbellyValue::True && right != RedbellyValue::False {
            return Err(ParseError::new(
                "Logical Operator used on non bool value",
                &self.operator,
            ));
        }
        Ok(right)
    }

    fn to_any(&self) -> &dyn Any {
        self
    }
}

fn try_cast_to_f64(token_type: &RedbellyValue) -> Option<f64> {
    match token_type {
        &RedbellyValue::Number(num) => Some(num),
        _ => None,
    }
}

fn try_cast_to_string(value_type: &RedbellyValue) -> Option<String> {
    match value_type {
        RedbellyValue::String(str) => Some(str.clone()),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use crate::{
        lexer::{Lexer, Token},
        parser::{environment::Environment, expression::Expression, ParseError, Parser},
        redbelly_value::RedbellyValue,
    };

    impl Parser {
        fn parse_tokens_to_expr(tokens: Vec<Token>) -> Result<Rc<dyn Expression>, Vec<ParseError>> {
            let mut errors = vec![];
            let mut parser = Self { tokens, index: 0 };

            match parser.expression() {
                Ok(expr) => return Ok(expr),
                Err(err) => {
                    errors.push(err);
                }
            }

            Err(errors)
        }
    }
    #[test]
    fn test_expr_eval() {
        let mut lexer = Lexer::new("1 + 3 * 9 / (3.6 + 3 * -7 )".to_string());
        let result = lexer.scan();
        assert_eq!(
            Parser::parse_tokens_to_expr(result.unwrap())
                .unwrap()
                .evaluate(&mut Environment::new(None))
                .unwrap(),
            RedbellyValue::Number(-0.5517241379310347)
        );
    }

    #[test]
    fn test_string_concat() {
        let mut lexer = Lexer::new("\"foo\" + \"bar\"".to_string());
        let result = lexer.scan();
        assert_eq!(
            Parser::parse_tokens_to_expr(result.unwrap())
                .unwrap()
                .evaluate(&mut Environment::new(None))
                .unwrap(),
            RedbellyValue::String("foobar".to_string())
        );
    }
}
