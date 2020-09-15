use crate::{
    helpers::{ignore, parse_in, parse_when},
    utils::{get_span, parse_expression_node, parse_statement_block},
    ThisParser,
};
use lispify::Lisp;
use pex::{helpers::str, ParseResult, ParseState};
use valkyrie_ast::{
    ControlNode, ControlType, ElseStatement, ExpressionContext, ExpressionNode, ForLoop, IfConditionNode, IfStatement,
    PatternExpression, PatternGuard, StatementBlock, StatementNode, WhileConditionNode, WhileLoop,
};

mod controller;
mod jmp_switch;
mod jum_if;
mod loop_for;
mod loop_while;

impl ThisParser for WhileConditionNode {
    fn parse(input: ParseState) -> ParseResult<Self> {
        input
            .begin_choice()
            .or_else(|s| {
                let (state, e) = ExpressionNode::parse(s)?;
                state.finish(WhileConditionNode::Expression(Box::new(e)))
            })
            .or_else(|s| s.finish(WhileConditionNode::AlwaysTrue))
            .end_choice()
    }

    fn as_lisp(&self) -> Lisp {
        match self {
            WhileConditionNode::AlwaysTrue => Lisp::keyword("true"),
            WhileConditionNode::Case => Lisp::keyword("case"),
            WhileConditionNode::Expression(v) => v.as_lisp(),
        }
    }
}

impl ThisParser for StatementBlock {
    fn parse(input: ParseState) -> ParseResult<Self> {
        parse_statement_block(input, StatementNode::parse)
    }

    fn as_lisp(&self) -> Lisp {
        Lisp::Any(vec![Lisp::keyword("body"), Lisp::Any(self.terms.iter().map(|s| s.as_lisp()).collect())])
    }
}
