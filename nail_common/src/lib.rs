#[derive(Debug, Clone)]
pub struct Token {
    pub variant: String,
    pub contents: String,
}

#[derive(Debug, Clone)]
pub enum Message {
    Initialize,
    RawInput(String),
    TokenMessage {
        action: Token,
        parameters: Vec<Token>,
    },
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
