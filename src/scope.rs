use::std::collections::HashMap;

#[derive(Clone)]
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

  

}

