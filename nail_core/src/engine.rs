use std::thread;

#[derive(Debug)]
pub enum Event<I>
where
    I: super::traits::Item,
{
    Use(I),
    UseOn(I, I),
    Move(String),
    Examine(String),
    Take(String),
    Drop(String),
}

pub struct Engine {
    pub listeners: Vec<Box<dyn super::traits::Listener>>,
}

impl Engine {
    pub fn tick(&mut self, messages: &[nail_common::Message]) {
        for listener in self.listeners.iter() {
            let sender = listener.get_sender();
            for message in messages.iter() {
                sender.send(message.clone()).unwrap();
            }
            let reader = listener.get_receiver().clone();
            let sender_clone = sender.clone();
            thread::spawn(move || {
                while !reader.is_empty() {
                    let message = reader.recv().unwrap();
                    sender_clone.send(message).unwrap();
                }
            });
        }
    }
}
