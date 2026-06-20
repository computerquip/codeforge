# CodeForge

CodeForge is a Rust library for generating source code through an AST-based approach. It provides language-agnostic emission primitives and pluggable AST backends for target languages.

## Workspace Structure

```
codeforge/
├── codeforge-emit/       # Language-agnostic emission primitives (CodeWriter, Emit trait)
├── codeforge-cpp/        # C++ backend — AST definitions and per-node emission
├── codeforge-python/     # Python backend — AST definitions and per-node emission
└── codeforge-rust/       # Rust backend — AST definitions and per-node emission
```

## C++ Backend Usage (`codeforge-cpp`)

Add `codeforge-cpp` to your `Cargo.toml`:

```toml
[dependencies]
codeforge-cpp = "0.1.0"
```

Build an AST and emit C++ code:

```rust
use codeforge_cpp::*;

let program = Program {
    directives: vec![Directive::Include(Include::System("iostream".into()))],
    namespaces: vec![],
    declarations: vec![Declaration::Function(Function {
        name: "greet".into(),
        return_type: Type::Int32,
        parameters: vec![],
        body: Some(Block {
            statements: vec![
                Statement::Expression(Expression::Call {
                    callee: Box::new(Expression::Identifier("std::cout".into())),
                    arguments: vec![Expression::Literal(Literal::String("Hello, World!".into()))],
                }),
                Statement::Return(Some(Expression::Literal(Literal::Integer(0)))),
            ],
        }),
        is_const: false,
        is_inline: false,
        is_static: false,
        is_virtual: false,
        is_pure_virtual: false,
        is_override: false,
        is_noexcept: false,
    })],
};

let cpp_code = emit(&program);
println!("{}", cpp_code);
```

## Rust Backend Usage (`codeforge-rust`)

Add `codeforge-rust` to your `Cargo.toml`:

```toml
[dependencies]
codeforge-rust = "0.3.0"
```

Build an AST and emit Rust code:

```rust
use codeforge_rust::*;

let module = Module {
    attributes: vec![Attribute::derive(vec!["Debug".into()])],
    items: vec![
        Item::Struct(Struct {
            attributes: vec![Attribute::derive(vec!["Debug".into()])],
            visibility: Visibility::Public,
            name: "Greeter".into(),
            generics: Generics::empty(),
            kind: StructKind::Named(vec![Field {
                attributes: vec![],
                visibility: Visibility::Private,
                name: "name".into(),
                ty: Type::path("String"),
            }]),
        }),
    ],
};

let rust_code = emit(&module);
println!("{}", rust_code);
```

## Features

See the [AST Feature Reference](docs/FEATURES.md) for a complete table of all AST node types across all backends.

**Emission core (`codeforge-emit`)**
- **Language-agnostic engine**: `CodeWriter` tracks indentation; `Emit` trait provides per-node codegen
- **Composable backend API**: Implement `Emit` for your target language's AST nodes to create a new backend

**C++ backend (`codeforge-cpp`)**
- **Full C++ AST**: Functions, classes, structs, enums, templates, namespaces, typedefs
- **Rich type system**: Primitives, pointers, references, const references, arrays, templates
- **Preprocessor directives**: `#include`, `#define`, `#ifdef`/`#ifndef`, `#if`/`#elif`/`#else`/`#endif`
- **Conditional compilation**: `Conditional<T>` wraps declarations, class members, or statements in `#if` blocks
- **Statements**: Control flow (if/else, for, while), expressions, variable declarations
- **Templates**: Type parameters, non-type parameters, template parameters with defaults
- **Per-node emission**: Clean separation between AST structure and code generation
- **Optional serde support**: Serialize/deserialize AST nodes

**Python backend (`codeforge-python`)**
- **Full Python AST**: Functions, classes, imports, decorators, control flow
- **Type annotations**: Type hints with generics, Optional, Union, Callable
- **Pythonic output**: PEP 8 spacing (2 blank lines between top-level defs, 1 between class methods)
- **Statements**: if/elif/else, for/while (with else clauses), break, continue, pass
- **Expressions**: Binary/unary ops (including `in`, `not in`, `is`, `is not`), tuples, lists, dicts, sets, lambdas, ternary
- **Function signatures**: Parameters with annotations/defaults, *args, keyword-only, **kwargs
- **Optional serde support**: Serialize/deserialize AST nodes

**Rust backend (`codeforge-rust`)**
- **Full Rust AST**: Functions, structs, enums, traits, impls, use declarations, modules
- **Generics**: Type/lifetime/const params, where clauses, turbofish
- **Visibility**: pub, pub(crate), pub(super), pub(in path)
- **Attributes & derives**: `#[derive(...)]`, `#[inline]`, `#![allow(...)]`
- **Statements**: let (with mut, type annotations, let-else), expression stmts, nested items
- **Expressions**: binary/unary ops, method calls, field/index access, references/deref/try, casts, closures
- **Control flow**: if/else if/else, if let, match with guards, loop/while/for with labels
- **Types**: references with lifetimes, raw pointers, tuples, slices, arrays, dyn/impl trait, fn pointers
- **Optional serde support**: Serialize/deserialize AST nodes

## Development

```sh
# Build all crates
cargo build --all-features

# Run tests
cargo test --all-features

# Check formatting
cargo fmt --all -- --check

# Run clippy
cargo clippy --all-features -- -D warnings
```

## What This Is

This library gives you programmatic control over source code generation. You define a target language's AST in Rust and emit valid source code as strings — no string concatenation, just structured types with emission logic. The `codeforge-emit` core is language-agnostic; language-specific backends (e.g. `codeforge-cpp`) plug into it.

## What This Is Not

- Not a source code parser (use `tree-sitter-*` or similar)
- Not a compiler (this generates source code, not binaries)
- Not a full preprocessor (supports `#include`, `#define`, `#if`/`#elif`/`#else`/`#endif`, but no macro expansion)
- Not a high-level framework (you build the AST manually or from your own DSL)

---

**Note:** This codebase was built almost entirely through AI-assisted development (vibe coded). Architecture, implementation, and tests were produced via conversation with an LLM rather than hand-written.
