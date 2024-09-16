use std::{
  collections::HashMap,
  fmt::{Debug, Display},
};
use crate::{scope::Scope, token::Literal};
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
  pub callable: fn(&mut Interpreter, &[Value]) -> Result<Value, String>
}

#[derive(Clone,Hash, PartialEq, Eq, Debug)]
pub struct Symbol {
  pub name: String,
  pub s_id: u64,
}

#[derive(Clone, Debug)]
pub enum Expression {
  Binary(Box<Expression>, Token, Box<Expression>),
  Grouping(Box<Expression>),
  Literal(Literal),
  Primary(Symbol),                               
  Call(Box<Expression>, Token, Vec<Expression>), 
}
#[derive(Debug)]
pub enum Statement {
  Expression(Expression)
}

pub trait Callable {
  fn arity(&self) -> usize;
  fn call(&mut self, interpreter: &mut Interpreter, args: &[Value]) -> Result<Value, String>;
}


#[derive(Clone, Debug)]
pub enum Value {
  Number(f64),
  String(String),
  NativeFunction(NativeFunction),
  Nill,
}

impl Interpreter {
  pub fn new(lex_scope: HashMap<u64, usize>) -> Interpreter {
    let mut mp: HashMap<String, Value> = HashMap::new();
    mp.insert(
      "println".to_string(),
      Value::NativeFunction(NativeFunction {
          name: "println".to_string(),
          arity: 1,
          callable: |_, args| {
              println!("{}", args[0].clone());
              return Ok(Value::Nill);
          },
      }),
  );

    let scope = Scope::new(None);
    let mut global = Scope::new(None);
    global.load(mp);

    Interpreter {
      program_scope: scope,
      global,
      return_val: None,
      lex_scope,
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
    Expression::Primary(v) => self.interp_variable(v),
    Expression::Literal(a) => self.interp_literal(a),
    Expression::Grouping(ex) => self.interp_expression(*ex),
    Expression::Binary(l, operation, r) => self.interp_binary(l, operation, r),
    Expression::Call(call,t , args ) => self.inter_call(call, t, args)
  }
}


fn interp_variable(&self, v: Symbol) -> Result<Value, String> {

     self.look_up(v)
 }
pub fn interp_literal(&self, expr: Literal) -> Result<Value, String> {
  match expr {
      Literal::Str(s) => return Ok(Value::String(s)),
      Literal::Number(n) => return Ok(Value::Number(n)),
  }
}

fn inter_call(
  &mut self,
  call: Box<Expression>,
  t: Token,
  args: Vec<Expression>
) -> Result<Value, String> {
  let arguments: Vec<Value> = args.iter().map(|a| {
    self.interp_expression(a.clone()).expect("Error ao interpretar argumentos de chamada")
  }).collect();
  self.call(call, t, arguments)
}

  fn call(
    &mut self,
    callee_expr: Box<Expression>,
    loc: Token,
    arg: Vec<Value>
  ) -> Result<Value, String> {
    let calle = self.interp_expression(*callee_expr)?;
    let fval: Value;
    match match_call(calle) {
      Some(mut f) => {
        if arg.len() != f.arity() {
          panic!("exesso de arumentos fornecidos!");
        } else {
          fval = f.call(self, &arg)?;
        }
      }
      None => todo!()
    }
    let return_val = self.return_val.clone();
    self.return_val = None;
    match return_val {
        Some(val) => Ok(val.clone()),
        None => Ok(fval)
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
    (Value::String(l), TokenType::Plus, Value::String(r))  => Ok(Value::String(l.as_str().to_string() + r.as_str())),  
    (Value::String(l), TokenType::Plus, Value::Number(r))  => {
      let l_parsed = l.parse::<f64>().map_err(|_| "erro ao parsear valor do tipo esquero")?;

      Ok(Value::Number(l_parsed + r))
    },
    (Value::Number(l), TokenType::Plus, Value::String(r))  => {
      let r_parsed = r.parse::<f64>().map_err(|_| "erro ao parsear valor do tipo esquero")?;

      Ok(Value::Number(l + r_parsed))
    },
    (_, _, _) => panic!("Operação binaria invalida")
  }
}

  pub fn look_up(&self, sym: Symbol) -> Result<Value, String> {
    let distance = self.lex_scope.get(&sym.s_id);
    if let Some(d) = distance {
      return Ok(self.program_scope.get_at(sym, *d))
    }else {
      return Ok(self.global.get_at(sym, 0))
    }
  }

}

fn match_call(val: Value) -> Option<Box<dyn Callable>> {
  match val {
    Value::NativeFunction(f) => Some(Box::new(f)),
    _ => panic!("Erro ao chamar função")
  }
}

impl Display for Value {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      match self {
          Value::String(s) => f.write_fmt(format_args!("{}", s)),
          Value::Number(n) => f.write_fmt(format_args!("{}", n)),
          Value::Nill => f.write_str("nill"),
          Value::NativeFunction(_) => f.write_str("Native function"),
      }
  }
}

impl Callable for NativeFunction {
  fn arity(&self) -> usize {
      self.arity
  }

  fn call(&mut self, interpreter: &mut Interpreter, args: &[Value]) -> Result<Value, String> {
      return (self.callable)(interpreter, args);
  }
}