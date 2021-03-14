use crossbeam::thread;

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

#[derive(Debug, Clone)]
pub struct Listener {
    read: crossbeam::channel::Receiver<Option<nail_common::Message>>,
    write: crossbeam::channel::Sender<Option<nail_common::Message>>,
}

impl Listener {
    pub fn new(
        read: crossbeam::channel::Receiver<Option<nail_common::Message>>,
        write: crossbeam::channel::Sender<Option<nail_common::Message>>,
    ) -> Listener {
        Listener { read, write }
    }

    pub fn get_receiver(&self) -> &crossbeam::channel::Receiver<Option<nail_common::Message>> {
        &self.read
    }

    pub fn get_sender(&self) -> &crossbeam::channel::Sender<Option<nail_common::Message>> {
        &self.write
    }
}

#[derive(Debug, Clone)]
pub struct Engine {
    pub listeners: Vec<Listener>,
}

impl Engine {
    pub fn new(listeners: &[Listener]) -> Engine {
        Engine {
            listeners: listeners.to_vec(),
        }
    }
    pub fn tick(&self, messages: &[nail_common::Message]) {
        for listener in self.listeners.iter() {
            let sender = listener.get_sender();
            for message in messages.iter() {
                sender.send(Some(message.clone())).unwrap();
            }
            let reader = listener.get_receiver();
            thread::scope(move |_| {
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
            })
            .unwrap();
        }
    }
}
