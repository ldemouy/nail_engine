use super::Message;
use crossbeam::channel::{Receiver, Sender};
use std::io;

#[derive(Debug, Default)]
pub struct StdinModule {}

impl StdinModule {
    pub fn new() -> StdinModule {
        StdinModule {}
    }

    fn read_message() -> Option<String> {
        let mut buffer = String::new();
        let stdin = io::stdin();
        stdin.read_line(&mut buffer).unwrap();
        let buffer = buffer.trim();

        if buffer.is_empty() {
            return None;
        }
        Some(buffer.to_string())
    }
}

impl super::Module for StdinModule {
    fn start(&self, core_write: Sender<Option<Message>>) -> Sender<Message> {
        let (thread_write, thread_read): (Sender<Message>, Receiver<Message>) =
            crossbeam::channel::unbounded();
        rayon::spawn(move || loop {
            if let Ok(message) = thread_read.recv() {
                if let Message::Initialize = message {
                    println!("Input: ");
                    loop {
                        if let Some(message) = StdinModule::read_message() {
                            core_write.send(Some(Message::RawInput(message))).unwrap();
                        }
                    }
                } else {
                    core_write.send(None).unwrap();
                }
            }
        });
        thread_write
    }
}
