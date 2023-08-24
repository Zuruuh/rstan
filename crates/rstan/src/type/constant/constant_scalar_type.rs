use super::{ConstantScalarValue, ConstantType};

pub trait ConstantScalarType: ConstantType {
    fn get_value(&self) -> ConstantScalarValue;
}
