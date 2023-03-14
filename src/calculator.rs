use std::rc::Rc;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Token {
    Number(u32),
    Op(Operator),
    Bracket(char),
}

pub struct Calculator {}

#[derive(Debug)]
pub enum Error {
    InvalidToken,
    InvalidExpression,
    InvalidOperator,
    InvalidNumber,
    InvalidBracket,
    InvalidParenthesis,
    InvalidDivisionByZero,
}

impl Calculator {
    pub fn parse(expr: impl AsRef<str>) -> Result<Vec<Token>, Error> {
        let expr = expr.as_ref();
        let chars = expr.chars();
        let mut tokens: Vec<Token> = Vec::new();
        let mut parens = Vec::new();
        for c in chars {
            match c {
                '0'..='9' => match tokens.last_mut() {
                    _ => {
                        let digit = c.to_digit(10).unwrap() as u32;
                        tokens.push(Token::Number(digit));
                    }
                },
                '+' => tokens.push(Token::Op(Operator::Add)),
                '-' => tokens.push(Token::Op(Operator::Subtract)),
                '*' => tokens.push(Token::Op(Operator::Multiply)),
                '/' => tokens.push(Token::Op(Operator::Divide)),
                '(' => {
                    tokens.push(Token::Bracket('('));
                    parens.push('(');
                }
                ')' => {
                    tokens.push(Token::Bracket(')'));
                    if parens.pop() != Some('(') {
                        return Err(Error::InvalidParenthesis);
                    }
                }
                '\n' => {}
                _ => return Err(Error::InvalidToken),
            }
        }
        if !parens.is_empty() {
            return Err(Error::InvalidParenthesis);
        }
        Ok(tokens)
    }

    pub fn expression(mut tokens: Vec<Token>) -> Vec<Token> {
        tokens.reverse();
        let mut queue: Vec<Token> = Vec::new();
        let mut stack: Vec<Token> = Vec::new();
        while let Some(token) = tokens.pop() {
            match token {
                Token::Number(_) => queue.push(token),
                Token::Op(_) => {
                    while !stack.is_empty() && stack.last().unwrap() >= &token {
                        queue.push(stack.pop().unwrap());
                    }
                    stack.push(token);
                }
                Token::Bracket('(') => stack.push(token),
                Token::Bracket(')') => {
                    while !stack.is_empty() && stack.last().unwrap() != &Token::Bracket('(') {
                        queue.push(stack.pop().unwrap());
                    }
                    stack.pop();
                }
                _ => {}
            }
        }
        while !stack.is_empty() {
            queue.push(stack.pop().unwrap());
        }
        queue
    }

    pub fn evaluate(mut tokens: Vec<Token>) -> Option<Result<u32, Error>> {
        tokens.reverse();
        let mut stack: Vec<u32> = Vec::new();
        while let Some(token) = tokens.pop() {
            match token {
                Token::Number(n) => stack.push(n),
                Token::Op(Operator::Add) => {
                    let r = stack.pop()?;
                    let l = stack.pop()?;
                    stack.push(l + r);
                }
                Token::Op(Operator::Multiply) => {
                    let r = stack.pop()?;
                    let l = stack.pop()?;
                    stack.push(l * r);
                }
                Token::Op(Operator::Subtract) => {
                    let r = stack.pop()?;
                    let l = stack.pop()?;
                    stack.push(l - r);
                }
                Token::Op(Operator::Divide) => {
                    let r = stack.pop()?;
                    let l = stack.pop()?;
                    if r == 0 {
                        return Some(Err(Error::InvalidDivisionByZero));
                    }
                    stack.push(l / r);
                }
                _ => {}
            }
        }
        if stack.len() > 1 {
            return None;
        }
        Some(Ok(stack.pop().unwrap()))
    }
}
