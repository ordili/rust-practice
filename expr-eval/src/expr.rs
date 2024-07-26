use std::{fmt::Display, iter::Peekable};
use crate::tokenizer::{Token, Tokenizer};

// 自定义 Result 类型
pub type Result<T> = std::result::Result<T, ExprError>;

// 自定义错误类型
#[derive(Debug)]
pub enum ExprError {
    Parse(String),
}

impl std::error::Error for ExprError {}

impl Display for ExprError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Parse(s) => write!(f, "{}", s),
        }
    }
}


pub struct Expr<'a> {
    iter: Peekable<Tokenizer<'a>>,
}

impl<'a> Expr<'a> {
    pub fn new(src: &'a str) -> Self {
        Self {
            iter: Tokenizer::new(src).peekable(),
        }
    }

    // 计算表达式，获取结果
    pub fn eval(&mut self) -> Result<i32> {
        let result = self.compute_expr().unwrap();
        // 如果还有 Token 没有处理，说明表达式存在错误
        if self.iter.peek().is_some() {
            return Err(ExprError::Parse("Unexpected end of expr".into()));
        }
        Ok(result)
    }

    fn compute_expr(&mut self) -> Option<i32> {
        let mut op_stack = Vec::new();
        let mut num_stack = Vec::new();
        while let Some(token) = self.iter.next() {
            match token {
                Token::Number(num) => num_stack.push(num),
                Token::LeftParen => {
                    let ret = self.compute_expr().unwrap();
                    num_stack.push(ret);
                }
                Token::RightParen => break,
                _ => {
                    calculate(&mut op_stack, &mut num_stack, token.precedence());
                    op_stack.push(token)
                }
            }
        }

        while let Some(op) = op_stack.pop() {
            let b = num_stack.pop().unwrap();
            let a = num_stack.pop().unwrap();
            num_stack.push(op.compute(a, b).unwrap());
        }
        Some(num_stack.pop().unwrap())
    }
}

fn calculate(op_stack: &mut Vec<Token>, num_stack: &mut Vec<i32>, precedence: i32) {
    while let Some(op) = op_stack.last() {
        if op.precedence() < precedence {
            break;
        }
        let b = num_stack.pop().unwrap();
        let a = num_stack.pop().unwrap();
        num_stack.push(op.compute(a, b).unwrap());
        op_stack.pop();
    }
}

#[cfg(test)]
mod tests {
    use crate::expr::Expr;
    #[test]
    fn test_expr() {
        let src = "92 + 5 + 5 * 27 - (92 - 12) / 4 + 26";
        let mut expr = Expr::new(src);
        let result = expr.eval().unwrap();
        assert_eq!(238, result);
    }
}