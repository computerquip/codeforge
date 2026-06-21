use codeforge_cpp::*;

fn program(declarations: Vec<Declaration>) -> Program {
    Program {
        directives: vec![],
        namespaces: vec![],
        declarations,
    }
}

#[test]
fn function_with_body() {
    let p = program(vec![Declaration::Function(Function {
        name: "add".into(),
        return_type: Type::Int32,
        parameters: vec![
            Parameter {
                name: "a".into(),
                param_type: Type::Int32,
                default_value: None,
            },
            Parameter {
                name: "b".into(),
                param_type: Type::Int32,
                default_value: None,
            },
        ],
        body: Some(Block {
            statements: vec![Statement::Return(Some(Expression::BinaryOp {
                left: Box::new(Expression::Identifier("a".into())),
                op: BinaryOperator::Add,
                right: Box::new(Expression::Identifier("b".into())),
            }))],
        }),
        is_const: false,
        is_inline: false,
        is_static: false,
        is_virtual: false,
        is_pure_virtual: false,
        is_override: false,
        is_noexcept: false,
    })]);

    let output = emit(&p);
    let expected = "\
int32_t add(int32_t a, int32_t b) {
    return a + b;
}
";
    assert_eq!(output, expected);
}

#[test]
fn function_declaration_only() {
    let p = program(vec![Declaration::Function(Function {
        name: "foo".into(),
        return_type: Type::Void,
        parameters: vec![],
        body: None,
        is_const: false,
        is_inline: false,
        is_static: false,
        is_virtual: false,
        is_pure_virtual: false,
        is_override: false,
        is_noexcept: false,
    })]);

    let output = emit(&p);
    let expected = "\
void foo();
";
    assert_eq!(output, expected);
}

#[test]
fn scoped_enum() {
    let p = program(vec![Declaration::Enum(Enum {
        name: "Color".into(),
        underlying_type: Some(Type::Int32),
        variants: vec![
            EnumVariant {
                name: "Red".into(),
                value: Some(Expression::Literal(Literal::Integer(0))),
            },
            EnumVariant {
                name: "Green".into(),
                value: Some(Expression::Literal(Literal::Integer(1))),
            },
            EnumVariant {
                name: "Blue".into(),
                value: Some(Expression::Literal(Literal::Integer(2))),
            },
        ],
        is_scoped: true,
    })]);

    let output = emit(&p);
    let expected = "\
enum class Color : int32_t {
    Red = 0,
    Green = 1,
    Blue = 2
};
";
    assert_eq!(output, expected);
}

#[test]
fn unscoped_enum() {
    let p = program(vec![Declaration::Enum(Enum {
        name: "Direction".into(),
        underlying_type: None,
        variants: vec![
            EnumVariant {
                name: "North".into(),
                value: None,
            },
            EnumVariant {
                name: "South".into(),
                value: None,
            },
        ],
        is_scoped: false,
    })]);

    let output = emit(&p);
    let expected = "\
enum Direction {
    North,
    South
};
";
    assert_eq!(output, expected);
}

#[test]
fn namespace_with_function() {
    let p = Program {
        directives: vec![],
        namespaces: vec![Namespace {
            name: "math".into(),
            declarations: vec![Declaration::Function(Function {
                name: "square".into(),
                return_type: Type::Float64,
                parameters: vec![Parameter {
                    name: "x".into(),
                    param_type: Type::Float64,
                    default_value: None,
                }],
                body: Some(Block {
                    statements: vec![Statement::Return(Some(Expression::BinaryOp {
                        left: Box::new(Expression::Identifier("x".into())),
                        op: BinaryOperator::Mul,
                        right: Box::new(Expression::Identifier("x".into())),
                    }))],
                }),
                is_const: false,
                is_inline: false,
                is_static: false,
                is_virtual: false,
                is_pure_virtual: false,
                is_override: false,
                is_noexcept: false,
            })],
        }],
        declarations: vec![],
    };

    let output = emit(&p);
    let expected = "\
namespace math {
    double square(double x) {
        return x * x;
    }
} // namespace math
";
    assert_eq!(output, expected);
}

#[test]
fn typedef_test() {
    let p = program(vec![Declaration::Typedef(Typedef {
        name: "StringVec".into(),
        alias: Type::Template {
            name: "std::vector".into(),
            arguments: vec![Type::String],
        },
    })]);

    let output = emit(&p);
    let expected = "\
using StringVec = std::vector<std::string>;
";
    assert_eq!(output, expected);
}

#[test]
fn struct_test() {
    let p = program(vec![Declaration::Struct(Struct {
        name: "Point".into(),
        fields: vec![
            Field {
                name: "x".into(),
                var_type: Type::Float64,
                initializer: None,
                access: AccessSpecifier::Public,
                is_const: false,
                is_static: false,
                is_thread_local: false,
            },
            Field {
                name: "y".into(),
                var_type: Type::Float64,
                initializer: None,
                access: AccessSpecifier::Public,
                is_const: false,
                is_static: false,
                is_thread_local: false,
            },
        ],
    })]);

    let output = emit(&p);
    let expected = "\
struct Point {
    public double x;
    public double y;
};
";
    assert_eq!(output, expected);
}

#[test]
fn class_with_ctor_dtor_methods() {
    let p = program(vec![Declaration::Class(Class {
        name: "Animal".into(),
        base_classes: vec![],
        members: vec![
            ClassMember::Access(AccessSpecifier::Private),
            ClassMember::Field(Field {
                name: "name_".into(),
                var_type: Type::String,
                initializer: None,
                access: AccessSpecifier::Private,
                is_const: false,
                is_static: false,
                is_thread_local: false,
            }),
            ClassMember::Access(AccessSpecifier::Public),
            ClassMember::Constructor(Constructor {
                parameters: vec![Parameter {
                    name: "name".into(),
                    param_type: Type::ConstReference(Box::new(Type::String)),
                    default_value: None,
                }],
                initializer_list: vec![MemberInitializer {
                    member_name: "name_".into(),
                    value: Expression::Identifier("name".into()),
                }],
                body: Block { statements: vec![] },
                is_explicit: false,
                is_deleted: false,
                is_defaulted: false,
            }),
            ClassMember::Destructor(Destructor {
                is_virtual: true,
                is_deleted: false,
                is_defaulted: false,
            }),
            ClassMember::Method(Function {
                name: "speak".into(),
                return_type: Type::Void,
                parameters: vec![],
                body: None,
                is_const: true,
                is_inline: false,
                is_static: false,
                is_virtual: true,
                is_pure_virtual: true,
                is_override: false,
                is_noexcept: false,
            }),
        ],
        is_final: false,
    })]);

    let output = emit(&p);
    let expected = "\
class Animal {
    private:
    private std::string name_;
    public:
    Animal(const std::string& name) :
        name_(name)
    {
    }
    virtual ~Animal() {}
    virtual void speak() const = 0;
};
";
    assert_eq!(output, expected);
}

#[test]
fn includes_and_multiple_declarations() {
    let p = Program {
        directives: vec![
            Directive::Include(Include::System("iostream".into())),
            Directive::Include(Include::System("vector".into())),
            Directive::Include(Include::Local("myheader.h".into())),
        ],
        namespaces: vec![],
        declarations: vec![
            Declaration::Typedef(Typedef {
                name: "i32".into(),
                alias: Type::Int32,
            }),
            Declaration::Function(Function {
                name: "main".into(),
                return_type: Type::Int32,
                parameters: vec![],
                body: Some(Block {
                    statements: vec![Statement::Return(Some(Expression::Literal(
                        Literal::Integer(0),
                    )))],
                }),
                is_const: false,
                is_inline: false,
                is_static: false,
                is_virtual: false,
                is_pure_virtual: false,
                is_override: false,
                is_noexcept: false,
            }),
        ],
    };

    let output = emit(&p);
    let expected = "\
#include <iostream>
#include <vector>
#include \"myheader.h\"

using i32 = int32_t;

int32_t main() {
    return 0;
}
";
    assert_eq!(output, expected);
}

#[test]
fn if_else_statement() {
    let p = program(vec![Declaration::Function(Function {
        name: "abs".into(),
        return_type: Type::Int32,
        parameters: vec![Parameter {
            name: "x".into(),
            param_type: Type::Int32,
            default_value: None,
        }],
        body: Some(Block {
            statements: vec![Statement::If(Box::new(IfStatement {
                condition: Expression::BinaryOp {
                    left: Box::new(Expression::Identifier("x".into())),
                    op: BinaryOperator::Lt,
                    right: Box::new(Expression::Literal(Literal::Integer(0))),
                },
                then_block: Block {
                    statements: vec![Statement::Return(Some(Expression::UnaryOp {
                        op: UnaryOperator::Neg,
                        operand: Box::new(Expression::Identifier("x".into())),
                    }))],
                },
                else_block: Some(Block {
                    statements: vec![Statement::Return(Some(Expression::Identifier("x".into())))],
                }),
            }))],
        }),
        is_const: false,
        is_inline: false,
        is_static: false,
        is_virtual: false,
        is_pure_virtual: false,
        is_override: false,
        is_noexcept: false,
    })]);

    let output = emit(&p);
    let expected = "\
int32_t abs(int32_t x) {
    if (x < 0) {
        return -x;
    } else {
        return x;
    }
}
";
    assert_eq!(output, expected);
}

#[test]
fn for_loop() {
    let p = program(vec![Declaration::Function(Function {
        name: "sum_to".into(),
        return_type: Type::Int32,
        parameters: vec![Parameter {
            name: "n".into(),
            param_type: Type::Int32,
            default_value: None,
        }],
        body: Some(Block {
            statements: vec![
                Statement::VariableDeclaration(LocalVariable {
                    name: "total".into(),
                    var_type: Type::Int32,
                    initializer: Some(Expression::Literal(Literal::Integer(0))),
                    is_const: false,
                    is_static: false,
                    is_thread_local: false,
                }),
                Statement::For(Box::new(ForStatement {
                    initializer: Some(Box::new(Statement::VariableDeclaration(LocalVariable {
                        name: "i".into(),
                        var_type: Type::Int32,
                        initializer: Some(Expression::Literal(Literal::Integer(0))),
                        is_const: false,
                        is_static: false,
                        is_thread_local: false,
                    }))),
                    condition: Some(Expression::BinaryOp {
                        left: Box::new(Expression::Identifier("i".into())),
                        op: BinaryOperator::Lt,
                        right: Box::new(Expression::Identifier("n".into())),
                    }),
                    update: Some(Expression::UnaryOp {
                        op: UnaryOperator::PostInc,
                        operand: Box::new(Expression::Identifier("i".into())),
                    }),
                    body: Block {
                        statements: vec![Statement::Expression(Expression::BinaryOp {
                            left: Box::new(Expression::Identifier("total".into())),
                            op: BinaryOperator::AddAssign,
                            right: Box::new(Expression::Identifier("i".into())),
                        })],
                    },
                })),
                Statement::Return(Some(Expression::Identifier("total".into()))),
            ],
        }),
        is_const: false,
        is_inline: false,
        is_static: false,
        is_virtual: false,
        is_pure_virtual: false,
        is_override: false,
        is_noexcept: false,
    })]);

    let output = emit(&p);
    let expected = "\
int32_t sum_to(int32_t n) {
    int32_t total = 0;
    for (int32_t i = 0; i < n; i++) {
        total += i;
    }
    return total;
}
";
    assert_eq!(output, expected);
}

#[test]
fn while_loop() {
    let p = program(vec![Declaration::Function(Function {
        name: "countdown".into(),
        return_type: Type::Void,
        parameters: vec![Parameter {
            name: "n".into(),
            param_type: Type::Int32,
            default_value: None,
        }],
        body: Some(Block {
            statements: vec![Statement::While(Box::new(WhileStatement {
                condition: Expression::BinaryOp {
                    left: Box::new(Expression::Identifier("n".into())),
                    op: BinaryOperator::Gt,
                    right: Box::new(Expression::Literal(Literal::Integer(0))),
                },
                body: Block {
                    statements: vec![
                        Statement::Expression(Expression::Call {
                            callee: Box::new(Expression::Identifier("print".into())),
                            arguments: vec![Expression::Identifier("n".into())],
                        }),
                        Statement::Expression(Expression::UnaryOp {
                            op: UnaryOperator::PreDec,
                            operand: Box::new(Expression::Identifier("n".into())),
                        }),
                    ],
                },
            }))],
        }),
        is_const: false,
        is_inline: false,
        is_static: false,
        is_virtual: false,
        is_pure_virtual: false,
        is_override: false,
        is_noexcept: false,
    })]);

    let output = emit(&p);
    let expected = "\
void countdown(int32_t n) {
    while (n > 0) {
        print(n);
        --n;
    }
}
";
    assert_eq!(output, expected);
}

#[test]
fn expressions_and_literals() {
    let p = program(vec![Declaration::Variable(LocalVariable {
        name: "golden".into(),
        var_type: Type::Float64,
        initializer: Some(Expression::Literal(Literal::Float(F64Wrapper(1.61803)))),
        is_const: true,
        is_static: false,
        is_thread_local: false,
    })]);

    let output = emit(&p);
    let expected = "\
const double golden = 1.61803;
";
    assert_eq!(output, expected);
}

#[test]
fn cast_expression() {
    let fn_decl = Declaration::Function(Function {
        name: "convert".into(),
        return_type: Type::Int32,
        parameters: vec![Parameter {
            name: "x".into(),
            param_type: Type::Float64,
            default_value: None,
        }],
        body: Some(Block {
            statements: vec![Statement::Return(Some(Expression::Cast {
                target_type: Type::Int32,
                expr: Box::new(Expression::Identifier("x".into())),
            }))],
        }),
        is_const: false,
        is_inline: false,
        is_static: false,
        is_virtual: false,
        is_pure_virtual: false,
        is_override: false,
        is_noexcept: false,
    });

    let output = emit(&program(vec![fn_decl]));
    let expected = "\
int32_t convert(double x) {
    return static_cast<int32_t>(x);
}
";
    assert_eq!(output, expected);
}

#[test]
fn template_function() {
    let p = program(vec![Declaration::Template(Box::new(Template {
        parameters: vec![TemplateParameter::Type {
            name: "T".into(),
            default: None,
        }],
        declaration: Box::new(Declaration::Function(Function {
            name: "identity".into(),
            return_type: Type::Custom("T".into()),
            parameters: vec![Parameter {
                name: "value".into(),
                param_type: Type::ConstReference(Box::new(Type::Custom("T".into()))),
                default_value: None,
            }],
            body: Some(Block {
                statements: vec![Statement::Return(Some(Expression::Identifier(
                    "value".into(),
                )))],
            }),
            is_const: false,
            is_inline: false,
            is_static: false,
            is_virtual: false,
            is_pure_virtual: false,
            is_override: false,
            is_noexcept: false,
        })),
    }))]);

    let output = emit(&p);
    let expected = "\
template <typename T>
T identity(const T& value) {
    return value;
}
";
    assert_eq!(output, expected);
}

#[test]
fn template_class() {
    let p = program(vec![Declaration::Template(Box::new(Template {
        parameters: vec![
            TemplateParameter::Type {
                name: "T".into(),
                default: None,
            },
            TemplateParameter::NonType {
                param_type: Type::UInt32,
                name: "N".into(),
                default: None,
            },
        ],
        declaration: Box::new(Declaration::Class(Class {
            name: "FixedArray".into(),
            base_classes: vec![],
            members: vec![
                ClassMember::Access(AccessSpecifier::Private),
                ClassMember::Field(Field {
                    name: "data_".into(),
                    var_type: Type::Array(Box::new(Type::Custom("T".into())), None),
                    initializer: None,
                    access: AccessSpecifier::Private,
                    is_const: false,
                    is_static: false,
                    is_thread_local: false,
                }),
                ClassMember::Access(AccessSpecifier::Public),
                ClassMember::Method(Function {
                    name: "size".into(),
                    return_type: Type::UInt32,
                    parameters: vec![],
                    body: Some(Block {
                        statements: vec![Statement::Return(Some(Expression::Identifier(
                            "N".into(),
                        )))],
                    }),
                    is_const: true,
                    is_inline: false,
                    is_static: false,
                    is_virtual: false,
                    is_pure_virtual: false,
                    is_override: false,
                    is_noexcept: true,
                }),
            ],
            is_final: false,
        })),
    }))]);

    let output = emit(&p);
    let expected = "\
template <typename T, uint32_t N>
class FixedArray {
    private:
    private T[] data_;
    public:
    uint32_t size() const noexcept {
        return N;
    }
};
";
    assert_eq!(output, expected);
}

#[test]
fn template_with_defaults() {
    let p = program(vec![Declaration::Template(Box::new(Template {
        parameters: vec![
            TemplateParameter::Type {
                name: "Key".into(),
                default: None,
            },
            TemplateParameter::Type {
                name: "Value".into(),
                default: None,
            },
            TemplateParameter::Type {
                name: "Hash".into(),
                default: Some(Type::Template {
                    name: "std::hash".into(),
                    arguments: vec![Type::Custom("Key".into())],
                }),
            },
        ],
        declaration: Box::new(Declaration::Class(Class {
            name: "HashMap".into(),
            base_classes: vec![],
            members: vec![],
            is_final: false,
        })),
    }))]);

    let output = emit(&p);
    let expected = "\
template <typename Key, typename Value, typename Hash = std::hash<Key>>
class HashMap {
};
";
    assert_eq!(output, expected);
}

// ── Helper-based tests ──────────────────────────────────────────────────────

#[test]
fn helpers_class_with_methods_and_fields() {
    let p = Program {
        directives: vec![
            directive::include_system("string"),
            directive::include_system("iostream"),
        ],
        namespaces: vec![],
        declarations: vec![Declaration::Class(Class {
            name: "Logger".into(),
            base_classes: vec![],
            members: vec![
                ClassMember::Access(AccessSpecifier::Private),
                ClassMember::Field(Field {
                    name: "name_".into(),
                    var_type: Type::String,
                    initializer: None,
                    access: AccessSpecifier::Private,
                    is_const: false,
                    is_static: false,
                    is_thread_local: false,
                }),
                ClassMember::Access(AccessSpecifier::Public),
                ClassMember::Constructor(Constructor {
                    parameters: vec![Parameter {
                        name: "name".into(),
                        param_type: Type::const_ref(Type::String),
                        default_value: None,
                    }],
                    initializer_list: vec![MemberInitializer {
                        member_name: "name_".into(),
                        value: Expression::ident("name"),
                    }],
                    body: Block { statements: vec![] },
                    is_explicit: true,
                    is_deleted: false,
                    is_defaulted: false,
                }),
                ClassMember::Destructor(Destructor {
                    is_virtual: false,
                    is_deleted: false,
                    is_defaulted: true,
                }),
                ClassMember::Method(Function {
                    name: "log".into(),
                    return_type: Type::Void,
                    parameters: vec![Parameter {
                        name: "msg".into(),
                        param_type: Type::const_ref(Type::String),
                        default_value: None,
                    }],
                    body: Some(Block {
                        statements: vec![stmt::expr_stmt(Expression::call(
                            Expression::ident("std::cout"),
                            vec![Expression::ptr_member(Expression::ident("this"), "name_")],
                        ))],
                    }),
                    is_const: false,
                    is_inline: false,
                    is_static: false,
                    is_virtual: false,
                    is_pure_virtual: false,
                    is_override: false,
                    is_noexcept: false,
                }),
                ClassMember::Method(Function {
                    name: "get_name".into(),
                    return_type: Type::const_ref(Type::String),
                    parameters: vec![],
                    body: Some(Block {
                        statements: vec![stmt::return_expr(Expression::ptr_member(
                            Expression::ident("this"),
                            "name_",
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
            ],
            is_final: false,
        })],
    };

    let output = emit(&p);
    let expected = "\
#include <string>
#include <iostream>

class Logger {
    private:
    private std::string name_;
    public:
    explicit Logger(const std::string& name) :
        name_(name)
    {
    }
    ~Logger() = default;
    void log(const std::string& msg) {
        std::cout(this->name_);
    }
    const std::string& get_name() const noexcept {
        return this->name_;
    }
};
";
    assert_eq!(output, expected);
}

#[test]
fn helpers_function_with_directives() {
    let p = Program {
        directives: vec![
            directive::ifndef("GUARD_H"),
            directive::define("GUARD_H"),
            directive::include_system("cstdint"),
            directive::include_system("vector"),
        ],
        namespaces: vec![],
        declarations: vec![Declaration::Function(Function {
            name: "compute".into(),
            return_type: Type::Int32,
            parameters: vec![Parameter {
                name: "n".into(),
                param_type: Type::Int32,
                default_value: None,
            }],
            body: Some(Block {
                statements: vec![
                    stmt::declaration(Type::Int32, "result", Some(Expression::int_lit(0))),
                    stmt::for_simple(
                        stmt::declaration(Type::Int32, "i", Some(Expression::int_lit(0))),
                        Expression::BinaryOp {
                            left: Box::new(Expression::ident("i")),
                            op: BinaryOperator::Lt,
                            right: Box::new(Expression::ident("n")),
                        },
                        Expression::UnaryOp {
                            op: UnaryOperator::PostInc,
                            operand: Box::new(Expression::ident("i")),
                        },
                        vec![stmt::expr_stmt(Expression::BinaryOp {
                            left: Box::new(Expression::ident("result")),
                            op: BinaryOperator::AddAssign,
                            right: Box::new(Expression::ident("i")),
                        })],
                    ),
                    stmt::return_expr(Expression::ident("result")),
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

    let output = emit(&p);
    let expected = "\
#ifndef GUARD_H
#define GUARD_H
#include <cstdint>
#include <vector>

int32_t compute(int32_t n) {
    int32_t result = 0;
    for (int32_t i = 0; i < n; i++) {
        result += i;
    }
    return result;
}
";
    assert_eq!(output, expected);
}

#[test]
fn helpers_template_with_type_helpers() {
    let p = program(vec![Declaration::Template(Box::new(Template {
        parameters: vec![TemplateParameter::Type {
            name: "T".into(),
            default: None,
        }],
        declaration: Box::new(Declaration::Function(Function {
            name: "make_vector".into(),
            return_type: Type::vector(Type::custom("T")),
            parameters: vec![Parameter {
                name: "value".into(),
                param_type: Type::const_ref(Type::custom("T")),
                default_value: None,
            }],
            body: Some(Block {
                statements: vec![
                    stmt::declaration(Type::vector(Type::custom("T")), "v", None),
                    stmt::expr_stmt(Expression::method_call(
                        Expression::ident("v"),
                        "push_back",
                        vec![Expression::ident("value")],
                    )),
                    stmt::return_expr(Expression::ident("v")),
                ],
            }),
            is_const: false,
            is_inline: false,
            is_static: false,
            is_virtual: false,
            is_pure_virtual: false,
            is_override: false,
            is_noexcept: false,
        })),
    }))]);

    let output = emit(&p);
    let expected = "\
template <typename T>
std::vector<T> make_vector(const T& value) {
    std::vector<T> v;
    v.push_back(value);
    return v;
}
";
    assert_eq!(output, expected);
}

#[test]
fn helpers_type_helpers_various() {
    let p = program(vec![
        Declaration::Variable(LocalVariable {
            name: "names".into(),
            var_type: Type::vector(Type::String),
            initializer: None,
            is_const: false,
            is_static: false,
            is_thread_local: false,
        }),
        Declaration::Variable(LocalVariable {
            name: "scores".into(),
            var_type: Type::map(Type::String, Type::Int32),
            initializer: None,
            is_const: false,
            is_static: false,
            is_thread_local: false,
        }),
        Declaration::Variable(LocalVariable {
            name: "maybe_name".into(),
            var_type: Type::optional(Type::String),
            initializer: None,
            is_const: false,
            is_static: false,
            is_thread_local: false,
        }),
        Declaration::Variable(LocalVariable {
            name: "ptr".into(),
            var_type: Type::unique_ptr(Type::custom("Widget")),
            initializer: Some(Expression::new_expr("Widget", vec![])),
            is_const: false,
            is_static: false,
            is_thread_local: false,
        }),
        Declaration::Variable(LocalVariable {
            name: "shared".into(),
            var_type: Type::shared_ptr(Type::custom("Widget")),
            initializer: None,
            is_const: false,
            is_static: false,
            is_thread_local: false,
        }),
    ]);

    let output = emit(&p);
    let expected = "\
std::vector<std::string> names;

std::map<std::string, int32_t> scores;

std::optional<std::string> maybe_name;

std::unique_ptr<Widget> ptr = new Widget();

std::shared_ptr<Widget> shared;
";
    assert_eq!(output, expected);
}
