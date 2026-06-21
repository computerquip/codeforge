pub mod ast;
pub mod emit;
pub mod helpers;

pub use ast::*;
pub use emit::emit_module;
pub use helpers::{attr, expr, function, param, stmt, ty, vis};

pub use codeforge_emit::{CodeWriter, Emit};

pub fn emit(module: &Module) -> String {
    emit_module(module)
}
