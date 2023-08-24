use super::Type;

pub trait ArrayType {}

impl Type for dyn ArrayType {
    fn describe(&self, _verbosity: super::VerbosityLevel) -> String { String::from("bool") }

    fn get_constant_string(&self) -> Vec<String> { vec![] }
}

pub struct DefaultArrayType;

impl ArrayType for DefaultArrayType {}
