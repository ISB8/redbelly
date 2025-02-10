use std::{
    fmt::{Debug, Display},
    rc::Rc,
    vec,
};

use crate::{
    lexer::{Token, TokenType},
    redbelly_value::RedbellyValue,
};

use environment::Environment;
use expression::{
    AssignmentExpression, Binary, CallExpression, Expression, Grouping, Literal, Logical, Unary,
    VariableExpression,
};
use statement::*;

pub struct Parser {
    tokens: Vec<Token>,
    index: usize,
}

impl Parser {
    pub fn parse_tokens(tokens: Vec<Token>) -> Result<Vec<Rc<dyn Statement>>, Vec<ParseError>> {
        let mut errors = vec![];
        let mut parser = Self { tokens, index: 0 };

        let mut statements = vec![];

        while !parser.is_at_end() {
            match parser.declaration() {
                Ok(stmt) => statements.push(stmt),
                Err(error) => {
                    errors.push(error);
                }
            }
        }
        if !errors.is_empty() {
            Err(errors)
        } else {
            Ok(statements)
        }
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
        self.tokens
            .get(self.index)
            .expect("Should never fail because index can never move beyond EOF token")
    }

    /// Returns the previous index
    fn previous(&self) -> &Token {
        self.tokens
            .get(self.index - 1)
            .expect("Should never fail because index can never move beyond EOF token")
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
            if self.check(i.clone()) {
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

// Implements Statements
impl Parser {
    fn declaration(&mut self) -> Result<Rc<dyn Statement>, ParseError> {
        if self.conditional_consume(vec![TokenType::Let]) {
            match self.var_declaration() {
                Err(error) => {
                    self.synchronize();
                    return Err(error);
                }
                Ok(stmt) => return Ok(stmt),
            }
        }
        self.statement()
    }

    fn var_declaration(&mut self) -> Result<Rc<dyn Statement>, ParseError> {
        if !self.conditional_consume(vec![TokenType::Identifier]) {
            return Err(ParseError::new("Expect Variable name", self.previous()));
        }

        let name = self.previous().clone();

        let mut initializer = None;
        if self.conditional_consume(vec![TokenType::Equal]) {
            initializer = Some(self.expression()?);
        }

        if !self.conditional_consume(vec![TokenType::Semicolon]) {
            return Err(ParseError::new(
                "Expect Semicolon after variable declaration",
                self.previous(),
            ));
        }

        Ok(Rc::from(VariableStatement::new(name, initializer)))
    }

    fn statement(&mut self) -> Result<Rc<dyn Statement>, ParseError> {
        if self.conditional_consume(vec![TokenType::Print]) {
            return self.print_statement();
        }
        if self.conditional_consume(vec![TokenType::LeftBrace]) {
            return self.block_statement();
        }
        if self.conditional_consume(vec![TokenType::If]) {
            return self.if_statement();
        }
        if self.conditional_consume(vec![TokenType::While]) {
            return self.while_statement();
        }
        if self.conditional_consume(vec![TokenType::For]) {
            return self.for_statement();
        }
        self.expression_statement()
    }

    fn expression_statement(&mut self) -> Result<Rc<dyn Statement>, ParseError> {
        let expr = self.expression()?;
        if self.conditional_consume(vec![TokenType::Semicolon]) {
            Ok(Rc::from(ExpressionStatement::new(expr)))
        } else {
            Err(ParseError::new(
                "Expected ; after expression",
                self.consume(),
            ))
        }
    }

    fn print_statement(&mut self) -> Result<Rc<dyn Statement>, ParseError> {
        let expr = self.expression()?;
        if self.conditional_consume(vec![TokenType::Semicolon]) {
            Ok(Rc::from(PrintStatement::new(expr)))
        } else {
            Err(ParseError::new("Expected ; after value", self.consume()))
        }
    }

    fn block_statement(&mut self) -> Result<Rc<dyn Statement>, ParseError> {
        let mut statements = vec![];

        while !self.check(TokenType::RightBrace) && !self.is_at_end() {
            statements.push(self.declaration()?);
        }
        if !self.conditional_consume(vec![TokenType::RightBrace]) {
            Err(ParseError::new("Expect '}' after block", self.consume()))
        } else {
            Ok(Rc::from(BlockStatement::new(statements)))
        }
    }

    fn if_statement(&mut self) -> Result<Rc<dyn Statement>, ParseError> {
        if !self.conditional_consume(vec![TokenType::LeftParen]) {
            return Err(ParseError::new("Expect '(' after if", self.consume()));
        }
        let expr = self.or()?;
        if !self.conditional_consume(vec![TokenType::RightParen]) {
            return Err(ParseError::new(
                "Expect ')' after if condition",
                self.consume(),
            ));
        }
        let then_branch = self.statement()?;
        let mut else_branch = None;

        if self.conditional_consume(vec![TokenType::Else]) {
            else_branch = Some(self.statement()?);
        }

        Ok(Rc::from(IfStatement::new(expr, then_branch, else_branch)))
    }

    fn while_statement(&mut self) -> Result<Rc<dyn Statement>, ParseError> {
        if !self.conditional_consume(vec![TokenType::LeftParen]) {
            return Err(ParseError::new("Expect '(' after while", self.consume()));
        }
        let condition = self.expression()?;
        if !self.conditional_consume(vec![TokenType::RightParen]) {
            return Err(ParseError::new(
                "Expect ')' after condition",
                self.consume(),
            ));
        }
        let body = self.statement()?;

        Ok(Rc::from(WhileStatement::new(condition, body)))
    }

    fn for_statement(&mut self) -> Result<Rc<dyn Statement>, ParseError> {
        if !self.conditional_consume(vec![TokenType::LeftParen]) {
            return Err(ParseError::new("Expect '(' after for", self.consume()));
        }

        // Initaliser
        let opt_initaliser: Option<Rc<dyn Statement>>;

        if self.conditional_consume(vec![TokenType::Semicolon]) {
            opt_initaliser = None;
        } else if self.conditional_consume(vec![TokenType::Let]) {
            opt_initaliser = Some(self.var_declaration()?);
        } else {
            opt_initaliser = Some(self.expression_statement()?);
        }

        // Condition
        let mut opt_condition = None;

        if !self.check(TokenType::Semicolon) {
            opt_condition = Some(self.expression()?);
        }

        if !self.conditional_consume(vec![TokenType::Semicolon]) {
            return Err(ParseError::new(
                "Expect ';' after loop condition",
                self.consume(),
            ));
        }

        // Increment
        let mut increment = None;
        if !self.check(TokenType::RightParen) {
            increment = Some(self.expression()?);
        }

        if !self.conditional_consume(vec![TokenType::RightParen]) {
            return Err(ParseError::new(
                "Expect ')' after for clauses",
                self.consume(),
            ));
        }

        let mut body = self.statement()?;

        // Desugaring
        if let Some(increment) = increment {
            body = Rc::from(BlockStatement::new(vec![
                Rc::from(ExpressionStatement::new(increment)),
                body,
            ]))
        }

        let condition;

        if let Some(expr) = opt_condition {
            condition = expr;
        } else {
            condition = Rc::from(Literal::new(RedbellyValue::True));
        }

        body = Rc::from(WhileStatement::new(condition, body));

        if let Some(init) = opt_initaliser {
            body = Rc::from(BlockStatement::new(vec![init, body]))
        }

        Ok(body)
    }
}

// This impl block contains all the grammer rules
impl Parser {
    fn expression(&mut self) -> Result<Rc<dyn Expression>, ParseError> {
        self.assignment()
    }

    fn assignment(&mut self) -> Result<Rc<dyn Expression>, ParseError> {
        let expr = self.equality()?;

        if self.conditional_consume(vec![TokenType::Equal]) {
            let equals = self.previous().clone();
            let value = self.assignment()?;

            match expr.to_any().downcast_ref::<VariableExpression>() {
                Some(var) => {
                    let name = var.name.clone();
                    return Ok(Rc::from(AssignmentExpression::new(name, value)));
                }
                None => return Err(ParseError::new("Invalid assignment target", &equals)),
            };
        }
        Ok(expr)
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

        self.call()
    }

    fn call(&mut self) -> Result<Rc<dyn Expression>, ParseError> {
        let mut expr = self.primary()?;

        loop {
            if self.conditional_consume(vec![TokenType::LeftParen]) {
                expr = self.finish_call(expr)?
            } else {
                break;
            }
        }

        Ok(expr)
    }

    fn finish_call(
        &mut self,
        callee: Rc<dyn Expression>,
    ) -> Result<Rc<dyn Expression>, ParseError> {
        let mut arguments = vec![];
        if !self.check(TokenType::RightParen) {
            loop {
                arguments.push(self.expression()?);
                if !self.conditional_consume(vec![TokenType::Comma]) {
                    break;
                }
            }
        }

        if arguments.len() >= 255 {
            return Err(ParseError::new(
                "A function cannot have more then 255 arguments",
                self.peek(),
            ));
        }

        let token = self.consume().clone();
        if token.token_type == TokenType::RightParen {
            Ok(Rc::from(CallExpression::new(callee, token, arguments)))
        } else {
            Err(ParseError::new(
                "Expected ')' after arguments",
                self.consume(),
            ))
        }
    }

    fn primary(&mut self) -> Result<Rc<dyn Expression>, ParseError> {
        if self.conditional_consume(vec![TokenType::False]) {
            return Ok(Rc::from(Literal::new(RedbellyValue::False)));
        }
        if self.conditional_consume(vec![TokenType::True]) {
            return Ok(Rc::from(Literal::new(RedbellyValue::True)));
        }
        if self.conditional_consume(vec![TokenType::Nil]) {
            return Ok(Rc::from(Literal::new(RedbellyValue::Nil)));
        }

        // For matching tokens with inner values
        match self.peek().token_type.clone() {
            TokenType::String(s) => {
                self.consume();
                return Ok(Rc::from(Literal::new(RedbellyValue::String(s))));
            }
            TokenType::Number(n) => {
                self.consume();
                return Ok(Rc::from(Literal::new(RedbellyValue::Number(n))));
            }
            _ => (),
        }
        if self.conditional_consume(vec![TokenType::Identifier]) {
            return Ok(Rc::from(VariableExpression::new(self.previous().clone())));
        }
        if self.conditional_consume(vec![TokenType::LeftParen]) {
            let expr = self.expression()?;
            if self.conditional_consume(vec![TokenType::RightParen]) {
                return Ok(Rc::from(Grouping::new(expr)));
            }
        }

        Err(ParseError::new("Expect Expression", self.consume()))
    }

    fn or(&mut self) -> Result<Rc<dyn Expression>, ParseError> {
        let mut expr = self.and()?;
        while self.conditional_consume(vec![TokenType::Or]) {
            let operator = self.previous().clone();
            let right = self.and()?;
            expr = Rc::from(Logical::new(expr, operator, right));
        }
        Ok(expr)
    }

    fn and(&mut self) -> Result<Rc<dyn Expression>, ParseError> {
        let mut expr = self.equality()?;
        while self.conditional_consume(vec![TokenType::And]) {
            let operator = self.previous().clone();
            let right = self.equality()?;
            expr = Rc::from(Logical::new(expr, operator, right));
        }
        Ok(expr)
    }
}

pub fn interpret(
    statements: Vec<Rc<dyn Statement>>,
    environment: &mut Environment,
) -> Option<ParseError> {
    for statement in statements {
        match statement.execute(environment) {
            Ok(()) => (),
            Err(error) => return Some(error),
        };
    }
    None
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

pub mod environment;
mod expression;
mod statement;
