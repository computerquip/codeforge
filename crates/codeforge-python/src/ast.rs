#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Module {
    pub imports: Vec<Import>,
    pub body: Vec<Statement>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Import {
    Simple(SimpleImport),
    From(FromImport),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SimpleImport {
    pub names: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct FromImport {
    pub module: String,
    pub names: Vec<ImportName>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ImportName {
    pub name: String,
    pub alias: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Statement {
    FunctionDef(Box<FunctionDef>),
    ClassDef(ClassDef),
    Return(Option<Expression>),
    Assign(Assign),
    AnnAssign(AnnAssign),
    AugAssign(AugAssign),
    Raise(Raise),
    If(Box<IfStatement>),
    While(Box<WhileStatement>),
    For(Box<ForStatement>),
    Expression(Expression),
    Pass,
    Break,
    Continue,
    Comment(String),
    Raw(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct FunctionDef {
    pub name: String,
    pub decorators: Vec<Expression>,
    pub parameters: Vec<Parameter>,
    pub vararg: Option<Parameter>,
    pub kw_only_params: Vec<Parameter>,
    pub kwarg: Option<Parameter>,
    pub return_annotation: Option<Type>,
    pub body: Vec<Statement>,
    pub docstring: Option<String>,
    pub is_async: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ClassDef {
    pub name: String,
    pub decorators: Vec<Expression>,
    pub bases: Vec<Expression>,
    pub keywords: Vec<Keyword>,
    pub body: Vec<Statement>,
    pub docstring: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Keyword {
    pub name: String,
    pub value: Expression,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Parameter {
    pub name: String,
    pub annotation: Option<Type>,
    pub default: Option<Expression>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Assign {
    pub target: Expression,
    pub value: Expression,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct AugAssign {
    pub target: Expression,
    pub op: BinaryOperator,
    pub value: Expression,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct AnnAssign {
    pub target: Expression,
    pub annotation: Type,
    pub value: Option<Expression>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Raise {
    pub exc: Option<Expression>,
    pub cause: Option<Expression>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct IfStatement {
    pub condition: Expression,
    pub body: Vec<Statement>,
    pub elif_clauses: Vec<ElifClause>,
    pub else_body: Option<Vec<Statement>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ElifClause {
    pub condition: Expression,
    pub body: Vec<Statement>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct WhileStatement {
    pub condition: Expression,
    pub body: Vec<Statement>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ForStatement {
    pub target: Expression,
    pub iter: Expression,
    pub body: Vec<Statement>,
    pub else_body: Option<Vec<Statement>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Type {
    None_,
    Int,
    Float,
    Str,
    Bool,
    Bytes,
    Any,
    Custom(String),
    Generic(String, Vec<Type>),
    Optional(Box<Type>),
    Union(Vec<Type>),
    Tuple(Vec<Type>),
    List(Box<Type>),
    Dict(Box<Type>, Box<Type>),
    Set(Box<Type>),
    Callable(Vec<Type>, Box<Type>),
    Self_,
    Raw(String),
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct F64Wrapper(pub f64);

impl Eq for F64Wrapper {}

impl std::hash::Hash for F64Wrapper {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.to_bits().hash(state);
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Literal {
    Integer(i64),
    Float(F64Wrapper),
    Boolean(bool),
    String(String),
    Bytes(Vec<u8>),
    None_,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Expression {
    Literal(Literal),
    Identifier(String),
    BinaryOp {
        left: Box<Expression>,
        op: BinaryOperator,
        right: Box<Expression>,
    },
    UnaryOp {
        op: UnaryOperator,
        operand: Box<Expression>,
    },
    Call {
        func: Box<Expression>,
        arguments: Vec<Expression>,
        keywords: Vec<Keyword>,
    },
    Attribute {
        object: Box<Expression>,
        name: String,
    },
    Subscript {
        value: Box<Expression>,
        index: Box<Expression>,
    },
    Starred(Box<Expression>),
    List(Vec<Expression>),
    Tuple(Vec<Expression>),
    Dict(Vec<(Expression, Expression)>),
    Set(Vec<Expression>),
    Ternary {
        condition: Box<Expression>,
        then_expr: Box<Expression>,
        else_expr: Box<Expression>,
    },
    Lambda {
        parameters: Vec<Parameter>,
        body: Box<Expression>,
    },
    Raw(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum BinaryOperator {
    Add,
    Sub,
    Mul,
    Div,
    FloorDiv,
    Mod,
    Pow,
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
    And,
    Or,
    BitAnd,
    BitOr,
    BitXor,
    ShiftLeft,
    ShiftRight,
    In,
    NotIn,
    Is,
    IsNot,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum UnaryOperator {
    Pos,
    Neg,
    Not,
    BitNot,
}

impl Type {
    pub fn custom(s: &str) -> Self {
        Type::Custom(s.to_string())
    }

    pub fn optional(inner: Type) -> Self {
        Type::Optional(Box::new(inner))
    }

    pub fn list_of(inner: Type) -> Self {
        Type::List(Box::new(inner))
    }

    pub fn optional_custom(s: &str) -> Self {
        Self::optional(Self::custom(s))
    }

    pub fn generic_custom(name: &str, args: Vec<Type>) -> Self {
        Type::Generic(name.to_string(), args)
    }

    pub fn dict_of(key: Type, value: Type) -> Self {
        Type::Dict(Box::new(key), Box::new(value))
    }

    pub fn set_of(inner: Type) -> Self {
        Type::Set(Box::new(inner))
    }

    pub fn union(types: Vec<Type>) -> Self {
        Type::Union(types)
    }

    pub fn tuple(types: Vec<Type>) -> Self {
        Type::Tuple(types)
    }

    pub fn callable(params: Vec<Type>, ret: Type) -> Self {
        Type::Callable(params, Box::new(ret))
    }
}

impl Expression {
    pub fn ident(s: &str) -> Self {
        Expression::Identifier(s.to_string())
    }

    pub fn self_attr(field: &str) -> Self {
        Expression::Attribute {
            object: Box::new(Expression::Identifier("self".to_string())),
            name: field.to_string(),
        }
    }

    pub fn attr(object: Expression, field: &str) -> Self {
        Expression::Attribute {
            object: Box::new(object),
            name: field.to_string(),
        }
    }

    pub fn call(func: Expression, args: Vec<Expression>) -> Self {
        Expression::Call {
            func: Box::new(func),
            arguments: args,
            keywords: vec![],
        }
    }

    pub fn method_call(receiver: Expression, method: &str, args: Vec<Expression>) -> Self {
        Expression::Call {
            func: Box::new(Expression::Attribute {
                object: Box::new(receiver),
                name: method.to_string(),
            }),
            arguments: args,
            keywords: vec![],
        }
    }

    pub fn str_lit(s: &str) -> Self {
        Expression::Literal(Literal::String(s.to_string()))
    }

    pub fn int_lit(i: i64) -> Self {
        Expression::Literal(Literal::Integer(i))
    }

    pub fn bool_lit(b: bool) -> Self {
        Expression::Literal(Literal::Boolean(b))
    }
}
