#[derive(Debug)]
pub struct Token {
    pub variant: String,
    pub contents: String,
}

#[derive(Debug, Clone)]
pub struct Message {
    action: String,
    parameters: Vec<String>,
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
