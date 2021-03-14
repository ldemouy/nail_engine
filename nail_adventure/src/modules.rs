use crossbeam::channel::{Receiver, Sender};
use nail_common::Message;
use std::thread;
pub trait Module {
    fn start(&self, core_write: Sender<Option<Message>>) -> Sender<Option<Message>>;
}

#[derive(Debug, Default)]
pub struct EchoModule {}

impl EchoModule {
    pub fn new() -> EchoModule {
        EchoModule {}
    }
}

impl Module for EchoModule {
    fn start(&self, core_write: Sender<Option<Message>>) -> Sender<Option<Message>> {
        let (thread_write, thread_read): (Sender<Option<Message>>, Receiver<Option<Message>>) =
            crossbeam::channel::unbounded();
        thread::spawn(move || loop {
            if let Some(message) = thread_read.recv().unwrap() {
                println!("{:?}", message);
                if message.action.variant == "foo" {
                    core_write
                        .send(Some(Message {
                            action: nail_common::Token {
                                variant: "BAR!".to_string(),
                                contents: "".to_string(),
                            },
                            parameters: vec![],
                        }))
                        .unwrap();
                } else if message.action.variant == "BAR!" {
                    println!("BAR!");
                    core_write.send(None).unwrap();
                } else {
                    core_write.send(None).unwrap();
                }
                println!();
            }
        });
        thread_write
    }
}
