use super::Type;

pub struct NeverType;

impl Type for NeverType {
    fn describe(&self, verbosity: super::VerbosityLevel) -> String { String::from("never") }
}
