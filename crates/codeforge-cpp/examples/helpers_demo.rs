use codeforge_cpp::*;

fn main() {
    let header_guard = "HANDLE_POOL_H";

    let pool_class = Class {
        name: "HandlePool".into(),
        base_classes: vec![],
        members: vec![
            ClassMember::Field(Field {
                name: "next_handle_".into(),
                var_type: Type::UInt32,
                initializer: Some(Expression::int_lit(0)),
                access: AccessSpecifier::Private,
                is_const: false,
                is_static: false,
                is_thread_local: false,
            }),
            ClassMember::Field(Field {
                name: "free_list_".into(),
                var_type: Type::vector(Type::UInt32),
                initializer: None,
                access: AccessSpecifier::Private,
                is_const: false,
                is_static: false,
                is_thread_local: false,
            }),
            ClassMember::Access(AccessSpecifier::Public),
            ClassMember::Constructor(Constructor {
                parameters: vec![],
                initializer_list: vec![],
                body: Block { statements: vec![] },
                is_explicit: false,
                is_deleted: false,
                is_defaulted: true,
            }),
            ClassMember::Destructor(Destructor {
                is_virtual: false,
                is_deleted: false,
                is_defaulted: true,
            }),
            ClassMember::Method(Function {
                name: "acquire".into(),
                return_type: Type::UInt32,
                parameters: vec![],
                body: Some(Block {
                    statements: vec![
                        stmt::declaration(Type::UInt32, "handle", None),
                        stmt::if_else(
                            Expression::call(
                                Expression::member(Expression::ident("free_list_"), "empty"),
                                vec![],
                            ),
                            vec![
                                stmt::assign_ident("handle", Expression::ident("next_handle_")),
                                stmt::expr_stmt(Expression::UnaryOp {
                                    op: UnaryOperator::PreInc,
                                    operand: Box::new(Expression::ident("next_handle_")),
                                }),
                            ],
                            vec![
                                stmt::assign_ident(
                                    "handle",
                                    Expression::call(
                                        Expression::member(Expression::ident("free_list_"), "back"),
                                        vec![],
                                    ),
                                ),
                                stmt::expr_stmt(Expression::method_call(
                                    Expression::ident("free_list_"),
                                    "pop_back",
                                    vec![],
                                )),
                            ],
                        ),
                        stmt::return_expr(Expression::ident("handle")),
                    ],
                }),
                is_const: false,
                is_inline: false,
                is_static: false,
                is_virtual: false,
                is_pure_virtual: false,
                is_override: false,
                is_noexcept: true,
            }),
            ClassMember::Method(Function {
                name: "release".into(),
                return_type: Type::Void,
                parameters: vec![Parameter {
                    name: "handle".into(),
                    param_type: Type::UInt32,
                    default_value: None,
                }],
                body: Some(Block {
                    statements: vec![stmt::expr_stmt(Expression::method_call(
                        Expression::ident("free_list_"),
                        "push_back",
                        vec![Expression::ident("handle")],
                    ))],
                }),
                is_const: false,
                is_inline: false,
                is_static: false,
                is_virtual: false,
                is_pure_virtual: false,
                is_override: false,
                is_noexcept: true,
            }),
            ClassMember::Method(Function {
                name: "free_count".into(),
                return_type: Type::UInt32,
                parameters: vec![],
                body: Some(Block {
                    statements: vec![stmt::return_expr(Expression::call(
                        Expression::member(Expression::ident("free_list_"), "size"),
                        vec![],
                    ))],
                }),
                is_const: true,
                is_inline: false,
                is_static: false,
                is_virtual: false,
                is_pure_virtual: false,
                is_override: false,
                is_noexcept: true,
            }),
            ClassMember::Method(Function {
                name: "reset".into(),
                return_type: Type::Void,
                parameters: vec![],
                body: Some(Block {
                    statements: vec![
                        stmt::assignment(Expression::ident("next_handle_"), Expression::int_lit(0)),
                        stmt::expr_stmt(Expression::method_call(
                            Expression::ident("free_list_"),
                            "clear",
                            vec![],
                        )),
                    ],
                }),
                is_const: false,
                is_inline: false,
                is_static: false,
                is_virtual: false,
                is_pure_virtual: false,
                is_override: false,
                is_noexcept: true,
            }),
        ],
        is_final: false,
    };

    let program = Program {
        directives: vec![
            directive::ifndef(header_guard),
            directive::define(header_guard),
            directive::include_system("cstdint"),
            directive::include_system("vector"),
        ],
        namespaces: vec![Namespace {
            name: "core".into(),
            declarations: vec![Declaration::Template(Box::new(Template {
                parameters: vec![TemplateParameter::Type {
                    name: "T".into(),
                    default: None,
                }],
                declaration: Box::new(Declaration::Class(pool_class.clone())),
            }))],
        }],
        declarations: vec![Declaration::Typedef(Typedef {
            name: "IntPool".into(),
            alias: Type::template_type("HandlePool", vec![Type::Int32]),
        })],
    };

    let output = emit(&program);
    println!("{}", output);

    drop(program);
}
