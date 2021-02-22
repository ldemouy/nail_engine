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
            let sender = &mut listener.get_sender();
            for message in messages.iter() {
                sender.send(message.clone()).unwrap();
            }
        }
    }
}
