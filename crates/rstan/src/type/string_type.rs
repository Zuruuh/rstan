use super::Type;
use crate::TrinaryLogic;

pub trait StringType {}

impl Type for dyn StringType {
    fn is_null(&self) -> TrinaryLogic { TrinaryLogic::No }

    fn is_true(&self) -> TrinaryLogic { TrinaryLogic::Maybe }

    fn is_false(&self) -> TrinaryLogic { TrinaryLogic::Maybe }

    fn is_boolean(&self) -> TrinaryLogic { TrinaryLogic::No }

    fn is_scalar(&self) -> TrinaryLogic { TrinaryLogic::Yes }

    fn describe(&self, verbosity: super::VerbosityLevel) -> String { String::from("string") }

    fn get_constant_string(&self) -> Vec<String> { vec![] }
}

pub struct DefaultStringType;

impl StringType for DefaultStringType {}
