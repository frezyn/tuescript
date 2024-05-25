
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TokenType {
  Plus,
  LeftParen,
  RightParen,
  Number,
  Identifier,

  TERMINATE,
}

#[derive(Clone)]
pub enum Literal {
  // Identifier(String),
  Number(f64)
}

pub struct Token {
  pub token_type: TokenType,
  pub literal: Option<Literal>,
  pub lex: String,
  pub line: usize,
}


