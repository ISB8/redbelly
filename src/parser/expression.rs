use std::{
    fmt::{self, Display},
    rc::Rc,
};

use crate::lexer::{Token, TokenType};

use super::ParseError;

pub(crate) trait Expression {
    fn print(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result;

    fn evaluate(&self) -> Result<TokenType, ParseError>;
}

impl Display for dyn Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.print(f)
    }
}

pub struct Literal {
    value: TokenType,
}

impl Literal {
    pub fn new(value: TokenType) -> Self {
        Self { value }
    }
}

impl Expression for Literal {
    fn print(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }

    fn evaluate(&self) -> Result<TokenType, ParseError> {
        Ok(self.value.clone())
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
    fn print(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", parenthesize("group", vec![self.expr.clone()]))
    }

    fn evaluate(&self) -> Result<TokenType, ParseError> {
        self.expr.evaluate()
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
    fn print(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            parenthesize(&self.operator.lexeme, vec![self.right.clone()])
        )
    }

    fn evaluate(&self) -> Result<TokenType, ParseError> {
        let expr = self.right.evaluate()?;

        match self.operator.token_type {
            TokenType::Minus => {
                if let Some(num) = try_cast_to_f64(&expr) {
                    return Ok(TokenType::Number(-num));
                } else {
                    return Err(ParseError::new("\"-\" not used on number", &self.operator));
                }
            }
            // No truthy values, might cause problems later
            // Returns the reverse of the expr
            TokenType::Bang => match expr {
                TokenType::False => return Ok(TokenType::True),
                TokenType::True => return Ok(TokenType::False),
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
    fn print(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            parenthesize(
                &self.operator.lexeme,
                vec![self.left.clone(), self.right.clone()]
            )
        )
    }

    fn evaluate(&self) -> Result<TokenType, ParseError> {
        let left = self.left.evaluate()?;
        let right = self.right.evaluate()?;

        match self.operator.token_type {
            // Arithmetic Operators
            TokenType::Minus => {
                if let (Some(left_num), Some(right_num)) =
                    (try_cast_to_f64(&left), try_cast_to_f64(&right))
                {
                    Ok(TokenType::Number(left_num - right_num))
                } else {
                    Err(ParseError::new("\"-\" not used on number", &self.operator))
                }
            }
            TokenType::Slash => {
                if let (Some(left_num), Some(right_num)) =
                    (try_cast_to_f64(&left), try_cast_to_f64(&right))
                {
                    Ok(TokenType::Number(left_num / right_num))
                } else {
                    Err(ParseError::new("\"/\" not used on number", &self.operator))
                }
            }
            TokenType::Star => {
                if let (Some(left_num), Some(right_num)) =
                    (try_cast_to_f64(&left), try_cast_to_f64(&right))
                {
                    Ok(TokenType::Number(left_num * right_num))
                } else {
                    Err(ParseError::new("\"*\" not used on number", &self.operator))
                }
            }
            TokenType::Plus => {
                if let (Some(left_str), Some(right_num)) =
                    (try_cast_to_f64(&left), try_cast_to_f64(&right))
                {
                    Ok(TokenType::Number(left_str + right_num))
                } else if let (Some(left_str), Some(right_str)) =
                    (try_cast_to_string(&left), try_cast_to_string(&right))
                {
                    return Ok(TokenType::String(left_str + &right_str));
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
                        true => Ok(TokenType::True),
                        false => Ok(TokenType::False),
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
                        true => Ok(TokenType::True),
                        false => Ok(TokenType::False),
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
                        true => Ok(TokenType::True),
                        false => Ok(TokenType::False),
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
                        true => Ok(TokenType::True),
                        false => Ok(TokenType::False),
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
                true => Ok(TokenType::True),
                false => Ok(TokenType::False),
            },
            TokenType::EqualEqual => match right == left {
                true => Ok(TokenType::True),
                false => Ok(TokenType::False),
            },
            _ => Err(ParseError::new("Unknown Operator", &self.operator)),
        }
    }
}

fn parenthesize(name: &str, exprs: Vec<Rc<dyn Expression>>) -> String {
    let mut string = String::new();
    string = string + "(" + name;

    for expr in exprs {
        string = string + " " + &format!("{}", expr);
    }
    string + ")"
}

fn try_cast_to_f64(token_type: &TokenType) -> Option<f64> {
    match token_type {
        TokenType::Number(num) => Some(*num),
        _ => None,
    }
}

fn try_cast_to_string(token_type: &TokenType) -> Option<String> {
    match token_type {
        TokenType::String(str) => Some(str.clone()),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use super::Grouping;
    use crate::{
        lexer::{Lexer, Token, TokenType},
        parser::{
            expression::{Binary, Expression, Literal, Unary},
            Parser,
        },
    };
    #[test]
    fn test_print() {
        let expr: Box<dyn Expression> = Box::new(Binary::new(
            Rc::new(Unary::new(
                Token::new(TokenType::Minus, String::from("-"), 1),
                Rc::new(Literal::new(TokenType::Number(123.))),
            )),
            Token::new(TokenType::Star, String::from("*"), 1),
            Rc::new(Grouping::new(Rc::new(Literal::new(TokenType::Number(
                45.67,
            ))))),
        ));
        assert_eq!(format!("{}", expr), "(* (- 123) (group 45.67))");
    }
    #[test]
    fn test_expr_eval() {
        let mut lexer = Lexer::new("1 + 3 * 9 / (3.6 + 3 * -7 )".to_string());
        let result = lexer.scan();
        assert_eq!(
            Parser::parse_tokens(result.unwrap())
                .unwrap()
                .evaluate()
                .unwrap(),
            TokenType::Number(-0.5517241379310347)
        );
    }

    #[test]
    fn test_string_concat() {
        let mut lexer = Lexer::new("\"foo\" + \"bar\"".to_string());
        let result = lexer.scan();
        assert_eq!(
            Parser::parse_tokens(result.unwrap())
                .unwrap()
                .evaluate()
                .unwrap(),
            TokenType::String("foobar".to_string())
        );
    }
}
