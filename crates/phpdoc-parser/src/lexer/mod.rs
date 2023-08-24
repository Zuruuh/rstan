mod lexer_token;

pub use lexer_token::LexerToken;

/// 0 Original String
/// 1 Extracted String
pub type ExtractionResult = (String, String);

pub struct Lexer;

impl Lexer {
    pub fn tokenize(string: &String) -> Result<Vec<LexerToken>, String> {
        let mut tokens: Vec<LexerToken> = vec![];
        let lines = string.lines().map(|line| line.chars().enumerate().peekable());

        for line in lines {
            for (col, char) in line {
                match char {
                    '' =>
                    _ => todo!(),
                }
            }
        }
        // let (string, start) = Self::extract(string, "/**").map_err(|_| "...".to_owned())?;
        //
        // tokens.push();
        //
        Ok(tokens)
    }

    fn extract(string: String, to_extract: &str) -> Result<ExtractionResult, ()> {
        let string = string.trim();
        let new_string = string.replace(to_extract, "");

        if new_string == string {
            return Err(());
        }

        Ok((new_string, to_extract.to_owned()))
    }

    // fn extract_word()
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
