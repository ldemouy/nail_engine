#[derive(Debug, Clone)]
pub struct Token {
    pub variant: String,
    pub contents: String,
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
