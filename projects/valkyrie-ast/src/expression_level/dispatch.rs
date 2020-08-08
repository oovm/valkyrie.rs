use super::*;
use crate::PrettyTree;

impl Default for ExpressionBody {
    fn default() -> Self {
        Self::Placeholder
    }
}
impl PrettyPrint for ExpressionNode {
    fn build<'a>(&self, allocator: &'a PrettyProvider<'a>) -> PrettyTree<'a> {
        self.body.build(allocator)
    }
}

impl PrettyPrint for ExpressionBody {
    fn build<'a>(&self, allocator: &'a PrettyProvider<'a>) -> PrettyTree<'a> {
        match self {
            ExpressionBody::Placeholder => allocator.text("???"),
            ExpressionBody::Symbol(node) => node.build(allocator),
            ExpressionBody::Number(node) => node.build(allocator),
            ExpressionBody::String(node) => node.build(allocator),
            ExpressionBody::Prefix(node) => node.build(allocator),
            ExpressionBody::Binary(node) => node.build(allocator),
            ExpressionBody::Suffix(node) => node.build(allocator),
            ExpressionBody::Table(node) => node.build(allocator),
            ExpressionBody::Apply(node) => node.build(allocator),
            ExpressionBody::ApplyDot(node) => node.build(allocator),
            ExpressionBody::LambdaCall(node) => node.build(allocator),
            ExpressionBody::LambdaDot(node) => node.build(allocator),
            ExpressionBody::Subscript(node) => node.build(allocator),
            ExpressionBody::GenericCall(node) => node.build(allocator),
            ExpressionBody::New(node) => node.build(allocator),
        }
    }
}

impl ExpressionBody {
    pub fn span(&self) -> Range<u32> {
        match self {
            ExpressionBody::Placeholder => unreachable!(),
            ExpressionBody::Symbol(node) => node.span.clone(),
            ExpressionBody::Number(node) => node.span.clone(),
            ExpressionBody::String(node) => node.span.clone(),
            ExpressionBody::New(node) => node.span.clone(),
            ExpressionBody::Prefix(node) => node.span.clone(),
            ExpressionBody::Binary(node) => node.span.clone(),
            ExpressionBody::Suffix(node) => node.span.clone(),
            ExpressionBody::Table(node) => node.span.clone(),
            ExpressionBody::Apply(node) => node.span.clone(),
            ExpressionBody::ApplyDot(node) => node.span.clone(),
            ExpressionBody::LambdaCall(node) => node.span.clone(),
            ExpressionBody::LambdaDot(node) => node.span.clone(),
            ExpressionBody::Subscript(node) => node.span.clone(),
            ExpressionBody::GenericCall(node) => node.span.clone(),
        }
    }
}
impl From<PrefixNode> for ExpressionBody {
    fn from(value: PrefixNode) -> Self {
        Self::Prefix(Box::new(value))
    }
}

impl From<NewConstructNode> for ExpressionBody {
    fn from(value: NewConstructNode) -> Self {
        Self::New(Box::new(value))
    }
}

// impl From<InfixNode> for ExpressionBody {
//     fn from(value: InfixNode) -> Self {
//         Self::Binary(Box::new(value))
//     }
// }
// impl From<PostfixNode> for ExpressionBody {
//     fn from(value: PostfixNode) -> Self {
//         Self::Suffix(Box::new(value))
//     }
// }
//
// impl From<TableNode> for ExpressionBody {
//     fn from(value: TableNode) -> Self {
//         Self::Table(Box::new(value))
//     }
// }

impl From<StringLiteralNode> for ExpressionBody {
    fn from(value: StringLiteralNode) -> Self {
        Self::String(Box::new(value))
    }
}

impl From<NumberLiteralNode> for ExpressionBody {
    fn from(value: NumberLiteralNode) -> Self {
        Self::Number(Box::new(value))
    }
}

impl From<NamePathNode> for ExpressionBody {
    fn from(value: NamePathNode) -> Self {
        Self::Symbol(Box::new(value))
    }
}
