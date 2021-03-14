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
    pub fn tick(&self, messages: &[nail_common::Message]) {
        for listener in self.listeners.iter() {
            let sender = listener.get_sender();
            for message in messages.iter() {
                sender.send(Some(message.clone())).unwrap();
            }
            let reader = listener.get_receiver();

            //we need to block to read first message
            if let Some(message) = reader.recv().unwrap() {
                let mut messages = vec![];
                messages.push(message);
                while !reader.is_empty() {
                    if let Some(message) = reader.recv().unwrap() {
                        messages.push(message);
                    }
                }
                self.tick(&messages);
            }
        }
    }
}
