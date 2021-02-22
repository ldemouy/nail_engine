use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Grammar {
    name: String,
    synonyms: Vec<String>,
}

#[derive(Debug)]
pub struct Lexer {
    grammar: Vec<Grammar>,
}

impl Lexer {
    pub fn new(grammar: Vec<Grammar>) -> Self {
        Lexer { grammar }
    }

    pub fn lex(line: &str) -> Vec<nail_common::Token> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
