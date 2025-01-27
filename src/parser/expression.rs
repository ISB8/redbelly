use std::{
    fmt::{self, Display},
    rc::Rc,
};

use crate::lexer::{Token, TokenType};

pub(crate) trait Expression {
    fn print(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result;
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
        write!(f, "{}", parenthesize(&self.operator.lexeme, vec![self.right.clone()]))
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
        write!(f, "{}", parenthesize(&self.operator.lexeme, vec![self.left.clone(), self.right.clone()]))
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

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use super::Grouping;
    use crate::{
        lexer::{Token, TokenType},
        parser::expression::{Binary, Expression, Literal, Unary},
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
}
