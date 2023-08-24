use super::Type;
use crate::TrinaryLogic;

pub trait BooleanType {}

impl Type for dyn BooleanType {
    fn is_null(&self) -> TrinaryLogic { TrinaryLogic::No }

    fn is_true(&self) -> TrinaryLogic { TrinaryLogic::Maybe }

    fn is_false(&self) -> TrinaryLogic { TrinaryLogic::Maybe }

    fn is_boolean(&self) -> TrinaryLogic { TrinaryLogic::Yes }

    fn is_scalar(&self) -> TrinaryLogic { TrinaryLogic::Yes }

    fn describe(&self, verbosity: super::VerbosityLevel) -> String { String::from("bool") }
}

pub struct DefaultBooleanType;

impl BooleanType for DefaultBooleanType {}
