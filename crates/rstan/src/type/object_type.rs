use super::Type;

pub trait ObjectType {}

impl Type for dyn ObjectType {
    fn describe(&self, _verbosity: super::VerbosityLevel) -> String { String::from("bool") }

    fn get_constant_string(&self) -> Vec<String> { vec![] }
}

pub struct DefaultObjectType;

impl ObjectType for DefaultObjectType {}
