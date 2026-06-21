use codeforge_rust::*;

fn module(items: Vec<Item>) -> Module {
    Module {
        attributes: vec![],
        items,
    }
}

fn simple_path(name: &str) -> Path {
    Path {
        segments: vec![PathSegment {
            name: name.to_string(),
            args: vec![],
        }],
    }
}

fn ident_expr(name: &str) -> Expression {
    Expression::Path(simple_path(name))
}

fn empty_generics() -> Generics {
    Generics::empty()
}

fn empty_fn() -> Function {
    Function {
        attributes: vec![],
        visibility: Visibility::Private,
        name: String::new(),
        generics: empty_generics(),
        parameters: vec![],
        return_type: None,
        body: None,
        is_async: false,
        is_const: false,
        is_unsafe: false,
        abi: None,
    }
}

fn typed_param(name: &str, ty: Type) -> Parameter {
    Parameter::Typed {
        pattern: Pattern::Ident {
            name: name.into(),
            is_ref: false,
            is_mut: false,
            subpattern: None,
        },
        ty,
    }
}

fn self_ref() -> Parameter {
    Parameter::Receiver(Receiver {
        is_ref: true,
        is_mut: false,
        lifetime: None,
    })
}

fn fn_with_body(name: &str, body: Vec<Statement>) -> Item {
    Item::Function(Function {
        body: Some(Block {
            statements: body,
            trailing_expr: None,
        }),
        ..Function {
            name: name.into(),
            ..empty_fn()
        }
    })
}

#[test]
fn free_function_no_params() {
    let m = module(vec![Item::Function(Function {
        name: "main".into(),
        body: Some(Block {
            statements: vec![],
            trailing_expr: None,
        }),
        ..empty_fn()
    })]);

    let output = emit(&m);
    let expected = "\
fn main() {
}
";
    assert_eq!(output, expected);
}

#[test]
fn free_function_with_params_and_return() {
    let m = module(vec![Item::Function(Function {
        name: "add".into(),
        parameters: vec![typed_param("a", Type::I32), typed_param("b", Type::I32)],
        return_type: Some(Type::I32),
        body: Some(Block {
            statements: vec![],
            trailing_expr: Some(Box::new(Expression::Binary {
                left: Box::new(ident_expr("a")),
                op: BinaryOperator::Add,
                right: Box::new(ident_expr("b")),
            })),
        }),
        ..empty_fn()
    })]);

    let output = emit(&m);
    let expected = "\
fn add(a: i32, b: i32) -> i32 {
    a + b
}
";
    assert_eq!(output, expected);
}

#[test]
fn async_function() {
    let m = module(vec![Item::Function(Function {
        name: "fetch".into(),
        is_async: true,
        return_type: Some(Type::path("String")),
        body: Some(Block {
            statements: vec![],
            trailing_expr: Some(Box::new(Expression::Call {
                callee: Box::new(ident_expr("do_fetch")),
                args: vec![],
            })),
        }),
        ..empty_fn()
    })]);

    let output = emit(&m);
    let expected = "\
async fn fetch() -> String {
    do_fetch()
}
";
    assert_eq!(output, expected);
}

#[test]
fn unsafe_const_function() {
    let m = module(vec![
        Item::Function(Function {
            name: "danger".into(),
            is_unsafe: true,
            body: Some(Block {
                statements: vec![],
                trailing_expr: None,
            }),
            ..empty_fn()
        }),
        Item::Function(Function {
            name: "compile_time".into(),
            is_const: true,
            return_type: Some(Type::I32),
            body: Some(Block {
                statements: vec![],
                trailing_expr: Some(Box::new(Expression::Literal(Literal::Integer(42)))),
            }),
            ..empty_fn()
        }),
    ]);

    let output = emit(&m);
    let expected = "\
unsafe fn danger() {
}

const fn compile_time() -> i32 {
    42
}
";
    assert_eq!(output, expected);
}

#[test]
fn extern_c_function() {
    let m = module(vec![Item::Function(Function {
        name: "malloc".into(),
        is_unsafe: true,
        abi: Some("C".into()),
        parameters: vec![typed_param("size", Type::Usize)],
        return_type: Some(Type::Pointer {
            is_mut: true,
            inner: Box::new(Type::Unit),
        }),
        body: None,
        ..empty_fn()
    })]);

    let output = emit(&m);
    let expected = "unsafe extern \"C\" fn malloc(size: usize) -> *mut ();\n";
    assert_eq!(output, expected);
}

#[test]
fn generic_function_with_where_clause() {
    let m = module(vec![Item::Function(Function {
        name: "identity".into(),
        generics: Generics {
            params: vec![GenericParam::Type {
                name: "T".into(),
                bounds: vec![],
                default: None,
            }],
            where_clause: vec![WherePredicate {
                ty: Type::path("T"),
                bounds: vec![Type::path("Clone"), Type::path("Debug")],
            }],
        },
        parameters: vec![typed_param("val", Type::path("T"))],
        return_type: Some(Type::path("T")),
        body: Some(Block {
            statements: vec![],
            trailing_expr: Some(Box::new(Expression::MethodCall {
                receiver: Box::new(ident_expr("val")),
                method: "clone".into(),
                turbofish: vec![],
                args: vec![],
            })),
        }),
        ..empty_fn()
    })]);

    let output = emit(&m);
    let expected = "\
fn identity<T>(val: T) -> T
where
    T: Clone + Debug
{
    val.clone()
}
";
    assert_eq!(output, expected);
}

#[test]
fn visibility_variants() {
    let m = module(vec![
        Item::Function(Function {
            name: "a".into(),
            visibility: Visibility::Public,
            body: Some(Block {
                statements: vec![],
                trailing_expr: None,
            }),
            ..empty_fn()
        }),
        Item::Function(Function {
            name: "b".into(),
            visibility: Visibility::Crate,
            body: Some(Block {
                statements: vec![],
                trailing_expr: None,
            }),
            ..empty_fn()
        }),
        Item::Function(Function {
            name: "c".into(),
            visibility: Visibility::Super,
            body: Some(Block {
                statements: vec![],
                trailing_expr: None,
            }),
            ..empty_fn()
        }),
        Item::Function(Function {
            name: "d".into(),
            visibility: Visibility::Restricted("crate::inner".into()),
            body: Some(Block {
                statements: vec![],
                trailing_expr: None,
            }),
            ..empty_fn()
        }),
    ]);

    let output = emit(&m);
    let expected = "\
pub fn a() {
}

pub(crate) fn b() {
}

pub(super) fn c() {
}

pub(in crate::inner) fn d() {
}
";
    assert_eq!(output, expected);
}

#[test]
fn attributes_and_derives() {
    let m = module(vec![Item::Struct(Struct {
        attributes: vec![
            Attribute::derive(vec!["Debug".into(), "Clone".into()]),
            Attribute {
                path: "allow".into(),
                tokens: Some("dead_code".into()),
                is_inner: false,
            },
        ],
        visibility: Visibility::Public,
        name: "Foo".into(),
        generics: empty_generics(),
        kind: StructKind::Named(vec![Field {
            attributes: vec![],
            visibility: Visibility::Public,
            name: "x".into(),
            ty: Type::I32,
        }]),
    })]);

    let output = emit(&m);
    let expected = "\
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Foo {
    pub x: i32,
}
";
    assert_eq!(output, expected);
}

#[test]
fn unit_struct() {
    let m = module(vec![Item::Struct(Struct {
        attributes: vec![],
        visibility: Visibility::Public,
        name: "Unit".into(),
        generics: empty_generics(),
        kind: StructKind::Unit,
    })]);

    let output = emit(&m);
    let expected = "pub struct Unit;\n";
    assert_eq!(output, expected);
}

#[test]
fn tuple_struct() {
    let m = module(vec![Item::Struct(Struct {
        attributes: vec![],
        visibility: Visibility::Public,
        name: "Point".into(),
        generics: empty_generics(),
        kind: StructKind::Tuple(vec![
            TupleField {
                attributes: vec![],
                visibility: Visibility::Public,
                ty: Type::F64,
            },
            TupleField {
                attributes: vec![],
                visibility: Visibility::Public,
                ty: Type::F64,
            },
        ]),
    })]);

    let output = emit(&m);
    let expected = "pub struct Point(pub f64, pub f64);\n";
    assert_eq!(output, expected);
}

#[test]
fn named_struct_with_fields() {
    let m = module(vec![Item::Struct(Struct {
        attributes: vec![],
        visibility: Visibility::Private,
        name: "Config".into(),
        generics: empty_generics(),
        kind: StructKind::Named(vec![
            Field {
                attributes: vec![],
                visibility: Visibility::Private,
                name: "host".into(),
                ty: Type::path("String"),
            },
            Field {
                attributes: vec![],
                visibility: Visibility::Private,
                name: "port".into(),
                ty: Type::U16,
            },
        ]),
    })]);

    let output = emit(&m);
    let expected = "\
struct Config {
    host: String,
    port: u16,
}
";
    assert_eq!(output, expected);
}

#[test]
fn generic_enum() {
    let m = module(vec![Item::Enum(Enum {
        attributes: vec![Attribute::derive(vec!["Debug".into()])],
        visibility: Visibility::Public,
        name: "Result".into(),
        generics: Generics {
            params: vec![
                GenericParam::Type {
                    name: "T".into(),
                    bounds: vec![],
                    default: None,
                },
                GenericParam::Type {
                    name: "E".into(),
                    bounds: vec![],
                    default: None,
                },
            ],
            where_clause: vec![],
        },
        variants: vec![
            EnumVariant {
                attributes: vec![],
                name: "Ok".into(),
                kind: VariantKind::Tuple(vec![Type::path("T")]),
                discriminant: None,
            },
            EnumVariant {
                attributes: vec![],
                name: "Err".into(),
                kind: VariantKind::Tuple(vec![Type::path("E")]),
                discriminant: None,
            },
        ],
    })]);

    let output = emit(&m);
    let expected = "\
#[derive(Debug)]
pub enum Result<T, E> {
    Ok(T),
    Err(E),
}
";
    assert_eq!(output, expected);
}

#[test]
fn enum_with_discriminants() {
    let m = module(vec![Item::Enum(Enum {
        attributes: vec![],
        visibility: Visibility::Public,
        name: "Color".into(),
        generics: empty_generics(),
        variants: vec![
            EnumVariant {
                attributes: vec![],
                name: "Red".into(),
                kind: VariantKind::Unit,
                discriminant: Some(Expression::Literal(Literal::Integer(0))),
            },
            EnumVariant {
                attributes: vec![],
                name: "Green".into(),
                kind: VariantKind::Unit,
                discriminant: Some(Expression::Literal(Literal::Integer(1))),
            },
            EnumVariant {
                attributes: vec![],
                name: "Blue".into(),
                kind: VariantKind::Unit,
                discriminant: Some(Expression::Literal(Literal::Integer(2))),
            },
        ],
    })]);

    let output = emit(&m);
    let expected = "\
pub enum Color {
    Red = 0,
    Green = 1,
    Blue = 2,
}
";
    assert_eq!(output, expected);
}

#[test]
fn enum_with_struct_variants() {
    let m = module(vec![Item::Enum(Enum {
        attributes: vec![],
        visibility: Visibility::Public,
        name: "Shape".into(),
        generics: empty_generics(),
        variants: vec![
            EnumVariant {
                attributes: vec![],
                name: "Circle".into(),
                kind: VariantKind::Named(vec![Field {
                    attributes: vec![],
                    visibility: Visibility::Private,
                    name: "radius".into(),
                    ty: Type::F64,
                }]),
                discriminant: None,
            },
            EnumVariant {
                attributes: vec![],
                name: "Rect".into(),
                kind: VariantKind::Named(vec![
                    Field {
                        attributes: vec![],
                        visibility: Visibility::Private,
                        name: "width".into(),
                        ty: Type::F64,
                    },
                    Field {
                        attributes: vec![],
                        visibility: Visibility::Private,
                        name: "height".into(),
                        ty: Type::F64,
                    },
                ]),
                discriminant: None,
            },
        ],
    })]);

    let output = emit(&m);
    let expected = "\
pub enum Shape {
    Circle {
        radius: f64,
    },
    Rect {
        width: f64,
        height: f64,
    },
}
";
    assert_eq!(output, expected);
}

#[test]
fn trait_with_methods() {
    let m = module(vec![Item::Trait(Trait {
        attributes: vec![],
        visibility: Visibility::Public,
        name: "Animal".into(),
        generics: empty_generics(),
        supertraits: vec![Type::path("Send"), Type::path("Sync")],
        items: vec![
            AssocItem::Function(Function {
                name: "speak".into(),
                parameters: vec![self_ref()],
                body: None,
                ..empty_fn()
            }),
            AssocItem::Function(Function {
                name: "name".into(),
                parameters: vec![self_ref()],
                return_type: Some(Type::path("String")),
                ..empty_fn()
            }),
        ],
    })]);

    let output = emit(&m);
    let expected = "\
pub trait Animal: Send + Sync {
    fn speak(&self);

    fn name(&self) -> String;
}
";
    assert_eq!(output, expected);
}

#[test]
fn trait_with_associated_type_and_const() {
    let m = module(vec![Item::Trait(Trait {
        attributes: vec![],
        visibility: Visibility::Public,
        name: "Iterator".into(),
        generics: empty_generics(),
        supertraits: vec![],
        items: vec![
            AssocItem::Type(AssocType {
                attributes: vec![],
                name: "Item".into(),
                generics: empty_generics(),
                bounds: vec![],
                value: None,
            }),
            AssocItem::Const(Const {
                attributes: vec![],
                visibility: Visibility::Private,
                name: "MAX_SIZE".into(),
                ty: Type::Usize,
                value: None,
            }),
            AssocItem::Function(Function {
                name: "next".into(),
                parameters: vec![Parameter::Receiver(Receiver {
                    is_ref: true,
                    is_mut: true,
                    lifetime: None,
                })],
                return_type: Some(Type::generic(
                    "Option",
                    vec![GenericArg::Type(Box::new(Type::SelfType))],
                )),
                body: None,
                ..empty_fn()
            }),
        ],
    })]);

    let output = emit(&m);
    let expected = "\
pub trait Iterator {
    type Item;

    const MAX_SIZE: usize;

    fn next(&mut self) -> Option<Self>;
}
";
    assert_eq!(output, expected);
}

#[test]
fn inherent_impl() {
    let m = module(vec![Item::Impl(Impl {
        attributes: vec![],
        generics: empty_generics(),
        trait_: None,
        self_ty: Type::path("Rect"),
        is_unsafe: false,
        items: vec![AssocItem::Function(Function {
            name: "area".into(),
            parameters: vec![self_ref()],
            return_type: Some(Type::F64),
            body: Some(Block {
                statements: vec![],
                trailing_expr: Some(Box::new(Expression::Binary {
                    left: Box::new(Expression::Field {
                        base: Box::new(ident_expr("self")),
                        name: "width".into(),
                    }),
                    op: BinaryOperator::Mul,
                    right: Box::new(Expression::Field {
                        base: Box::new(ident_expr("self")),
                        name: "height".into(),
                    }),
                })),
            }),
            ..empty_fn()
        })],
    })]);

    let output = emit(&m);
    let expected = "\
impl Rect {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}
";
    assert_eq!(output, expected);
}

#[test]
fn impl_trait_for_type() {
    let m = module(vec![Item::Impl(Impl {
        attributes: vec![],
        generics: empty_generics(),
        trait_: Some(Type::path("ToString")),
        self_ty: Type::path("MyStruct"),
        is_unsafe: false,
        items: vec![AssocItem::Function(Function {
            name: "to_string".into(),
            parameters: vec![self_ref()],
            return_type: Some(Type::path("String")),
            body: Some(Block {
                statements: vec![],
                trailing_expr: Some(Box::new(Expression::Call {
                    callee: Box::new(ident_expr("String::from")),
                    args: vec![Expression::MethodCall {
                        receiver: Box::new(ident_expr("self")),
                        method: "name".into(),
                        turbofish: vec![],
                        args: vec![],
                    }],
                })),
            }),
            ..empty_fn()
        })],
    })]);

    let output = emit(&m);
    let expected = "\
impl ToString for MyStruct {
    fn to_string(&self) -> String {
        String::from(self.name())
    }
}
";
    assert_eq!(output, expected);
}

#[test]
fn unsafe_impl() {
    let m = module(vec![Item::Impl(Impl {
        attributes: vec![],
        generics: empty_generics(),
        trait_: Some(Type::path("Send")),
        self_ty: Type::path("Data"),
        is_unsafe: true,
        items: vec![],
    })]);

    let output = emit(&m);
    let expected = "unsafe impl Send for Data {}\n";
    assert_eq!(output, expected);
}

#[test]
fn impl_with_generics() {
    let m = module(vec![Item::Impl(Impl {
        attributes: vec![],
        generics: Generics {
            params: vec![GenericParam::Type {
                name: "T".into(),
                bounds: vec![],
                default: None,
            }],
            where_clause: vec![WherePredicate {
                ty: Type::path("T"),
                bounds: vec![Type::path("Clone")],
            }],
        },
        trait_: None,
        self_ty: Type::generic("Box", vec![GenericArg::Type(Box::new(Type::path("T")))]),
        is_unsafe: false,
        items: vec![AssocItem::Function(Function {
            name: "duplicate".into(),
            parameters: vec![self_ref()],
            return_type: Some(Type::path("T")),
            body: Some(Block {
                statements: vec![],
                trailing_expr: Some(Box::new(Expression::Call {
                    callee: Box::new(ident_expr("todo!")),
                    args: vec![],
                })),
            }),
            ..empty_fn()
        })],
    })]);

    let output = emit(&m);
    let expected = "\
impl<T> Box<T>
where
    T: Clone
{
    fn duplicate(&self) -> T {
        todo!()
    }
}
";
    assert_eq!(output, expected);
}

#[test]
fn use_statements() {
    let m = module(vec![
        Item::Use(Use {
            visibility: Visibility::Private,
            tree: UseTree::Path("std::collections::HashMap".into()),
        }),
        Item::Use(Use {
            visibility: Visibility::Public,
            tree: UseTree::Alias {
                path: "std::io::Result".into(),
                alias: "IoResult".into(),
            },
        }),
        Item::Use(Use {
            visibility: Visibility::Public,
            tree: UseTree::Glob("std::sync".into()),
        }),
        Item::Use(Use {
            visibility: Visibility::Private,
            tree: UseTree::Group {
                prefix: "std".into(),
                items: vec![UseTree::Path("io".into()), UseTree::Path("fs".into())],
            },
        }),
    ]);

    let output = emit(&m);
    let expected = "\
use std::collections::HashMap;
pub use std::io::Result as IoResult;
pub use std::sync::*;
use std::{io, fs};
";
    assert_eq!(output, expected);
}

#[test]
fn type_alias_const_static() {
    let m = module(vec![
        Item::TypeAlias(TypeAlias {
            attributes: vec![],
            visibility: Visibility::Public,
            name: "Result".into(),
            generics: Generics {
                params: vec![GenericParam::Type {
                    name: "T".into(),
                    bounds: vec![],
                    default: None,
                }],
                where_clause: vec![],
            },
            ty: Type::generic(
                "Result",
                vec![
                    GenericArg::Type(Box::new(Type::path("T"))),
                    GenericArg::Type(Box::new(Type::path("Error"))),
                ],
            ),
        }),
        Item::Const(Const {
            attributes: vec![],
            visibility: Visibility::Public,
            name: "MAX".into(),
            ty: Type::I32,
            value: Some(Expression::Literal(Literal::Integer(100))),
        }),
        Item::Static(Static {
            attributes: vec![],
            visibility: Visibility::Private,
            name: "COUNTER".into(),
            ty: Type::path("AtomicIsize"),
            is_mut: false,
            value: Expression::Call {
                callee: Box::new(ident_expr("AtomicIsize::new")),
                args: vec![Expression::Literal(Literal::Integer(0))],
            },
        }),
        Item::Static(Static {
            attributes: vec![],
            visibility: Visibility::Public,
            name: "MUTABLE".into(),
            ty: Type::I32,
            is_mut: true,
            value: Expression::Literal(Literal::Integer(0)),
        }),
    ]);

    let output = emit(&m);
    let expected = "\
pub type Result<T> = Result<T, Error>;

pub const MAX: i32 = 100;

static COUNTER: AtomicIsize = AtomicIsize::new(0);

pub static mut MUTABLE: i32 = 0;
";
    assert_eq!(output, expected);
}

#[test]
fn mod_inline_and_declaration() {
    let m = module(vec![
        Item::Mod(Mod {
            attributes: vec![],
            visibility: Visibility::Public,
            name: "inner".into(),
            items: Some(vec![
                Item::Use(Use {
                    visibility: Visibility::Private,
                    tree: UseTree::Path("std::io".into()),
                }),
                Item::Function(Function {
                    name: "helper".into(),
                    body: Some(Block {
                        statements: vec![],
                        trailing_expr: None,
                    }),
                    ..empty_fn()
                }),
            ]),
        }),
        Item::Mod(Mod {
            attributes: vec![],
            visibility: Visibility::Private,
            name: "foo".into(),
            items: None,
        }),
    ]);

    let output = emit(&m);
    let expected = "\
pub mod inner {
    use std::io;

    fn helper() {
    }
}

mod foo;
";
    assert_eq!(output, expected);
}

#[test]
fn let_statements() {
    let m = module(vec![fn_with_body(
        "main",
        vec![
            Statement::Let(Let {
                pattern: Pattern::Ident {
                    name: "x".into(),
                    is_ref: false,
                    is_mut: false,
                    subpattern: None,
                },
                ty: Some(Type::I32),
                value: Some(Expression::Literal(Literal::Integer(42))),
                is_mut: false,
                else_block: None,
            }),
            Statement::Let(Let {
                pattern: Pattern::Ident {
                    name: "mut_y".into(),
                    is_ref: false,
                    is_mut: false,
                    subpattern: None,
                },
                ty: None,
                value: Some(Expression::Literal(Literal::Integer(10))),
                is_mut: true,
                else_block: None,
            }),
        ],
    )]);

    let output = emit(&m);
    let expected = "\
fn main() {
    let x: i32 = 42;
    let mut mut_y = 10;
}
";
    assert_eq!(output, expected);
}

#[test]
fn nested_item_in_fn_body() {
    let m = module(vec![fn_with_body(
        "main",
        vec![Statement::Item(Box::new(Item::Struct(Struct {
            attributes: vec![],
            visibility: Visibility::Private,
            name: "Helper".into(),
            generics: empty_generics(),
            kind: StructKind::Unit,
        })))],
    )]);

    let output = emit(&m);
    let expected = "\
fn main() {
    struct Helper;
}
";
    assert_eq!(output, expected);
}

#[test]
fn comment_statement() {
    let m = module(vec![fn_with_body(
        "main",
        vec![
            Statement::Comment("This is a comment".into()),
            Statement::Let(Let {
                pattern: Pattern::Ident {
                    name: "x".into(),
                    is_ref: false,
                    is_mut: false,
                    subpattern: None,
                },
                ty: None,
                value: Some(Expression::Literal(Literal::Integer(1))),
                is_mut: false,
                else_block: None,
            }),
        ],
    )]);

    let output = emit(&m);
    let expected = "\
fn main() {
    // This is a comment
    let x = 1;
}
";
    assert_eq!(output, expected);
}

#[test]
fn if_else_expr() {
    let m = module(vec![Item::Function(Function {
        name: "classify".into(),
        parameters: vec![typed_param("x", Type::I32)],
        return_type: Some(Type::path("String")),
        body: Some(Block {
            statements: vec![],
            trailing_expr: Some(Box::new(Expression::If(Box::new(IfExpr {
                condition: IfCondition::Expr(Box::new(Expression::Binary {
                    left: Box::new(ident_expr("x")),
                    op: BinaryOperator::Gt,
                    right: Box::new(Expression::Literal(Literal::Integer(0))),
                })),
                then_block: Block {
                    statements: vec![],
                    trailing_expr: Some(Box::new(Expression::Literal(Literal::String(
                        "positive".into(),
                    )))),
                },
                else_branch: Some(ElseBranch::Block(Block {
                    statements: vec![],
                    trailing_expr: Some(Box::new(Expression::Literal(Literal::String(
                        "other".into(),
                    )))),
                })),
            })))),
        }),
        ..empty_fn()
    })]);

    let output = emit(&m);
    let expected = "\
fn classify(x: i32) -> String {
    if x > 0 {
        \"positive\"
    } else {
        \"other\"
    }
}
";
    assert_eq!(output, expected);
}

#[test]
fn if_let_else_if() {
    let m = module(vec![fn_with_body(
        "handle",
        vec![Statement::Expression(Expression::If(Box::new(IfExpr {
            condition: IfCondition::Let {
                pattern: Pattern::TupleStruct {
                    path: simple_path("Some"),
                    elems: vec![Pattern::Ident {
                        name: "val".into(),
                        is_ref: false,
                        is_mut: false,
                        subpattern: None,
                    }],
                },
                value: Box::new(ident_expr("opt")),
            },
            then_block: Block {
                statements: vec![fn_call_stmt("println!", "\"some\"")],
                trailing_expr: None,
            },
            else_branch: Some(ElseBranch::If(Box::new(IfExpr {
                condition: IfCondition::Expr(Box::new(ident_expr("other_cond"))),
                then_block: Block {
                    statements: vec![fn_call_stmt("println!", "\"else_if\"")],
                    trailing_expr: None,
                },
                else_branch: None,
            }))),
        })))],
    )]);

    let output = emit(&m);
    let expected = "\
fn handle() {
    if let Some(val) = opt {
        println!(\"some\");
    } else if other_cond {
        println!(\"else_if\");
    }
}
";
    assert_eq!(output, expected);
}

#[test]
fn match_expr_test() {
    let m = module(vec![Item::Function(Function {
        name: "handle".into(),
        parameters: vec![typed_param("val", Type::path("Option<i32>"))],
        return_type: Some(Type::I32),
        body: Some(Block {
            statements: vec![],
            trailing_expr: Some(Box::new(Expression::Match {
                scrutinee: Box::new(ident_expr("val")),
                arms: vec![
                    MatchArm {
                        pattern: Pattern::TupleStruct {
                            path: simple_path("Some"),
                            elems: vec![Pattern::Ident {
                                name: "n".into(),
                                is_ref: false,
                                is_mut: false,
                                subpattern: None,
                            }],
                        },
                        guard: Some(Expression::Binary {
                            left: Box::new(ident_expr("n")),
                            op: BinaryOperator::Gt,
                            right: Box::new(Expression::Literal(Literal::Integer(0))),
                        }),
                        body: ident_expr("n"),
                    },
                    MatchArm {
                        pattern: Pattern::TupleStruct {
                            path: simple_path("Some"),
                            elems: vec![Pattern::Ident {
                                name: "n".into(),
                                is_ref: false,
                                is_mut: false,
                                subpattern: None,
                            }],
                        },
                        guard: None,
                        body: Expression::Unary {
                            op: UnaryOperator::Neg,
                            operand: Box::new(ident_expr("n")),
                        },
                    },
                    MatchArm {
                        pattern: Pattern::Path(simple_path("None")),
                        guard: None,
                        body: Expression::Literal(Literal::Integer(0)),
                    },
                ],
            })),
        }),
        ..empty_fn()
    })]);

    let output = emit(&m);
    let expected = "\
fn handle(val: Option<i32>) -> i32 {
    match val {
        Some(n) if n > 0 => n,
        Some(n) => -n,
        None => 0,
    }
}
";
    assert_eq!(output, expected);
}

#[test]
fn loop_and_while() {
    let m = module(vec![fn_with_body(
        "loops",
        vec![
            Statement::Expression(Expression::Loop {
                label: None,
                body: Box::new(Block {
                    statements: vec![Statement::Expression(Expression::If(Box::new(IfExpr {
                        condition: IfCondition::Expr(Box::new(Expression::Literal(
                            Literal::Boolean(true),
                        ))),
                        then_block: Block {
                            statements: vec![Statement::Expression(Expression::Break {
                                label: None,
                                value: None,
                            })],
                            trailing_expr: None,
                        },
                        else_branch: None,
                    })))],
                    trailing_expr: None,
                }),
            }),
            Statement::Expression(Expression::While {
                label: None,
                condition: IfCondition::Expr(Box::new(Expression::Literal(Literal::Boolean(true)))),
                body: Box::new(Block {
                    statements: vec![Statement::Expression(Expression::Continue { label: None })],
                    trailing_expr: None,
                }),
            }),
        ],
    )]);

    let output = emit(&m);
    let expected = "\
fn loops() {
    loop {
        if true {
            break;
        }
    }
    while true {
        continue;
    }
}
";
    assert_eq!(output, expected);
}

#[test]
fn for_loop() {
    let m = module(vec![fn_with_body(
        "iter",
        vec![Statement::Expression(Expression::For {
            label: None,
            pattern: Pattern::Ident {
                name: "item".into(),
                is_ref: false,
                is_mut: false,
                subpattern: None,
            },
            iter: Box::new(ident_expr("items")),
            body: Box::new(Block {
                statements: vec![Statement::Expression(Expression::Call {
                    callee: Box::new(ident_expr("process")),
                    args: vec![ident_expr("item")],
                })],
                trailing_expr: None,
            }),
        })],
    )]);

    let output = emit(&m);
    let expected = "\
fn iter() {
    for item in items {
        process(item);
    }
}
";
    assert_eq!(output, expected);
}

#[test]
fn labeled_loop_with_break_value() {
    let m = module(vec![Item::Function(Function {
        name: "search".into(),
        return_type: Some(Type::I32),
        body: Some(Block {
            statements: vec![],
            trailing_expr: Some(Box::new(Expression::Loop {
                label: Some("outer".into()),
                body: Box::new(Block {
                    statements: vec![],
                    trailing_expr: Some(Box::new(Expression::Break {
                        label: Some("outer".into()),
                        value: Some(Box::new(Expression::Literal(Literal::Integer(42)))),
                    })),
                }),
            })),
        }),
        ..empty_fn()
    })]);

    let output = emit(&m);
    let expected = "\
fn search() -> i32 {
    'outer: loop {
        break 'outer 42;
    }
}
";
    assert_eq!(output, expected);
}

#[test]
fn type_to_rust() {
    let cases: Vec<(Type, &str)> = vec![
        (Type::Unit, "()"),
        (Type::Bool, "bool"),
        (Type::Char, "char"),
        (Type::Str, "str"),
        (Type::I32, "i32"),
        (Type::U64, "u64"),
        (Type::F32, "f32"),
        (Type::F64, "f64"),
        (Type::path("String"), "String"),
        (
            Type::generic("Vec", vec![GenericArg::Type(Box::new(Type::I32))]),
            "Vec<i32>",
        ),
        (
            Type::Reference {
                lifetime: None,
                is_mut: false,
                inner: Box::new(Type::Str),
            },
            "&str",
        ),
        (
            Type::Reference {
                lifetime: Some("a".into()),
                is_mut: true,
                inner: Box::new(Type::I32),
            },
            "&'a mut i32",
        ),
        (
            Type::Pointer {
                is_mut: false,
                inner: Box::new(Type::U8),
            },
            "*const u8",
        ),
        (
            Type::Pointer {
                is_mut: true,
                inner: Box::new(Type::U8),
            },
            "*mut u8",
        ),
        (Type::Tuple(vec![Type::I32, Type::F64]), "(i32, f64)"),
        (Type::Slice(Box::new(Type::U8)), "[u8]"),
        (
            Type::Array(
                Box::new(Type::I32),
                Box::new(Expression::Literal(Literal::Integer(4))),
            ),
            "[i32; 4]",
        ),
        (
            Type::TraitObject(vec![Type::path("Write"), Type::path("Send")]),
            "dyn Write + Send",
        ),
        (
            Type::ImplTrait(vec![Type::path("Iterator")]),
            "impl Iterator",
        ),
        (
            Type::Fn {
                params: vec![Type::I32],
                return_type: Some(Box::new(Type::Bool)),
            },
            "fn(i32) -> bool",
        ),
        (Type::Infer, "_"),
        (Type::SelfType, "Self"),
    ];

    for (ty, expected) in cases {
        assert_eq!(ty.to_rust(), expected, "Type::to_rust() mismatch");
    }
}

#[test]
fn literal_to_rust() {
    let cases: Vec<(Literal, &str)> = vec![
        (Literal::Integer(42), "42"),
        (Literal::UnsignedInteger(10), "10u64"),
        (Literal::Float(F64Wrapper(3.14)), "3.14"),
        (Literal::Boolean(true), "true"),
        (Literal::Boolean(false), "false"),
        (Literal::String("hello".into()), "\"hello\""),
        (
            Literal::String("escape \"me\"".into()),
            "\"escape \\\"me\\\"\"",
        ),
        (Literal::String("new\nline".into()), "\"new\\nline\""),
        (Literal::Char('a'), "'a'"),
        (Literal::Char('\''), "'\\''"),
        (Literal::Char('\\'), "'\\\\'"),
    ];

    for (lit, expected) in cases {
        assert_eq!(
            Expression::Literal(lit).to_rust(),
            expected,
            "Literal::to_rust() mismatch"
        );
    }
}

#[test]
fn float_special_values() {
    let cases: Vec<(Literal, &str)> = vec![
        (Literal::Float(F64Wrapper(f64::NAN)), "f64::NAN"),
        (Literal::Float(F64Wrapper(f64::INFINITY)), "f64::INFINITY"),
        (
            Literal::Float(F64Wrapper(f64::NEG_INFINITY)),
            "f64::NEG_INFINITY",
        ),
    ];

    for (lit, expected) in cases {
        assert_eq!(Expression::Literal(lit).to_rust(), expected);
    }
}

#[test]
fn integer_literal_ensures_decimal() {
    let expr = Expression::Literal(Literal::Float(F64Wrapper(1.0)));
    assert_eq!(expr.to_rust(), "1.0");
}

#[test]
fn expression_to_rust() {
    let cases: Vec<(Expression, &str)> = vec![
        (ident_expr("x"), "x"),
        (ident_expr("a::b::c"), "a::b::c"),
        (
            Expression::Binary {
                left: Box::new(ident_expr("x")),
                op: BinaryOperator::Add,
                right: Box::new(ident_expr("y")),
            },
            "x + y",
        ),
        (
            Expression::Unary {
                op: UnaryOperator::Neg,
                operand: Box::new(ident_expr("x")),
            },
            "-x",
        ),
        (
            Expression::Unary {
                op: UnaryOperator::Not,
                operand: Box::new(ident_expr("x")),
            },
            "!x",
        ),
        (
            Expression::Call {
                callee: Box::new(ident_expr("foo")),
                args: vec![ident_expr("a"), Expression::Literal(Literal::Integer(1))],
            },
            "foo(a, 1)",
        ),
        (
            Expression::MethodCall {
                receiver: Box::new(ident_expr("x")),
                method: "len".into(),
                turbofish: vec![],
                args: vec![],
            },
            "x.len()",
        ),
        (
            Expression::MethodCall {
                receiver: Box::new(ident_expr("x")),
                method: "into_iter".into(),
                turbofish: vec![],
                args: vec![],
            },
            "x.into_iter()",
        ),
        (
            Expression::Field {
                base: Box::new(ident_expr("obj")),
                name: "field".into(),
            },
            "obj.field",
        ),
        (
            Expression::Index {
                base: Box::new(ident_expr("arr")),
                index: Box::new(ident_expr("i")),
            },
            "arr[i]",
        ),
        (
            Expression::Reference {
                is_mut: false,
                inner: Box::new(ident_expr("x")),
            },
            "&x",
        ),
        (
            Expression::Reference {
                is_mut: true,
                inner: Box::new(ident_expr("x")),
            },
            "&mut x",
        ),
        (Expression::Deref(Box::new(ident_expr("ptr"))), "*ptr"),
        (Expression::Try(Box::new(ident_expr("result"))), "result?"),
        (
            Expression::Cast {
                expr: Box::new(ident_expr("x")),
                ty: Box::new(Type::U64),
            },
            "x as u64",
        ),
        (
            Expression::Tuple(vec![
                Expression::Literal(Literal::Integer(1)),
                Expression::Literal(Literal::Integer(2)),
            ]),
            "(1, 2)",
        ),
        (
            Expression::Array(vec![
                Expression::Literal(Literal::Integer(1)),
                Expression::Literal(Literal::Integer(2)),
            ]),
            "[1, 2]",
        ),
        (
            Expression::Repeat {
                value: Box::new(Expression::Literal(Literal::Integer(0))),
                count: Box::new(Expression::Literal(Literal::Integer(10))),
            },
            "[0; 10]",
        ),
        (
            Expression::StructLiteral {
                path: simple_path("Point"),
                fields: vec![
                    FieldInit {
                        name: "x".into(),
                        value: Some(Expression::Literal(Literal::Integer(1))),
                    },
                    FieldInit {
                        name: "y".into(),
                        value: None,
                    },
                ],
                rest: None,
            },
            "Point { x: 1, y }",
        ),
        (
            Expression::StructLiteral {
                path: simple_path("Point"),
                fields: vec![FieldInit {
                    name: "x".into(),
                    value: Some(Expression::Literal(Literal::Integer(1))),
                }],
                rest: Some(Box::new(ident_expr("default_pt"))),
            },
            "Point { x: 1, ..default_pt }",
        ),
        (Expression::Return(None), "return"),
        (
            Expression::Return(Some(Box::new(Expression::Literal(Literal::Integer(0))))),
            "return 0",
        ),
        (
            Expression::Break {
                label: None,
                value: None,
            },
            "break",
        ),
        (
            Expression::Break {
                label: Some("outer".into()),
                value: None,
            },
            "break 'outer",
        ),
        (Expression::Continue { label: None }, "continue"),
        (
            Expression::Continue {
                label: Some("top".into()),
            },
            "continue 'top",
        ),
        (
            Expression::Range {
                start: Some(Box::new(Expression::Literal(Literal::Integer(0)))),
                end: Some(Box::new(Expression::Literal(Literal::Integer(10)))),
                inclusive: true,
            },
            "0..=10",
        ),
        (
            Expression::Range {
                start: Some(Box::new(Expression::Literal(Literal::Integer(0)))),
                end: Some(Box::new(Expression::Literal(Literal::Integer(10)))),
                inclusive: false,
            },
            "0..10",
        ),
        (
            Expression::Range {
                start: None,
                end: Some(Box::new(Expression::Literal(Literal::Integer(10)))),
                inclusive: false,
            },
            "..10",
        ),
        (
            Expression::Range {
                start: Some(Box::new(Expression::Literal(Literal::Integer(0)))),
                end: None,
                inclusive: false,
            },
            "0..",
        ),
        (
            Expression::MacroCall {
                path: "println!".into(),
                tokens: "\"Hello\"".into(),
            },
            "println!(\"Hello\")",
        ),
    ];

    for (expr, expected) in cases {
        assert_eq!(expr.to_rust(), expected, "Expression::to_rust() mismatch");
    }
}

#[test]
fn pattern_to_rust_cases() {
    let cases: Vec<(Pattern, &str)> = vec![
        (Pattern::Wildcard, "_"),
        (Pattern::Rest, ".."),
        (
            Pattern::Ident {
                name: "x".into(),
                is_ref: false,
                is_mut: false,
                subpattern: None,
            },
            "x",
        ),
        (
            Pattern::Ident {
                name: "ref_x".into(),
                is_ref: true,
                is_mut: true,
                subpattern: None,
            },
            "ref mut ref_x",
        ),
        (Pattern::Literal(Literal::Integer(42)), "42"),
        (
            Pattern::Tuple(vec![
                Pattern::Ident {
                    name: "x".into(),
                    is_ref: false,
                    is_mut: false,
                    subpattern: None,
                },
                Pattern::Ident {
                    name: "y".into(),
                    is_ref: false,
                    is_mut: false,
                    subpattern: None,
                },
            ]),
            "(x, y)",
        ),
        (
            Pattern::Slice(vec![
                Pattern::Ident {
                    name: "first".into(),
                    is_ref: false,
                    is_mut: false,
                    subpattern: None,
                },
                Pattern::Rest,
            ]),
            "[first, ..]",
        ),
        (
            Pattern::TupleStruct {
                path: simple_path("Some"),
                elems: vec![Pattern::Ident {
                    name: "val".into(),
                    is_ref: false,
                    is_mut: false,
                    subpattern: None,
                }],
            },
            "Some(val)",
        ),
        (
            Pattern::Struct {
                path: simple_path("Point"),
                fields: vec![
                    FieldPattern {
                        name: "x".into(),
                        pattern: None,
                    },
                    FieldPattern {
                        name: "y".into(),
                        pattern: Some(Pattern::Ident {
                            name: "py".into(),
                            is_ref: false,
                            is_mut: false,
                            subpattern: None,
                        }),
                    },
                ],
                has_rest: false,
            },
            "Point { x, y: py }",
        ),
        (
            Pattern::Or(vec![
                Pattern::Literal(Literal::Integer(1)),
                Pattern::Literal(Literal::Integer(2)),
            ]),
            "1 | 2",
        ),
        (
            Pattern::Reference {
                is_mut: true,
                inner: Box::new(Pattern::Ident {
                    name: "x".into(),
                    is_ref: false,
                    is_mut: false,
                    subpattern: None,
                }),
            },
            "&mut x",
        ),
    ];

    for (pat, expected) in cases {
        assert_eq!(
            pat.to_rust_str(),
            expected,
            "Pattern::to_rust_str() mismatch"
        );
    }
}

#[test]
fn integration_struct_and_impl() {
    let m = module(vec![
        Item::Struct(Struct {
            attributes: vec![Attribute::derive(vec!["Debug".into()])],
            visibility: Visibility::Public,
            name: "Greeter".into(),
            generics: empty_generics(),
            kind: StructKind::Named(vec![Field {
                attributes: vec![],
                visibility: Visibility::Private,
                name: "name".into(),
                ty: Type::path("String"),
            }]),
        }),
        Item::Impl(Impl {
            attributes: vec![],
            generics: empty_generics(),
            trait_: None,
            self_ty: Type::path("Greeter"),
            is_unsafe: false,
            items: vec![
                AssocItem::Function(Function {
                    name: "new".into(),
                    parameters: vec![typed_param("name", Type::path("&str"))],
                    return_type: Some(Type::SelfType),
                    body: Some(Block {
                        statements: vec![],
                        trailing_expr: Some(Box::new(Expression::StructLiteral {
                            path: simple_path("Greeter"),
                            fields: vec![FieldInit {
                                name: "name".into(),
                                value: Some(Expression::Call {
                                    callee: Box::new(ident_expr("String::from")),
                                    args: vec![ident_expr("name")],
                                }),
                            }],
                            rest: None,
                        })),
                    }),
                    ..empty_fn()
                }),
                AssocItem::Function(Function {
                    name: "greet".into(),
                    parameters: vec![self_ref()],
                    return_type: Some(Type::path("String")),
                    body: Some(Block {
                        statements: vec![],
                        trailing_expr: Some(Box::new(Expression::Call {
                            callee: Box::new(ident_expr("format!")),
                            args: vec![
                                Expression::Literal(Literal::String("Hello, {}!".into())),
                                Expression::Field {
                                    base: Box::new(ident_expr("self")),
                                    name: "name".into(),
                                },
                            ],
                        })),
                    }),
                    ..empty_fn()
                }),
            ],
        }),
    ]);

    let output = emit(&m);
    let expected = "\
#[derive(Debug)]
pub struct Greeter {
    name: String,
}

impl Greeter {
    fn new(name: &str) -> Self {
        Greeter { name: String::from(name) }
    }

    fn greet(&self) -> String {
        format!(\"Hello, {}!\", self.name)
    }
}
";
    assert_eq!(output, expected);
}

#[test]
fn inter_item_spacing_consecutive_uses() {
    let m = module(vec![
        Item::Use(Use {
            visibility: Visibility::Private,
            tree: UseTree::Path("std::io".into()),
        }),
        Item::Use(Use {
            visibility: Visibility::Private,
            tree: UseTree::Path("std::fs".into()),
        }),
        Item::Function(Function {
            name: "main".into(),
            body: Some(Block {
                statements: vec![],
                trailing_expr: None,
            }),
            ..empty_fn()
        }),
    ]);

    let output = emit(&m);
    let expected = "\
use std::io;
use std::fs;

fn main() {
}
";
    assert_eq!(output, expected);
}

#[test]
fn impl_with_assoc_type_and_const() {
    let m = module(vec![Item::Impl(Impl {
        attributes: vec![],
        generics: empty_generics(),
        trait_: Some(Type::path("Iterator")),
        self_ty: Type::path("MyIter"),
        is_unsafe: false,
        items: vec![
            AssocItem::Type(AssocType {
                attributes: vec![],
                name: "Item".into(),
                generics: empty_generics(),
                bounds: vec![],
                value: Some(Type::I32),
            }),
            AssocItem::Const(Const {
                attributes: vec![],
                visibility: Visibility::Private,
                name: "MAX".into(),
                ty: Type::Usize,
                value: Some(Expression::Literal(Literal::Integer(100))),
            }),
            AssocItem::Function(Function {
                name: "next".into(),
                parameters: vec![Parameter::Receiver(Receiver {
                    is_ref: true,
                    is_mut: true,
                    lifetime: None,
                })],
                return_type: Some(Type::generic(
                    "Option",
                    vec![GenericArg::Type(Box::new(Type::I32))],
                )),
                body: Some(Block {
                    statements: vec![],
                    trailing_expr: Some(Box::new(Expression::Return(Some(Box::new(
                        Expression::Literal(Literal::Integer(42)),
                    ))))),
                }),
                ..empty_fn()
            }),
        ],
    })]);

    let output = emit(&m);
    let expected = "\
impl Iterator for MyIter {
    type Item = i32;

    const MAX: usize = 100;

    fn next(&mut self) -> Option<i32> {
        return 42;
    }
}
";
    assert_eq!(output, expected);
}

fn fn_call_stmt(name: &str, tokens: &str) -> Statement {
    Statement::Expression(Expression::MacroCall {
        path: name.into(),
        tokens: tokens.into(),
    })
}

#[test]
fn let_else_statement() {
    let m = module(vec![fn_with_body(
        "parse",
        vec![Statement::Let(Let {
            pattern: Pattern::Ident {
                name: "value".into(),
                is_ref: false,
                is_mut: false,
                subpattern: None,
            },
            ty: Some(Type::I32),
            value: Some(Expression::Call {
                callee: Box::new(ident_expr("try_parse")),
                args: vec![],
            }),
            is_mut: false,
            else_block: Some(Block {
                statements: vec![Statement::Expression(Expression::Return(Some(Box::new(
                    Expression::Literal(Literal::Integer(0)),
                ))))],
                trailing_expr: None,
            }),
        })],
    )]);

    let output = emit(&m);
    let expected = "\
fn parse() {
    let value: i32 = try_parse() else {
        return 0;
    };
}
";
    assert_eq!(output, expected);
}

#[test]
fn let_else_preserves_indent() {
    let m = module(vec![fn_with_body(
        "main",
        vec![
            Statement::Let(Let {
                pattern: Pattern::Ident {
                    name: "x".into(),
                    is_ref: false,
                    is_mut: false,
                    subpattern: None,
                },
                ty: None,
                value: Some(Expression::Call {
                    callee: Box::new(ident_expr("get")),
                    args: vec![],
                }),
                is_mut: false,
                else_block: Some(Block {
                    statements: vec![],
                    trailing_expr: None,
                }),
            }),
            Statement::Let(Let {
                pattern: Pattern::Ident {
                    name: "y".into(),
                    is_ref: false,
                    is_mut: false,
                    subpattern: None,
                },
                ty: None,
                value: Some(Expression::Literal(Literal::Integer(1))),
                is_mut: false,
                else_block: None,
            }),
        ],
    )]);

    let output = emit(&m);
    let expected = "\
fn main() {
    let x = get() else {
    };
    let y = 1;
}
";
    assert_eq!(output, expected);
}

#[test]
fn control_char_escaping_in_string() {
    let cases: Vec<(Literal, &str)> = vec![
        (Literal::String("hello\0world".into()), "\"hello\\0world\""),
        (Literal::String("\x07".into()), "\"\\u{7}\""),
        (Literal::String("\x1b".into()), "\"\\u{1b}\""),
    ];
    for (lit, expected) in cases {
        assert_eq!(
            Expression::Literal(lit).to_rust(),
            expected,
            "String escaping mismatch"
        );
    }
}

#[test]
fn control_char_escaping_in_char() {
    let cases: Vec<(Literal, &str)> = vec![
        (Literal::Char('\0'), "'\\0'"),
        (Literal::Char('\n'), "'\\n'"),
        (Literal::Char('\t'), "'\\t'"),
        (Literal::Char('\x07'), "'\\u{7}'"),
        (Literal::Char('\x1b'), "'\\u{1b}'"),
    ];
    for (lit, expected) in cases {
        assert_eq!(
            Expression::Literal(lit).to_rust(),
            expected,
            "Char escaping mismatch"
        );
    }
}

#[test]
fn struct_with_where_clause_commas() {
    let m = module(vec![Item::Struct(Struct {
        attributes: vec![],
        visibility: Visibility::Public,
        name: "Container".into(),
        generics: Generics {
            params: vec![
                GenericParam::Type {
                    name: "T".into(),
                    bounds: vec![],
                    default: None,
                },
                GenericParam::Type {
                    name: "U".into(),
                    bounds: vec![],
                    default: None,
                },
            ],
            where_clause: vec![
                WherePredicate {
                    ty: Type::path("T"),
                    bounds: vec![Type::path("Clone")],
                },
                WherePredicate {
                    ty: Type::path("U"),
                    bounds: vec![Type::path("Debug")],
                },
            ],
        },
        kind: StructKind::Named(vec![Field {
            attributes: vec![],
            visibility: Visibility::Private,
            name: "data".into(),
            ty: Type::path("T"),
        }]),
    })]);

    let output = emit(&m);
    assert!(
        output.contains("T: Clone,"),
        "Missing comma after T bound: {}",
        output
    );
    assert!(output.contains("U: Debug"), "Missing U bound: {}", output);
}

#[test]
fn mut_self_receiver() {
    let m = module(vec![Item::Function(Function {
        name: "consume".into(),
        parameters: vec![Parameter::Receiver(Receiver {
            is_ref: false,
            is_mut: true,
            lifetime: None,
        })],
        body: Some(Block {
            statements: vec![],
            trailing_expr: None,
        }),
        ..empty_fn()
    })]);

    let output = emit(&m);
    let expected = "\
fn consume(mut self) {
}
";
    assert_eq!(output, expected);
}

#[test]
fn enum_variant_no_visibility() {
    let m = module(vec![Item::Enum(Enum {
        attributes: vec![],
        visibility: Visibility::Public,
        name: "Status".into(),
        generics: empty_generics(),
        variants: vec![EnumVariant {
            attributes: vec![],
            name: "Active".into(),
            kind: VariantKind::Named(vec![Field {
                attributes: vec![],
                visibility: Visibility::Public,
                name: "id".into(),
                ty: Type::I32,
            }]),
            discriminant: None,
        }],
    })]);

    let output = emit(&m);
    assert!(
        !output.contains("pub id"),
        "Enum variant field should not emit visibility: {}",
        output
    );
    let expected = "\
pub enum Status {
    Active {
        id: i32,
    },
}
";
    assert_eq!(output, expected);
}

#[test]
fn abi_escaping() {
    let m = module(vec![Item::Function(Function {
        name: "foo".into(),
        abi: Some("C\"injected".into()),
        body: None,
        ..empty_fn()
    })]);

    let output = emit(&m);
    assert!(
        output.contains("extern \"C\\\"injected\""),
        "ABI quotes not escaped: {}",
        output
    );
}

#[test]
fn helpers_struct_with_methods() {
    use codeforge_rust::{attr, expr, param, ty, vis};

    let m = module(vec![
        Item::Struct(Struct {
            attributes: vec![attr::derive(&["Debug", "Clone"])],
            visibility: vis::public(),
            name: "Counter".into(),
            generics: empty_generics(),
            kind: StructKind::Named(vec![
                Field {
                    attributes: vec![],
                    visibility: vis::private(),
                    name: "value".into(),
                    ty: Type::I64,
                },
                Field {
                    attributes: vec![],
                    visibility: vis::private(),
                    name: "name".into(),
                    ty: ty::string(),
                },
            ]),
        }),
        Item::Impl(Impl {
            attributes: vec![],
            generics: empty_generics(),
            trait_: None,
            self_ty: Type::path("Counter"),
            is_unsafe: false,
            items: vec![
                AssocItem::Function(
                    function::build("new")
                        .param(param::typed("name", ty::reference(Type::Str)))
                        .returns(Type::SelfType)
                        .body_trailing(
                            vec![],
                            expr::struct_literal(
                                "Counter",
                                vec![
                                    expr::field_init("value", expr::int_lit(0)),
                                    expr::field_init(
                                        "name",
                                        expr::call(
                                            expr::ident("String::from"),
                                            vec![expr::ident("name")],
                                        ),
                                    ),
                                ],
                                None,
                            ),
                        )
                        .build(),
                ),
                AssocItem::Function(
                    function::build("value")
                        .param(param::self_ref())
                        .returns(Type::I64)
                        .body_trailing(vec![], expr::self_field("value"))
                        .build(),
                ),
                AssocItem::Function(
                    function::build("increment")
                        .param(param::self_mut())
                        .empty_body()
                        .build(),
                ),
            ],
        }),
    ]);

    let output = emit(&m);
    let expected = "\
#[derive(Debug, Clone)]
pub struct Counter {
    value: i64,
    name: String,
}

impl Counter {
    fn new(name: &str) -> Self {
        Counter { value: 0, name: String::from(name) }
    }

    fn value(&self) -> i64 {
        self.value
    }

    fn increment(&mut self) {
    }
}
";
    assert_eq!(output, expected);
}

#[test]
fn helpers_impl_block_with_expressions() {
    use codeforge_rust::{expr, param, stmt, ty};

    let m = module(vec![Item::Impl(Impl {
        attributes: vec![],
        generics: empty_generics(),
        trait_: Some(Type::path("Display")),
        self_ty: Type::path("Point"),
        is_unsafe: false,
        items: vec![AssocItem::Function(
            function::build("fmt")
                .param(param::self_ref())
                .param(param::typed(
                    "f",
                    ty::mut_reference(ty::reference(Type::path("Formatter"))),
                ))
                .returns(ty::result(Type::Unit, Type::path("fmt::Error")))
                .body_trailing(
                    vec![stmt::expr_stmt(expr::method_call(
                        expr::ident("f"),
                        "write_str",
                        vec![expr::macro_call("format!", "\"({}, {})\"")],
                    ))],
                    expr::return_none(),
                )
                .build(),
        )],
    })]);

    let output = emit(&m);
    let expected = "\
impl Display for Point {
    fn fmt(&self, f: &mut &Formatter) -> Result<(), fmt::Error> {
        f.write_str(format!(\"({}, {})\"));
        return;
    }
}
";
    assert_eq!(output, expected);
}

#[test]
fn helpers_enum_with_variants() {
    use codeforge_rust::{attr, ty, vis};

    let m = module(vec![Item::Enum(Enum {
        attributes: vec![attr::derive(&["Debug", "Clone", "PartialEq"])],
        visibility: vis::public(),
        name: "Command".into(),
        generics: empty_generics(),
        variants: vec![
            EnumVariant {
                attributes: vec![],
                name: "Quit".into(),
                kind: VariantKind::Unit,
                discriminant: None,
            },
            EnumVariant {
                attributes: vec![],
                name: "Move".into(),
                kind: VariantKind::Named(vec![
                    Field {
                        attributes: vec![],
                        visibility: vis::private(),
                        name: "x".into(),
                        ty: Type::I32,
                    },
                    Field {
                        attributes: vec![],
                        visibility: vis::private(),
                        name: "y".into(),
                        ty: Type::I32,
                    },
                ]),
                discriminant: None,
            },
            EnumVariant {
                attributes: vec![],
                name: "Write".into(),
                kind: VariantKind::Tuple(vec![ty::string()]),
                discriminant: None,
            },
            EnumVariant {
                attributes: vec![],
                name: "ChangeColor".into(),
                kind: VariantKind::Tuple(vec![Type::I32, Type::I32, Type::I32]),
                discriminant: None,
            },
        ],
    })]);

    let output = emit(&m);
    let expected = "\
#[derive(Debug, Clone, PartialEq)]
pub enum Command {
    Quit,
    Move {
        x: i32,
        y: i32,
    },
    Write(String),
    ChangeColor(i32, i32, i32),
}
";
    assert_eq!(output, expected);
}

#[test]
fn helpers_type_constructors() {
    use codeforge_rust::ty;

    assert_eq!(ty::reference(Type::Str).to_rust(), "&str");
    assert_eq!(ty::mut_reference(Type::I32).to_rust(), "&mut i32");
    assert_eq!(
        ty::reference_with_lifetime("a", true, Type::I32).to_rust(),
        "&'a mut i32"
    );
    assert_eq!(ty::pointer(Type::U8).to_rust(), "*const u8");
    assert_eq!(ty::mut_pointer(Type::U8).to_rust(), "*mut u8");
    assert_eq!(ty::slice(Type::U8).to_rust(), "[u8]");
    assert_eq!(ty::array(Type::I32, 4).to_rust(), "[i32; 4]");
    assert_eq!(ty::option(Type::I32).to_rust(), "Option<i32>");
    assert_eq!(ty::vec(ty::string()).to_rust(), "Vec<String>");
    assert_eq!(ty::box_(Type::I32).to_rust(), "Box<i32>");
    assert_eq!(
        ty::result(Type::I32, Type::path("Error")).to_rust(),
        "Result<i32, Error>"
    );
    assert_eq!(ty::string().to_rust(), "String");
    assert_eq!(
        ty::tuple(vec![Type::I32, Type::F64]).to_rust(),
        "(i32, f64)"
    );
    assert_eq!(
        ty::fn_ptr(vec![Type::I32], Some(Type::Bool)).to_rust(),
        "fn(i32) -> bool"
    );
    assert_eq!(
        ty::trait_object(vec![Type::path("Write"), Type::path("Send")]).to_rust(),
        "dyn Write + Send"
    );
    assert_eq!(
        ty::impl_trait(vec![Type::path("Iterator")]).to_rust(),
        "impl Iterator"
    );
}

#[test]
fn helpers_visibility_and_attrs() {
    use codeforge_rust::{attr, vis};

    assert_eq!(vis::public(), Visibility::Public);
    assert_eq!(vis::private(), Visibility::Private);
    assert_eq!(vis::crate_(), Visibility::Crate);
    assert_eq!(vis::super_(), Visibility::Super);
    assert_eq!(
        vis::restricted("crate::inner"),
        Visibility::Restricted("crate::inner".to_string())
    );

    let d = attr::derive(&["Debug", "Clone"]);
    assert_eq!(d.path, "derive");
    assert_eq!(d.tokens, Some("Debug, Clone".to_string()));

    let a = attr::allow("dead_code");
    assert_eq!(a.path, "allow");
    assert_eq!(a.tokens, Some("dead_code".to_string()));

    let c = attr::cfg("test");
    assert_eq!(c.path, "cfg");
    assert_eq!(c.tokens, Some("test".to_string()));
}

#[test]
fn helpers_expr_constructors() {
    use codeforge_rust::expr;

    assert_eq!(expr::ident("x").to_rust(), "x");
    assert_eq!(
        expr::path(&["std", "io", "Result"]).to_rust(),
        "std::io::Result"
    );
    assert_eq!(expr::str_lit("hi").to_rust(), "\"hi\"");
    assert_eq!(expr::int_lit(42).to_rust(), "42");
    assert_eq!(expr::bool_lit(true).to_rust(), "true");
    assert_eq!(expr::char_lit('z').to_rust(), "'z'");
    assert_eq!(expr::self_().to_rust(), "self");
    assert_eq!(expr::self_field("x").to_rust(), "self.x");
    assert_eq!(
        expr::call(expr::ident("foo"), vec![expr::int_lit(1)]).to_rust(),
        "foo(1)"
    );
    assert_eq!(
        expr::method_call(expr::ident("v"), "push", vec![expr::int_lit(1)]).to_rust(),
        "v.push(1)"
    );
    assert_eq!(expr::field(expr::ident("s"), "len").to_rust(), "s.len");
    assert_eq!(expr::ref_expr(expr::ident("x")).to_rust(), "&x");
    assert_eq!(expr::mut_ref(expr::ident("x")).to_rust(), "&mut x");
    assert_eq!(
        expr::struct_literal("P", vec![expr::field_init("x", expr::int_lit(1))], None).to_rust(),
        "P { x: 1 }"
    );
    assert_eq!(
        expr::binary(expr::ident("a"), BinaryOperator::Add, expr::ident("b")).to_rust(),
        "a + b"
    );
    assert_eq!(expr::return_expr(expr::int_lit(0)).to_rust(), "return 0");
    assert_eq!(expr::return_none().to_rust(), "return");
    assert_eq!(
        expr::macro_call("println!", "\"hi\"").to_rust(),
        "println!(\"hi\")"
    );
}

#[test]
fn helpers_stmt_constructors() {
    use codeforge_rust::expr;
    use codeforge_rust::stmt;

    let m = module(vec![fn_with_body(
        "main",
        vec![
            stmt::let_binding("x", expr::int_lit(10)),
            stmt::let_mut("y", expr::int_lit(20)),
            stmt::let_typed("z", Type::I32, expr::int_lit(30)),
            stmt::comment("compute result"),
            stmt::return_expr(expr::ident("z")),
        ],
    )]);

    let output = emit(&m);
    let expected = "\
fn main() {
    let x = 10;
    let mut y = 20;
    let z: i32 = 30;
    // compute result
    return z;
}
";
    assert_eq!(output, expected);
}
