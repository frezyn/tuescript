use std::{fs, path::PathBuf};

mod lexer;
mod token;
mod parser;
mod statement;

use clap::Parser;
use lexer::Lexer;

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
        let s = fs::read_to_string(path).expect("arquivo nao encontrado");
        run(s)
}

fn run(src: String) {
        let mut scann = Lexer::new(&src);
        scann.scan_tokens();
        println!("{:?}", scann)


}
