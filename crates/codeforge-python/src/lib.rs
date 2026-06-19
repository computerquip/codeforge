pub mod ast;
pub mod emit;

pub use ast::*;
pub use emit::emit_module;

pub use codeforge_emit::{CodeWriter, Emit};

pub fn emit(module: &Module) -> String {
    emit_module(module)
}
