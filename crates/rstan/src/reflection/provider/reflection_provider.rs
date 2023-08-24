use php_parser_rs::parser::ast::classes::ClassStatement;

use crate::reflection::{ClassReflection, NamespaceAnswerer, FunctionReflection, GlobalConstantReflection};

pub trait ReflectionProvider {
    fn has_class(&self, class_name: String) -> bool;
    fn get_class(&self, class_name: String) -> ClassReflection;
    fn get_class_name(&self, class_name: String) -> String;
    fn supports_anonymous_classes(&self) -> bool;
    fn get_anonymous_class_reflection(&self, class_node: ClassStatement) -> ClassReflection;
    fn has_function(&self, name_node: String, namespace_answerer: Option<impl NamespaceAnswerer>) -> bool;
    fn get_function(&self, name_node: String, namespace_answerer: Option<impl NamespaceAnswerer>) -> FunctionReflection;
    fn resolve_function(&self, name_node: String, namespace_answerer: Option<impl NamespaceAnswerer>) -> Option<String>;
    fn has_constant(&self, name_node: String, namespace_answerer: Option<impl NamespaceAnswerer>) -> bool;
    fn get_constant(&self, name_node: String, namespace_answerer: Option<impl NamespaceAnswerer>) -> GlobalConstantReflection;
    fn resolve_constant(&self, name_node: String, namespace_answerer: Option<impl NamespaceAnswerer>) -> Option<String>;
}
