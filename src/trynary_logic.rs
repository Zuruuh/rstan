use std::fmt::Display;

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub enum TrynaryLogic {
    Yes,
    No,
    Maybe
}

impl TrynaryLogic {
    pub fn yes(&self) -> bool {
        self == &Self::Yes
    }

    pub fn no(&self) -> bool {
        self == &Self::No
    }

    pub fn maybe(&self) -> bool {
        self == &Self::Maybe
    }

    pub fn and(&self, other: &Self) -> Self {
        if self.yes() && other.yes() {
            return Self::Yes;
        }

        if self.no() || other.no() {
            return Self::No;
        }

        Self::Maybe
    }

    pub fn or(&self, other: &Self) -> Self {
        if self.yes() || other.yes() {
            return Self::Yes;
        }

        if self.no() && other.no() {
            return Self::No;
        }

        Self::Maybe
    }
}

impl Display for TrynaryLogic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Self::Yes => "yes",
            Self::No => "no",
            Self::Maybe => "maybe",
        })
    }
}
