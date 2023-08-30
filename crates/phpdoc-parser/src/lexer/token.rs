use std::fmt::Display;

use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    pub static ref WHITESPACE_REGEX: Regex = Regex::new(r#"^ +"#).unwrap();
    pub static ref LINE_JUMP_TOKEN: &'static str = "\n";
    pub static ref LINE_START_TOKEN: &'static str = "*";
    pub static ref REFERENCE_TOKEN: &'static str = "&";
    pub static ref UNION_TOKEN: &'static str = "|";
    pub static ref INTERSECTION_TOKEN: &'static str = "&";
    pub static ref NULLABLE_TOKEN: &'static str = "?";
    pub static ref NEGATED_TOKEN: &'static str = "!";
    pub static ref OPEN_PARENTHESE_TOKEN: &'static str = "(";
    pub static ref CLOSE_PARENTHESE_TOKEN: &'static str = ")";
    pub static ref OPEN_ANGLE_BRACKET_TOKEN: &'static str = "<";
    pub static ref CLOSE_ANGLE_BRACKET_TOKEN: &'static str = ">";
    pub static ref OPEN_SQUARE_BRACKET_TOKEN: &'static str = "[";
    pub static ref CLOSE_SQUARE_BRACKET_TOKEN: &'static str = "]";
    pub static ref OPEN_CURLY_BRACKET_TOKEN: &'static str = "{";
    pub static ref CLOSE_CURLY_BRACKET_TOKEN: &'static str = "}";
    pub static ref COMMA_TOKEN: &'static str = ",";
    pub static ref COLON_TOKEN: &'static str = ":";
    pub static ref VARIADIC_TOKEN: &'static str = "...";
    pub static ref DOUBLE_COLON_TOKEN: &'static str = "::";
    pub static ref DOUBLE_ARROW_TOKEN: &'static str = "=>";
    pub static ref ARROW_TOKEN: &'static str = "->";
    pub static ref EQUAL_TOKEN: &'static str = "=";
    pub static ref OPEN_PHPDOC_TOKEN: &'static str = "/**";
    pub static ref CLOSE_PHPDOC_TOKEN: &'static str = "*/";
    pub static ref LITERAL_INTEGER_REGEX: Regex = Regex::new(r#"^-?[0-9]+"#,).unwrap();
    pub static ref LITERAL_FLOAT_REGEX: Regex = Regex::new(r#"^-?[0-9]+\.[0-9]+"#,).unwrap();
    pub static ref LITERAL_SINGLE_QUOTED_STRING_REGEX: Regex = Regex::new(r#"^'.*'"#,).unwrap();
    pub static ref LITERAL_DOUBLE_QUOTED_STRING_REGEX: Regex = Regex::new(r#"^".*""#,).unwrap();
    pub static ref THIS_TOKEN: &'static str = "$this";
    pub static ref STATIC_TOKEN: &'static str = "static";
    pub static ref SELF_TOKEN: &'static str = "self";
    pub static ref IS_TOKEN: &'static str = "is";
    pub static ref TAG_REGEX: Regex = Regex::new(r#"^@[\w-]+"#).unwrap();
    pub static ref VARIABLE_REGEX: Regex = Regex::new(r#"^\$[\w_][\w\d_]*"#).unwrap();
    pub static ref IDENTIFIER_REGEX: Regex = Regex::new(r#"^[\w_][\w\d_\-\\]*"#).unwrap();
    pub static ref TEXT_REGEX: Regex = Regex::new(r#"^[.]*"#).unwrap();
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Token {
    Whitespace(usize),
    LineJump,
    LineStart,
    Reference,
    Union,
    Intersection,
    Nullable,
    Negated,
    OpenParentheses,
    CloseParentheses,
    OpenAngleBracket,
    CloseAngleBracket,
    OpenSquareBracket,
    CloseSquareBracket,
    OpenCurlyBracket,
    CloseCurlyBracket,
    Comma,
    Colon,
    Variadic,
    DoubleColon,
    DoubleArrow,
    Arrow,
    Equal,
    OpenPhpdoc,
    ClosePhpdoc,
    LiteralInteger(String),
    LiteralFloat(String),
    LiteralSingleQuotedString(String),
    LiteralDoubleQuotedString(String),
    This,
    Static,
    _Self,
    Is,
    Tag(String),
    Variable(String),
    Identifier(String),
    Text(String),
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
                Self::Is => "is".to_owned(),
                Self::Tag(tag) => tag.clone(),
                Self::Variable(variable) => variable.clone(),
                Self::Text(text) => text.clone(),
            }
        )
    }
}
