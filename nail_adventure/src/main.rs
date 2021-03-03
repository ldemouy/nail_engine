use std::io;

fn main() {
    println!("Input: ");
    let lexer = load_lexer().unwrap();
    loop {
        let mut buffer = String::new();
        let stdin = io::stdin();
        stdin.read_line(&mut buffer).unwrap();
        let buffer = buffer.trim();

        if buffer.is_empty() {
            continue;
        }

        let tokens = lexer.lex(&buffer);
        for token in tokens {
            println!("{:?}", token);
            if token.variant == "foo" {
                println!("BAR!");
            }
        }

        println!();
    }
}

fn load_lexer() -> Result<nail_lexer::Lexer, std::io::Error> {
    use nail_lexer::*;
    use std::{fs::File, path::Path};

    let path = Path::new("data/grammar.json");
    let file = File::open(&path)?;
    let mut reader = std::io::BufReader::new(&file);
    let grammar: Vec<Grammar> = serde_json::from_reader(&mut reader)?;

    Ok(Lexer::new(grammar))
}
