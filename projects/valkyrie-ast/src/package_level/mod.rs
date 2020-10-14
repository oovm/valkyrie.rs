pub mod classes;
pub mod documentation;
pub mod enumerates;
pub mod flags;
pub mod function;
pub mod guarantee;
pub mod import;
pub mod let_bind;
pub mod license;
pub mod namespace;
pub mod program;
pub mod statements;
pub mod tagged;
pub mod try_catch;
pub mod unions;

use crate::{
    AnnotationList, AnnotationNode, ApplyArgument, ArgumentTermNode, ClassDeclaration, ClassFieldDeclaration,
    ClassMethodDeclaration, ControlNode, DocumentationNode, EnumerateDeclaration, EnumerateFieldDeclaration, ExpressionBody,
    ExpressionNode, FlagsDeclaration, ForLoop, FunctionDeclaration, FunctionEffectNode, FunctionReturnNode, GenericArgument,
    GuardLetStatement, GuardStatement, IdentifierNode, ImportStatement, LetBindNode, ModifiersNode, NamePathNode,
    NamespaceDeclaration, PatternExpression, StatementBlock, StatementBody, StatementNode, StringLiteralNode, StringTextNode,
    TaggedDeclaration, UnionDeclaration, UnionFieldDeclaration, VariantDeclaration, WhileLoop,
};
use alloc::{
    boxed::Box,
    string::{String, ToString},
    sync::Arc,
    vec,
    vec::Vec,
};
use core::ops::Range;
use deriver::From;
use pretty_print::helpers::SoftBlock;

#[cfg(feature = "pretty-print")]
use pretty_print::{
    helpers::{KAndRBracket, PrettySequence},
    PrettyPrint, PrettyProvider, PrettyTree,
};
