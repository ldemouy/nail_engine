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
    name: String,
    read: crossbeam::channel::Receiver<Option<T>>,
    write: crossbeam::channel::Sender<T>,
}

impl<T: Clone + Send + Sync> Listener<T> {
    pub fn new(
        read: crossbeam::channel::Receiver<Option<T>>,
        write: crossbeam::channel::Sender<T>,
    ) -> Listener<T> {
        use guid_create::GUID;
        Listener {
            name: GUID::rand().to_string(),
            read,
            write,
        }
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_receiver(&self) -> &crossbeam::channel::Receiver<Option<T>> {
        &self.read
    }

    pub fn get_sender(&self) -> &crossbeam::channel::Sender<T> {
        &self.write
    }
}
#[derive(Debug, Clone)]
pub enum EngineMessage<T> {
    Register(
        crossbeam::channel::Receiver<Option<T>>,
        crossbeam::channel::Sender<T>,
    ),
    DeRegister(String),
    None,
}

#[derive(Debug, Clone)]
pub struct Engine<T: Clone + Send + Sync + Into<EngineMessage<T>>> {
    pub listeners: Vec<Listener<T>>,
}

impl<T: Clone + Send + Sync + Into<EngineMessage<T>>> Engine<T> {
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
                        match message.clone().into() {
                            EngineMessage::Register(read, write) => {
                                unimplemented!()
                            }
                            EngineMessage::DeRegister(guid) => {
                                unimplemented!()
                            }
                            EngineMessage::None => {}
                        }

                        messages.push(message);
                    }
                }

                self.send(&messages);
            }
        });
    }
}
