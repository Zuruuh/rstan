use php_parser_rs::{node::Node, parser::ast::classes::ClassStatement};

use super::node;

#[node(ClassStatement)]
pub struct ClassNode {
    pub node: Node,
}
