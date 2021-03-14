use crossbeam::channel::{Receiver, Sender};
use nail_common::Message;
use std::thread;
pub trait Module {
    fn start(&self, core_write: Sender<Message>) -> Sender<Message>;
}

pub struct EchoModule {}

impl Module for EchoModule {
    fn start(&self, _: Sender<Message>) -> Sender<Message> {
        let (thread_write, thread_read): (Sender<Message>, Receiver<Message>) =
            crossbeam::channel::unbounded();
        thread::spawn(move || loop {
            let message = thread_read.recv().unwrap();
            println!("{:?}", message);
            if message.action.variant == "foo" {
                println!("BAR!");
            }
            println!();
        });
        thread_write
    }
}
