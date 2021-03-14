use crossbeam::channel::Sender;
use nail_common::Message;

pub mod echomodule;
pub mod foomodule;
pub mod lexermodule;
pub mod stdinmodule;
pub trait Module {
    fn start(&self, core_write: Sender<Option<Message>>) -> Sender<Message>;
}
