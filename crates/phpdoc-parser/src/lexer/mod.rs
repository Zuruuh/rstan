mod token;

use logos::Logos;
pub use token::Token;

pub struct Lexer;

impl Lexer {
    pub fn tokenize(string: &String) -> Result<Vec<Token>, String> {
        let lexer = Token::lexer(string);

        let tokens = lexer
            .collect::<Result<Vec<_>, _>>()
            .map_err(|_| "Invalid phpdoc".to_owned())?;

        dbg!(tokens);

        Ok(vec![])
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
         * @param bool $test My custom text
         */
        "#,
        );

        Lexer::tokenize(&string).unwrap();
    }
}
