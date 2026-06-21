pub mod ast;
pub mod emit;
pub mod helpers;

pub use ast::*;
pub use emit::emit_program;
pub use helpers::{directive, stmt};

pub use codeforge_emit::{CodeWriter, Emit};

pub fn emit(program: &Program) -> String {
    emit_program(program)
}
