pub mod classes;
mod dispatch;
pub mod import;
pub mod license;
pub mod namespace;
use crate::{
    expression_level::ExpressionNode,
    package_level::{classes::ClassDeclarationNode, namespace::NamespaceDeclarationNode},
    ForLoopNode, IdentifierNode, ImportStatementNode, WhileLoopNode,
};
use alloc::{boxed::Box, string::String, vec::Vec};
use core::{
    fmt::{Display, Formatter, Write},
    ops::Range,
};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct StatementNode {
    pub r#type: StatementType,
    pub eos: bool,
    pub range: Range<usize>,
}

/// The top level elements in script mode.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum StatementType {
    Nothing,
    Namespace(Box<NamespaceDeclarationNode>),
    Import(Box<ImportStatementNode>),
    Class(Box<ClassDeclarationNode>),
    While(Box<WhileLoopNode>),
    For(Box<ForLoopNode>),
    Expression(Box<ExpressionNode>),
}
