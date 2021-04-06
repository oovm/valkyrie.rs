use crate::{ModifierCallNode, ProgramContext, TupleTermsNode};
use nyar_error::{Success, Validation};
use valkyrie_ast::{ArgumentsList, ModifiersNode};

pub fn build_modifiers(m: &[ModifierCallNode], ctx: &ProgramContext) -> ModifiersNode {
    ModifiersNode { terms: m.iter().map(|v| v.identifier.build(ctx)).collect() }
}
pub fn build_arguments(this: &Option<TupleTermsNode>, ctx: &ProgramContext) -> Validation<ArgumentsList> {
    match this {
        Some(s) => s.build(ctx),
        None => Success { value: ArgumentsList { terms: vec![] }, diagnostics: vec![] },
    }
}
