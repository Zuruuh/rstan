use crate::reflection::{
    ClassReflection, ConstantReflection, MethodReflection, PropertyReflection,
};

pub trait ClassMemberAccessAnswerer {
    fn is_in_class(&self) -> bool;

    fn get_class_reflection(&self) -> Option<ClassReflection>;

    fn can_access_property(&self, property_reflection: PropertyReflection) -> bool;

    fn can_call_method(&self, method_reflection: MethodReflection) -> bool;

    fn can_access_constant(&self, constant_reflection: ConstantReflection) -> bool;
}
