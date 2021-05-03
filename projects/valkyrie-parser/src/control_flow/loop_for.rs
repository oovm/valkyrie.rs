use super::*;
use valkyrie_ast::ForLoop;

impl crate::ForStatementNode {
    pub fn build(&self, ctx: &ProgramContext) -> Validation<ForLoop> {
        Success {
            value: ForLoop {
                pattern: self.let_pattern.build(ctx)?,
                iterator: Default::default(),
                condition: None,
                then: self.continuation.build(ctx)?,
                span: self.span.clone(),
            },
            diagnostics: vec![],
        }
    }
}
