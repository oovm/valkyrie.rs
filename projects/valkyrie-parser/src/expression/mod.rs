mod expression_kind;
mod infix;
mod pattern_match;
mod prefix;
mod suffix;

use crate::{
    helpers::{ignore, parse_when},
    table::TupleNode,
    traits::ThisParser,
    utils::{get_span, parse_expression_body, parse_expression_node},
};
use lispify::Lisp;
use pex::{helpers::str, BracketPattern, ParseResult, ParseState, Regex, StopBecause};
use pratt::{Affix, Associativity, PrattError, PrattParser, Precedence};
use std::{
    fmt::{Debug, Formatter},
    ops::Range,
    sync::LazyLock,
};
use valkyrie_ast::{
    ApplyCallNode, ApplyDotNode, ArgumentKeyNode, ExpressionBody, ExpressionContext, ExpressionNode, GenericCallNode,
    IfStatement, InfixNode, LambdaCallNode, LambdaDotNode, LambdaSlotNode, NamePathNode, NewConstructNode, NumberLiteralNode,
    OperatorNode, PatternBranch, PatternCaseNode, PatternCondition, PatternElseNode, PatternExpression, PatternGuard,
    PatternStatements, PatternTypeNode, PatternWhenNode, PostfixCallPart, PostfixNode, PrefixNode, RaiseNode, StatementNode,
    StringLiteralNode, SubscriptNode, SwitchStatement, TableNode, TypingExpression, ValkyrieOperator,
};

/// A resolver
#[derive(Default)]
pub struct ExpressionResolver {}

#[derive(Clone, Debug)]
pub enum ExpressionStream {
    Prefix(ValkyriePrefix),
    Postfix(ValkyrieSuffix),
    Infix(ValkyrieInfix),
    Term(ExpressionBody),
    Group(Vec<ExpressionStream>),
}

#[derive(Clone)]
pub struct ValkyrieInfix {
    pub normalized: String,
    pub span: Range<u32>,
}

#[derive(Clone)]
pub struct ValkyriePrefix {
    pub normalized: String,
    pub span: Range<u32>,
}

#[derive(Clone)]
pub struct ValkyrieSuffix {
    pub normalized: String,
    pub span: Range<u32>,
}

impl ThisParser for TypingExpression {
    fn parse(input: ParseState) -> ParseResult<Self> {
        let (state, out) = parse_expression_node(input, ExpressionContext::in_type())?;
        state.finish(TypingExpression { body: out.body, span: get_span(input, state) })
    }

    fn as_lisp(&self) -> Lisp {
        unreachable!()
    }
}

impl ExpressionResolver {
    pub fn resolve(&self, stream: Vec<ExpressionStream>) -> Result<ExpressionBody, StopBecause> {
        // println!("stream: {stream:#?}");
        let mut effect = ExpressionResolver {};
        match effect.parse(stream.into_iter()) {
            Ok(o) => Ok(o),
            Err(e) => say_stop_reason(e),
        }
    }
}

fn say_stop_reason<T>(e: PrattError<ExpressionStream, StopBecause>) -> Result<T, StopBecause> {
    match e {
        PrattError::UserError(e) => Err(e),
        PrattError::EmptyInput => Err(StopBecause::ShouldNotBe { message: "EOF", position: 0 }),
        PrattError::UnexpectedNilfix(_) => unreachable!(),
        PrattError::UnexpectedPrefix(_) => unreachable!(),
        PrattError::UnexpectedInfix(_) => unreachable!(),
        PrattError::UnexpectedPostfix(_) => unreachable!(),
    }
}

impl<I> PrattParser<I> for ExpressionResolver
where
    I: Iterator<Item = ExpressionStream>,
{
    type Error = StopBecause;
    type Input = ExpressionStream;
    type Output = ExpressionBody;

    // Query information about an operator (Affix, Precedence, Associativity)
    fn query(&mut self, tree: &ExpressionStream) -> Result<Affix, StopBecause> {
        let affix = match tree {
            ExpressionStream::Infix(o) => Affix::Infix(o.precedence(), o.associativity()),
            ExpressionStream::Postfix(o) => Affix::Postfix(o.precedence()),
            ExpressionStream::Prefix(o) => Affix::Prefix(o.precedence()),
            ExpressionStream::Group(_) => Affix::Nilfix,
            ExpressionStream::Term(_) => Affix::Nilfix,
        };
        Ok(affix)
    }

    // Construct a primary expression, e.g. a number
    fn primary(&mut self, tree: ExpressionStream) -> Result<ExpressionBody, StopBecause> {
        match tree {
            ExpressionStream::Term(term) => Ok(term),
            ExpressionStream::Group(group) => match self.parse(&mut group.into_iter()) {
                Ok(o) => Ok(o),
                Err(e) => say_stop_reason(e),
            },
            _ => unreachable!(),
        }
    }

    // Construct a binary infix expression, e.g. 1+1
    fn infix(
        &mut self,
        lhs: ExpressionBody,
        tree: ExpressionStream,
        rhs: ExpressionBody,
    ) -> Result<ExpressionBody, StopBecause> {
        match tree {
            ExpressionStream::Infix(o) => Ok(ExpressionBody::binary(o.as_operator(), lhs, rhs)),
            _ => unreachable!(),
        }
    }

    // Construct a unary prefix expression, e.g. !1
    fn prefix(&mut self, tree: ExpressionStream, rhs: ExpressionBody) -> Result<ExpressionBody, StopBecause> {
        match tree {
            ExpressionStream::Prefix(o) => Ok(ExpressionBody::prefix(o.as_operator(), rhs)),
            _ => unreachable!(),
        }
    }

    // Construct a unary postfix expression, e.g. 1?
    fn postfix(&mut self, lhs: ExpressionBody, tree: ExpressionStream) -> Result<ExpressionBody, StopBecause> {
        match tree {
            ExpressionStream::Postfix(o) => Ok(ExpressionBody::suffix(o.as_operator(), lhs)),
            _ => unreachable!(),
        }
    }
}
