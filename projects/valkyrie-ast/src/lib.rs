#![no_std]
#![deny(missing_debug_implementations, missing_copy_implementations)]
#![warn(missing_docs, rustdoc::missing_crate_level_docs)]
#![doc = include_str!("../readme.md")]
#![doc(html_logo_url = "https://avatars.githubusercontent.com/u/91894079")]
#![doc(html_favicon_url = "https://avatars.githubusercontent.com/u/91894079")]

extern crate alloc;

mod control_flow;
mod expression_level;
pub mod helper;
mod package_level;
mod patterns;
mod string_like;

pub use crate::{
    control_flow::{
        control::{ControlNode, ControlType, RaiseNode, TailCallNode},
        jmp_guard::{GuardPattern, GuardStatement},
        jmp_if::{BreakStatement, ElseStatement, IfBranchNode, IfStatement, JumpStatement},
        jmp_switch::SwitchStatement,
        loop_for::{ForBarePattern, ForLoop},
        loop_while::{OtherwiseStatement, WhileConditionNode, WhileLoop, WhileLoopKind},
    },
    expression_level::{
        annotations::{
            AnnotationKind, AnnotationList, AnnotationNode, AnnotationPathNode, AnnotationTerm, ModifiedNode, ModifiersNode,
        },
        apply::{ApplyArgument, ApplyArgumentTerm, ApplyCallItem, ApplyCallNode, ApplyCallTerms, ApplyCaller, ArgumentKeyNode},
        array::{ArrayKind, ArrayNode, ArrayTermNode, SubscriptCallNode},
        common::{ArgumentTermNode, CallNode, CallTermNode, MonadicCall, MonadicDotCall},
        ctor::{CollectsNode, NewConstructNode},
        generic::{GenericArgument, GenericArgumentTerm, GenericCallNode, GenericCallTerm},
        lambda::{ClosureCallNode, ClosureCaller, FunctionBlock, LambdaNode},
        matches::{MatchDotStatement, MatchKind},
        number::NumberLiteralNode,
        operators::{BinaryNode, LogicMatrix, OperatorNode, PostfixNode, UnaryNode, ValkyrieOperator},
        symbol::{BooleanNode, IdentifierNode, LambdaSlotNode, NamePathNode, NullNode, OutputNode},
        tuple::{TupleKeyType, TupleKind, TupleNode, TupleTermNode},
        ExpressionContext, ExpressionNode, ExpressionType, PostfixCallPart, TypingExpression,
    },
    package_level::{
        classes::{ClassDeclaration, ClassFieldDeclaration, ClassKind, ClassMethodDeclaration},
        documentation::DocumentationNode,
        enumerates::{EnumerateDeclaration, EnumerateFieldDeclaration},
        flags::{FlagsDeclaration, FlagsIterator, FlagsTerm},
        function::{
            FunctionDeclaration, FunctionDeclarationInline, FunctionEffectNode, FunctionReturnNode, FunctionType,
            StatementBlock,
        },
        guarantee::{EffectTypeNode, GuaranteeNode},
        import::{ImportAliasNode, ImportGroupNode, ImportResolvedItem, ImportState, ImportStatement, ImportTermNode},
        labeled::{GotoStatement, LabelStatement},
        let_bind::LetBindNode,
        namespace::{NamespaceDeclaration, NamespaceKind},
        program::ProgramRoot,
        statements::{StatementContext, StatementNode},
        tagged::{TaggedDeclaration, VariantDeclaration},
        traits::{ExtendsStatement, TraitDeclaration},
        try_catch::TryStatement,
        unions::{UnionDeclaration, UnionFieldDeclaration},
    },
    patterns::{
        ArrayPatternNode, ClassPatternNode, IdentifierPattern, ImplicitCaseNode, LetPattern, PatternBlock, PatternBranch,
        PatternCaseNode, PatternCondition, PatternElseNode, PatternGuard, PatternStatements, PatternTypeNode, PatternWhenNode,
        TuplePatternNode, UnionPatternNode,
    },
    string_like::{
        string_formatter::{ExpressionFormatted, StringFormatter},
        string_literal::{StringLiteralNode, StringTextNode},
        string_template::{
            StringTemplateNode, TemplateCloseNode, TemplateCommentNode, TemplateInlineNode, TemplateLineType, TemplateOpenNode,
        },
    },
};
pub use num_bigint::BigUint;
