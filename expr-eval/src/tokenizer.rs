use std::iter::Peekable;
use std::str::Chars;

#[derive(Copy, Clone, Debug,PartialEq)]
pub enum Token {
    Number(i32),
    Plus,       // 加
    Minus,      // 减
    Multiply,   // 乘
    Divide,     // 除
    LeftParen,  // 左括号
    RightParen, // 右括号
}

impl Token {
    // 获取运算符的优先级
    pub fn precedence(&self) -> i32 {
        match self {
            Token::Plus | Token::Minus => 1,
            Token::Multiply | Token::Divide => 2,
            _ => 0,
        }
    }

    // 根据当前运算符进行计算
    pub fn compute(&self, l: i32, r: i32) -> Option<i32> {
        match self {
            Token::Plus => Some(l + r),
            Token::Minus => Some(l - r),
            Token::Multiply => Some(l * r),
            Token::Divide => Some(l / r),
            _ => None,
        }
    }
}

pub struct Tokenizer<'a> {
    iter: Peekable<Chars<'a>>,
}

impl<'a> Tokenizer<'a> {
    pub fn new(str: &'a str) -> Self {
        Self { iter: str.chars().peekable() }
    }

    pub fn consumer_whitespace(&mut self) {
        while let Some(x) = self.iter.peek() {
            if x.is_whitespace() {
                self.iter.next();
            } else {
                break;
            }
        }
    }

    pub fn consumer_number(&mut self) -> Option<Token> {
        let mut num_str = String::new();
        while let Some(c) = self.iter.peek() {
            if c.is_numeric() {
                num_str.push(*c);
                self.iter.next();
            } else {
                break;
            }
        }
        match num_str.parse() {
            Ok(num) => Some(Token::Number(num)),
            Err(_) => None,
        }
    }

    pub fn consumer_operator(&mut self) -> Option<Token> {
        match self.iter.next() {
            Some('+') => Some(Token::Plus),
            Some('-') => Some(Token::Minus),
            Some('*') => Some(Token::Multiply),
            Some('/') => Some(Token::Divide),
            Some('(') => Some(Token::LeftParen),
            Some(')') => Some(Token::RightParen),
            _ => None,
        }
    }
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.consumer_whitespace();
        match self.iter.peek() {
            Some(x) if x.is_numeric() => self.consumer_number(),
            Some(_) => self.consumer_operator(),
            _ => None,
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::tokenizer::{Token, Tokenizer};

    #[test]
    pub fn test_tokenizer() {
        let str = " 3 + 4 + ( 5) ".to_string();
        let mut tokenizer = Tokenizer::new(&str);

        assert_eq!(Token::Number(3), tokenizer.next().unwrap());
        assert_eq!(Token::Plus, tokenizer.next().unwrap());
        assert_eq!(Token::Number(4), tokenizer.next().unwrap());
        assert_eq!(Token::Plus, tokenizer.next().unwrap());
        assert_eq!(Token::LeftParen, tokenizer.next().unwrap());
        assert_eq!(Token::Number(5), tokenizer.next().unwrap());
        assert_eq!(Token::RightParen, tokenizer.next().unwrap());
    }
}