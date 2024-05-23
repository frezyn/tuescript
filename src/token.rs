
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TokenType {
  TERMINATE,
}

#[derive(Clone, Debug)]
pub enum Literal {
  Str(String),
  Number(f32),
  True,
  False,
  Null
}

pub struct Token {
  pub token_type: TokenType,
  pub literal: Option<Literal>,
  pub lex: String,
  pub line: usize,
}
