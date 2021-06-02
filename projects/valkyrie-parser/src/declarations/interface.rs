use super::*;

impl crate::DefineTraitNode {
    pub(crate) fn build(&self, ctx: &mut ProgramState) -> Result<TraitDeclaration> {
        Ok(TraitDeclaration { name: self.identifier.build(ctx), terms: vec![] })
    }
}

impl crate::KwTraitNode {
    // pub(crate) fn build(&self) -> FunctionType {
    //     match self {
    //
    //     }
    // }
}
