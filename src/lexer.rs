use std::{collections::HashMap, vec};


use crate::token::{TokenType, Token};

#[derive(Debug)] // Add this line
pub struct Lexer {
  start: usize,
  current: usize,
  pub tokens: Vec<Token>,
  source: String,
  line: usize,
  keywords: HashMap<String, TokenType>,
}

impl Lexer {
  pub fn new(src: &String) -> Lexer {
    let mut _keywords = HashMap::new();
    Lexer {
      source: src.to_string(),
      tokens: vec![],
      start: 0,
      current: 0,
      line: 1,
      keywords: _keywords

    }
  }

  fn is_the_end(&self) -> bool {
    self.current > self.source.len() - 1
  }

  pub fn scan_tokens(&mut self) {
    while !self.is_the_end() {
        self.start = self.current;
        self.scan_tokens();
    }
    self.tokens.push(Token {
        token_type: TokenType::TERMINATE,
        literal: None,
        lex: "".to_string(),
        line: self.line,
    });
}
}