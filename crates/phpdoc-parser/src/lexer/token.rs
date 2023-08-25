use std::fmt::Display;

use logos::Logos;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Logos)]
pub enum Token {
    #[regex(r#" +"#, |lex| lex.slice().parse().map(|whitespace: String| whitespace.len()).ok())]
    Whitespace(usize),
    #[token("\n")]
    LineJump,
    #[token("*")]
    LineStart,
    #[token("&", priority = 2)]
    Reference,
    #[token("|")]
    Union,
    #[token("&", priority = 1)]
    Intersection,
    #[token("?")]
    Nullable,
    #[token("!")]
    Negated,
    #[token("(")]
    OpenParentheses,
    #[token(")")]
    CloseParentheses,
    #[token("<")]
    OpenAngleBracket,
    #[token(">")]
    CloseAngleBracket,
    #[token("[")]
    OpenSquareBracket,
    #[token("]")]
    CloseSquareBracket,
    #[token("{")]
    OpenCurlyBracket,
    #[token("}")]
    CloseCurlyBracket,
    #[token(",")]
    Comma,
    #[token(":")]
    Colon,
    #[token("...")]
    Variadic,
    #[token("::")]
    DoubleColon,
    #[token("=>")]
    DoubleArrow,
    #[token("->")]
    Arrow,
    #[token("=")]
    Equal,
    #[token("/**")]
    OpenPhpdoc,
    #[token("*/")]
    ClosePhpdoc,
    #[regex(r#"-?[0-9]+"#, |lex| lex.slice().parse().ok(), priority = 2)]
    LiteralInteger(String),
    #[regex(r#"-?[0-9]+\.[0-9]+"#, |lex| lex.slice().parse().ok())]
    LiteralFloat(String),
    #[regex(r#"'.*'"#, |lex| lex.slice().parse().ok())]
    LiteralSingleQuotedString(String),
    #[regex(r#"".*""#, |lex| lex.slice().parse().ok())]
    LiteralDoubleQuotedString(String),
    #[token("\\$this")]
    This,
    #[token("static")]
    Static,
    #[token("self")]
    _Self,
    #[regex(r#"@[\w-]+"#, |lex| lex.slice().parse().ok(), priority = 3)]
    Tag(String),
    #[regex(r#"\$[\w_][\w\d_]*"#, |lex| lex.slice().parse().ok())]
    Variable(String),
    #[regex(r#"[\w_][\w\d_\-\\]*"#, |lex| lex.slice().parse().ok())]
    Identifier(String),
    #[regex(r#"[.]*"#r, |lex| lex.slice().parse().ok(), priority = 0)]
    Text(String),
}

fn parse_identifier(lex: &mut Lexer<Token>) -> Option<String> {
    lex.pre
    let slice: Result<String, _> = lex.slice().parse();

    if let Ok(text) = slice {
        if text.len() > 1 {
            return Some(text);
        }
    }

    None
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Whitespace(length) => " ".repeat(length.clone()),
                Self::LineJump => "\n".to_owned(),
                Self::LineStart => "*".to_owned(),
                Self::Reference => "&".to_owned(),
                Self::Union => "|".to_owned(),
                Self::Intersection => "&".to_owned(),
                Self::Nullable => "?".to_owned(),
                Self::Negated => "!".to_owned(),
                Self::OpenParentheses => "(".to_owned(),
                Self::CloseParentheses => ")".to_owned(),
                Self::OpenAngleBracket => "<".to_owned(),
                Self::CloseAngleBracket => ">".to_owned(),
                Self::OpenSquareBracket => "[".to_owned(),
                Self::CloseSquareBracket => "]".to_owned(),
                Self::OpenCurlyBracket => "{".to_owned(),
                Self::CloseCurlyBracket => "}".to_owned(),
                Self::Comma => ",".to_owned(),
                Self::Colon => ":".to_owned(),
                Self::Variadic => "...".to_owned(),
                Self::DoubleColon => "::".to_owned(),
                Self::DoubleArrow => "=>".to_owned(),
                Self::Arrow => "->".to_owned(),
                Self::Equal => "=".to_owned(),
                Self::OpenPhpdoc => "/**".to_owned(),
                Self::ClosePhpdoc => "*/".to_owned(),
                Self::LiteralFloat(literal) => literal.clone(),
                Self::LiteralInteger(literal) => literal.clone(),
                Self::LiteralSingleQuotedString(literal) => format!("'{}'", literal),
                Self::LiteralDoubleQuotedString(literal) => format!("\"{}\"", literal),
                Self::Identifier(identifier) => identifier.clone(),
                Self::This => "$this".to_owned(),
                Self::_Self => "self".to_owned(),
                Self::Static => "static".to_owned(),
                Self::Tag(tag) => tag.clone(),
                Self::Variable(variable) => variable.clone(),
                Self::Text(text) => text.clone(),
            }
        )
    }
}
