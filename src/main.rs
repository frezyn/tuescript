use std::{collections::HashMap, fs, path::PathBuf};

mod lexer;
mod token;
mod parser;
mod scope;
mod interpreter;

use clap::Parser;
use lexer::Lexer;
use parser::Parser as parse;
use interpreter::Interpreter;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    path: Option<PathBuf>,
}


pub struct Resolver{
        scope_stack : Vec<HashMap<String, bool>>,
        pub lex_scope : HashMap<u64, usize>,
    }

fn main() {
        let args = Args::parse();
        if let Some(a) = args.path {
                read_file(a)
        } else {
                panic!("forneça o endereço de um arquivo valido.")
        }
}

fn read_file(path: PathBuf) {
        let s = fs::read_to_string(path).expect("arquivo nao encontrado");
        run(s)
}

fn run(src: String) {
        let mut scann = Lexer::new(&src);
        scann.scan_tokens();
        let mut p = parse::new(scann.tokens);
        let smts = p.parse().expect("erro ao parsear estrutura sintatica");
        let mut interpreter = Interpreter::new(HashMap::new()); 
        interpreter.inter(smts)
        // lexer: tokens: [Token Identifier, Token LeftParen, Token Number(Number(2.0)), Token Plus, Token Number(Number(2.0)), Token RightParen, Token TERMINATE], start: 11, current: 12, line: 1, keywords: {"super": Super, "this": This, "or": Or, "let": Let, "nil": Nil, "else": Else, "while": While, "for": For, "return": Return, "struct": Struct, "false": False, "fun": Fun, "class": Class, "break": Break, "continue": Continue, "true": True, "and": And, "loop": Loop, "if": If} } 
        // Parser { tokens: [Token Identifier, Token LeftParen, Token Number(Number(2.0)), Token Plus, Token Number(Number(2.0)), Token RightParen, Token TERMINATE], current: 0, inloop: false, function_stack: [], s_id: 0 }
        // [Expression(Call(Primary(Symbol { name: "println" }), Token RightParen, [Binary(Literal(Number(2.0)), Token Plus, Literal(Number(2.0)))]))]
}
