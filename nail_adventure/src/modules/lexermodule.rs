use crossbeam::channel::{Receiver, Sender};
use nail_common::Message;
use nail_lexer::Lexer;
use std::thread;

#[derive(Debug)]
pub struct LexerModule {
    lexer: Lexer,
}

impl LexerModule {
    pub fn new() -> LexerModule {
        let lexer = LexerModule::load_lexer().unwrap();
        LexerModule { lexer }
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
}

impl super::Module for LexerModule {
    fn start(&self, core_write: Sender<Option<Message>>) -> Sender<Message> {
        let (thread_write, thread_read): (Sender<Message>, Receiver<Message>) =
            crossbeam::channel::unbounded();
        let lexer = self.lexer.clone();
        thread::spawn(move || loop {
            if let Ok(message) = thread_read.recv() {
                if let Message::RawInput(message) = message {
                    let mut tokens = lexer.lex(&message);
                    let action = tokens.first().unwrap().to_owned();
                    tokens.remove(0);
                    let response = Some(nail_common::Message::TokenMessage {
                        action,
                        parameters: tokens,
                    });
                    core_write.send(response).unwrap();
                } else {
                    core_write.send(None).unwrap();
                }
            }
        });

        thread_write
    }
}
