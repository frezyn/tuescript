use crate::{interpreter::Statement, token::Token};




pub struct Parser {
  tokens: Vec<Token>,
  current: usize,
  inloop: bool,
  s_id: u64,
}

impl Parser {
  pub fn new(tokens: Vec<Token>) -> Parser {
    Parser {
      tokens,
      current: 0, 
      inloop: false,
      s_id: 0,
    }
  }

  pub fn parse(&mut self) -> Result<Vec<Statement>, String> {
    let smts = vec![];
    return Ok(smts)
  }
}