use core::fmt;
use std::fmt::write;


#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TokenType {
  Plus,
  LeftParen,
  RightParen,
  String,
  Comma,
  Number,
  Identifier,

  TERMINATE,
}

#[derive(Clone, Debug)]
pub enum Literal {
  // Identifier(String),
  Number(f64)
}

#[derive(Clone)]
pub struct Token {
  pub token_type: TokenType,
  pub literal: Option<Literal>,
  pub lex: String,
  pub line: usize,
}


impl fmt::Debug for Token {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self.token_type {
        TokenType::Number => write!(f, "Token {:?} {:?}", self.token_type, self.literal.as_ref().unwrap()),
        _ => write!(f, "Token {:?}", self.token_type),
    }
  }
}
