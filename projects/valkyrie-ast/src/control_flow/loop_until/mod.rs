use super::*;

mod display;

#[doc = include_str!("readme.md")]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LoopUntil {
    /// The kind of while loop, including `while` and `until`
    pub keyword: Range<u32>,
    /// The condition of the loop
    pub condition: UntilConditionNode,
    /// The main body of the loop
    pub then: StatementBlock,
    /// The range of the node
    pub span: Range<u32>,
}

/// `while true`
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum UntilConditionNode {
    /// `while true {}`
    Expression(ExpressionKind),
    /// `while let Some(_) = ... {}`
    NotCase(PatternNode),
}

impl ValkyrieNode for LoopUntil {
    fn get_range(&self) -> Range<u32> {
        self.span.clone()
    }
}
