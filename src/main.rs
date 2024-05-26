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


fn main() {
        let args = Args::parse();
        if let Some(a) = args.path {
                read_file(a)
        } else {
                panic!("forneça o endereço de um arquivo valido.")
        }
}

fn read_file(path: PathBuf) {
        let s = fs::read_to_string(path).expect("Erro ao ler arquivo");
        run(s)
}

fn run(src: String) {
        let mut scann = Lexer::new(&src);
        scann.scan_tokens();
        let mut p = parse::new(scann.tokens);
        let smts = p.parse().expect("erro ao parsear para estrutura sintatica");
        let mut interpreter = Interpreter::new(HashMap::new()); 
        interpreter.inter(smts)
}
