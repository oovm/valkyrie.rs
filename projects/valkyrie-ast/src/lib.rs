// #![feature(trivial_bounds)]
// #![feature(never_type)]
//
// use serde::{Deserialize, Serialize};
//
// use valkyrie_errors::FileSpan;
//
// pub use crate::{
//     control_flow::{
//         for_loop::ForLoop, match_case::MatchCase, pattern::ValkyriePattern, which_case::WhichCase, while_loop::WhileLoop,
//     },
//     expression_level::{
//         binary::BinaryExpression, decimal::ValkyrieDecimalNode, dict::HeterogeneousDict, identifier::ValkyrieIdentifier,
//         integer::ValkyrieIntegerNode, list::HeterogeneousList, string::ValkyrieStringNode, unary::UnaryExpression,
//     },
//     operators::{
//         annotaiton::ValkyrieAnnotation, keywords::ValkyrieKeyword, resolver::ExpressionOrderResolver, OperatorKind,
//         UnknownOrder, ValkyrieOperator,
//     },
//     package_level::{
//         class_field::ClassFieldDeclare, class_method::ClassMethodDeclare, classes::ClassDeclare, NamespaceDeclare,
//         NamespaceKind,
//     },
// };
//
// mod control_flow;
// mod display;
mod expression_level;
mod package_level;
mod utils;

pub use crate::{
    expression_level::{
        apply::{ApplyArgumentNode, ApplyCallNode, ApplyDotNode, ApplyTermNode},
        generic::{GenericArgumentNode, GenericCall},
        number::NumberLiteralNode,
        operators::{InfixNode, OperatorNode, PostfixNode, PrefixNode},
        string::StringLiteralNode,
        symbol::{IdentifierNode, MacroKind, MacroPathNode, NamePathNode},
        table::{TableKind, TableNode},
        view::{ViewNode, ViewRangeNode, ViewTermNode},
    },
    package_level::{NamespaceDeclareNode, NamespaceKind},
    utils::{FileID, FileSpan, FileStorage},
};
pub use valkyrie_types::ValkyrieOperator;
// mod package_level;
//
// #[derive(Clone, Debug, Serialize, Deserialize)]
// pub struct ValkyrieASTNode {
//     pub kind: ValkyrieASTKind,
//     pub span: FileSpan,
// }
//
// #[derive(Clone, Serialize, Deserialize)]
// pub enum ValkyrieASTKind {
//     Statement(Vec<ValkyrieASTNode>),
//     Namespace(Box<NamespaceDeclare>),
//     Class(Box<ClassDeclare>),
//     For(Box<ForLoop>),
//     While(Box<WhileLoop>),
//     Match(Box<MatchCase>),
//     Which(Box<WhichCase>),
//     Binary(Box<BinaryExpression>),
//     Unary(Box<UnaryExpression>),
//     // ()
//     // (1, )
//     // (1, 2, 3)
//     HList(Box<HeterogeneousList>),
//     HDict(Box<HeterogeneousDict>),
//     StringInterpolation(Box<ValkyrieStringNode>),
//     String(String),
//     Namepath(Vec<ValkyrieIdentifier>),
//     Integer(Box<ValkyrieIntegerNode>),
//     Decimal(Box<ValkyrieDecimalNode>),
//     Bytes(Vec<u8>),
//     Boolean(bool),
//     Null,
// }
