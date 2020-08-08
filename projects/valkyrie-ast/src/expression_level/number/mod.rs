use super::*;
mod display;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct NumberLiteralNode {
    pub value: String,
    pub unit: Option<IdentifierNode>,
    pub span: Range<u32>,
}

impl NumberLiteralNode {
    pub fn new<S: ToString>(text: S, start: u32, end: u32) -> NumberLiteralNode {
        NumberLiteralNode { value: text.to_string(), unit: None, span: start..end }
    }
}
