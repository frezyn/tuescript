
use clap::builder::Str;

use crate::{statement::Statement, token::{Token, TokenType}};
pub struct Parser {
  tokens: Vec<Token>,
  current: usize,
  inloop: bool,
  function_stack: Vec<u8>,
  i_ds: u64
}

impl Parser{ 
  pub fn new(tokens: Vec<Token>) -> Parser {
    Parser {
      tokens,
      current: 0,
      inloop: false,
      function_stack: vec![],
      i_ds: 0,
    }
  }

  pub fn parse(&mut self) -> Result<Vec<Statement>, String> {
    let mut smts = vec![];
  }

  fn end_of_line(&self) -> bool {
    return self.peek().token_type == TokenType::TERMINATE;
  }

  fn peek(&self) -> Token {
    return self.tokens.get(self.current).unwrap().clone();
}

}