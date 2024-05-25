use std::collections::HashMap;

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
  Null
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
          print!("{:?}", a[0].clone());
          return Ok(Value::Null)
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
  
//   pub fn inter(&mut self, smts: Vec<Statement>) {
//     for s in smts {
//       match self. {
          
//       }
//   }
// }

// pub fn inter_statment(&mut self, stms: Statement) -> Result<(), String> {
// //   match stms {
// //       // Statement::Expression(ex) => {
// //       //   // let _ =  self.inter_expression(ex).unwrap();
// //       //   // Ok(())
// //       // }
// //   }
// // }
// }



}

