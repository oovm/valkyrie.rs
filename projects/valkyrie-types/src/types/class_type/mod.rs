use super::*;
use crate::types::method_type::MethodDefinition;
use indexmap::map::Values;
use nyar_error::NyarError;
use std::ops::AddAssign;

mod codegen;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ValkyrieStructure {
    symbol: Vec<Arc<str>>,
    fields: IndexMap<String, FieldDefinition>,
    methods: IndexMap<String, MethodDefinition>,
    span: FileSpan,
}

impl Hash for ValkyrieStructure {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.symbol.hash(state);
        for (k, v) in self.fields.iter() {
            k.hash(state);
        }
    }
}

impl AddAssign<FieldDefinition> for ValkyrieStructure {
    fn add_assign(&mut self, rhs: FieldDefinition) {
        self.fields.insert(rhs.name(), rhs);
    }
}
impl AddAssign<MethodDefinition> for ValkyrieStructure {
    fn add_assign(&mut self, rhs: MethodDefinition) {
        self.methods.insert(rhs.name(), rhs);
    }
}

impl ValkyrieStructure {
    pub fn new(space: &NamePathNode, name: &IdentifierNode) -> Self {
        let mut symbol = Vec::with_capacity(space.path.len() + 1);
        symbol.extend(space.path.iter().map(|s| Arc::from(s.name.as_str())));
        symbol.extend_one(Arc::from(name.name.as_str()));
        Self { symbol, fields: Default::default(), methods: Default::default(), span: Default::default() }
    }
    pub fn name(&self) -> String {
        self.symbol.join("∷")
    }
    pub fn get_field(&self, name: &str) -> Option<&FieldDefinition> {
        self.fields.get(name)
    }
    pub fn add_field(&mut self, field: FieldDefinition) -> Result<()> {
        let name = field.name();
        let span = field.get_span();
        match self.fields.insert(field.name(), field) {
            Some(old) => Err(NyarError::duplicate_key(name, old.get_span(), span)),
            None => Ok(()),
        }
    }
    pub fn get_fields(&self) -> Values<String, FieldDefinition> {
        self.fields.values()
    }
    pub fn add_method(&mut self, method: MethodDefinition) -> Result<()> {
        let name = method.name();
        let span = method.get_span();
        match self.methods.insert(method.name(), method) {
            Some(old) => Err(NyarError::duplicate_key(name, old.get_span(), span)),
            None => Ok(()),
        }
    }
}
