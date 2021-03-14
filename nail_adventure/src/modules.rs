use crossbeam::channel::{Receiver, Sender};
use nail_common::Message;
use std::thread;
pub trait Module {
    fn start(&self, core_write: Sender<Message>) -> Sender<Message>;
}

#[derive(Debug, Default)]
pub struct EchoModule {}

impl EchoModule {
    pub fn new() -> EchoModule {
        EchoModule {}
    }
}

impl Module for EchoModule {
    fn start(&self, core_write: Sender<Message>) -> Sender<Message> {
        let (thread_write, thread_read): (Sender<Message>, Receiver<Message>) =
            crossbeam::channel::unbounded();
        thread::spawn(move || loop {
            let message = thread_read.recv().unwrap();
            println!("{:?}", message);
            if message.action.variant == "foo" {
                core_write
                    .send(Message {
                        action: nail_common::Token {
                            variant: "BAR!".to_string(),
                            contents: "".to_string(),
                        },
                        parameters: vec![],
                    })
                    .unwrap();
            } else if message.action.variant == "BAR!" {
                println!("BAR!");
            }
            println!();
        });
        thread_write
    }
}
