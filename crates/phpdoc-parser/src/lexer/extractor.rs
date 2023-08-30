use regex::{Match, Regex};

use crate::lexer::token::{self, Token};

#[derive(Debug, Clone)]
pub struct Extractor {
    pub string: String,
}

impl Extractor {
    pub fn extract_from_start<'a>(
        &'a mut self,
        to_remove: &'a str,
        instanciator: fn(extracted: Option<&'a str>) -> Token,
    ) -> Result<Token, String> {
        if !self.string.starts_with(to_remove) {
            return Err(format!(""));
        }

        self.string = self.string.strip_prefix(to_remove).unwrap_or("").to_owned();

        Ok(instanciator(Some(to_remove)))
    }

    pub fn extract_from_start_by_regex<'a>(
        &'a mut self,
        to_remove: &'a Regex,
        instanciator: fn(extracted: Option<Match>) -> Token,
    ) -> Result<Token, String> {
        let message = "did not find anything".to_owned();
        let cloned_string = self.string.clone();

        let r#match = to_remove
            .captures(&cloned_string)
            .ok_or(message.clone())?
            .iter()
            .collect::<Vec<_>>()
            .first()
            .ok_or(message.clone())?
            .ok_or(message)?;
        self.string = to_remove.replace(&self.string, "").into();

        Ok(instanciator(Some(r#match)))
    }

    pub fn extract_whitespace(&mut self, tokens: &mut Vec<Token>) -> Result<(), String> {
        loop {
            tokens.push(match &self.string[0..1] {
                " " => self.extract_from_start_by_regex(
                    &*token::WHITESPACE_REGEX,
                    |maybe_whitespace| {
                        maybe_whitespace
                            .map(|whitespace| Token::Whitespace(whitespace.len()))
                            .unwrap_or(Token::Whitespace(0))
                    },
                )?,
                "\n" => self.extract_from_start(&*token::LINE_JUMP_TOKEN, |_| Token::LineJump)?,
                _ => break,
            });
        }

        Ok(())
    }
}
