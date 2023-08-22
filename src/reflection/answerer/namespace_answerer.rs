use crate::internals::Namespace;

pub trait NamespaceAnswerer {
    fn get_namespace(&self) -> Namespace;
}
