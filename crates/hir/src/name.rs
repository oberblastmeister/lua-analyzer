use smol_str::SmolStr;
use syntax::ast::{self, AstNode};

#[derive(Debug, Clone, Eq, PartialEq, Hash, Default)]
pub struct MultiName {
    names: Vec<Name>,
}

impl MultiName {
    pub fn new(names: Vec<Name>) -> Self {
        Self { names }
    }

    pub fn shrink_to_fit(&mut self) {
        self.names.shrink_to_fit()
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Name(SmolStr);

impl Name {
    fn new_text(text: SmolStr) -> Self {
        Self(text)
    }

    fn resolve(raw_text: &str) -> Self {
        Name::new_text(raw_text.into())
    }
}

pub trait AsName {
    fn as_name(&self) -> Name;
}

impl AsName for ast::Name {
    fn as_name(&self) -> Name {
        Name::resolve(&self.text())
    }
}

impl AsName for ast::NameRef {
    fn as_name(&self) -> Name {
        Name::resolve(&self.text())
    }
}
