use std::io;

use adventure_lib::modules::*;
use nail_lexer::Lexer;

fn main() {
    println!("Input: ");
    let (write, read) = crossbeam::channel::unbounded();
    let module = EchoModule {};
    let thread_write = module.start(write.clone());
    let listener = adventure_lib::Listener {
        read,
        write: thread_write,
    };

    let lexer = load_lexer().unwrap();
    let mut core = nail_core::engine::Engine {
        listeners: vec![Box::new(listener)],
    };
    loop {
        if let Some(message) = read_message(&lexer) {
            core.tick(&[message]);
        }
    }
}

fn load_lexer() -> Result<nail_lexer::Lexer, std::io::Error> {
    use nail_lexer::*;
    use std::{fs::File, path::Path};

    let path = Path::new("data/grammar.json");
    let file = File::open(&path)?;
    let mut reader = std::io::BufReader::new(&file);
    let grammar: Vec<Grammar> = serde_json::from_reader(&mut reader)?;

    Ok(Lexer::new(grammar))
}

fn read_message(lexer: &Lexer) -> Option<nail_common::Message> {
    let mut buffer = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut buffer).unwrap();
    let buffer = buffer.trim();

    if buffer.is_empty() {
        return None;
    }

    let mut tokens = lexer.lex(&buffer);
    let action = tokens.first().unwrap().to_owned();
    tokens.remove(0);
    Some(nail_common::Message {
        action,
        parameters: tokens,
    })
}
