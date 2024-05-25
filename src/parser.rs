use crate::{interpreter::{Expression, Statement, Symbol}, token::{Token, TokenType}};



#[derive(Debug)]
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
    let mut smts: Vec<Statement> = vec![];
    while !self.end_of_file() {
      smts.push(self.expression_statment().expect("Statment nao valido para parser"));
    } 
    return Ok(smts)
  }


  fn expression_statment(&mut self) -> Result<Statement, String> {
    let ex = self.term();

    return Ok(Statement::Expression(ex)) 
  }

  fn term(&mut self) -> Expression {  
    let mut expr = self.call();
    while self.matcher(TokenType::Plus) {
      let oprator = self.previus();
      let right = self.call();
      expr = Expression::Binary(Box::new(expr), oprator, Box::new(right));
    }
    return expr;
  }


  fn call(&mut self) -> Expression {
    let mut expr = self.primary();
    loop {
      if self.matcher(TokenType::LeftParen) {
        expr = self.finish_call(expr)
      } else {
        break;
      }
    }

    expr
  }

  fn finish_call(&mut self, callee: Expression) -> Expression {
    let mut args: Vec<Expression> = vec![];
    if !self.check(TokenType::RightParen) {
      loop {
        let a = self.expression();
        args.push(a);
        if !self.matcher(TokenType::Comma) {
          break;
        }
      }
    }
    let token = self.consume(TokenType::RightParen).expect(") e esperado no final de argumentos");
    return Expression::Call(Box::new(callee), token, args);
  }

  fn primary(&mut self) -> Expression {
    if  self.matcher(TokenType::Number) || self.matcher(TokenType::String) {
      return Expression::Literal(self.previus().literal.unwrap());
    }
    if self.matcher(TokenType::LeftParen) {
      let expr = self.expression();
      self.consume(TokenType::RightParen).expect("E esperado um ) depois da expeção");
      return Expression::Grouping(Box::new(expr));
    }
    if self.matcher(TokenType::Identifier) {
      return Expression::Primary(Symbol {
          name: self.previous().lex,
          s_id: self.alloc_sid(),
      });
  }
    else {
      panic!(
        "linha: {}: Token nao esperado: {:?}",
        self.tokens[self.current].line,
        self.peek().token_type
    );
    }
  }


  fn alloc_sid(&mut self) -> u64 {
    self.s_id = self.s_id + 1;
    self.s_id
}

  fn expression(&mut self) -> Expression {
    return self.binary();
}

fn binary(&mut self) -> Expression {
  self.term()
}


fn previous(&self) -> Token {
  return self.tokens.get(self.current - 1).unwrap().clone();
}

fn consume(&mut self, t: TokenType) -> Result<Token, String> {
  if self.check(t) {
    return Ok(self.advance());
  }
  Err(format!("Parser erro!"))
}
  
  fn previus(&self) -> Token {
    return self.tokens.get(self.current - 1).unwrap().clone();
  }

  

  fn matcher(&mut self, t: TokenType) -> bool {
    if self.check(t) {
      self.advance();
      return true;
    } else {
      return false 
    }
  }

  fn check(&self, t: TokenType) -> bool {
    if self.end_of_file() {
      return false
    }
    return self.peek().token_type == t
  }
  fn advance(&mut self) -> Token {
    if !self.end_of_file() {
        self.current += 1;
    }
    return self.previus();
}

  
  fn end_of_file(&self) -> bool {
    return self.peek().token_type == TokenType::TERMINATE
  }

  fn peek(&self) -> Token {
    return self.tokens.get(self.current).unwrap().clone();
}

}