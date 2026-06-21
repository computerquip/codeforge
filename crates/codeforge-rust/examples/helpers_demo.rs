use codeforge_rust::*;
use codeforge_rust::{attr, expr, function, param, stmt, ty, vis};

fn main() {
    let m = Module {
        attributes: vec![],
        items: vec![
            Item::Struct(Struct {
                attributes: vec![attr::derive(&["Debug", "Clone"])],
                visibility: vis::public(),
                name: "Cache".into(),
                generics: Generics {
                    params: vec![
                        GenericParam::Type {
                            name: "K".into(),
                            bounds: vec![],
                            default: None,
                        },
                        GenericParam::Type {
                            name: "V".into(),
                            bounds: vec![],
                            default: None,
                        },
                    ],
                    where_clause: vec![],
                },
                kind: StructKind::Named(vec![
                    Field {
                        attributes: vec![],
                        visibility: vis::private(),
                        name: "entries".into(),
                        ty: ty::generic(
                            "HashMap",
                            vec![
                                GenericArg::Type(Box::new(Type::path("K"))),
                                GenericArg::Type(Box::new(ty::option(Type::path("V")))),
                            ],
                        ),
                    },
                    Field {
                        attributes: vec![],
                        visibility: vis::private(),
                        name: "max_size".into(),
                        ty: Type::Usize,
                    },
                ]),
            }),
            Item::Impl(Impl {
                attributes: vec![],
                generics: Generics {
                    params: vec![
                        GenericParam::Type {
                            name: "K".into(),
                            bounds: vec![],
                            default: None,
                        },
                        GenericParam::Type {
                            name: "V".into(),
                            bounds: vec![],
                            default: None,
                        },
                    ],
                    where_clause: vec![],
                },
                trait_: None,
                self_ty: ty::generic(
                    "Cache",
                    vec![
                        GenericArg::Type(Box::new(Type::path("K"))),
                        GenericArg::Type(Box::new(Type::path("V"))),
                    ],
                ),
                is_unsafe: false,
                items: vec![
                    AssocItem::Function(
                        function::build("new")
                            .param(param::typed("max_size", Type::Usize))
                            .returns(Type::SelfType)
                            .body_trailing(
                                vec![],
                                expr::struct_literal(
                                    "Cache",
                                    vec![
                                        expr::field_init(
                                            "entries",
                                            expr::call(expr::ident("HashMap::new"), vec![]),
                                        ),
                                        expr::field_shorthand("max_size"),
                                    ],
                                    None,
                                ),
                            )
                            .build(),
                    ),
                    AssocItem::Function(
                        function::build("get")
                            .vis(vis::public())
                            .param(param::typed("key", ty::reference(Type::path("K"))))
                            .returns(ty::option(ty::reference(Type::path("V"))))
                            .body_trailing(
                                vec![],
                                expr::method_call(
                                    expr::self_field("entries"),
                                    "get",
                                    vec![expr::ident("key")],
                                ),
                            )
                            .build(),
                    ),
                    AssocItem::Function(
                        function::build("insert")
                            .vis(vis::public())
                            .param(param::self_mut())
                            .param(param::typed("key", Type::path("K")))
                            .param(param::typed("value", Type::path("V")))
                            .empty_body()
                            .build(),
                    ),
                    AssocItem::Function(
                        function::build("len")
                            .vis(vis::public())
                            .param(param::self_ref())
                            .returns(Type::Usize)
                            .body_trailing(
                                vec![],
                                expr::method_call(expr::self_field("entries"), "len", vec![]),
                            )
                            .build(),
                    ),
                ],
            }),
            Item::Enum(Enum {
                attributes: vec![attr::derive(&["Debug"])],
                visibility: vis::public(),
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
            }),
            Item::Function(
                function::build("process")
                    .vis(vis::public())
                    .param(param::typed("input", ty::reference(ty::slice(Type::U8))))
                    .returns(ty::result(ty::vec(Type::U8), Type::path("Error")))
                    .body_trailing(
                        vec![
                            stmt::let_binding(
                                "result",
                                expr::call(expr::ident("validate"), vec![expr::ident("input")]),
                            ),
                            stmt::expr_stmt(expr::method_call(
                                expr::ident("result"),
                                "map",
                                vec![expr::closure(
                                    vec![ClosureParam {
                                        pattern: Pattern::Ident {
                                            name: "v".into(),
                                            is_ref: false,
                                            is_mut: false,
                                            subpattern: None,
                                        },
                                        ty: None,
                                    }],
                                    expr::call(expr::ident("transform"), vec![expr::ident("v")]),
                                )],
                            )),
                        ],
                        expr::method_call(
                            expr::ident("result"),
                            "map_err",
                            vec![expr::ident("wrap_error")],
                        ),
                    )
                    .build(),
            ),
        ],
    };

    let output = emit(&m);
    println!("{}", output);
}
