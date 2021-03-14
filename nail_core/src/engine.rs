use rayon::prelude::*;
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
pub struct Listener<T: Clone + Send + Sync> {
    read: crossbeam::channel::Receiver<Option<T>>,
    write: crossbeam::channel::Sender<T>,
}

impl<T: Clone + Send + Sync> Listener<T> {
    pub fn new(
        read: crossbeam::channel::Receiver<Option<T>>,
        write: crossbeam::channel::Sender<T>,
    ) -> Listener<T> {
        Listener { read, write }
    }

    pub fn get_receiver(&self) -> &crossbeam::channel::Receiver<Option<T>> {
        &self.read
    }

    pub fn get_sender(&self) -> &crossbeam::channel::Sender<T> {
        &self.write
    }
}

#[derive(Debug, Clone)]
pub struct Engine<T: Clone + Send + Sync> {
    pub listeners: Vec<Listener<T>>,
}

impl<T: Clone + Send + Sync> Engine<T> {
    pub fn new(listeners: &[Listener<T>]) -> Engine<T> {
        Engine {
            listeners: listeners.to_vec(),
        }
    }
    pub fn send(&self, messages: &[T]) {
        self.listeners.par_iter().for_each(|listener| {
            let sender = listener.get_sender();
            messages.par_iter().for_each(|message| {
                sender.send(message.clone()).unwrap();
            });

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

                self.send(&messages);
            }
        });
    }
}
