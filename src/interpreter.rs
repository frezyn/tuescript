use std::{
  collections::HashMap,
  fmt::{Debug, Display},
};
use crate::scope::Scope;
use crate::token::{Token, TokenType};

#[derive(Clone, Debug)]
pub struct Interpreter {
  pub program_scope: Scope,
  pub return_val: Option<Value>,  
  pub global: Scope,
  pub lex_scope: HashMap<u64, usize>
}

#[derive(Clone, Debug)]
pub struct NativeFunction {
  pub name: String,
  pub arity: usize,
  pub call: fn(&mut Interpreter, &[Value]) -> Result<Value, String>
}

pub enum Expression {
  Binary(Box<Expression>, Token, Box<Expression>),
}

pub enum Statement {
  Expression(Expression)
}


#[derive(Clone, Debug)]
pub enum Value {
  Number(f64),
  NativeFunction(NativeFunction),
  Nill
}

impl Interpreter {
  pub fn new(lex_scope: HashMap<u64, usize>) -> Interpreter {
    let mut mp: HashMap<String, Value> = HashMap::new();
    mp.insert(
      "print".to_string(),
      Value::NativeFunction(NativeFunction {
        name: "print".to_string(),
        arity: 1,
        call: |_, a| {
          print!("{}", a[0].clone());
          return Ok(Value::Nill)
        }
      })
    );

    let scope = Scope::new(None);
    let mut global = Scope::new(None);
    global.load(mp);

    Interpreter {
      program_scope: scope,
      global: global,
      return_val: None,
      lex_scope: lex_scope,
    }
  }
  
  pub fn inter(&mut self, smts: Vec<Statement>) {
    for s in smts {
      match self.interp_statement(s){
          Ok(_) => (),
          Err(e) => panic!("{}", e)
      }
  }
}

fn interp_statement(&mut self, stmt: Statement) -> Result<(), String> {
  match stmt {
    Statement::Expression(exp) => {
      let _ = self.interp_expression(exp)?;
      Ok(())
    }
  }
}

pub fn interp_expression(&mut self, exp: Expression) -> Result<Value, String> {
  match exp {
    Expression::Binary(l, operation, r) => self.interp_binary(l, operation, r),
  }
}

fn interp_binary(
  &mut self,
  l: Box<Expression>,
  o: Token,
  r: Box<Expression>
) -> Result<Value, String> {
  let left = self.interp_expression(*l).expect("valor esquerdo invalido");
  let right = self.interp_expression(*r).expect("valor a direira invalido");

  match (left, o.token_type, right) {
    (Value::Number(l), TokenType::Plus, Value::Number(r)) => Ok(Value::Number(l + r)),
    (_, _, _) => panic!("Operação binaria invalida")
  }
}

}

impl Display for Value {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      match self {
          Value::Number(n) => f.write_fmt(format_args!("{}", n)),
          Value::Nill => f.write_str("Nil"),
          Value::NativeFunction(_) => f.write_str("Native Function"),
      }
  }
}
