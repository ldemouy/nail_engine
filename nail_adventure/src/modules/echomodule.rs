use crossbeam::channel::{Receiver, Sender};
use nail_common::Message;
use std::thread;

#[derive(Debug, Default)]
pub struct EchoModule {}

impl EchoModule {
    pub fn new() -> EchoModule {
        EchoModule {}
    }
}

impl super::Module for EchoModule {
    fn start(&self, core_write: Sender<Option<Message>>) -> Sender<Message> {
        let (thread_write, thread_read): (Sender<Message>, Receiver<Message>) =
            crossbeam::channel::unbounded();
        thread::spawn(move || loop {
            if let Ok(message) = thread_read.recv() {
                println!("{:?}", message);
                core_write.send(None).unwrap();
                println!();
            }
        });
        thread_write
    }
}
