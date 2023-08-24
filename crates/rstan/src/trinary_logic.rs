use std::fmt::Display;

use crate::r#type::BooleanType;

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub enum TrinaryLogic {
    Yes,
    No,
    Maybe,
}

impl TrinaryLogic {
    pub fn yes(&self) -> bool { self == &Self::Yes }

    pub fn no(&self) -> bool { self == &Self::No }

    pub fn maybe(&self) -> bool { self == &Self::Maybe }

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

    pub fn compare(&self, other: &Self) -> Option<Self> {
        let self_value: i8 = self.clone().into();
        let other_value: i8 = other.clone().into();

        match (self_value > other_value, other_value > self_value) {
            (true, false) => Some(self.clone()),
            (false, true) => Some(other.clone()),
            _ => None,
        }
    }

    pub fn negate(self) -> Self {
        match self {
            Self::Yes => Self::No,
            Self::No => Self::Yes,
            Self::Maybe => Self::Maybe,
        }
    }
}

impl Into<i8> for TrinaryLogic {
    fn into(self) -> i8 {
        match self {
            Self::Yes => 1,
            Self::Maybe => 0,
            Self::No => -1,
        }
    }
}

impl Into<Box<dyn BooleanType>> for TrinaryLogic {
    fn into(self) -> Box<dyn BooleanType> { todo!() }
}

impl From<bool> for TrinaryLogic {
    fn from(value: bool) -> Self {
        if value == true {
            return Self::Yes;
        }

        Self::No
    }
}

impl Display for TrinaryLogic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Yes => "Yes",
                Self::No => "No",
                Self::Maybe => "Maybe",
            }
        )
    }
}
