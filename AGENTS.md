# AGENTS.md

## Project Overview

CodeForge is a Rust library for generating source code through an AST-based approach. It provides language-agnostic emission primitives and pluggable AST backends for target languages.

## Workspace Structure

- `codeforge-emit`: Language-agnostic emission primitives (`CodeWriter`, `Emit` trait)
- `codeforge-cpp`: C++ backend — AST definitions and per-node emission implementations
- `codeforge-python`: Python backend — AST definitions and per-node emission implementations

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
- Template support via `Template` wrapping any `Declaration`
- Preprocessor directives: `Directive` enum with `Include`, `Define`, `Undef`, `Ifdef`, `Ifndef`, `Error`, `Pragma`, `Conditional`
- `Include` enum: `System(String)` (angle brackets) and `Local(String)` (double quotes)
- `Conditional<T>` wraps declarations, class members, or statements in `#if`/`#elif`/`#else`/`#endif` blocks
- `ClassMember::Conditional` uses `emit_conditional_class_members()` to pass class name context for constructors/destructors
- `Program.directives` replaces former `includes` field

### Python AST (`codeforge-python`)
- AST includes `Module`, `FunctionDef`, `ClassDef`, imports, decorators, and full statement/expression types
- Per-node `Emit` implementations in `emit.rs`
- Inline conversion methods: `Type::to_python()`, `Expression::to_python()`, `Literal::to_python()`
- Shared `emit_body()` helper with configurable spacing between definitions (2-blank at module level, 1-blank in class bodies)
- PEP 8 blank-line rules: 2 blank lines between top-level defs/classes
- `pass` emitted automatically for empty bodies without docstrings
- `F64Wrapper` newtype mirrors C++ crate's approach for `Literal::Float`

## Key Design Decisions

1. **Variable split**: `Field` and `LocalVariable` are separate types to eliminate the contextual `access` field inconsistency
2. **Per-node Emit**: Each AST node implements `Emit` directly rather than using a central generator
3. **Context-aware emission**: `Constructor`/`Destructor` use free functions `emit_constructor()`/`emit_destructor()` that take class name context
4. **Conditional compilation**: `Conditional<T>` generic struct supports `#if`/`#elif`/`#else`/`#endif` for declarations, class members, and statements

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
- Preprocessor directives (#include with system/local includes)

Golden tests in `crates/codeforge-python/tests/golden.rs` cover:
- Function definitions with params, annotations, defaults, *args/**kwargs
- Class definitions with bases, keywords, methods
- Imports (simple, from, star, aliases)
- Control flow (if/elif/else, for/while with else clauses)
- Decorators, docstrings, async functions
- Expressions (binary/unary ops, calls, attributes, subscripts)
- Literals, tuples, lists, dicts, sets, lambdas, ternary
- PEP 8 blank-line spacing between definitions
