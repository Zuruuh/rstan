use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum LexerTokenKind {
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
    LiteralFloat(String),
    LiteralInteger(String),
    LiteralSingleQuotedString(String),
    LiteralDoubleQuotedString(String),
    Identifier(String),
    This,
    Variable(String),
}

pub struct Position {
    pub start_col: usize,
    pub line: usize,
}

pub struct LexerToken {
    pub content: String,
    pub token: LexerTokenKind,
    pub position: Position,
}

impl Display for LexerTokenKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
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
                Self::Variable(variable) => format!("${variable}"),
            }
        )
    }
}
