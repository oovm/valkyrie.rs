use super::*;
use crate::helpers::Mir2Lir;
use nyar_wasm::{DependentGraph, WasiEnumeration};

#[derive(Debug)]
pub struct ValkyrieEnumeration {
    pub enumeration_name: Identifier,
    pub indexes: IndexMap<Arc<str>, ValkyrieSemanticNumber>,
}

#[derive(Debug)]
pub struct ValkyrieSemanticNumber {
    pub number_name: Arc<str>,
}

impl AddAssign<ValkyrieEnumeration> for ResolveState {
    fn add_assign(&mut self, rhs: ValkyrieEnumeration) {
        self.items.insert(rhs.enumeration_name.clone(), ModuleItem::Enums(rhs));
    }
}

impl Mir2Lir for ValkyrieEnumeration {
    type Output = ();
    type Context<'a> = &'a ResolveState;

    fn to_lir<'a>(&self, graph: &mut DependentGraph, context: Self::Context<'a>) -> nyar_error::Result<Self::Output> {
        *graph += WasiEnumeration { name: self.enumeration_name.clone(), variants: Default::default() };

        Ok(())
    }
}
