use super::{AcceptsResult, BooleanType, DefaultBooleanType, VerbosityLevel};
use crate::{
    reflection::{
        ClassMemberAccessAnswerer, ClassReflection, ConstantReflection, PropertyReflection,
    },
    TrinaryLogic,
};

pub trait Type {
    fn get_referenced_classes(&self) -> Vec<String> { vec![] }

    fn get_object_class_names(&self) -> Vec<String> { vec![] }

    fn get_object_class_reflections(&self) -> Vec<ClassReflection> { vec![] }

    fn get_class_string_object_type(&self) -> Result<Box<dyn Type>, ()> { Err(()) }

    fn get_object_type_or_class_string_object_type(&self) -> Result<Box<dyn Type>, ()> { Err(()) }

    fn is_object(&self) -> TrinaryLogic { TrinaryLogic::No }

    fn is_enum(&self) -> TrinaryLogic { TrinaryLogic::No }

    fn get_arrays(&self) -> Vec<ArrayType> { vec![] }

    fn get_constant_arrays(&self) -> Vec<ConstantArrayType> { vec![] }

    fn get_constant_strings(&self) -> Vec<ConstantStringType> { vec![] }

    fn accepts(&self, r#type: Box<&dyn Type>, strict_types: bool) -> AcceptsResult {
        AcceptsResult::create_no()
    }

    fn describe(&self, verbosity: super::VerbosityLevel) -> String;

    fn get_constant_string(&self) -> Vec<String> { vec![] }

    fn is_super_type_of(&self, r#type: Box<&dyn Type>) -> TrinaryLogic { TrinaryLogic::No }

    fn equals(&self, r#type: Box<&dyn Type>) -> bool {
        self.describe(VerbosityLevel::Value) == r#type.describe(VerbosityLevel::Value)
    }

    fn can_access_properties(&self) -> TrinaryLogic { TrinaryLogic::No }

    fn has_property(&self, property: String) -> TrinaryLogic { TrinaryLogic::No }

    fn get_property(
        &self,
        property: String,
        scope: Box<&dyn ClassMemberAccessAnswerer>,
    ) -> Result<PropertyReflection, ()> {
        Err(())
    }

    fn get_unresolved_property_prototype(
        &self,
        property: String,
        scope: Box<&dyn ClassMemberAccessAnswerer>,
    ) -> Result<UnresolvedPropertyPrototypeReflection, ()> {
        Err(())
    }

    fn can_call_methods(&self) -> TrinaryLogic { TrinaryLogic::No }

    fn has_method(&self, method_name: String) -> TrinaryLogic { TrinaryLogic::No }

    fn get_method(
        &self,
        method_name: String,
        scope: Box<&dyn ClassMemberAccessAnswerer>,
    ) -> Result<ExtendedMethodReflection, ()> {
        Err(())
    }

    fn get_unresolved_method_prototype(
        &self,
        method_name: String,
        scope: Box<&dyn ClassMemberAccessAnswerer>,
    ) -> Result<UnresolvedMethodPrototypeReflection, ()> {
        Err(())
    }

    fn can_access_constants(&self) -> TrinaryLogic { TrinaryLogic::No }

    fn has_constant(&self, constant_name: String) -> TrinaryLogic { TrinaryLogic::No }

    fn get_constant(&self, constant_name: String) -> Result<ConstantReflection, ()> { Err(()) }

    fn is_iterable(&self) -> TrinaryLogic { TrinaryLogic::No }

    fn is_iterable_at_least_once(&self) -> TrinaryLogic { TrinaryLogic::No }

    fn get_array_size(&self) -> Result<Box<dyn Type>, ()> { Err(()) }

    fn get_iterable_key_type(&self) -> Result<Box<dyn Type>, ()> { Err(()) }

    fn get_first_iterable_key_type(&self) -> Result<Box<dyn Type>, ()> { Err(()) }

    fn get_last_iterable_key_type(&self) -> Result<Box<dyn Type>, ()> { Err(()) }

    fn get_iterable_value_type(&self) -> Result<Box<dyn Type>, ()> { Err(()) }

    fn get_first_iterable_value_type(&self) -> Result<Box<dyn Type>, ()> { Err(()) }

    fn get_last_iterable_value_type(&self) -> Result<Box<dyn Type>, ()> { Err(()) }

    fn is_array(&self) -> TrinaryLogic { TrinaryLogic::No }

    fn is_constant_array(&self) -> TrinaryLogic { TrinaryLogic::No }

    fn is_oversized_array(&self) -> TrinaryLogic { TrinaryLogic::No }

    fn is_list(&self) -> TrinaryLogic { TrinaryLogic::No }

    fn is_offset_accessible(&self) -> TrinaryLogic { TrinaryLogic::No }

    fn has_offset_value_type(&self, offset_type: Box<&dyn Type>) -> TrinaryLogic {
        TrinaryLogic::No
    }

    fn get_offset_value_type(&self, offset_type: Box<&dyn Type>) -> Result<Box<dyn Type>, ()> {
        Err(())
    }

    fn set_offset_value_type(
        &self,
        offset_type: Option<Box<&dyn Type>>,
        value_type: Box<&dyn Type>,
        union_values: bool,
    ) -> Result<Box<dyn Type>, ()> {
        Err(())
    }

    fn unset_offset(&self, offset_type: Box<&dyn Type>) -> Result<Box<dyn Type>, ()> { Err(()) }

    fn get_keys_array(&self) -> Result<Box<dyn Type>, ()> { Err(()) }

    fn get_values_array(&self) -> Result<Box<dyn Type>, ()> { Err(()) }

    fn fill_keys_array(&self, value_type: Box<&dyn Type>) -> Result<Box<dyn Type>, ()> { Err(()) }

    fn flip_array(&self) -> Result<Box<dyn Type>, ()> { Err(()) }

    fn intersect_key_array(&self, other_arrays_type: Box<&dyn Type>) -> Result<Box<dyn Type>, ()> {
        Err(())
    }

    fn pop_array(&self) -> Result<Box<dyn Type>, ()> { Err(()) }

    fn search_array(&self, needle_type: Box<&dyn Type>) -> Result<Box<dyn Type>, ()> { Err(()) }

    fn shift_array(&self) -> Result<Box<dyn Type>, ()> { Err(()) }

    fn shuffle_array(&self) -> Result<Box<dyn Type>, ()> { Err(()) }

    fn get_enum_cases(&self) -> Vec<EnumCaseObjectType> { vec![] }

    fn get_finite_types(&self) -> Vec<Box<dyn Type>> { vec![] }

    fn exponentiate(&self, exponent: Box<&dyn Type>) -> Result<Box<dyn Type>, ()> { Err(()) }

    fn is_callable(&self) -> TrinaryLogic { TrinaryLogic::No }

    fn get_callable_parameters_acceptors(
        &self,
        scope: Box<&dyn ClassMemberAccessAnswerer>,
    ) -> Vec<ParametersAcceptor> {
        vec![]
    }

    fn is_cloneable(&self) -> TrinaryLogic { TrinaryLogic::No }

    fn to_boolean(&self) -> Box<dyn BooleanType> { Box::new(DefaultBooleanType) }

    fn to_number(&self) -> Result<Box<dyn Type>, ()> { Err(()) }

    fn to_integer(&self) -> Result<Box<dyn Type>, ()> { Err(()) }

    fn to_float(&self) -> Result<Box<dyn Type>, ()> { Err(()) }

    fn to_string(&self) -> Result<Box<dyn Type>, ()> { Err(()) }

    fn to_array(&self) -> Result<Box<dyn Type>, ()> { Err(()) }

    fn to_array_key(&self) -> Result<Box<dyn Type>, ()> { Err(()) }

    fn is_smaller_than(&self, other_type: Box<&dyn Type>) -> TrinaryLogic { TrinaryLogic::No }

    fn is_smaller_than_or_equal(&self, other_type: Box<&dyn Type>) -> TrinaryLogic {
        TrinaryLogic::No
    }

    /// Is Type of a known constant value? Includes literal strings, integers, floats, true, false, null, and array shapes.
    fn is_constant_value(&self) -> TrinaryLogic { TrinaryLogic::No }

    /// Is Type of a known constant scalar value? Includes literal strings, integers, floats, true, false, and null.
    fn is_constant_scalar_value(&self) -> TrinaryLogic { TrinaryLogic::No }

    fn is_constant_scalar_types(&self) -> Vec<ConstantScalarType> { vec![] }

    fn get_constant_scalar_values(&self) -> Vec<ConstantScalarValue> { vec![] }

    fn is_null(&self) -> TrinaryLogic { TrinaryLogic::No }

    fn is_true(&self) -> TrinaryLogic { TrinaryLogic::No }

    fn is_false(&self) -> TrinaryLogic { TrinaryLogic::No }

    fn is_boolean(&self) -> TrinaryLogic { TrinaryLogic::No }

    fn is_float(&self) -> TrinaryLogic { TrinaryLogic::No }

    fn is_integer(&self) -> TrinaryLogic { TrinaryLogic::No }

    fn is_string(&self) -> TrinaryLogic { TrinaryLogic::No }

    fn is_numeric_string(&self) -> TrinaryLogic { TrinaryLogic::No }

    fn is_non_empty_string(&self) -> TrinaryLogic { TrinaryLogic::No }

    fn is_non_falsy_string(&self) -> TrinaryLogic { TrinaryLogic::No }

    fn is_literal_string(&self) -> TrinaryLogic { TrinaryLogic::No }

    fn is_class_string_type(&self) -> TrinaryLogic { TrinaryLogic::No }

    fn is_void(&self) -> TrinaryLogic { TrinaryLogic::No }

    fn is_scalar(&self) -> TrinaryLogic { TrinaryLogic::No }

    fn loose_compare(
        &self,
        r#type: Box<&dyn Type>,
        php_version: PhpVersion,
    ) -> Box<dyn BooleanType> {
        Box::new(DefaultBooleanType)
    }

    fn get_smaller_type(&self) -> Box<&dyn Type> { Box::new(NeverType) }

    fn get_smaller_or_equal_type(&self) -> Box<&dyn Type> { Box::new(NeverType) }

    fn get_greater_type(&self) -> Box<&dyn Type> { Box::new(NeverType) }

    fn get_greater_or_equal_type(&self) -> Box<&dyn Type> { Box::new(NeverType) }

    fn get_template_type(
        &self,
        ancestor_class_name: String,
        template_type_name: String,
    ) -> Result<Box<dyn Type>, ()> {
        Err(())
    }

    fn infer_template_types(&self, received_type: Box<&dyn Type>) -> TemplateTypeMap {
        todo!()
        // TemplateTypeMap::default()
    }

    fn get_referenced_template_types(
        &self,
        position_variance: TemplateTypeVariance,
    ) -> Vec<TemplateTypeReference> {
        vec![]
    }

    fn traverse(&self, callback: fn(r#type: &dyn Type) -> dyn Type) -> Box<dyn Type> { todo!() }

    fn traverse_simultaneously(
        &self,
        right: Box<&dyn Type>,
        callback: fn(left: &dyn Type, right: &dyn Type) -> dyn Type,
    ) -> Box<&dyn Type> {
        todo!()
    }

    fn to_php_doc_node(&self) -> TypeNode;

    fn try_remove(&self) -> Option<Box<dyn Type>> { None }

    fn generalize(&self, precision: GeneralizePrecision) -> Box<dyn Type> { todo!() }
}
