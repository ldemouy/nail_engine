use crossbeam::channel::Sender;
use nail_common::Token;
use nail_core::engine::EngineMessage;
pub mod echomodule;
pub mod foomodule;
pub mod lexermodule;
pub mod stdinmodule;
pub trait Module {
    fn start(&self, core_write: Sender<Option<Message>>) -> Sender<Message>;
}

#[derive(Debug, Clone)]
pub enum Message {
    Initialize,
    RawInput(String),
    TokenMessage {
        action: Token,
        parameters: Vec<Token>,
    },
    EngineMessage(EngineMessage<Message>),
}

impl Into<EngineMessage<Message>> for Message {
    fn into(self) -> EngineMessage<Message> {
        match self {
            Message::EngineMessage(engine) => engine,
            _ => EngineMessage::None,
        }
    }
}
