use codeforge_python::*;

fn module(body: Vec<Statement>) -> Module {
    Module {
        imports: vec![],
        body,
    }
}

fn func(name: &str, body: Vec<Statement>) -> FunctionDef {
    FunctionDef {
        name: name.into(),
        decorators: vec![],
        parameters: vec![],
        vararg: None,
        kw_only_params: vec![],
        kwarg: None,
        return_annotation: None,
        body,
        docstring: None,
        is_async: false,
    }
}

fn func_stmt(name: &str, body: Vec<Statement>) -> Statement {
    Statement::FunctionDef(Box::new(func(name, body)))
}

#[test]
fn simple_function() {
    let m = module(vec![func_stmt(
        "add",
        vec![Statement::Return(Some(Expression::BinaryOp {
            left: Box::new(Expression::Identifier("a".into())),
            op: BinaryOperator::Add,
            right: Box::new(Expression::Identifier("b".into())),
        }))],
    )]);

    let output = emit(&m);
    let expected = "\
def add():
    return a + b
";
    assert_eq!(output, expected);
}

#[test]
fn function_with_params_annotations_defaults() {
    let m = module(vec![Statement::FunctionDef(Box::new(FunctionDef {
        name: "greet".into(),
        decorators: vec![],
        parameters: vec![
            Parameter {
                name: "name".into(),
                annotation: Some(Type::Str),
                default: None,
            },
            Parameter {
                name: "greeting".into(),
                annotation: Some(Type::Str),
                default: Some(Expression::Literal(Literal::String("Hello".into()))),
            },
        ],
        vararg: None,
        kw_only_params: vec![],
        kwarg: None,
        return_annotation: Some(Type::Str),
        body: vec![Statement::Return(Some(Expression::BinaryOp {
            left: Box::new(Expression::BinaryOp {
                left: Box::new(Expression::Identifier("greeting".into())),
                op: BinaryOperator::Add,
                right: Box::new(Expression::Literal(Literal::String(" ".into()))),
            }),
            op: BinaryOperator::Add,
            right: Box::new(Expression::Identifier("name".into())),
        }))],
        docstring: None,
        is_async: false,
    }))]);

    let output = emit(&m);
    let expected = "\
def greet(name: str, greeting: str = 'Hello') -> str:
    return greeting + ' ' + name
";
    assert_eq!(output, expected);
}

#[test]
fn function_with_vararg_and_kwarg() {
    let m = module(vec![Statement::FunctionDef(Box::new(FunctionDef {
        name: "foo".into(),
        decorators: vec![],
        parameters: vec![Parameter {
            name: "x".into(),
            annotation: None,
            default: None,
        }],
        vararg: Some(Parameter {
            name: "args".into(),
            annotation: None,
            default: None,
        }),
        kw_only_params: vec![Parameter {
            name: "verbose".into(),
            annotation: None,
            default: Some(Expression::Literal(Literal::Boolean(false))),
        }],
        kwarg: Some(Parameter {
            name: "kwargs".into(),
            annotation: None,
            default: None,
        }),
        return_annotation: None,
        body: vec![Statement::Pass],
        docstring: None,
        is_async: false,
    }))]);

    let output = emit(&m);
    let expected = "\
def foo(x, *args, verbose=False, **kwargs):
    pass
";
    assert_eq!(output, expected);
}

#[test]
fn function_with_kw_only_no_vararg() {
    let m = module(vec![Statement::FunctionDef(Box::new(FunctionDef {
        name: "bar".into(),
        decorators: vec![],
        parameters: vec![],
        vararg: None,
        kw_only_params: vec![Parameter {
            name: "key".into(),
            annotation: None,
            default: None,
        }],
        kwarg: None,
        return_annotation: None,
        body: vec![Statement::Pass],
        docstring: None,
        is_async: false,
    }))]);

    let output = emit(&m);
    let expected = "\
def bar(*, key):
    pass
";
    assert_eq!(output, expected);
}

#[test]
fn empty_class() {
    let m = module(vec![Statement::ClassDef(ClassDef {
        name: "Empty".into(),
        decorators: vec![],
        bases: vec![],
        keywords: vec![],
        body: vec![],
        docstring: None,
    })]);

    let output = emit(&m);
    let expected = "\
class Empty:
    pass
";
    assert_eq!(output, expected);
}

#[test]
fn class_with_bases() {
    let m = module(vec![Statement::ClassDef(ClassDef {
        name: "Dog".into(),
        decorators: vec![],
        bases: vec![Expression::Identifier("Animal".into())],
        keywords: vec![],
        body: vec![],
        docstring: None,
    })]);

    let output = emit(&m);
    let expected = "\
class Dog(Animal):
    pass
";
    assert_eq!(output, expected);
}

#[test]
fn class_with_keyword_arg() {
    let m = module(vec![Statement::ClassDef(ClassDef {
        name: "MyMeta".into(),
        decorators: vec![],
        bases: vec![],
        keywords: vec![Keyword {
            name: "metaclass".into(),
            value: Expression::Identifier("ABCMeta".into()),
        }],
        body: vec![],
        docstring: None,
    })]);

    let output = emit(&m);
    let expected = "\
class MyMeta(metaclass=ABCMeta):
    pass
";
    assert_eq!(output, expected);
}

#[test]
fn class_with_methods() {
    let m = module(vec![Statement::ClassDef(ClassDef {
        name: "Counter".into(),
        decorators: vec![],
        bases: vec![],
        keywords: vec![],
        body: vec![
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
                body: vec![Statement::Assign(Assign {
                    target: Expression::Attribute {
                        object: Box::new(Expression::Identifier("self".into())),
                        name: "count".into(),
                    },
                    value: Expression::Literal(Literal::Integer(0)),
                })],
                docstring: None,
                is_async: false,
            })),
            Statement::FunctionDef(Box::new(FunctionDef {
                name: "increment".into(),
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
                body: vec![Statement::AugAssign(AugAssign {
                    target: Expression::Attribute {
                        object: Box::new(Expression::Identifier("self".into())),
                        name: "count".into(),
                    },
                    op: BinaryOperator::Add,
                    value: Expression::Literal(Literal::Integer(1)),
                })],
                docstring: None,
                is_async: false,
            })),
        ],
        docstring: None,
    })]);

    let output = emit(&m);
    let expected = "\
class Counter:
    def __init__(self):
        self.count = 0

    def increment(self):
        self.count += 1
";
    assert_eq!(output, expected);
}

#[test]
fn imports_and_multiple_defs() {
    let m = Module {
        imports: vec![
            Import::Simple(SimpleImport {
                names: vec!["os".into(), "sys".into()],
            }),
            Import::From(FromImport {
                module: "os.path".into(),
                names: vec![
                    ImportName {
                        name: "join".into(),
                        alias: None,
                    },
                    ImportName {
                        name: "exists".into(),
                        alias: Some("path_exists".into()),
                    },
                ],
            }),
        ],
        body: vec![
            Statement::FunctionDef(Box::new(func(
                "foo",
                vec![Statement::Return(Some(Expression::Literal(
                    Literal::Integer(1),
                )))],
            ))),
            Statement::FunctionDef(Box::new(func(
                "bar",
                vec![Statement::Return(Some(Expression::Literal(
                    Literal::Integer(2),
                )))],
            ))),
        ],
    };

    let output = emit(&m);
    let expected = "\
import os, sys
from os.path import join, exists as path_exists


def foo():
    return 1


def bar():
    return 2
";
    assert_eq!(output, expected);
}

#[test]
fn if_elif_else() {
    let m = module(vec![Statement::FunctionDef(Box::new(FunctionDef {
        name: "classify".into(),
        decorators: vec![],
        parameters: vec![Parameter {
            name: "x".into(),
            annotation: None,
            default: None,
        }],
        vararg: None,
        kw_only_params: vec![],
        kwarg: None,
        return_annotation: None,
        body: vec![Statement::If(Box::new(IfStatement {
            condition: Expression::BinaryOp {
                left: Box::new(Expression::Identifier("x".into())),
                op: BinaryOperator::Gt,
                right: Box::new(Expression::Literal(Literal::Integer(0))),
            },
            body: vec![Statement::Return(Some(Expression::Literal(
                Literal::String("positive".into()),
            )))],
            elif_clauses: vec![ElifClause {
                condition: Expression::BinaryOp {
                    left: Box::new(Expression::Identifier("x".into())),
                    op: BinaryOperator::Eq,
                    right: Box::new(Expression::Literal(Literal::Integer(0))),
                },
                body: vec![Statement::Return(Some(Expression::Literal(
                    Literal::String("zero".into()),
                )))],
            }],
            else_body: Some(vec![Statement::Return(Some(Expression::Literal(
                Literal::String("negative".into()),
            )))]),
        }))],
        docstring: None,
        is_async: false,
    }))]);

    let output = emit(&m);
    let expected = "\
def classify(x):
    if x > 0:
        return 'positive'
    elif x == 0:
        return 'zero'
    else:
        return 'negative'
";
    assert_eq!(output, expected);
}

#[test]
fn for_loop() {
    let m = module(vec![Statement::FunctionDef(Box::new(FunctionDef {
        name: "sum_list".into(),
        decorators: vec![],
        parameters: vec![Parameter {
            name: "items".into(),
            annotation: None,
            default: None,
        }],
        vararg: None,
        kw_only_params: vec![],
        kwarg: None,
        return_annotation: None,
        body: vec![
            Statement::Assign(Assign {
                target: Expression::Identifier("total".into()),
                value: Expression::Literal(Literal::Integer(0)),
            }),
            Statement::For(Box::new(ForStatement {
                target: Expression::Identifier("item".into()),
                iter: Expression::Identifier("items".into()),
                body: vec![Statement::AugAssign(AugAssign {
                    target: Expression::Identifier("total".into()),
                    op: BinaryOperator::Add,
                    value: Expression::Identifier("item".into()),
                })],
                else_body: None,
            })),
            Statement::Return(Some(Expression::Identifier("total".into()))),
        ],
        docstring: None,
        is_async: false,
    }))]);

    let output = emit(&m);
    let expected = "\
def sum_list(items):
    total = 0
    for item in items:
        total += item
    return total
";
    assert_eq!(output, expected);
}

#[test]
fn while_loop() {
    let m = module(vec![Statement::FunctionDef(Box::new(FunctionDef {
        name: "countdown".into(),
        decorators: vec![],
        parameters: vec![Parameter {
            name: "n".into(),
            annotation: None,
            default: None,
        }],
        vararg: None,
        kw_only_params: vec![],
        kwarg: None,
        return_annotation: None,
        body: vec![Statement::While(Box::new(WhileStatement {
            condition: Expression::BinaryOp {
                left: Box::new(Expression::Identifier("n".into())),
                op: BinaryOperator::Gt,
                right: Box::new(Expression::Literal(Literal::Integer(0))),
            },
            body: vec![
                Statement::Expression(Expression::Call {
                    func: Box::new(Expression::Identifier("print".into())),
                    arguments: vec![Expression::Identifier("n".into())],
                    keywords: vec![],
                }),
                Statement::AugAssign(AugAssign {
                    target: Expression::Identifier("n".into()),
                    op: BinaryOperator::Sub,
                    value: Expression::Literal(Literal::Integer(1)),
                }),
            ],
        }))],
        docstring: None,
        is_async: false,
    }))]);

    let output = emit(&m);
    let expected = "\
def countdown(n):
    while n > 0:
        print(n)
        n -= 1
";
    assert_eq!(output, expected);
}

#[test]
fn decorated_function() {
    let m = module(vec![Statement::FunctionDef(Box::new(FunctionDef {
        name: "cached".into(),
        decorators: vec![
            Expression::Identifier("staticmethod".into()),
            Expression::Call {
                func: Box::new(Expression::Attribute {
                    object: Box::new(Expression::Identifier("app".into())),
                    name: "route".into(),
                }),
                arguments: vec![Expression::Literal(Literal::String("/".into()))],
                keywords: vec![],
            },
        ],
        parameters: vec![],
        vararg: None,
        kw_only_params: vec![],
        kwarg: None,
        return_annotation: None,
        body: vec![Statement::Pass],
        docstring: None,
        is_async: false,
    }))]);

    let output = emit(&m);
    let expected = "\
@staticmethod
@app.route('/')
def cached():
    pass
";
    assert_eq!(output, expected);
}

#[test]
fn decorated_class() {
    let m = module(vec![Statement::ClassDef(ClassDef {
        name: "Model".into(),
        decorators: vec![Expression::Call {
            func: Box::new(Expression::Identifier("dataclass".into())),
            arguments: vec![],
            keywords: vec![Keyword {
                name: "frozen".into(),
                value: Expression::Literal(Literal::Boolean(true)),
            }],
        }],
        bases: vec![],
        keywords: vec![],
        body: vec![],
        docstring: None,
    })]);

    let output = emit(&m);
    let expected = "\
@dataclass(frozen=True)
class Model:
    pass
";
    assert_eq!(output, expected);
}

#[test]
fn docstring_in_function() {
    let m = module(vec![Statement::FunctionDef(Box::new(FunctionDef {
        name: "helper".into(),
        decorators: vec![],
        parameters: vec![],
        vararg: None,
        kw_only_params: vec![],
        kwarg: None,
        return_annotation: None,
        body: vec![Statement::Return(Some(Expression::Literal(
            Literal::Integer(42),
        )))],
        docstring: Some("A helper function.".into()),
        is_async: false,
    }))]);

    let output = emit(&m);
    let expected = "\
def helper():
    \"\"\"A helper function.\"\"\"
    return 42
";
    assert_eq!(output, expected);
}

#[test]
fn docstring_only_acts_as_body() {
    let m = module(vec![Statement::FunctionDef(Box::new(FunctionDef {
        name: "placeholder".into(),
        decorators: vec![],
        parameters: vec![],
        vararg: None,
        kw_only_params: vec![],
        kwarg: None,
        return_annotation: None,
        body: vec![],
        docstring: Some("Not yet implemented.".into()),
        is_async: false,
    }))]);

    let output = emit(&m);
    let expected = "\
def placeholder():
    \"\"\"Not yet implemented.\"\"\"
";
    assert_eq!(output, expected);
}

#[test]
fn async_function() {
    let m = module(vec![Statement::FunctionDef(Box::new(FunctionDef {
        name: "fetch".into(),
        decorators: vec![],
        parameters: vec![Parameter {
            name: "url".into(),
            annotation: Some(Type::Str),
            default: None,
        }],
        vararg: None,
        kw_only_params: vec![],
        kwarg: None,
        return_annotation: Some(Type::Any),
        body: vec![Statement::Return(Some(Expression::Call {
            func: Box::new(Expression::Identifier("await_response".into())),
            arguments: vec![Expression::Identifier("url".into())],
            keywords: vec![],
        }))],
        docstring: None,
        is_async: true,
    }))]);

    let output = emit(&m);
    let expected = "\
async def fetch(url: str) -> Any:
    return await_response(url)
";
    assert_eq!(output, expected);
}

#[test]
fn expressions_and_literals() {
    let cases: Vec<(Expression, &str)> = vec![
        (Expression::Literal(Literal::Integer(42)), "42"),
        (
            Expression::Literal(Literal::Float(F64Wrapper(2.72))),
            "2.72",
        ),
        (Expression::Literal(Literal::Boolean(true)), "True"),
        (Expression::Literal(Literal::Boolean(false)), "False"),
        (
            Expression::Literal(Literal::String("hello".into())),
            "'hello'",
        ),
        (
            Expression::Literal(Literal::String("it's".into())),
            "'it\\'s'",
        ),
        (Expression::Literal(Literal::None_), "None"),
    ];

    for (expr, expected) in cases {
        let m = module(vec![Statement::Assign(Assign {
            target: Expression::Identifier("x".into()),
            value: expr,
        })]);
        let output = emit(&m);
        assert_eq!(output, format!("x = {}\n", expected));
    }
}

#[test]
fn python_specific_operators() {
    let cases: Vec<(BinaryOperator, &str)> = vec![
        (BinaryOperator::FloorDiv, "//"),
        (BinaryOperator::Mod, "%"),
        (BinaryOperator::Pow, "**"),
        (BinaryOperator::In, "in"),
        (BinaryOperator::NotIn, "not in"),
        (BinaryOperator::Is, "is"),
        (BinaryOperator::IsNot, "is not"),
        (BinaryOperator::And, "and"),
        (BinaryOperator::Or, "or"),
    ];

    for (op, expected) in cases {
        let expr = Expression::BinaryOp {
            left: Box::new(Expression::Identifier("x".into())),
            op,
            right: Box::new(Expression::Identifier("y".into())),
        };
        let m = module(vec![Statement::Assign(Assign {
            target: Expression::Identifier("r".into()),
            value: expr,
        })]);
        let output = emit(&m);
        assert_eq!(output, format!("r = x {} y\n", expected));
    }
}

#[test]
fn not_operator() {
    let expr = Expression::UnaryOp {
        op: UnaryOperator::Not,
        operand: Box::new(Expression::Identifier("flag".into())),
    };
    let m = module(vec![Statement::Assign(Assign {
        target: Expression::Identifier("r".into()),
        value: expr,
    })]);
    let output = emit(&m);
    assert_eq!(output, "r = not flag\n");
}

#[test]
fn tuple_variants() {
    let empty_tuple = Expression::Tuple(vec![]);
    let single_tuple = Expression::Tuple(vec![Expression::Literal(Literal::Integer(1))]);
    let multi_tuple = Expression::Tuple(vec![
        Expression::Literal(Literal::Integer(1)),
        Expression::Literal(Literal::Integer(2)),
        Expression::Literal(Literal::Integer(3)),
    ]);

    let m = module(vec![
        Statement::Assign(Assign {
            target: Expression::Identifier("a".into()),
            value: empty_tuple,
        }),
        Statement::Assign(Assign {
            target: Expression::Identifier("b".into()),
            value: single_tuple,
        }),
        Statement::Assign(Assign {
            target: Expression::Identifier("c".into()),
            value: multi_tuple,
        }),
    ]);

    let output = emit(&m);
    let expected = "\
a = ()
b = (1,)
c = (1, 2, 3)
";
    assert_eq!(output, expected);
}

#[test]
fn list_dict_set() {
    let m = module(vec![
        Statement::Assign(Assign {
            target: Expression::Identifier("xs".into()),
            value: Expression::List(vec![
                Expression::Literal(Literal::Integer(1)),
                Expression::Literal(Literal::Integer(2)),
            ]),
        }),
        Statement::Assign(Assign {
            target: Expression::Identifier("d".into()),
            value: Expression::Dict(vec![(
                Expression::Literal(Literal::String("key".into())),
                Expression::Literal(Literal::Integer(1)),
            )]),
        }),
        Statement::Assign(Assign {
            target: Expression::Identifier("s".into()),
            value: Expression::Set(vec![
                Expression::Literal(Literal::Integer(1)),
                Expression::Literal(Literal::Integer(2)),
            ]),
        }),
    ]);

    let output = emit(&m);
    let expected = "\
xs = [1, 2]
d = {'key': 1}
s = {1, 2}
";
    assert_eq!(output, expected);
}

#[test]
fn ternary_expression() {
    let m = module(vec![Statement::Assign(Assign {
        target: Expression::Identifier("x".into()),
        value: Expression::Ternary {
            condition: Box::new(Expression::BinaryOp {
                left: Box::new(Expression::Identifier("a".into())),
                op: BinaryOperator::Gt,
                right: Box::new(Expression::Identifier("b".into())),
            }),
            then_expr: Box::new(Expression::Identifier("a".into())),
            else_expr: Box::new(Expression::Identifier("b".into())),
        },
    })]);

    let output = emit(&m);
    let expected = "x = a if a > b else b\n";
    assert_eq!(output, expected);
}

#[test]
fn lambda_expression() {
    let m = module(vec![Statement::Assign(Assign {
        target: Expression::Identifier("double".into()),
        value: Expression::Lambda {
            parameters: vec![Parameter {
                name: "x".into(),
                annotation: None,
                default: None,
            }],
            body: Box::new(Expression::BinaryOp {
                left: Box::new(Expression::Identifier("x".into())),
                op: BinaryOperator::Mul,
                right: Box::new(Expression::Literal(Literal::Integer(2))),
            }),
        },
    })]);

    let output = emit(&m);
    let expected = "double = lambda x: x * 2\n";
    assert_eq!(output, expected);
}

#[test]
fn attribute_and_subscript() {
    let m = module(vec![
        Statement::Assign(Assign {
            target: Expression::Identifier("x".into()),
            value: Expression::Attribute {
                object: Box::new(Expression::Identifier("obj".into())),
                name: "field".into(),
            },
        }),
        Statement::Assign(Assign {
            target: Expression::Identifier("y".into()),
            value: Expression::Subscript {
                value: Box::new(Expression::Identifier("lst".into())),
                index: Box::new(Expression::Literal(Literal::Integer(0))),
            },
        }),
    ]);

    let output = emit(&m);
    let expected = "\
x = obj.field
y = lst[0]
";
    assert_eq!(output, expected);
}

#[test]
fn function_call_with_keywords() {
    let m = module(vec![Statement::Expression(Expression::Call {
        func: Box::new(Expression::Identifier("foo".into())),
        arguments: vec![Expression::Literal(Literal::Integer(1))],
        keywords: vec![
            Keyword {
                name: "bar".into(),
                value: Expression::Literal(Literal::Integer(2)),
            },
            Keyword {
                name: "baz".into(),
                value: Expression::Literal(Literal::String("hello".into())),
            },
        ],
    })]);

    let output = emit(&m);
    let expected = "foo(1, bar=2, baz='hello')\n";
    assert_eq!(output, expected);
}

#[test]
fn break_continue_pass() {
    let m = module(vec![Statement::FunctionDef(Box::new(FunctionDef {
        name: "loop".into(),
        decorators: vec![],
        parameters: vec![],
        vararg: None,
        kw_only_params: vec![],
        kwarg: None,
        return_annotation: None,
        body: vec![Statement::While(Box::new(WhileStatement {
            condition: Expression::Literal(Literal::Boolean(true)),
            body: vec![Statement::If(Box::new(IfStatement {
                condition: Expression::UnaryOp {
                    op: UnaryOperator::Not,
                    operand: Box::new(Expression::Identifier("ready".into())),
                },
                body: vec![Statement::Continue],
                elif_clauses: vec![],
                else_body: Some(vec![Statement::Break]),
            }))],
        }))],
        docstring: None,
        is_async: false,
    }))]);

    let output = emit(&m);
    let expected = "\
def loop():
    while True:
        if not ready:
            continue
        else:
            break
";
    assert_eq!(output, expected);
}

#[test]
fn comments_and_raw() {
    let m = module(vec![
        Statement::Comment("This is a comment".into()),
        Statement::Assign(Assign {
            target: Expression::Identifier("x".into()),
            value: Expression::Literal(Literal::Integer(1)),
        }),
        Statement::Raw("y, z = 2, 3".into()),
    ]);

    let output = emit(&m);
    let expected = "\
# This is a comment
x = 1
y, z = 2, 3
";
    assert_eq!(output, expected);
}

#[test]
fn star_expression() {
    let m = module(vec![Statement::Assign(Assign {
        target: Expression::Tuple(vec![
            Expression::Identifier("first".into()),
            Expression::Starred(Box::new(Expression::Identifier("rest".into()))),
        ]),
        value: Expression::Identifier("items".into()),
    })]);

    let output = emit(&m);
    let expected = "(first, *rest) = items\n";
    assert_eq!(output, expected);
}

#[test]
fn type_annotations() {
    let types: Vec<(Type, &str)> = vec![
        (Type::Int, "int"),
        (Type::Str, "str"),
        (Type::Float, "float"),
        (Type::Bool, "bool"),
        (Type::None_, "None"),
        (Type::Any, "Any"),
        (Type::Self_, "Self"),
        (Type::Custom("MyClass".into()), "MyClass"),
        (Type::List(Box::new(Type::Int)), "list[int]"),
        (
            Type::Dict(Box::new(Type::Str), Box::new(Type::Any)),
            "dict[str, Any]",
        ),
        (Type::Optional(Box::new(Type::Str)), "Optional[str]"),
        (Type::Union(vec![Type::Int, Type::Str]), "Union[int, str]"),
        (Type::Tuple(vec![Type::Int, Type::Str]), "tuple[int, str]"),
        (Type::Set(Box::new(Type::Int)), "set[int]"),
        (
            Type::Generic("Callable".into(), vec![Type::Int, Type::Str]),
            "Callable[int, str]",
        ),
        (
            Type::Callable(vec![Type::Int, Type::Str], Box::new(Type::Bool)),
            "Callable[[int, str], bool]",
        ),
        (Type::Raw("ForwardRef".into()), "ForwardRef"),
    ];

    for (ty, expected) in types {
        assert_eq!(ty.to_python(), expected, "Type::to_python() mismatch");
    }
}

#[test]
fn return_without_value() {
    let m = module(vec![Statement::FunctionDef(Box::new(FunctionDef {
        name: "noop".into(),
        decorators: vec![],
        parameters: vec![],
        vararg: None,
        kw_only_params: vec![],
        kwarg: None,
        return_annotation: None,
        body: vec![Statement::Return(None)],
        docstring: None,
        is_async: false,
    }))]);

    let output = emit(&m);
    let expected = "\
def noop():
    return
";
    assert_eq!(output, expected);
}

#[test]
fn module_with_imports_no_body() {
    let m = Module {
        imports: vec![Import::Simple(SimpleImport {
            names: vec!["os".into()],
        })],
        body: vec![],
    };

    let output = emit(&m);
    let expected = "import os\n";
    assert_eq!(output, expected);
}

#[test]
fn consecutive_non_defs_no_blanks() {
    let m = module(vec![
        Statement::Assign(Assign {
            target: Expression::Identifier("x".into()),
            value: Expression::Literal(Literal::Integer(1)),
        }),
        Statement::Assign(Assign {
            target: Expression::Identifier("y".into()),
            value: Expression::Literal(Literal::Integer(2)),
        }),
        Statement::Assign(Assign {
            target: Expression::Identifier("z".into()),
            value: Expression::Literal(Literal::Integer(3)),
        }),
    ]);

    let output = emit(&m);
    let expected = "\
x = 1
y = 2
z = 3
";
    assert_eq!(output, expected);
}

#[test]
fn mixed_defs_and_exprs_blanks() {
    let m = module(vec![
        Statement::Assign(Assign {
            target: Expression::Identifier("x".into()),
            value: Expression::Literal(Literal::Integer(1)),
        }),
        Statement::FunctionDef(Box::new(func(
            "foo",
            vec![Statement::Return(Some(Expression::Literal(
                Literal::Integer(42),
            )))],
        ))),
        Statement::Assign(Assign {
            target: Expression::Identifier("y".into()),
            value: Expression::Literal(Literal::Integer(2)),
        }),
    ]);

    let output = emit(&m);
    let expected = "\
x = 1


def foo():
    return 42


y = 2
";
    assert_eq!(output, expected);
}

#[test]
fn for_with_else() {
    let m = module(vec![Statement::For(Box::new(ForStatement {
        target: Expression::Identifier("item".into()),
        iter: Expression::Identifier("items".into()),
        body: vec![Statement::If(Box::new(IfStatement {
            condition: Expression::BinaryOp {
                left: Box::new(Expression::Identifier("item".into())),
                op: BinaryOperator::Eq,
                right: Box::new(Expression::Identifier("target".into())),
            },
            body: vec![Statement::Break],
            elif_clauses: vec![],
            else_body: None,
        }))],
        else_body: Some(vec![Statement::Expression(Expression::Call {
            func: Box::new(Expression::Identifier("not_found".into())),
            arguments: vec![],
            keywords: vec![],
        })]),
    }))]);

    let output = emit(&m);
    let expected = "\
for item in items:
    if item == target:
        break
else:
    not_found()
";
    assert_eq!(output, expected);
}

#[test]
fn from_import_star() {
    let m = Module {
        imports: vec![Import::From(FromImport {
            module: "module".into(),
            names: vec![ImportName {
                name: "*".into(),
                alias: None,
            }],
        })],
        body: vec![],
    };

    let output = emit(&m);
    let expected = "from module import *\n";
    assert_eq!(output, expected);
}

#[test]
fn empty_set_emits_set_call() {
    let m = module(vec![Statement::Assign(Assign {
        target: Expression::Identifier("s".into()),
        value: Expression::Set(vec![]),
    })]);

    let output = emit(&m);
    assert_eq!(output, "s = set()\n");
}

#[test]
fn string_escapes_control_chars() {
    let m = module(vec![Statement::Assign(Assign {
        target: Expression::Identifier("s".into()),
        value: Expression::Literal(Literal::String("line1\nline2\ttab\rreturn".into())),
    })]);

    let output = emit(&m);
    assert_eq!(output, "s = 'line1\\nline2\\ttab\\rreturn'\n");
}

#[test]
fn float_special_values() {
    let cases: Vec<(Literal, &str)> = vec![
        (Literal::Float(F64Wrapper(f64::INFINITY)), "float('inf')"),
        (
            Literal::Float(F64Wrapper(f64::NEG_INFINITY)),
            "float('-inf')",
        ),
        (Literal::Float(F64Wrapper(f64::NAN)), "float('nan')"),
    ];

    for (lit, expected) in cases {
        let m = module(vec![Statement::Assign(Assign {
            target: Expression::Identifier("x".into()),
            value: Expression::Literal(lit),
        })]);
        let output = emit(&m);
        assert_eq!(output, format!("x = {}\n", expected));
    }
}

#[test]
fn not_with_and_gets_parenthesized() {
    let m = module(vec![Statement::Assign(Assign {
        target: Expression::Identifier("r".into()),
        value: Expression::UnaryOp {
            op: UnaryOperator::Not,
            operand: Box::new(Expression::BinaryOp {
                left: Box::new(Expression::Identifier("a".into())),
                op: BinaryOperator::And,
                right: Box::new(Expression::Identifier("b".into())),
            }),
        },
    })]);

    let output = emit(&m);
    assert_eq!(output, "r = not (a and b)\n");
}

#[test]
fn comment_with_newlines() {
    let m = module(vec![Statement::Comment("line 1\nline 2\nline 3".into())]);

    let output = emit(&m);
    let expected = "\
# line 1
# line 2
# line 3
";
    assert_eq!(output, expected);
}

#[test]
fn docstring_escapes_triple_quotes() {
    let m = module(vec![Statement::FunctionDef(Box::new(FunctionDef {
        name: "f".into(),
        decorators: vec![],
        parameters: vec![],
        vararg: None,
        kw_only_params: vec![],
        kwarg: None,
        return_annotation: None,
        body: vec![Statement::Pass],
        docstring: Some(r#"Has """triple""" quotes"#.into()),
        is_async: false,
    }))]);

    let output = emit(&m);
    let expected = "\
def f():
    \"\"\"Has \\\"\\\"\\\"triple\\\"\\\"\\\" quotes\"\"\"
    pass
";
    assert_eq!(output, expected);
}

#[test]
fn ann_assign_annotation_only() {
    let m = module(vec![Statement::AnnAssign(AnnAssign {
        target: Expression::Identifier("name".into()),
        annotation: Type::Str,
        value: None,
    })]);

    let output = emit(&m);
    assert_eq!(output, "name: str\n");
}

#[test]
fn ann_assign_with_value() {
    let m = module(vec![Statement::AnnAssign(AnnAssign {
        target: Expression::Identifier("age".into()),
        annotation: Type::Int,
        value: Some(Expression::Literal(Literal::Integer(0))),
    })]);

    let output = emit(&m);
    assert_eq!(output, "age: int = 0\n");
}

#[test]
fn ann_assign_dataclass_fields() {
    let m = module(vec![Statement::ClassDef(ClassDef {
        name: "Person".into(),
        decorators: vec![Expression::Call {
            func: Box::new(Expression::Identifier("dataclass".into())),
            arguments: vec![],
            keywords: vec![],
        }],
        bases: vec![],
        keywords: vec![],
        body: vec![
            Statement::AnnAssign(AnnAssign {
                target: Expression::Identifier("name".into()),
                annotation: Type::Str,
                value: None,
            }),
            Statement::AnnAssign(AnnAssign {
                target: Expression::Identifier("age".into()),
                annotation: Type::Int,
                value: Some(Expression::Literal(Literal::Integer(0))),
            }),
        ],
        docstring: None,
    })]);

    let output = emit(&m);
    let expected = "\
@dataclass()
class Person:
    name: str
    age: int = 0
";
    assert_eq!(output, expected);
}

#[test]
fn raise_with_expression() {
    let m = module(vec![Statement::Raise(Raise {
        exc: Some(Expression::Call {
            func: Box::new(Expression::Identifier("ValueError".into())),
            arguments: vec![Expression::Literal(Literal::String("bad value".into()))],
            keywords: vec![],
        }),
        cause: None,
    })]);

    let output = emit(&m);
    assert_eq!(output, "raise ValueError('bad value')\n");
}

#[test]
fn raise_without_expression() {
    let m = module(vec![Statement::FunctionDef(Box::new(FunctionDef {
        name: "handle".into(),
        decorators: vec![],
        parameters: vec![],
        vararg: None,
        kw_only_params: vec![],
        kwarg: None,
        return_annotation: None,
        body: vec![Statement::Raise(Raise {
            exc: None,
            cause: None,
        })],
        docstring: None,
        is_async: false,
    }))]);

    let output = emit(&m);
    let expected = "\
def handle():
    raise
";
    assert_eq!(output, expected);
}

#[test]
fn raise_from_cause() {
    let m = module(vec![Statement::Raise(Raise {
        exc: Some(Expression::Call {
            func: Box::new(Expression::Identifier("ValueError".into())),
            arguments: vec![Expression::Literal(Literal::String("inner".into()))],
            keywords: vec![],
        }),
        cause: Some(Expression::Identifier("e".into())),
    })]);

    let output = emit(&m);
    assert_eq!(output, "raise ValueError('inner') from e\n");
}

#[test]
fn literal_bytes_simple() {
    let m = module(vec![Statement::Assign(Assign {
        target: Expression::Identifier("b".into()),
        value: Expression::Literal(Literal::Bytes(vec![0xff, 0x00])),
    })]);

    let output = emit(&m);
    assert_eq!(output, "b = b'\\xff\\x00'\n");
}

#[test]
fn literal_bytes_printable_ascii() {
    let m = module(vec![Statement::Assign(Assign {
        target: Expression::Identifier("b".into()),
        value: Expression::Literal(Literal::Bytes(b"hello".to_vec())),
    })]);

    let output = emit(&m);
    assert_eq!(output, "b = b'hello'\n");
}

#[test]
fn literal_bytes_with_escapes() {
    let m = module(vec![Statement::Assign(Assign {
        target: Expression::Identifier("b".into()),
        value: Expression::Literal(Literal::Bytes(vec![b'h', b'\n', b'\'', b'\\', 0x80])),
    })]);

    let output = emit(&m);
    assert_eq!(output, "b = b'h\\n\\'\\\\\\x80'\n");
}
