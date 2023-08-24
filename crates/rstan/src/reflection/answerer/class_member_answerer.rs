use crate::reflection::ClassReflection;

pub trait ClassMemberAnswerer {
    fn is_in_class(&self) -> bool;

    fn get_class_reflection(&self) -> Option<ClassReflection>;
}
