mod lexer_offset;
mod lexer_token;

use lazy_static::lazy_static;
pub use lexer_token::LexerToken;
pub use regex::Regex;

pub struct Lexer;

pub struct Token(LexerToken, usize, usize);

lazy_static! {
    static ref LEXER_TOKENS_REGEXPS: Regex =
        Regex::new(include_str!("../../static/lexer_regex.txt")).unwrap();
}

impl Lexer {
    pub fn tokenize(string: String) -> Vec<Token> { vec![] }
}
