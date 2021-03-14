use adventure_lib::modules::*;
use crossbeam::channel::{Receiver, Sender};
use nail_common::Message;
use nail_core::engine::Listener;
use nail_lexer::Lexer;
use std::io;

fn main() {
    println!("Input: ");
    let (write, read) = crossbeam::channel::unbounded();
    let modules = initialize_modules();
    let listeners = wire_modules_to_core(&modules, read, write);
    let lexer = load_lexer().unwrap();
    let core = nail_core::engine::Engine::new(&listeners);
    loop {
        if let Some(message) = read_message(&lexer) {
            core.tick(&[message]);
        }
    }
}

fn initialize_modules() -> Vec<Box<dyn Module>> {
    vec![Box::new(EchoModule::new())]
}

fn wire_modules_to_core(
    modules: &[Box<dyn Module>],
    read: Receiver<Option<Message>>,
    write: Sender<Option<Message>>,
) -> Vec<Listener> {
    let mut result: Vec<Listener> = vec![];
    for module in modules {
        let thread_write = module.start(write.clone());
        let listener = Listener::new(read.clone(), thread_write);
        result.push(listener);
    }
    result
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
