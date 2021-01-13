use super::*;

#[cfg(feature = "pretty-print")]
impl PrettyPrint for StatementNode {
    fn pretty(&self, theme: &PrettyProvider) -> PrettyTree {
        match self {
            Self::Nothing => ";;".into(),
            Self::Annotation(node) => node.pretty(theme),
            Self::Namespace(node) => node.pretty(theme),
            Self::Import(node) => node.pretty(theme),
            Self::Class(node) => node.pretty(theme),
            Self::Tagged(node) => node.pretty(theme),
            Self::Variant(node) => node.pretty(theme),
            Self::Union(node) => node.pretty(theme),
            Self::UnionField(node) => node.pretty(theme),
            Self::Enumerate(node) => node.pretty(theme),
            Self::EnumerateField(node) => node.pretty(theme),
            Self::Function(node) => node.pretty(theme),
            Self::While(node) => node.pretty(theme),
            Self::For(node) => node.pretty(theme),
            Self::Expression(node) => node.pretty(theme),
            Self::Control(node) => node.pretty(theme),
            Self::Document(node) => node.pretty(theme),
            Self::LetBind(node) => node.pretty(theme),
            Self::Guard(node) => node.pretty(theme),
            Self::Flags(node) => node.pretty(theme),
            Self::Trait(node) => node.pretty(theme),
            Self::Extends(node) => node.pretty(theme),
        }
    }
}

#[cfg(feature = "lispify")]
impl Lispify for StatementNode {
    type Output = Lisp;

    fn lispify(&self) -> Self::Output {
        match self {
            Self::Nothing => Lisp::default(),
            Self::Namespace(v) => v.lispify(),
            Self::Import(v) => todo!(),
            Self::While(v) => v.lispify(),
            Self::For(v) => v.lispify(),
            Self::Class(v) => v.lispify(),
            Self::Expression(v) => v.lispify(),
            Self::Function(v) => v.lispify(),
            Self::Control(v) => todo!(),
            Self::Document(v) => todo!(),
            Self::LetBind(v) => todo!(),
            Self::Guard(v) => v.lispify(),
            Self::Flags(v) => v.lispify(),
            Self::EnumerateField(v) => todo!(),
            Self::Tagged(v) => todo!(),
            Self::Variant(v) => todo!(),
            Self::Union(v) => v.lispify(),
            Self::Enumerate(v) => todo!(),
            Self::UnionField(v) => todo!(),
            Self::Annotation(v) => todo!(),
            Self::Trait(v) => v.lispify(),
            Self::Extends(v) => v.lispify(),
        }
    }
}
