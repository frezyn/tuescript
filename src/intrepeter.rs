

pub struct Interpreter {
  pub program_scope: Scope,
  pub return_val: Option<Value>,
  pub function_map: HashMap<u64, Function>,
  pub f_count: u64,
  pub global: Scope,
  pub lex_scope: HashMap<u64, usize>
}

impl Interpreter {
  
}