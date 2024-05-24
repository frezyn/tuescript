

pub struct Parser {
  tokens: Vec<Token>,
  current: usize,
  inloop: bool,
  function_stack: Vec<u8>,
  i_ds: u64
}

impl Parser{ 
  pub fn new(tokens: Vec<Token>) -> Parser {
    Parser {
      tokens,
      current: 0,
      inloop: false,
      function_stack: vec![],
      i_ds: 0,
    }
  }


}