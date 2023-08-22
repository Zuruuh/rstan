use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Namespace {
    pub raw: String,
}

impl Display for Namespace {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.raw)
    }
}
