mod token;

use logos::Logos;
pub use token::Token;

pub struct Lexer;

impl Lexer {
    pub fn tokenize(string: &String) -> Result<Vec<Token>, String> {
        let mut tokens: Vec<Token> = vec![];
        let lexer = Token::lexer(string);

        dbg!(lexer.collect::<Vec<_>>());

        Ok(tokens)
    }
}

#[cfg(test)]
mod test {
    use super::Lexer;

    #[test]
    pub fn yes() {
        let string = String::from(
            r#"
        /**
         * @param bool $test
         */
        "#,
        );

        Lexer::tokenize(&string).unwrap();
    }
}
