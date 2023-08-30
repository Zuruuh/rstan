mod extractor;
mod token;

use extractor::Extractor;
pub use token::Token;

pub struct Lexer;

impl Lexer {
    pub fn tokenize(string: &String) -> Result<Vec<Token>, String> {
        let mut tokens: Vec<Token> = vec![];
        let mut extractor = Extractor {
            string: string.clone(),
        };

        extractor.extract_whitespace(&mut tokens)?;
        tokens
            .push(extractor.extract_from_start(&*token::OPEN_PHPDOC_TOKEN, |_| Token::OpenPhpdoc)?);

        loop {
            if extractor.string.is_empty() {
                break;
            }

            extractor.extract_whitespace(&mut tokens)?;
            if let Ok(line_start) =
                extractor.extract_from_start(&*token::LINE_START_TOKEN, |_| Token::LineStart)
            {
                tokens.push(line_start);
            }

            // if let Ok()
        }

        dbg!(&tokens);
        dbg!(&extractor.string);

        Ok(tokens)
    }
}
