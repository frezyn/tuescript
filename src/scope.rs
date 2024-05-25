use::std::collections::HashMap;

use crate::interpreter::{Symbol, Value};

#[derive(Clone, Debug)]

pub struct Scope {
  pub values: HashMap<String, Value>,
  pub enclosing: Option<Box<Scope>>
}

impl Scope {
  pub fn new(enclosing: Option<Box<Scope>>) -> Scope {
    Scope {
      values: HashMap::new(),
      enclosing
    }
  }

  pub fn load(&mut self, load: HashMap<String, Value>) {
    self.values.extend(load)
  }

  pub(crate) fn get_at(&self, sym: Symbol, d: usize) -> Value {
    if let Some(val) = self.ancestor(d).values.get(&sym.name){
      return val.clone();
    } else {
      panic!("erro aso pegar valor em lex scope!")
    }
  }

  pub fn ancestor(&self, dist : usize) -> Scope{
    let mut ret = self.clone();
    for _ in 0..dist{
        if let Some(e) = ret.enclosing.clone(){
        ret = (*e).clone();
        }else{
            panic!("Enclosing scope not found");
        }
    }
    return ret.clone();
}

}

