use super::Message;
use crossbeam::channel::{Receiver, Sender};

#[derive(Debug, Default)]
pub struct FooModule {}

impl FooModule {
    pub fn new() -> FooModule {
        FooModule {}
    }
}

impl super::Module for FooModule {
    fn start(&self, core_write: Sender<Option<Message>>) -> Sender<Message> {
        let (thread_write, thread_read): (Sender<Message>, Receiver<Message>) =
            crossbeam::channel::unbounded();
        rayon::spawn(move || loop {
            if let Ok(message) = thread_read.recv() {
                if let Message::TokenMessage {
                    action,
                    parameters: _,
                } = message
                {
                    if action.variant == "foo" {
                        core_write
                            .send(Some(Message::TokenMessage {
                                action: nail_common::Token {
                                    variant: "BAR!".to_string(),
                                    contents: "".to_string(),
                                },
                                parameters: vec![],
                            }))
                            .unwrap();
                    } else if action.variant == "BAR!" {
                        println!("BAR!");
                        core_write.send(None).unwrap();
                    }
                } else {
                    core_write.send(None).unwrap();
                }
            }
        });
        thread_write
    }
}
