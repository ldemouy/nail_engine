#[derive(Debug, Clone)]
pub struct Token {
    pub variant: String,
    pub contents: String,
}

#[derive(Debug, Clone)]
pub struct Message {
    pub action: Token,
    pub parameters: Vec<Token>,
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
