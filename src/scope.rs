use::std::collections::HashMap;

use crate::interpreter::Value;

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

}

