#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Module {
    pub attributes: Vec<Attribute>,
    pub items: Vec<Item>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Item {
    Use(Use),
    Function(Function),
    Struct(Struct),
    Enum(Enum),
    Trait(Trait),
    Impl(Impl),
    TypeAlias(TypeAlias),
    Const(Const),
    Static(Static),
    Mod(Mod),
    Raw(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Visibility {
    Private,
    Public,
    Crate,
    Super,
    Restricted(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Attribute {
    pub path: String,
    pub tokens: Option<String>,
    pub is_inner: bool,
}

impl Attribute {
    pub fn derive(names: Vec<String>) -> Self {
        Self {
            path: "derive".to_string(),
            tokens: Some(names.join(", ")),
            is_inner: false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Use {
    pub visibility: Visibility,
    pub tree: UseTree,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum UseTree {
    Path(String),
    Alias { path: String, alias: String },
    Glob(String),
    Group { prefix: String, items: Vec<UseTree> },
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Function {
    pub attributes: Vec<Attribute>,
    pub visibility: Visibility,
    pub name: String,
    pub generics: Generics,
    pub parameters: Vec<Parameter>,
    pub return_type: Option<Type>,
    pub body: Option<Block>,
    pub is_async: bool,
    pub is_const: bool,
    pub is_unsafe: bool,
    pub abi: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Parameter {
    Receiver(Receiver),
    Typed { pattern: Pattern, ty: Type },
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Receiver {
    pub is_ref: bool,
    pub is_mut: bool,
    pub lifetime: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Struct {
    pub attributes: Vec<Attribute>,
    pub visibility: Visibility,
    pub name: String,
    pub generics: Generics,
    pub kind: StructKind,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum StructKind {
    Unit,
    Tuple(Vec<TupleField>),
    Named(Vec<Field>),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct TupleField {
    pub attributes: Vec<Attribute>,
    pub visibility: Visibility,
    pub ty: Type,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Field {
    pub attributes: Vec<Attribute>,
    pub visibility: Visibility,
    pub name: String,
    pub ty: Type,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Enum {
    pub attributes: Vec<Attribute>,
    pub visibility: Visibility,
    pub name: String,
    pub generics: Generics,
    pub variants: Vec<EnumVariant>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct EnumVariant {
    pub attributes: Vec<Attribute>,
    pub name: String,
    pub kind: VariantKind,
    pub discriminant: Option<Expression>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum VariantKind {
    Unit,
    Tuple(Vec<Type>),
    Named(Vec<Field>),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Trait {
    pub attributes: Vec<Attribute>,
    pub visibility: Visibility,
    pub name: String,
    pub generics: Generics,
    pub supertraits: Vec<Type>,
    pub items: Vec<AssocItem>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Impl {
    pub attributes: Vec<Attribute>,
    pub generics: Generics,
    pub trait_: Option<Type>,
    pub self_ty: Type,
    pub is_unsafe: bool,
    pub items: Vec<AssocItem>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum AssocItem {
    Function(Function),
    Const(Const),
    Type(AssocType),
    Raw(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct AssocType {
    pub attributes: Vec<Attribute>,
    pub name: String,
    pub generics: Generics,
    pub bounds: Vec<Type>,
    pub value: Option<Type>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct TypeAlias {
    pub attributes: Vec<Attribute>,
    pub visibility: Visibility,
    pub name: String,
    pub generics: Generics,
    pub ty: Type,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Const {
    pub attributes: Vec<Attribute>,
    pub visibility: Visibility,
    pub name: String,
    pub ty: Type,
    pub value: Option<Expression>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Static {
    pub attributes: Vec<Attribute>,
    pub visibility: Visibility,
    pub name: String,
    pub ty: Type,
    pub is_mut: bool,
    pub value: Expression,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Mod {
    pub attributes: Vec<Attribute>,
    pub visibility: Visibility,
    pub name: String,
    pub items: Option<Vec<Item>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Generics {
    pub params: Vec<GenericParam>,
    pub where_clause: Vec<WherePredicate>,
}

impl Generics {
    pub fn empty() -> Self {
        Self {
            params: vec![],
            where_clause: vec![],
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum GenericParam {
    Lifetime {
        name: String,
        bounds: Vec<String>,
    },
    Type {
        name: String,
        bounds: Vec<Type>,
        default: Option<Type>,
    },
    Const {
        name: String,
        ty: Type,
        default: Option<Expression>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct WherePredicate {
    pub ty: Type,
    pub bounds: Vec<Type>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Type {
    Unit,
    Bool,
    Char,
    Str,
    I8,
    I16,
    I32,
    I64,
    I128,
    Isize,
    U8,
    U16,
    U32,
    U64,
    U128,
    Usize,
    F32,
    F64,
    Path(Path),
    Reference {
        lifetime: Option<String>,
        is_mut: bool,
        inner: Box<Type>,
    },
    Pointer {
        is_mut: bool,
        inner: Box<Type>,
    },
    Tuple(Vec<Type>),
    Slice(Box<Type>),
    Array(Box<Type>, Box<Expression>),
    TraitObject(Vec<Type>),
    ImplTrait(Vec<Type>),
    Fn {
        params: Vec<Type>,
        return_type: Option<Box<Type>>,
    },
    Infer,
    SelfType,
    Raw(String),
}

impl Type {
    pub fn path(name: &str) -> Self {
        Type::Path(Path {
            segments: vec![PathSegment {
                name: name.to_string(),
                args: vec![],
            }],
        })
    }

    pub fn generic(name: &str, args: Vec<GenericArg>) -> Self {
        Type::Path(Path {
            segments: vec![PathSegment {
                name: name.to_string(),
                args,
            }],
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Path {
    pub segments: Vec<PathSegment>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PathSegment {
    pub name: String,
    pub args: Vec<GenericArg>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum GenericArg {
    Lifetime(String),
    Type(Box<Type>),
    Binding { name: String, ty: Type },
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Block {
    pub statements: Vec<Statement>,
    pub trailing_expr: Option<Box<Expression>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Statement {
    Let(Let),
    Expression(Expression),
    Item(Box<Item>),
    Comment(String),
    Raw(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Let {
    pub pattern: Pattern,
    pub ty: Option<Type>,
    pub value: Option<Expression>,
    pub is_mut: bool,
    pub else_block: Option<Block>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Pattern {
    Wildcard,
    Rest,
    Ident {
        name: String,
        is_ref: bool,
        is_mut: bool,
        subpattern: Option<Box<Pattern>>,
    },
    Literal(Literal),
    Tuple(Vec<Pattern>),
    Slice(Vec<Pattern>),
    TupleStruct {
        path: Path,
        elems: Vec<Pattern>,
    },
    Struct {
        path: Path,
        fields: Vec<FieldPattern>,
        has_rest: bool,
    },
    Or(Vec<Pattern>),
    Reference {
        is_mut: bool,
        inner: Box<Pattern>,
    },
    Path(Path),
    Raw(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct FieldPattern {
    pub name: String,
    pub pattern: Option<Pattern>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Expression {
    Literal(Literal),
    Path(Path),
    Binary {
        left: Box<Expression>,
        op: BinaryOperator,
        right: Box<Expression>,
    },
    Unary {
        op: UnaryOperator,
        operand: Box<Expression>,
    },
    Call {
        callee: Box<Expression>,
        args: Vec<Expression>,
    },
    MethodCall {
        receiver: Box<Expression>,
        method: String,
        turbofish: Vec<GenericArg>,
        args: Vec<Expression>,
    },
    Field {
        base: Box<Expression>,
        name: String,
    },
    Index {
        base: Box<Expression>,
        index: Box<Expression>,
    },
    Reference {
        is_mut: bool,
        inner: Box<Expression>,
    },
    Deref(Box<Expression>),
    Try(Box<Expression>),
    Cast {
        expr: Box<Expression>,
        ty: Box<Type>,
    },
    Tuple(Vec<Expression>),
    Array(Vec<Expression>),
    Repeat {
        value: Box<Expression>,
        count: Box<Expression>,
    },
    StructLiteral {
        path: Path,
        fields: Vec<FieldInit>,
        rest: Option<Box<Expression>>,
    },
    Closure {
        is_move: bool,
        params: Vec<ClosureParam>,
        return_type: Option<Type>,
        body: Box<Expression>,
    },
    If(Box<IfExpr>),
    Match {
        scrutinee: Box<Expression>,
        arms: Vec<MatchArm>,
    },
    Loop {
        label: Option<String>,
        body: Box<Block>,
    },
    While {
        label: Option<String>,
        condition: IfCondition,
        body: Box<Block>,
    },
    For {
        label: Option<String>,
        pattern: Pattern,
        iter: Box<Expression>,
        body: Box<Block>,
    },
    Block(Block),
    Return(Option<Box<Expression>>),
    Break {
        label: Option<String>,
        value: Option<Box<Expression>>,
    },
    Continue {
        label: Option<String>,
    },
    Range {
        start: Option<Box<Expression>>,
        end: Option<Box<Expression>>,
        inclusive: bool,
    },
    MacroCall {
        path: String,
        tokens: String,
    },
    Raw(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct IfExpr {
    pub condition: IfCondition,
    pub then_block: Block,
    pub else_branch: Option<ElseBranch>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum IfCondition {
    Expr(Box<Expression>),
    Let {
        pattern: Pattern,
        value: Box<Expression>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ElseBranch {
    Block(Block),
    If(Box<IfExpr>),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct MatchArm {
    pub pattern: Pattern,
    pub guard: Option<Expression>,
    pub body: Expression,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ClosureParam {
    pub pattern: Pattern,
    pub ty: Option<Type>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct FieldInit {
    pub name: String,
    pub value: Option<Expression>,
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
    UnsignedInteger(u64),
    Float(F64Wrapper),
    Boolean(bool),
    String(String),
    Char(char),
    ByteString(Vec<u8>),
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
    And,
    Or,
    BitAnd,
    BitOr,
    BitXor,
    Shl,
    Shr,
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
    Assign,
    AddAssign,
    SubAssign,
    MulAssign,
    DivAssign,
    RemAssign,
    BitAndAssign,
    BitOrAssign,
    BitXorAssign,
    ShlAssign,
    ShrAssign,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum UnaryOperator {
    Neg,
    Not,
}
