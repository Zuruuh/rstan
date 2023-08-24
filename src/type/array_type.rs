use super::Type;
use crate::TrinaryLogic;

pub trait ArrayType {}

impl Type for dyn ArrayType {
    fn is_null(&self) -> TrinaryLogic { TrinaryLogic::No }

    fn is_true(&self) -> TrinaryLogic { TrinaryLogic::Maybe }

    fn is_false(&self) -> TrinaryLogic { TrinaryLogic::Maybe }

    fn is_boolean(&self) -> TrinaryLogic { TrinaryLogic::Yes }

    fn is_scalar(&self) -> TrinaryLogic { TrinaryLogic::Yes }

    fn describe(&self, verbosity: super::VerbosityLevel) -> String { String::from("bool") }

    fn get_constant_string(&self) -> Vec<String> { vec![] }

    fn to_php_doc_node(&self) -> TypeNode { todo!() }
}

pub struct DefaultArrayType;

impl ArrayType for DefaultArrayType {}
