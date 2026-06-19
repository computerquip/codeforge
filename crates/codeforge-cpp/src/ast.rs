#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Program {
    pub directives: Vec<Directive>,
    pub namespaces: Vec<Namespace>,
    pub declarations: Vec<Declaration>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Namespace {
    pub name: String,
    pub declarations: Vec<Declaration>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Declaration {
    Function(Function),
    Class(Class),
    Struct(Struct),
    Variable(LocalVariable),
    Enum(Enum),
    Typedef(Typedef),
    Template(Box<Template>),
    Conditional(Conditional<Declaration>),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Conditional<T> {
    pub condition: String,
    pub body: Vec<T>,
    pub elif_branches: Vec<(String, Vec<T>)>,
    pub else_body: Option<Vec<T>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Include {
    System(String),
    Local(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Directive {
    Include(Include),
    Define { name: String, value: Option<String> },
    Undef(String),
    Ifdef(String),
    Ifndef(String),
    Error(String),
    Pragma(String),
    Conditional(Conditional<Directive>),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Template {
    pub parameters: Vec<TemplateParameter>,
    pub declaration: Box<Declaration>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum TemplateParameter {
    Type {
        name: String,
        default: Option<Type>,
    },
    NonType {
        param_type: Type,
        name: String,
        default: Option<Expression>,
    },
    Template {
        parameters: Vec<TemplateParameter>,
        name: String,
        default: Option<Type>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Function {
    pub name: String,
    pub return_type: Type,
    pub parameters: Vec<Parameter>,
    pub body: Option<Block>,
    pub is_const: bool,
    pub is_inline: bool,
    pub is_static: bool,
    pub is_virtual: bool,
    pub is_pure_virtual: bool,
    pub is_override: bool,
    pub is_noexcept: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Class {
    pub name: String,
    pub base_classes: Vec<BaseClass>,
    pub members: Vec<ClassMember>,
    pub is_final: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct BaseClass {
    pub name: String,
    pub access: AccessSpecifier,
    pub is_virtual: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum AccessSpecifier {
    Public,
    Protected,
    Private,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ClassMember {
    Field(Field),
    Method(Function),
    Constructor(Constructor),
    Destructor(Destructor),
    Access(AccessSpecifier),
    Conditional(Conditional<ClassMember>),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Constructor {
    pub parameters: Vec<Parameter>,
    pub initializer_list: Vec<MemberInitializer>,
    pub body: Block,
    pub is_explicit: bool,
    pub is_deleted: bool,
    pub is_defaulted: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct MemberInitializer {
    pub member_name: String,
    pub value: Expression,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Destructor {
    pub is_virtual: bool,
    pub is_deleted: bool,
    pub is_defaulted: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Struct {
    pub name: String,
    pub fields: Vec<Field>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Field {
    pub name: String,
    pub var_type: Type,
    pub initializer: Option<Expression>,
    pub access: AccessSpecifier,
    pub is_const: bool,
    pub is_static: bool,
    pub is_thread_local: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct LocalVariable {
    pub name: String,
    pub var_type: Type,
    pub initializer: Option<Expression>,
    pub is_const: bool,
    pub is_static: bool,
    pub is_thread_local: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Enum {
    pub name: String,
    pub underlying_type: Option<Type>,
    pub variants: Vec<EnumVariant>,
    pub is_scoped: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct EnumVariant {
    pub name: String,
    pub value: Option<Expression>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Typedef {
    pub name: String,
    pub alias: Type,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Parameter {
    pub name: String,
    pub param_type: Type,
    pub default_value: Option<Expression>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Type {
    Void,
    Bool,
    Int8,
    Int16,
    Int32,
    Int64,
    UInt8,
    UInt16,
    UInt32,
    UInt64,
    Float32,
    Float64,
    Char,
    String,
    Custom(String),
    Pointer(Box<Type>),
    Reference(Box<Type>),
    ConstReference(Box<Type>),
    Array(Box<Type>, Option<usize>),
    Template { name: String, arguments: Vec<Type> },
    Auto,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Block {
    pub statements: Vec<Statement>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Statement {
    Expression(Expression),
    Return(Option<Expression>),
    If(Box<IfStatement>),
    While(Box<WhileStatement>),
    For(Box<ForStatement>),
    VariableDeclaration(LocalVariable),
    Break,
    Continue,
    Comment(String),
    Raw(String),
    Conditional(Conditional<Statement>),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct IfStatement {
    pub condition: Expression,
    pub then_block: Block,
    pub else_block: Option<Block>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct WhileStatement {
    pub condition: Expression,
    pub body: Block,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ForStatement {
    pub initializer: Option<Box<Statement>>,
    pub condition: Option<Expression>,
    pub update: Option<Expression>,
    pub body: Block,
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
    Character(char),
    Null,
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
        callee: Box<Expression>,
        arguments: Vec<Expression>,
    },
    MemberAccess {
        object: Box<Expression>,
        member: String,
        is_pointer: bool,
    },
    ArrayAccess {
        array: Box<Expression>,
        index: Box<Expression>,
    },
    Cast {
        target_type: Type,
        expr: Box<Expression>,
    },
    Ternary {
        condition: Box<Expression>,
        then_expr: Box<Expression>,
        else_expr: Box<Expression>,
    },
    Sizeof(Type),
    Raw(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum BinaryOperator {
    Add,
    Sub,
    Mul,
    Div,
    Rem,
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
    Assign,
    AddAssign,
    SubAssign,
    MulAssign,
    DivAssign,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum UnaryOperator {
    Pos,
    Neg,
    Not,
    BitNot,
    PreInc,
    PreDec,
    PostInc,
    PostDec,
    Deref,
    AddressOf,
}
