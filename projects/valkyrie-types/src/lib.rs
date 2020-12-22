#![feature(trivial_bounds)]
#![feature(allocator_api)]

mod builtin;
mod codegen;
mod functions;
mod modifiers;
// #[cfg(test)]
mod encoding;
pub mod testing;
pub mod third_party;
mod traits;
mod types;
mod utils;
mod validation;
mod values;

pub use self::{
    builtin::{
        images::ValkyrieImage,
        result::{ValkyrieFailure, ValkyrieSuccess},
        TokenType,
    },
    functions::{ValkyrieFunction, ValkyrieFunctionType, ValkyrieMonomorphicFunction},
    types::{
        atomic_type::ValkyrieAtomicType, class_type::ValkyrieClassType, literal_type::ValkyrieLiteralType,
        tuple_type::ValkyrieTable, union_type::ValkyrieUnionType, variant_type::ValkyrieVariantType, ValkyrieType,
    },
    values::ValkyrieValue,
};
pub use valkyrie_ast::ValkyrieOperator;
pub use valkyrie_error::{JsonValue, RuntimeError, SyntaxError, ValkyrieError, ValkyrieResult};
