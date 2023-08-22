/**
 
	public function isInClass(): bool;

	public function getClassReflection(): ?ClassReflection;

	public function canAccessProperty(PropertyReflection $propertyReflection): bool;

	public function canCallMethod(MethodReflection $methodReflection): bool;

	public function canAccessConstant(ConstantReflection $constantReflection): bool;

    **/

pub trait ClassMemberAnswerer {
    fn is_in_class(&self) -> bool;

    fn get_class_reflection(&self) -> Option<ClassReflection>;
}
