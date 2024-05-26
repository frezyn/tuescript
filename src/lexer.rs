use core::panic;
use std::{collections::HashMap, vec};


use crate::token::{Literal, Token, TokenType};
#[derive(Debug)]
pub struct Lexer {
  source: String,
  pub tokens: Vec<Token>,
  start: usize,
  current: usize,
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
        self.scan_token();
    }
    self.tokens.push(Token {
        token_type: TokenType::TERMINATE,
        literal: None,
        lex: "".to_string(),
        line: self.line,
    });
}

    pub fn scan_token(&mut self) {
      let c = self.advance();
      match c {
          '(' => self.add_token(TokenType::LeftParen),
          ')' => self.add_token(TokenType::RightParen),
          '+' => self.add_token(TokenType::Plus),
          '\n' => self.line += 1,
          '"' => self.string(),
          _ => {
            if c.is_digit(10) {
              self.number()
            } else if c.is_alphabetic() || c == '_' {
              self.indentifier();
            }
            else {
              panic!("caractere nao suportado! caractere: ({}) na linha: {}", c, self.line)
            }
          }
      }
    }

    fn indentifier(&mut self) {
      while self.peek().is_alphabetic() || self.peek() == '_' {
        self.advance();
      }
      let a = &self.source[self.start..self.current];
      let t = self.keywords.get(a);
      if let Some(istype) = t {
        self.add_token(*istype);
      }else {
        self.add_token(TokenType::Identifier)
      }
    }

    pub fn advance(&mut self) -> char {
      self.current += 1;
      return self.source.chars().nth(self.current - 1).unwrap();
    }

    fn string(&mut self) {
      while self.peek() != '"' && !self.is_the_end() {
        if self.peek() == '\n' {
          self.line += 1
        };
        self.advance();
      }
      if self.is_the_end() {
        return;
    }
    self.advance();
    let value: String = self.source[self.start + 1..self.current -1].to_string();
    self.add_literal_token(TokenType::String, Some(Literal::Str(value)))
  }
    pub fn add_token(&mut self, token_type: TokenType) {
      self.add_literal_token(token_type, None)
    }

    fn add_literal_token(&mut self, token_type: TokenType, literal: Option<Literal>) {
      let lex = &self.source[self.start..self.current];
      self.tokens.push(Token {
        token_type,
        literal,
        lex: lex.to_string(),
        line: self.line
      })
    }

    fn number(&mut self) {
      while self.peek().is_digit(10) {
        self.advance();
      }
      if self.peek() == '.' && self.peek_next().is_digit(10) {
        self.advance();

        while self.peek().is_digit(10) {
            self.advance();
        }
    }

    self.add_literal_token(TokenType::Number, Some(Literal::Number(
      self.source[self.start..self.current].parse::<f64>().unwrap()
    )))
    }

    pub fn peek(&self) -> char {
      if self.is_the_end() {
        return '\0'
      }
      return self.source.chars().nth(self.current).unwrap();
    }

    fn peek_next(&self) -> char {
      if self.current + 1 >= self.source.len() {
          return '\0';
      }
      return self.source.chars().nth(self.current + 1).unwrap();
  }

}