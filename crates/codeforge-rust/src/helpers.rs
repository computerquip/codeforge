use crate::ast::*;

pub mod ty {
    use super::*;

    pub fn generic(name: &str, args: Vec<GenericArg>) -> Type {
        Type::generic(name, args)
    }

    pub fn reference(inner: Type) -> Type {
        Type::Reference {
            lifetime: None,
            is_mut: false,
            inner: Box::new(inner),
        }
    }

    pub fn mut_reference(inner: Type) -> Type {
        Type::Reference {
            lifetime: None,
            is_mut: true,
            inner: Box::new(inner),
        }
    }

    pub fn reference_with_lifetime(lifetime: &str, is_mut: bool, inner: Type) -> Type {
        Type::Reference {
            lifetime: Some(lifetime.to_string()),
            is_mut,
            inner: Box::new(inner),
        }
    }

    pub fn pointer(inner: Type) -> Type {
        Type::Pointer {
            is_mut: false,
            inner: Box::new(inner),
        }
    }

    pub fn mut_pointer(inner: Type) -> Type {
        Type::Pointer {
            is_mut: true,
            inner: Box::new(inner),
        }
    }

    pub fn slice(inner: Type) -> Type {
        Type::Slice(Box::new(inner))
    }

    pub fn array(inner: Type, count: i64) -> Type {
        Type::Array(
            Box::new(inner),
            Box::new(Expression::Literal(Literal::Integer(count))),
        )
    }

    pub fn tuple(types: Vec<Type>) -> Type {
        Type::Tuple(types)
    }

    pub fn option(inner: Type) -> Type {
        Type::generic("Option", vec![GenericArg::Type(Box::new(inner))])
    }

    pub fn vec(inner: Type) -> Type {
        Type::generic("Vec", vec![GenericArg::Type(Box::new(inner))])
    }

    pub fn box_(inner: Type) -> Type {
        Type::generic("Box", vec![GenericArg::Type(Box::new(inner))])
    }

    pub fn result(ok: Type, err: Type) -> Type {
        Type::generic(
            "Result",
            vec![
                GenericArg::Type(Box::new(ok)),
                GenericArg::Type(Box::new(err)),
            ],
        )
    }

    pub fn string() -> Type {
        Type::path("String")
    }

    pub fn trait_object(bounds: Vec<Type>) -> Type {
        Type::TraitObject(bounds)
    }

    pub fn impl_trait(bounds: Vec<Type>) -> Type {
        Type::ImplTrait(bounds)
    }

    pub fn fn_ptr(params: Vec<Type>, return_type: Option<Type>) -> Type {
        Type::Fn {
            params,
            return_type: return_type.map(Box::new),
        }
    }
}

pub mod expr {
    use super::*;

    pub fn ident(name: &str) -> Expression {
        Expression::Path(Path {
            segments: vec![PathSegment {
                name: name.to_string(),
                args: vec![],
            }],
        })
    }

    pub fn path(segments: &[&str]) -> Expression {
        Expression::Path(Path {
            segments: segments
                .iter()
                .map(|s| PathSegment {
                    name: s.to_string(),
                    args: vec![],
                })
                .collect(),
        })
    }

    pub fn call(callee: Expression, args: Vec<Expression>) -> Expression {
        Expression::Call {
            callee: Box::new(callee),
            args,
        }
    }

    pub fn method_call(receiver: Expression, method: &str, args: Vec<Expression>) -> Expression {
        Expression::MethodCall {
            receiver: Box::new(receiver),
            method: method.to_string(),
            turbofish: vec![],
            args,
        }
    }

    pub fn field(base: Expression, name: &str) -> Expression {
        Expression::Field {
            base: Box::new(base),
            name: name.to_string(),
        }
    }

    pub fn self_() -> Expression {
        ident("self")
    }

    pub fn self_field(name: &str) -> Expression {
        field(self_(), name)
    }

    pub fn str_lit(s: &str) -> Expression {
        Expression::Literal(Literal::String(s.to_string()))
    }

    pub fn int_lit(i: i64) -> Expression {
        Expression::Literal(Literal::Integer(i))
    }

    pub fn uint_lit(u: u64) -> Expression {
        Expression::Literal(Literal::UnsignedInteger(u))
    }

    pub fn bool_lit(b: bool) -> Expression {
        Expression::Literal(Literal::Boolean(b))
    }

    pub fn char_lit(c: char) -> Expression {
        Expression::Literal(Literal::Char(c))
    }

    pub fn ref_expr(inner: Expression) -> Expression {
        Expression::Reference {
            is_mut: false,
            inner: Box::new(inner),
        }
    }

    pub fn mut_ref(inner: Expression) -> Expression {
        Expression::Reference {
            is_mut: true,
            inner: Box::new(inner),
        }
    }

    pub fn tuple(exprs: Vec<Expression>) -> Expression {
        Expression::Tuple(exprs)
    }

    pub fn array(exprs: Vec<Expression>) -> Expression {
        Expression::Array(exprs)
    }

    pub fn struct_literal(
        path: &str,
        fields: Vec<FieldInit>,
        rest: Option<Expression>,
    ) -> Expression {
        Expression::StructLiteral {
            path: Path {
                segments: vec![PathSegment {
                    name: path.to_string(),
                    args: vec![],
                }],
            },
            fields,
            rest: rest.map(Box::new),
        }
    }

    pub fn macro_call(path: &str, tokens: &str) -> Expression {
        Expression::MacroCall {
            path: path.to_string(),
            tokens: tokens.to_string(),
        }
    }

    pub fn return_expr(expr: Expression) -> Expression {
        Expression::Return(Some(Box::new(expr)))
    }

    pub fn return_none() -> Expression {
        Expression::Return(None)
    }

    pub fn closure(params: Vec<ClosureParam>, body: Expression) -> Expression {
        Expression::Closure {
            is_move: false,
            params,
            return_type: None,
            body: Box::new(body),
        }
    }

    pub fn move_closure(params: Vec<ClosureParam>, body: Expression) -> Expression {
        Expression::Closure {
            is_move: true,
            params,
            return_type: None,
            body: Box::new(body),
        }
    }

    pub fn binary(left: Expression, op: BinaryOperator, right: Expression) -> Expression {
        Expression::Binary {
            left: Box::new(left),
            op,
            right: Box::new(right),
        }
    }

    pub fn field_init(name: &str, value: Expression) -> FieldInit {
        FieldInit {
            name: name.to_string(),
            value: Some(value),
        }
    }

    pub fn field_shorthand(name: &str) -> FieldInit {
        FieldInit {
            name: name.to_string(),
            value: None,
        }
    }
}

pub mod stmt {
    use super::*;

    pub fn let_binding(name: &str, value: Expression) -> Statement {
        Statement::Let(Let {
            pattern: Pattern::Ident {
                name: name.to_string(),
                is_ref: false,
                is_mut: false,
                subpattern: None,
            },
            ty: None,
            value: Some(value),
            is_mut: false,
            else_block: None,
        })
    }

    pub fn let_mut(name: &str, value: Expression) -> Statement {
        Statement::Let(Let {
            pattern: Pattern::Ident {
                name: name.to_string(),
                is_ref: false,
                is_mut: false,
                subpattern: None,
            },
            ty: None,
            value: Some(value),
            is_mut: true,
            else_block: None,
        })
    }

    pub fn let_typed(name: &str, ty: Type, value: Expression) -> Statement {
        Statement::Let(Let {
            pattern: Pattern::Ident {
                name: name.to_string(),
                is_ref: false,
                is_mut: false,
                subpattern: None,
            },
            ty: Some(ty),
            value: Some(value),
            is_mut: false,
            else_block: None,
        })
    }

    pub fn expr_stmt(expression: Expression) -> Statement {
        Statement::Expression(expression)
    }

    pub fn return_expr(expr: Expression) -> Statement {
        Statement::Expression(Expression::Return(Some(Box::new(expr))))
    }

    pub fn return_none() -> Statement {
        Statement::Expression(Expression::Return(None))
    }

    pub fn comment(text: &str) -> Statement {
        Statement::Comment(text.to_string())
    }
}

pub mod vis {
    use super::*;

    pub fn public() -> Visibility {
        Visibility::Public
    }

    pub fn private() -> Visibility {
        Visibility::Private
    }

    pub fn crate_() -> Visibility {
        Visibility::Crate
    }

    pub fn super_() -> Visibility {
        Visibility::Super
    }

    pub fn restricted(path: &str) -> Visibility {
        Visibility::Restricted(path.to_string())
    }
}

pub mod attr {
    use super::*;

    pub fn derive(names: &[&str]) -> Attribute {
        Attribute {
            path: "derive".to_string(),
            tokens: Some(names.join(", ")),
            is_inner: false,
        }
    }

    pub fn allow(what: &str) -> Attribute {
        Attribute {
            path: "allow".to_string(),
            tokens: Some(what.to_string()),
            is_inner: false,
        }
    }

    pub fn cfg(condition: &str) -> Attribute {
        Attribute {
            path: "cfg".to_string(),
            tokens: Some(condition.to_string()),
            is_inner: false,
        }
    }

    pub fn inline() -> Attribute {
        Attribute {
            path: "inline".to_string(),
            tokens: None,
            is_inner: false,
        }
    }

    pub fn test() -> Attribute {
        Attribute {
            path: "test".to_string(),
            tokens: None,
            is_inner: false,
        }
    }

    pub fn named(path: &str, tokens: Option<&str>) -> Attribute {
        Attribute {
            path: path.to_string(),
            tokens: tokens.map(|s| s.to_string()),
            is_inner: false,
        }
    }
}

pub mod param {
    use super::*;

    pub fn typed(name: &str, ty: Type) -> Parameter {
        Parameter::Typed {
            pattern: Pattern::Ident {
                name: name.to_string(),
                is_ref: false,
                is_mut: false,
                subpattern: None,
            },
            ty,
        }
    }

    pub fn self_ref() -> Parameter {
        Parameter::Receiver(Receiver {
            is_ref: true,
            is_mut: false,
            lifetime: None,
        })
    }

    pub fn self_mut() -> Parameter {
        Parameter::Receiver(Receiver {
            is_ref: true,
            is_mut: true,
            lifetime: None,
        })
    }

    pub fn self_owned() -> Parameter {
        Parameter::Receiver(Receiver {
            is_ref: false,
            is_mut: false,
            lifetime: None,
        })
    }
}

pub mod function {
    use super::*;

    pub fn build(name: &str) -> FunctionBuilder {
        FunctionBuilder {
            attributes: vec![],
            visibility: Visibility::Private,
            name: name.to_string(),
            generics: Generics::empty(),
            parameters: vec![],
            return_type: None,
            body: None,
            is_async: false,
            is_const: false,
            is_unsafe: false,
            abi: None,
        }
    }

    pub struct FunctionBuilder {
        attributes: Vec<Attribute>,
        visibility: Visibility,
        name: String,
        generics: Generics,
        parameters: Vec<Parameter>,
        return_type: Option<Type>,
        body: Option<Block>,
        is_async: bool,
        is_const: bool,
        is_unsafe: bool,
        abi: Option<String>,
    }

    impl FunctionBuilder {
        pub fn vis(mut self, vis: Visibility) -> Self {
            self.visibility = vis;
            self
        }

        pub fn param(mut self, param: Parameter) -> Self {
            self.parameters.push(param);
            self
        }

        pub fn params(mut self, params: Vec<Parameter>) -> Self {
            self.parameters = params;
            self
        }

        pub fn returns(mut self, ty: Type) -> Self {
            self.return_type = Some(ty);
            self
        }

        pub fn body_stmts(mut self, stmts: Vec<Statement>) -> Self {
            self.body = Some(Block {
                statements: stmts,
                trailing_expr: None,
            });
            self
        }

        pub fn body_trailing(mut self, stmts: Vec<Statement>, trailing: Expression) -> Self {
            self.body = Some(Block {
                statements: stmts,
                trailing_expr: Some(Box::new(trailing)),
            });
            self
        }

        pub fn empty_body(mut self) -> Self {
            self.body = Some(Block {
                statements: vec![],
                trailing_expr: None,
            });
            self
        }

        pub fn attr(mut self, attr: Attribute) -> Self {
            self.attributes.push(attr);
            self
        }

        pub fn async_(mut self) -> Self {
            self.is_async = true;
            self
        }

        pub fn unsafe_(mut self) -> Self {
            self.is_unsafe = true;
            self
        }

        pub fn build(self) -> Function {
            Function {
                attributes: self.attributes,
                visibility: self.visibility,
                name: self.name,
                generics: self.generics,
                parameters: self.parameters,
                return_type: self.return_type,
                body: self.body,
                is_async: self.is_async,
                is_const: self.is_const,
                is_unsafe: self.is_unsafe,
                abi: self.abi,
            }
        }
    }
}
