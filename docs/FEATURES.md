# CodeForge AST Reference

Complete reference of AST node types available in each backend.

## C++ Backend (`codeforge-cpp`)

### Program Structure

| Type | Fields | Notes |
|------|--------|-------|
| `Program` | `directives`, `namespaces`, `declarations` | Top-level entry point |
| `Namespace` | `name`, `declarations` | Wraps declarations in `namespace {}` |
| `Declaration` (enum) | `Function`, `Class`, `Struct`, `Variable`, `Enum`, `Typedef`, `Template`, `Conditional` | Top-level declaration variants |

### Preprocessor Directives

| Type | Fields | Notes |
|------|--------|-------|
| `Directive` (enum) | `Include`, `Define`, `Undef`, `Ifdef`, `Ifndef`, `Error`, `Pragma`, `Conditional` | Preprocessor directives |
| `Include` (enum) | `System(String)`, `Local(String)` | System uses angle brackets, local uses double quotes |

### Functions

| Type | Fields | Notes |
|------|--------|-------|
| `Function` | `name`, `return_type`, `parameters`, `body`, `is_const`, `is_inline`, `is_static`, `is_virtual`, `is_pure_virtual`, `is_override`, `is_noexcept` | `body: Option<Block>` — declaration-only when `None` |
| `Parameter` | `name`, `param_type`, `default_value` | |

### Classes

| Type | Fields | Notes |
|------|--------|-------|
| `Class` | `name`, `base_classes`, `members`, `is_final` | |
| `BaseClass` | `name`, `access`, `is_virtual` | |
| `AccessSpecifier` (enum) | `Public`, `Protected`, `Private` | |
| `ClassMember` (enum) | `Field`, `Method`, `Constructor`, `Destructor`, `Access`, `Conditional` | `Access` emits `public:`/`protected:`/`private:` labels |

### Constructors & Destructors

| Type | Fields | Notes |
|------|--------|-------|
| `Constructor` | `parameters`, `initializer_list`, `body`, `is_explicit`, `is_deleted`, `is_defaulted` | Supports `= delete` and `= default` |
| `MemberInitializer` | `member_name`, `value` | Emitted in constructor initializer list |
| `Destructor` | `is_virtual`, `is_deleted`, `is_defaulted` | Supports `virtual`, `= delete`, `= default` |

### Structs

| Type | Fields | Notes |
|------|--------|-------|
| `Struct` | `name`, `fields` | Simplified — no methods, base classes, or access specifiers |

### Fields & Variables

| Type | Fields | Notes |
|------|--------|-------|
| `Field` | `name`, `var_type`, `initializer`, `access`, `is_const`, `is_static`, `is_thread_local` | Class/struct member with access specifier |
| `LocalVariable` | `name`, `var_type`, `initializer`, `is_const`, `is_static`, `is_thread_local` | Namespace/block scope — no access specifier |

### Enums

| Type | Fields | Notes |
|------|--------|-------|
| `Enum` | `name`, `underlying_type`, `variants`, `is_scoped` | `is_scoped` selects `enum class` vs `enum` |
| `EnumVariant` | `name`, `value` | `value` is optional custom initializer |

### Typedefs

| Type | Fields | Notes |
|------|--------|-------|
| `Typedef` | `name`, `alias` | Emits `using name = alias;` |

### Templates

| Type | Fields | Notes |
|------|--------|-------|
| `Template` | `parameters`, `declaration` | Wraps any `Declaration` with template prefix |
| `TemplateParameter` (enum) | `Type`, `NonType`, `Template` | Each variant supports optional `default` |

### Types

| Variant | Output | Notes |
|---------|--------|-------|
| `Void` | `void` | |
| `Bool` | `bool` | |
| `Int8`..`Int64` | `int8_t`..`int64_t` | Fixed-width signed integers |
| `UInt8`..`UInt64` | `uint8_t`..`uint64_t` | Fixed-width unsigned integers |
| `Float32` | `float` | |
| `Float64` | `double` | |
| `Char` | `char` | |
| `String` | `std::string` | |
| `Custom(String)` | verbatim | User-defined type name |
| `Pointer(Box<Type>)` | `T*` | |
| `Reference(Box<Type>)` | `T&` | |
| `ConstReference(Box<Type>)` | `const T&` | |
| `Array(Box<Type>, Option<usize>)` | `T[N]` or `T[]` | |
| `Template { name, arguments }` | `Name<Args...>` | |
| `Auto` | `auto` | |

### Statements

| Variant | Notes |
|---------|-------|
| `Expression(Expression)` | Expression statement (with semicolon) |
| `Return(Option<Expression>)` | `return expr;` or `return;` |
| `If(Box<IfStatement>)` | `if`/`else` |
| `While(Box<WhileStatement>)` | `while` loop |
| `For(Box<ForStatement>)` | C-style `for (init; cond; update)` |
| `VariableDeclaration(LocalVariable)` | Local variable declaration |
| `Break` | `break;` |
| `Continue` | `continue;` |
| `Comment(String)` | `// comment` |
| `Raw(String)` | Escape hatch — emitted verbatim |
| `Conditional(Conditional<Statement>)` | `#if`/`#elif`/`#else`/`#endif` around statements |

### Control Flow

| Type | Fields | Notes |
|------|--------|-------|
| `IfStatement` | `condition`, `then_block`, `else_block` | |
| `WhileStatement` | `condition`, `body` | |
| `ForStatement` | `initializer`, `condition`, `update`, `body` | Traditional C-style 3-clause for |
| `Block` | `statements` | Brace-delimited statement block |

### Expressions

| Variant | Notes |
|---------|-------|
| `Literal(Literal)` | |
| `Identifier(String)` | |
| `BinaryOp { left, op, right }` | |
| `UnaryOp { op, operand }` | |
| `Call { callee, arguments }` | |
| `MemberAccess { object, member, is_pointer }` | `.` or `->` |
| `ArrayAccess { array, index }` | `arr[i]` |
| `Cast { target_type, expr }` | `static_cast<T>(expr)` |
| `Ternary { condition, then_expr, else_expr }` | `cond ? a : b` |
| `Sizeof(Type)` | `sizeof(T)` |
| `Raw(String)` | Escape hatch — emitted verbatim |

### Operators

| BinaryOperator | Output | UnaryOperator | Output |
|---------------|--------|---------------|--------|
| `Add` | `+` | `Pos` | `+` |
| `Sub` | `-` | `Neg` | `-` |
| `Mul` | `*` | `Not` | `!` |
| `Div` | `/` | `BitNot` | `~` |
| `Rem` | `%` | `PreInc` | `++` (prefix) |
| `Eq` | `==` | `PreDec` | `--` (prefix) |
| `Ne` | `!=` | `PostInc` | `++` (postfix) |
| `Lt` | `<` | `PostDec` | `--` (postfix) |
| `Le` | `<=` | `Deref` | `*` (dereference) |
| `Gt` | `>` | `AddressOf` | `&` |
| `Ge` | `>=` | | |
| `And` | `&&` | | |
| `Or` | `\|\|` | | |
| `BitAnd` | `&` | | |
| `BitOr` | `\|` | | |
| `BitXor` | `^` | | |
| `ShiftLeft` | `<<` | | |
| `ShiftRight` | `>>` | | |
| `Assign` | `=` | | |
| `AddAssign` | `+=` | | |
| `SubAssign` | `-=` | | |
| `MulAssign` | `*=` | | |
| `DivAssign` | `/=` | | |

### Literals

| Variant | Rust Type | Output Example |
|---------|-----------|----------------|
| `Integer` | `i64` | `42` |
| `Float` | `F64Wrapper` | `3.14` |
| `Boolean` | `bool` | `true` / `false` |
| `String` | `String` | `"hello"` |
| `Character` | `char` | `'a'` |
| `Null` | — | `nullptr` |

### Conditional Compilation

| Type | Fields | Notes |
|------|--------|-------|
| `Conditional<T>` | `condition`, `body`, `elif_branches`, `else_body` | Generic — used for `Declaration`, `ClassMember`, `Statement`, and `Directive` |

---

## Python Backend (`codeforge-python`)

### Module Structure

| Type | Fields | Notes |
|------|--------|-------|
| `Module` | `imports`, `body` | Top-level entry point |

### Imports

| Type | Fields | Notes |
|------|--------|-------|
| `Import` (enum) | `Simple`, `From` | |
| `SimpleImport` | `names` | `import os, sys` |
| `FromImport` | `module`, `names` | `from os.path import join` |
| `ImportName` | `name`, `alias` | Supports `as` aliases |

### Functions

| Type | Fields | Notes |
|------|--------|-------|
| `FunctionDef` | `name`, `decorators`, `parameters`, `vararg`, `kw_only_params`, `kwarg`, `return_annotation`, `body`, `docstring`, `is_async` | Full Python function signature support |

### Classes

| Type | Fields | Notes |
|------|--------|-------|
| `ClassDef` | `name`, `decorators`, `bases`, `keywords`, `body`, `docstring` | `bases: Vec<Expression>` to support complex base expressions |
| `Keyword` | `name`, `value` | For `metaclass=` and similar class keywords |

### Parameters

| Type | Fields | Notes |
|------|--------|-------|
| `Parameter` | `name`, `annotation`, `default` | Used in both functions and lambdas |

### Types

| Variant | Output | Notes |
|---------|--------|-------|
| `None_` | `None` | |
| `Int` | `int` | |
| `Float` | `float` | |
| `Str` | `str` | |
| `Bool` | `bool` | |
| `Bytes` | `bytes` | |
| `Any` | `Any` | |
| `Custom(String)` | verbatim | User-defined type name |
| `Generic(String, Vec<Type>)` | `name[T, ...]` | |
| `Optional(Box<Type>)` | `Optional[T]` | |
| `Union(Vec<Type>)` | `Union[T, ...]` | |
| `Tuple(Vec<Type>)` | `tuple[T, ...]` | |
| `List(Box<Type>)` | `list[T]` | |
| `Dict(Box<Type>, Box<Type>)` | `dict[K, V]` | |
| `Set(Box<Type>)` | `set[T]` | |
| `Callable(Vec<Type>, Box<Type>)` | `Callable[[Args], Ret]` | |
| `Self_` | `Self` | |
| `Raw(String)` | verbatim | Escape hatch for custom type syntax |

### Statements

| Variant | Notes |
|---------|-------|
| `FunctionDef(Box<FunctionDef>)` | |
| `ClassDef(ClassDef)` | |
| `Return(Option<Expression>)` | `return expr` or bare `return` |
| `Assign(Assign)` | `target = value` |
| `AugAssign(AugAssign)` | `target += value` etc. |
| `If(Box<IfStatement>)` | `if`/`elif`/`else` |
| `While(Box<WhileStatement>)` | `while` loop |
| `For(Box<ForStatement>)` | `for target in iter` with optional `else` |
| `Expression(Expression)` | Expression statement |
| `Pass` | `pass` |
| `Break` | `break` |
| `Continue` | `continue` |
| `Comment(String)` | `# comment` |
| `Raw(String)` | Escape hatch — emitted verbatim |

### Assignments

| Type | Fields | Notes |
|------|--------|-------|
| `Assign` | `target`, `value` | Simple assignment |
| `AugAssign` | `target`, `op`, `value` | `op` reuses `BinaryOperator` |

### Control Flow

| Type | Fields | Notes |
|------|--------|-------|
| `IfStatement` | `condition`, `body`, `elif_clauses`, `else_body` | `elif_clauses: Vec<ElifClause>` |
| `ElifClause` | `condition`, `body` | |
| `WhileStatement` | `condition`, `body` | No `else_body` |
| `ForStatement` | `target`, `iter`, `body`, `else_body` | Python-style `for…in`; `else_body` is the `for…else` clause |

### Expressions

| Variant | Notes |
|---------|-------|
| `Literal(Literal)` | |
| `Identifier(String)` | |
| `BinaryOp { left, op, right }` | |
| `UnaryOp { op, operand }` | |
| `Call { func, arguments, keywords }` | Positional and keyword arguments |
| `Attribute { object, name }` | `obj.name` |
| `Subscript { value, index }` | `obj[key]` |
| `Starred(Box<Expression>)` | `*expr` |
| `List(Vec<Expression>)` | `[a, b, c]` |
| `Tuple(Vec<Expression>)` | `(a, b, c)` |
| `Dict(Vec<(Expression, Expression)>)` | `{k: v, ...}` |
| `Set(Vec<Expression>)` | `{a, b, c}` |
| `Ternary { condition, then_expr, else_expr }` | `then_expr if condition else else_expr` |
| `Lambda { parameters, body }` | `lambda params: body` |
| `Raw(String)` | Escape hatch — emitted verbatim |

### Operators

| BinaryOperator | Output | UnaryOperator | Output |
|---------------|--------|---------------|--------|
| `Add` | `+` | `Pos` | `+` |
| `Sub` | `-` | `Neg` | `-` |
| `Mul` | `*` | `Not` | `not` |
| `Div` | `/` | `BitNot` | `~` |
| `FloorDiv` | `//` | | |
| `Mod` | `%` | | |
| `Pow` | `**` | | |
| `Eq` | `==` | | |
| `Ne` | `!=` | | |
| `Lt` | `<` | | |
| `Le` | `<=` | | |
| `Gt` | `>` | | |
| `Ge` | `>=` | | |
| `And` | `and` | | |
| `Or` | `or` | | |
| `BitAnd` | `&` | | |
| `BitOr` | `\|` | | |
| `BitXor` | `^` | | |
| `ShiftLeft` | `<<` | | |
| `ShiftRight` | `>>` | | |
| `In` | `in` | | |
| `NotIn` | `not in` | | |
| `Is` | `is` | | |
| `IsNot` | `is not` | | |

### Literals

| Variant | Rust Type | Output Example |
|---------|-----------|----------------|
| `Integer` | `i64` | `42` |
| `Float` | `F64Wrapper` | `3.14` |
| `Boolean` | `bool` | `True` / `False` |
| `String` | `String` | `'hello'` |
| `None_` | — | `None` |

---

## Rust Backend (`codeforge-rust`)

### Module Structure

| Type | Fields | Notes |
|------|--------|-------|
| `Module` | `attributes`, `items` | Top-level entry point. Inner attributes emitted first |

### Items

| Type | Fields | Notes |
|------|--------|-------|
| `Item` (enum) | `Use`, `Function`, `Struct`, `Enum`, `Trait`, `Impl`, `TypeAlias`, `Const`, `Static`, `Mod`, `Raw` | Top-level item variants |
| `Visibility` (enum) | `Private`, `Public`, `Crate`, `Super`, `Restricted(String)` | Rust visibility modifiers |
| `Attribute` | `path`, `tokens`, `is_inner` | `#[path(tokens)]` or `#![...]` when inner. `Attribute::derive()` helper |

### Use Declarations

| Type | Fields | Notes |
|------|--------|-------|
| `Use` | `visibility`, `tree` | |
| `UseTree` (enum) | `Path`, `Alias`, `Glob`, `Group` | `use std::io`, `use path as alias`, `use prefix::*`, `use prefix::{a, b}` |

### Functions

| Type | Fields | Notes |
|------|--------|-------|
| `Function` | `attributes`, `visibility`, `name`, `generics`, `parameters`, `return_type`, `body`, `is_async`, `is_const`, `is_unsafe`, `abi` | `body: None` → trait method signature |
| `Parameter` (enum) | `Receiver`, `Typed` | Self receivers and typed params |
| `Receiver` | `is_ref`, `is_mut`, `lifetime` | Covers `self`, `&self`, `&mut self`, `&'a self` |

### Generics

| Type | Fields | Notes |
|------|--------|-------|
| `Generics` | `params`, `where_clause` | Empty → emits nothing |
| `GenericParam` (enum) | `Lifetime`, `Type`, `Const` | Each with optional bounds/defaults |
| `WherePredicate` | `ty`, `bounds` | `T: Bound + Bound2` |

### Structs & Enums

| Type | Fields | Notes |
|------|--------|-------|
| `Struct` | `attributes`, `visibility`, `name`, `generics`, `kind` | |
| `StructKind` (enum) | `Unit`, `Tuple`, `Named` | `struct Foo;`, `struct Foo(i32)`, `struct Foo { x: i32 }` |
| `Enum` | `attributes`, `visibility`, `name`, `generics`, `variants` | |
| `EnumVariant` | `attributes`, `name`, `kind`, `discriminant` | |
| `VariantKind` (enum) | `Unit`, `Tuple`, `Named` | Variant data shapes |
| `Field` | `attributes`, `visibility`, `name`, `ty` | Named struct/variant field |
| `TupleField` | `attributes`, `visibility`, `ty` | Tuple struct field |

### Traits & Impls

| Type | Fields | Notes |
|------|--------|-------|
| `Trait` | `attributes`, `visibility`, `name`, `generics`, `supertraits`, `items` | |
| `Impl` | `attributes`, `generics`, `trait_`, `self_ty`, `is_unsafe`, `items` | `trait_: Some` → `impl Trait for Type` |
| `AssocItem` (enum) | `Function`, `Const`, `Type`, `Raw` | |
| `AssocType` | `attributes`, `name`, `generics`, `bounds`, `value` | `type Item;` or `type Item = T;` |

### Other Items

| Type | Fields | Notes |
|------|--------|-------|
| `TypeAlias` | `attributes`, `visibility`, `name`, `generics`, `ty` | `type Name<T> = ...;` |
| `Const` | `attributes`, `visibility`, `name`, `ty`, `value` | `value: None` for trait consts |
| `Static` | `attributes`, `visibility`, `name`, `ty`, `is_mut`, `value` | `static`/`static mut` |
| `Mod` | `attributes`, `visibility`, `name`, `items` | `None` → `mod foo;`, `Some` → `mod foo { ... }` |

### Types

| Variant | Output | Notes |
|---------|--------|-------|
| `Unit` | `()` | |
| `Bool` | `bool` | |
| `Char` | `char` | |
| `Str` | `str` | |
| `I8`..`I128` | `i8`..`i128` | Signed integers |
| `U8`..`U128` | `u8`..`u128` | Unsigned integers |
| `Isize`/`Usize` | `isize`/`usize` | |
| `F32`/`F64` | `f32`/`f64` | |
| `Path(Path)` | user-defined | Named/generic types via `Path` |
| `Reference { lifetime, is_mut, inner }` | `&T`, `&mut T`, `&'a T` | |
| `Pointer { is_mut, inner }` | `*const T`/`*mut T` | |
| `Tuple(Vec<Type>)` | `(A, B)` | |
| `Slice(Box<Type>)` | `[T]` | |
| `Array(Box<Type>, Expression)` | `[T; N]` | |
| `TraitObject(Vec<Type>)` | `dyn A + B` | |
| `ImplTrait(Vec<Type>)` | `impl A + B` | |
| `Fn { params, return_type }` | `fn(A, B) -> C` | |
| `Infer` | `_` | |
| `SelfType` | `Self` | |
| `Raw(String)` | verbatim | Escape hatch |

### Statements & Blocks

| Type | Fields | Notes |
|------|--------|-------|
| `Block` | `statements`, `trailing_expr` | Optional trailing expression (no `;`) |
| `Statement` (enum) | `Let`, `Expression`, `Item`, `Comment`, `Raw` | |
| `Let` | `pattern`, `ty`, `value`, `is_mut`, `else_block` | Supports `let ... else` |

### Patterns

| Variant | Notes |
|---------|-------|
| `Wildcard` | `_` |
| `Rest` | `..` |
| `Ident { name, is_ref, is_mut, subpattern }` | Binding pattern with optional `@` subpattern |
| `Literal(Literal)` | Literal pattern |
| `Tuple(Vec<Pattern>)` | `(a, b)` |
| `Slice(Vec<Pattern>)` | `[first, ..]` |
| `TupleStruct { path, elems }` | `Some(val)` |
| `Struct { path, fields, has_rest }` | `Point { x, y: py, .. }` |
| `Or(Vec<Pattern>)` | `1 | 2` |
| `Reference { is_mut, inner }` | `&x`, `&mut x` |
| `Path(Path)` | `None` |
| `Raw(String)` | Escape hatch |

### Expressions

| Variant | Notes |
|---------|-------|
| `Literal(Literal)` | |
| `Path(Path)` | Identifiers & paths |
| `Binary { left, op, right }` | |
| `Unary { op, operand }` | |
| `Call { callee, args }` | |
| `MethodCall { receiver, method, turbofish, args }` | `.method::<T>(args)` |
| `Field { base, name }` | `obj.field` |
| `Index { base, index }` | `arr[i]` |
| `Reference { is_mut, inner }` | `&x`, `&mut x` |
| `Deref(inner)` | `*ptr` |
| `Try(inner)` | `expr?` |
| `Cast { expr, ty }` | `expr as T` |
| `Tuple(Vec<Expression>)` | `(a, b)` |
| `Array(Vec<Expression>)` | `[a, b]` |
| `Repeat { value, count }` | `[v; n]` |
| `StructLiteral { path, fields, rest }` | `Point { x: 1, ..base }` |
| `Closure { is_move, params, return_type, body }` | `move |x: i32| -> i32 { ... }` |
| `If(Box<IfExpr>)` | `if`/`else if`/`else`, `if let` |
| `Match { scrutinee, arms }` | `match` with guards |
| `Loop { label, body }` | `'label: loop { ... }` |
| `While { label, condition, body }` | `while`/`while let` |
| `For { label, pattern, iter, body }` | `for pat in iter` |
| `Block(Block)` | `{ ... }` |
| `Return`, `Break`, `Continue` | With optional labels/values |
| `Range { start, end, inclusive }` | `..`, `..=`, partial ranges |
| `MacroCall { path, tokens }` | `println!(...)`, `vec![...]` |
| `Raw(String)` | Escape hatch |

### Operators

| BinaryOperator | Output | UnaryOperator | Output |
|---------------|--------|---------------|--------|
| `Add` | `+` | `Neg` | `-` |
| `Sub` | `-` | `Not` | `!` |
| `Mul` | `*` | | |
| `Div` | `/` | | |
| `Rem` | `%` | | |
| `And` | `&&` | | |
| `Or` | `\|\|` | | |
| `BitAnd` | `&` | | |
| `BitOr` | `\|` | | |
| `BitXor` | `^` | | |
| `Shl` | `<<` | | |
| `Shr` | `>>` | | |
| `Eq` | `==` | | |
| `Ne` | `!=` | | |
| `Lt` | `<` | | |
| `Le` | `<=` | | |
| `Gt` | `>` | | |
| `Ge` | `>=` | | |
| `Assign` | `=` | | |
| `AddAssign` | `+=` | | |
| `SubAssign` | `-=` | | |
| `MulAssign` | `*=` | | |
| `DivAssign` | `/=` | | |
| `RemAssign` | `%=` | | |
| `BitAndAssign` | `&=` | | |
| `BitOrAssign` | `\|=` | | |
| `BitXorAssign` | `^=` | | |
| `ShlAssign` | `<<=` | | |
| `ShrAssign` | `>>=` | | |

### Literals

| Variant | Rust Type | Output Example |
|---------|-----------|----------------|
| `Integer` | `i64` | `42` |
| `UnsignedInteger` | `u64` | `10u64` |
| `Float` | `F64Wrapper` | `3.14`, `f64::NAN`, `f64::INFINITY` |
| `Boolean` | `bool` | `true` / `false` |
| `String` | `String` | `"hello"` |
| `Char` | `char` | `'a'` |
| `ByteString` | `Vec<u8>` | `b"hello"` |
| `Raw` | — | Verbatim |

---

## Shared Patterns

### Escape Hatches

All three backends provide `Raw(String)` variants in `Statement` and `Expression` for emitting arbitrary text when the AST doesn't cover a specific construct.

### Serde Support

All AST types in all backends conditionally derive `Serialize` and `Deserialize` behind the `serde` feature flag.

### F64Wrapper

All backends wrap `f64` in a `F64Wrapper` newtype that implements `Eq` and `Hash` via `f64::to_bits()`, allowing literals to be used in hash-based collections.

### Box Recursive Types

Recursive `Statement` variants (`If`, `While`, `For`) are boxed in all backends to break size cycles. The C++ backend also boxes `Template::declaration` since it wraps `Declaration` which can contain `Template`. The Rust backend boxes additional recursive paths including `Block::trailing_expr`, `Expression::Cast::ty`, loop/while/for bodies, and `IfCondition` value expressions.
