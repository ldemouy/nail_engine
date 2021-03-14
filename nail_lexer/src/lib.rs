use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Grammar {
    name: String,
    synonyms: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct Lexer {
    grammar: Vec<Grammar>,
}

impl Lexer {
    pub fn new(grammar: Vec<Grammar>) -> Self {
        Lexer { grammar }
    }

    /// Takes a line of text, presumably from direct textual user input
    /// and turns it into a vector of tokens based upon the grammar that
    /// was passed into the new function. Tokens are split on spaces.
    /// Numbers are automatically handled, for purposes of the lexer
    /// no distinction is made between floats and integers.
    pub fn lex(&self, line: &str) -> Vec<nail_common::Token> {
        let lexemes: Vec<&str> = line.split(' ').collect();
        let mut tokens = vec![];
        for lexeme in lexemes {
            //Special Case: Number
            if lexeme.parse::<f64>().is_ok() {
                tokens.push(nail_common::Token {
                    variant: "number".to_string(),
                    contents: lexeme.to_string(),
                });
                continue;
            }

            //Normal Case: In Grammar
            let mut translated = false;
            for gramm in &self.grammar {
                if lexeme == gramm.name || gramm.synonyms.iter().any(|f| f == lexeme) {
                    tokens.push(nail_common::Token {
                        variant: gramm.name.to_string(),
                        contents: lexeme.to_string(),
                    });
                    translated = true;
                }
            }
            //Case Unknown: not in grammar
            if !translated {
                tokens.push(nail_common::Token {
                    variant: "unknown".to_string(),
                    contents: lexeme.to_string(),
                });
            }
        }
        tokens
    }
}

#[cfg(test)]
mod tests {
    use super::Lexer;
    use std::io::Error;
    use std::{fs::File, path::Path};

    use crate::Grammar;

    fn create_lexer() -> Result<Lexer, Error> {
        let path = Path::new("tests/grammar.json");
        let file = File::open(&path)?;
        let mut reader = std::io::BufReader::new(&file);
        let grammar: Vec<Grammar> = serde_json::from_reader(&mut reader)?;

        assert_eq!(grammar.len(), 2);

        Ok(Lexer::new(grammar))
    }

    #[test]
    fn lex_single_integer() -> Result<(), Error> {
        let lexer = create_lexer()?;

        let tokens = lexer.lex("1");
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].variant, "number");
        assert_eq!(tokens[0].contents, "1");

        Ok(())
    }

    #[test]
    fn lex_single_float() -> Result<(), Error> {
        let lexer = create_lexer()?;

        let tokens = lexer.lex("1.0");
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].variant, "number");
        assert_eq!(tokens[0].contents, "1.0");

        Ok(())
    }

    #[test]
    fn lex_single_token_name() -> Result<(), Error> {
        let lexer = create_lexer()?;

        let tokens = lexer.lex("foo");
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].variant, "foo");
        assert_eq!(tokens[0].contents, "foo");

        Ok(())
    }

    #[test]
    fn lex_single_token_synonym() -> Result<(), Error> {
        let lexer = create_lexer()?;

        let tokens = lexer.lex("bar");
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].variant, "foo");
        assert_eq!(tokens[0].contents, "bar");

        Ok(())
    }

    #[test]
    fn lex_single_token_second_gramm() -> Result<(), Error> {
        let lexer = create_lexer()?;

        let tokens = lexer.lex("qux");
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].variant, "qux");
        assert_eq!(tokens[0].contents, "qux");

        Ok(())
    }

    #[test]
    fn lex_single_token_unknown() -> Result<(), Error> {
        let lexer = create_lexer()?;

        let tokens = lexer.lex("none");
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].variant, "unknown");
        assert_eq!(tokens[0].contents, "none");

        Ok(())
    }

    #[test]
    fn lex_multiple_token_same() -> Result<(), Error> {
        let lexer = create_lexer()?;

        let tokens = lexer.lex("foo bar");
        assert_eq!(tokens.len(), 2);

        assert_eq!(tokens[0].variant, "foo");
        assert_eq!(tokens[0].contents, "foo");

        assert_eq!(tokens[1].variant, "foo");
        assert_eq!(tokens[1].contents, "bar");

        Ok(())
    }

    #[test]
    fn lex_multiple_token_different() -> Result<(), Error> {
        let lexer = create_lexer()?;

        let tokens = lexer.lex("foo qux");
        assert_eq!(tokens.len(), 2);

        assert_eq!(tokens[0].variant, "foo");
        assert_eq!(tokens[0].contents, "foo");

        assert_eq!(tokens[1].variant, "qux");
        assert_eq!(tokens[1].contents, "qux");

        Ok(())
    }

    #[test]
    fn lex_multiple_token_unknown() -> Result<(), Error> {
        let lexer = create_lexer()?;

        let tokens = lexer.lex("foo qux none");
        assert_eq!(tokens.len(), 3);

        assert_eq!(tokens[0].variant, "foo");
        assert_eq!(tokens[0].contents, "foo");

        assert_eq!(tokens[1].variant, "qux");
        assert_eq!(tokens[1].contents, "qux");

        assert_eq!(tokens[2].variant, "unknown");
        assert_eq!(tokens[2].contents, "none");

        Ok(())
    }
}
