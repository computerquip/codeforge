use codeforge_python::{AnnAssign, ClassDef, FunctionDef, Parameter};
use codeforge_python::{Expression, Module, Statement, Type, decorator, emit, stmt};

fn main() {
    let module = Module {
        imports: vec![
            codeforge_python::Import::From(codeforge_python::FromImport {
                module: "dataclasses".into(),
                names: vec![codeforge_python::ImportName {
                    name: "dataclass".into(),
                    alias: None,
                }],
            }),
            codeforge_python::Import::From(codeforge_python::FromImport {
                module: "typing".into(),
                names: vec![
                    codeforge_python::ImportName {
                        name: "Optional".into(),
                        alias: None,
                    },
                    codeforge_python::ImportName {
                        name: "List".into(),
                        alias: None,
                    },
                ],
            }),
        ],
        body: vec![
            Statement::ClassDef(ClassDef {
                name: "DerEncoder".into(),
                decorators: vec![decorator::call("dataclass", vec![])],
                bases: vec![],
                keywords: vec![],
                body: vec![
                    Statement::AnnAssign(AnnAssign {
                        target: Expression::ident("buffer"),
                        annotation: Type::list_of(Type::Int),
                        value: Some(Expression::call(Expression::ident("list"), vec![])),
                    }),
                    Statement::AnnAssign(AnnAssign {
                        target: Expression::ident("nested"),
                        annotation: Type::optional_custom("DerEncoder"),
                        value: None,
                    }),
                    Statement::FunctionDef(Box::new(FunctionDef {
                        name: "__init__".into(),
                        decorators: vec![],
                        parameters: vec![Parameter {
                            name: "self".into(),
                            annotation: None,
                            default: None,
                        }],
                        vararg: None,
                        kw_only_params: vec![],
                        kwarg: None,
                        return_annotation: None,
                        body: vec![stmt::assign(
                            "self.buffer",
                            Expression::call(Expression::ident("list"), vec![]),
                        )],
                        docstring: None,
                        is_async: false,
                    })),
                    Statement::FunctionDef(Box::new(FunctionDef {
                        name: "write_tag".into(),
                        decorators: vec![],
                        parameters: vec![
                            Parameter {
                                name: "self".into(),
                                annotation: None,
                                default: None,
                            },
                            Parameter {
                                name: "tag".into(),
                                annotation: Some(Type::Int),
                                default: None,
                            },
                            Parameter {
                                name: "length".into(),
                                annotation: Some(Type::optional(Type::Int)),
                                default: Some(Expression::Literal(
                                    codeforge_python::Literal::None_,
                                )),
                            },
                        ],
                        vararg: None,
                        kw_only_params: vec![],
                        kwarg: None,
                        return_annotation: Some(Type::Self_),
                        body: vec![
                            stmt::method_call(
                                Expression::self_attr("buffer"),
                                "append",
                                vec![Expression::ident("tag")],
                            ),
                            stmt::if_simple(
                                Expression::BinaryOp {
                                    left: Box::new(Expression::ident("length")),
                                    op: codeforge_python::BinaryOperator::IsNot,
                                    right: Box::new(Expression::Literal(
                                        codeforge_python::Literal::None_,
                                    )),
                                },
                                vec![stmt::method_call(
                                    Expression::self_attr("buffer"),
                                    "append",
                                    vec![Expression::ident("length")],
                                )],
                            ),
                            stmt::return_value(Expression::ident("self")),
                        ],
                        docstring: None,
                        is_async: false,
                    })),
                    Statement::FunctionDef(Box::new(FunctionDef {
                        name: "write_int".into(),
                        decorators: vec![],
                        parameters: vec![
                            Parameter {
                                name: "self".into(),
                                annotation: None,
                                default: None,
                            },
                            Parameter {
                                name: "value".into(),
                                annotation: Some(Type::Int),
                                default: None,
                            },
                        ],
                        vararg: None,
                        kw_only_params: vec![],
                        kwarg: None,
                        return_annotation: Some(Type::Self_),
                        body: vec![
                            stmt::method_call(
                                Expression::self_attr("buffer"),
                                "extend",
                                vec![Expression::call(
                                    Expression::attr(Expression::ident("value"), "to_bytes"),
                                    vec![Expression::int_lit(1), Expression::str_lit("big")],
                                )],
                            ),
                            stmt::return_value(Expression::ident("self")),
                        ],
                        docstring: None,
                        is_async: false,
                    })),
                    Statement::FunctionDef(Box::new(FunctionDef {
                        name: "finish".into(),
                        decorators: vec![],
                        parameters: vec![Parameter {
                            name: "self".into(),
                            annotation: None,
                            default: None,
                        }],
                        vararg: None,
                        kw_only_params: vec![],
                        kwarg: None,
                        return_annotation: Some(Type::Bytes),
                        body: vec![stmt::return_value(Expression::call(
                            Expression::ident("bytes"),
                            vec![Expression::self_attr("buffer")],
                        ))],
                        docstring: None,
                        is_async: false,
                    })),
                ],
                docstring: Some("DER encoder for ASN.1 encoding.".into()),
            }),
            Statement::ClassDef(ClassDef {
                name: "Integer".into(),
                decorators: vec![decorator::call("dataclass", vec![])],
                bases: vec![],
                keywords: vec![],
                body: vec![
                    Statement::AnnAssign(AnnAssign {
                        target: Expression::ident("value"),
                        annotation: Type::Int,
                        value: None,
                    }),
                    Statement::FunctionDef(Box::new(FunctionDef {
                        name: "encode_der".into(),
                        decorators: vec![],
                        parameters: vec![Parameter {
                            name: "self".into(),
                            annotation: None,
                            default: None,
                        }],
                        vararg: None,
                        kw_only_params: vec![],
                        kwarg: None,
                        return_annotation: Some(Type::Bytes),
                        body: vec![
                            stmt::assign_call("enc", "DerEncoder", vec![]),
                            stmt::method_call(
                                Expression::ident("enc"),
                                "write_tag",
                                vec![Expression::Literal(codeforge_python::Literal::Bytes(vec![
                                    0x02,
                                ]))],
                            ),
                            stmt::method_call(
                                Expression::ident("enc"),
                                "write_int",
                                vec![Expression::self_attr("value")],
                            ),
                            stmt::return_value(Expression::method_call(
                                Expression::ident("enc"),
                                "finish",
                                vec![],
                            )),
                        ],
                        docstring: Some("Encode this integer as DER bytes.".into()),
                        is_async: false,
                    })),
                    Statement::FunctionDef(Box::new(FunctionDef {
                        name: "from_bytes".into(),
                        decorators: vec![decorator::classmethod()],
                        parameters: vec![
                            Parameter {
                                name: "cls".into(),
                                annotation: None,
                                default: None,
                            },
                            Parameter {
                                name: "data".into(),
                                annotation: Some(Type::Bytes),
                                default: None,
                            },
                        ],
                        vararg: None,
                        kw_only_params: vec![],
                        kwarg: None,
                        return_annotation: Some(Type::Self_),
                        body: vec![
                            stmt::assign(
                                "value",
                                Expression::call(
                                    Expression::attr(Expression::ident("int"), "from_bytes"),
                                    vec![Expression::ident("data"), Expression::str_lit("big")],
                                ),
                            ),
                            stmt::return_value(Expression::call(
                                Expression::ident("cls"),
                                vec![Expression::ident("value")],
                            )),
                        ],
                        docstring: None,
                        is_async: false,
                    })),
                ],
                docstring: None,
            }),
        ],
    };

    let output = emit(&module);
    println!("{}", output);
}
