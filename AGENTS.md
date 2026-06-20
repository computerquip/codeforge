# AGENTS.md

## Project Overview

CodeForge is a Rust library for generating source code through an AST-based approach. It provides language-agnostic emission primitives and pluggable AST backends for target languages.

## Workspace Structure

- `codeforge-emit`: Language-agnostic emission primitives (`CodeWriter`, `Emit` trait)
- `codeforge-cpp`: C++ backend — AST definitions and per-node emission implementations
- `codeforge-python`: Python backend — AST definitions and per-node emission implementations
- `codeforge-rust`: Rust backend — AST definitions and per-node emission implementations

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

### Rust AST (`codeforge-rust`)
- AST includes `Module`, `Function`, `Struct`, `Enum`, `Trait`, `Impl`, `TypeAlias`, `Const`, `Static`, `Mod`
- Per-node `Emit` implementations in `emit.rs`
- Inline conversion methods: `Type::to_rust()`, `Expression::to_rust()`, `Literal::to_rust()`
- Full generic support: type params, lifetime params, const params, where clauses
- Visibility: `Private`, `Public`, `Crate`, `Super`, `Restricted(path)`
- Attributes/derives as first-class AST nodes
- `Block` has optional `trailing_expr` for expression blocks
- `emit_expr_stmt()` helper decides block-like vs value-like expression statement emission
- `F64Wrapper` newtype for `Literal::Float`
- `Box` recursive types: `Block::trailing_expr`, `Expression::Cast::ty`, loop/while/for bodies, `IfCondition`, `Type::Array` count

## Key Design Decisions

1. **Variable split**: `Field` and `LocalVariable` are separate types to eliminate the contextual `access` field inconsistency
2. **Per-node Emit**: Each AST node implements `Emit` directly rather than using a central generator
3. **Context-aware emission**: `Constructor`/`Destructor` use free functions `emit_constructor()`/`emit_destructor()` that take class name context
4. **Conditional compilation**: `Conditional<T>` generic struct supports `#if`/`#elif`/`#else`/`#endif` for declarations, class members, and statements
5. **Expression statement dispatch**: `emit_expr_stmt()` decides block-like (if/match/loop/while/for/block) vs value-like (single-line + `;`) emission for Rust
6. **Item spacing**: One blank line between top-level Rust items; consecutive `use` items kept tight (no blank between them)

## Code Style

- Edition 2024 inherited from workspace
- Optional serde feature on AST types via `#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]`
- `F64Wrapper` newtype for `f64` in `Literal::Float` implements `Eq`/`Hash` via `to_bits()`
- Box recursive types to break size cycles: `Statement::If/While/For`, `Block::trailing_expr`, loop/while/for bodies, `IfCondition`, `Expression::Cast::ty`, `Type::Array` count

## Testing

Golden tests in `crates/codeforge-cpp/tests/golden.rs` cover:
- Function declarations with/without bodies
- Class with constructor/destructor/methods/fields
- Enums (scoped/unscoped)
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

Golden tests in `crates/codeforge-rust/tests/golden.rs` cover:
- Free functions: params, generics, where clauses, async/const/unsafe, extern ABI
- Visibility variants (pub, pub(crate), pub(super), pub(in path))
- Attributes and derives
- Structs (unit, tuple, named with field visibility)
- Enums (unit/tuple/struct variants, discriminants)
- Traits (methods, associated types/consts, supertraits)
- Impls (inherent, trait impl, unsafe, generics with where clause)
- Use declarations (simple, alias, glob, nested groups)
- Type alias, const, static (incl. static mut)
- Mod (inline and declaration form)
- Statements (let, expression, nested items, comments)
- Control flow (if/else if/else, if let, match with guards, loop/while/for with labels)
- Expressions (binary/unary, method calls, field/index access, references/deref/try, casts)
- Struct literals (with shorthand and ..rest spread)
- Closures (move, typed, block body)
- Macro calls, tuples, arrays, ranges
- Types (references with lifetimes, pointers, slices, dyn/impl trait, fn pointers)
- Literals (integers, floats incl. NaN/inf, strings with escapes, chars)
- Inter-item spacing: consecutive use items tight, blank lines between other items

## Multi-Component Audits

When verifying multiple similar components (e.g., both C++ and Python backends) for feature parity, documentation accuracy, or implementation completeness:

1. **Use parallel agents** — one per component. This prevents attention decay and ensures each component gets independent scrutiny.

2. **Define a per-agent checklist** that each must execute independently. For backend verification:
   - For each AST struct, verify every field is read in `emit.rs` (flags set-but-never-emitted fields)
   - For each documented output, verify the actual emission matches (flags wrong documentation)
   - For each documented feature/behavior, confirm the emit path exists (flags missing implementations)

3. **Both the parallelization and the checklist matter** — splitting into agents without a procedure risks inconsistent depth; a checklist on a single agent risks attention decay. The procedure guarantees thoroughness; the agents guarantee attention.
