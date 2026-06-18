# AGENTS.md

## Project Overview

CodeForge is a Rust library for generating C++ code through an AST-based approach. It provides structured type definitions and emission primitives to build C++ code generators.

## Workspace Structure

- `codeforge-emit`: Core emission primitives (`CodeWriter`, `Emit` trait)
- `codeforge-cpp`: C++ AST definitions and per-node emission implementations

## Commands

```sh
# Build all crates
cargo build --all-features

# Run tests
cargo test --all-features

# Check formatting
cargo fmt --all -- --check

# Apply formatting
cargo fmt --all

# Run clippy lints
cargo clippy --all-features -- -D warnings

# Clean build from scratch
cargo clean && cargo build --all-features
```

## Architecture

### Emission Primitives (`codeforge-emit`)
- `CodeWriter`: Indent-tracking text builder with `write_indent()`, `line()`, `writeln()`, `indent()`, `dedent()`
- `Emit` trait: `fn emit(&self, w: &mut CodeWriter)` implemented per AST node

### C++ AST (`codeforge-cpp`)
- AST types split by context: `Field` (class/struct members with access specifiers) vs `LocalVariable` (statement/namespace scope)
- Per-node `Emit` implementations in `emit.rs`
- Inline conversion methods: `Type::to_cpp()`, `Expression::to_cpp()`, `Literal::to_cpp()`
- Template support via `TemplateDeclaration` wrapping any `Declaration`

## Key Design Decisions

1. **Variable split**: `Field` and `LocalVariable` are separate types to eliminate the contextual `access` field inconsistency
2. **Per-node Emit**: Each AST node implements `Emit` directly rather than using a central generator
3. **Context-aware emission**: `Constructor`/`Destructor` use free functions `emit_constructor()`/`emit_destructor()` that take class name context
4. **No macros**: Template declarations supported, but preprocessor macros intentionally omitted

## Code Style

- Edition 2024 inherited from workspace
- Optional serde feature on AST types via `#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]`
- `F64Wrapper` newtype for `f64` in `Literal::Float` implements `Eq`/`Hash` via `to_bits()`
- Box recursive types to break size cycles: `Statement::If/While/For`, `TemplateDeclaration::declaration`

## Testing

Golden tests in `crates/codeforge-cpp/tests/golden.rs` cover:
- Function declarations with/without bodies
- Class with constructor/destructor/methods/fields
- Enums (scoped and unscoped)
- Templates (type/non-type/template parameters)
- Control flow (if/else, for, while)
- Expressions, casts, literals
- Structs, typedefs, namespaces
