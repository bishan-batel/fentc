#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct Identifier(String);

impl Identifier {
    pub fn new(name: String) -> Self {
        Self(name)
    }
}
