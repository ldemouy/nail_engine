use adventure_lib::modules::*;
use crossbeam::channel::{Receiver, Sender};
use nail_common::Message;
use nail_core::engine::Listener;
use std::thread;

fn main() {
    thread::spawn(move || {
        let (write, read) = crossbeam::channel::unbounded();
        let modules = initialize_modules();
        let listeners = wire_modules_to_core(&modules, read, write);
        let core = nail_core::engine::Engine::new(&listeners);
        core.send(&[Message::Initialize]);
    });

    loop {
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}

fn initialize_modules() -> Vec<Box<dyn Module>> {
    vec![
        Box::new(echomodule::EchoModule::new()),
        Box::new(foomodule::FooModule::new()),
        Box::new(lexermodule::LexerModule::new()),
        Box::new(stdinmodule::StdinModule::new()),
    ]
}

fn wire_modules_to_core(
    modules: &[Box<dyn Module>],
    read: Receiver<Option<Message>>,
    write: Sender<Option<Message>>,
) -> Vec<Listener<Message>> {
    let mut result: Vec<Listener<Message>> = vec![];
    for module in modules {
        let thread_write = module.start(write.clone());
        let listener = Listener::new(read.clone(), thread_write);
        result.push(listener);
    }
    result
}
