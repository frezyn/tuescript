
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TokenType {
  Plus,
  TERMINATE,
}

#[derive(Clone, Debug)]
pub enum Literal {
  // Identifier(String),
  Str(String),
  Number(f64),
  True,
  False,
  Nil,
}
#[derive(Clone, Debug)] // Add this line

pub struct Token {
  pub token_type: TokenType,
  pub literal: Option<Literal>,
  pub lex: String,
  pub line: usize,
}


