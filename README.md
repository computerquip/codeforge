# CodeForge

CodeForge is a Rust library for generating C++ code through an AST-based approach. It provides structured type definitions and emission primitives to build C++ code generators.

## Workspace Structure

```
codeforge/
├── codeforge-emit/    # Core emission primitives (CodeWriter, Emit trait)
└── codeforge-cpp/     # C++ AST definitions and per-node emission
```

## Usage

Add `codeforge-cpp` to your `Cargo.toml`:

```toml
[dependencies]
codeforge-cpp = "0.1.0"
```

Build an AST and emit C++ code:

```rust
use codeforge_cpp::*;

let program = Program {
    includes: vec!["<iostream>".into()],
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

## Features

- **Full C++ AST**: Functions, classes, structs, enums, templates, namespaces, typedefs
- **Rich type system**: Primitives, pointers, references, const references, arrays, templates
- **Statements**: Control flow (if/else, for, while), expressions, variable declarations
- **Templates**: Type parameters, non-type parameters, template parameters with defaults
- **Per-node emission**: Clean separation between AST structure and code generation
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

This library gives you programmatic control over C++ code generation. You build an AST in Rust, then emit valid C++ code as strings. No templates, no string concatenation — just structured types with emission logic.

## What This Is Not

- Not a C++ parser (use `tree-sitter-cpp` or similar)
- Not a C++ compiler (this generates source code, not binaries)
- Not preprocessor support (no `#define`, `#include` management, or macro expansion)
- Not a high-level framework (you build the AST manually or from your own DSL)

---

**Note:** This codebase was built almost entirely through AI-assisted development (vibe coded). Architecture, implementation, and tests were produced via conversation with an LLM rather than hand-written.
