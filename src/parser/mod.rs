use std::{fmt::{Debug, Display}, rc::Rc, vec};

use crate::lexer::{Token, TokenType};

use expression::{Binary, Expression, Grouping, Literal, Unary};

pub struct Parser {
    tokens: Vec<Token>,
    index: usize,
}

impl Parser {
    pub fn parse_tokens(tokens: Vec<Token>) -> Result<Rc<dyn Expression>, Vec<ParseError>> {
        let mut errors = vec![];
        let mut parser = Self { tokens, index: 0 };

        match parser.expression() {
            Ok(expr) => return Ok(expr),
            Err(err) => {
                errors.push(err);
            }
        }

        return Err(errors);
    }

    /// Increments the index, then returns the token at index - 1
    fn consume(&mut self) -> &Token {
        if !self.is_at_end() {
            self.index += 1;
        }
        self.previous()
    }

    /// Returns the current index
    fn peek(&self) -> &Token {
        return self
            .tokens
            .get(self.index)
            .expect("Should never fail because index can never move beyond EOF token");
    }

    /// Returns the previous index
    fn previous(&self) -> &Token {
        return self
            .tokens
            .get(self.index - 1)
            .expect("Should never fail because index can never move beyond EOF token");
    }

    fn synchronize(&mut self) {
        self.consume();

        while !self.is_at_end() {
            if self.previous().token_type == TokenType::Semicolon {
                return;
            }
            use crate::lexer::TokenType as t;
            match self.peek().token_type {
                t::Class | t::Func | t::Let | t::For | t::If | t::While | t::Print | t::Return => {
                    return
                }
                _ => (),
            }

            self.consume();
        }
    }

    /// Checks if the token at index matches a given pattern
    fn conditional_consume(&mut self, types: Vec<TokenType>) -> bool {
        for i in types {
            if self.check(i) {
                self.consume();
                return true;
            }
        }
        false
    }

    fn check(&self, token_type: TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        self.peek().token_type == token_type
    }

    fn is_at_end(&self) -> bool {
        self.peek().token_type == TokenType::Eof
    }
}

// This impl block contains all the grammer rules
impl Parser {
    fn expression(&mut self) -> Result<Rc<dyn Expression>, ParseError> {
        self.equality()
    }

    fn equality(&mut self) -> Result<Rc<dyn Expression>, ParseError> {
        let mut expr = self.comparison()?;

        while self.conditional_consume(vec![TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.previous().clone();
            let right = self.comparison()?;
            expr = Rc::from(Binary::new(expr, operator, right));
        }

        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Rc<dyn Expression>, ParseError> {
        let mut expr = self.term()?;

        while self.conditional_consume(vec![
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let operator = self.previous().clone();
            let right = self.term()?;
            expr = Rc::from(Binary::new(expr, operator, right));
        }

        Ok(expr)
    }

    fn term(&mut self) -> Result<Rc<dyn Expression>, ParseError> {
        let mut expr = self.factor()?;

        while self.conditional_consume(vec![TokenType::Minus, TokenType::Plus]) {
            let operator = self.previous().clone();
            let right = self.factor()?;
            expr = Rc::from(Binary::new(expr, operator, right))
        }

        Ok(expr)
    }

    fn factor(&mut self) -> Result<Rc<dyn Expression>, ParseError> {
        let mut expr = self.unary()?;

        while self.conditional_consume(vec![TokenType::Slash, TokenType::Star]) {
            let operator = self.previous().clone();
            let right = self.unary()?;
            expr = Rc::from(Binary::new(expr, operator, right))
        }

        Ok(expr)
    }

    fn unary(&mut self) -> Result<Rc<dyn Expression>, ParseError> {
        if self.conditional_consume(vec![TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous().clone();
            let right = self.unary()?;
            return Ok(Rc::from(Unary::new(operator, right)));
        }

        self.primary()
    }

    fn primary(&mut self) -> Result<Rc<dyn Expression>, ParseError> {
        if self.conditional_consume(vec![TokenType::False]) {
            return Ok(Rc::from(Literal::new(TokenType::False)));
        }
        if self.conditional_consume(vec![TokenType::True]) {
            return Ok(Rc::from(Literal::new(TokenType::True)));
        }
        if self.conditional_consume(vec![TokenType::Nil]) {
            return Ok(Rc::from(Literal::new(TokenType::Nil)));
        }

        // For matching tokens with inner values
        match self.peek().token_type.clone() {
            TokenType::String(s) => {
                self.consume();
                return Ok(Rc::from(Literal::new(TokenType::String(s))));
            }
            TokenType::Number(n) => {
                self.consume();
                return Ok(Rc::from(Literal::new(TokenType::Number(n))));
            }
            _ => (),
        }

        if self.conditional_consume(vec![TokenType::LeftParen]) {
            let expr = self.expression()?;
            if self.conditional_consume(vec![TokenType::RightParen]) {
                return Ok(Rc::from(Grouping::new(expr)));
            }
        }

        return Err(ParseError::new("Expect ')' after expression", self.peek()));
    }
}

pub fn interpret(expr: Rc<dyn Expression>) -> Option<ParseError> {
    match expr.evaluate() {
        Ok(evaluated) => {
            println!("{}", evaluated);
            None
        }
        Err(err) => Some(err),
    }
}

pub struct ParseError {
    message: String,
    token: Token,
}

impl ParseError {
    pub fn new(message: &str, token: &Token) -> Self {
        Self {
            message: String::from(message),
            token: token.clone(),
        }
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.token.token_type == TokenType::Eof {
            write!(f, "[Line {}]: {} at end", self.token.line, self.message)
        } else {
            write!(
                f,
                "[Line {}]: {} at \"{}\"",
                self.token.line, self.message, self.token.lexeme
            )
        }
    }
}
impl Debug for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

mod expression;
mod statment;