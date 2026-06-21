use crate::ast::*;

impl Type {
    pub fn custom(s: &str) -> Self {
        Type::Custom(s.to_string())
    }

    pub fn pointer(inner: Type) -> Self {
        Type::Pointer(Box::new(inner))
    }

    pub fn reference(inner: Type) -> Self {
        Type::Reference(Box::new(inner))
    }

    pub fn const_ref(inner: Type) -> Self {
        Type::ConstReference(Box::new(inner))
    }

    pub fn array(inner: Type, size: Option<usize>) -> Self {
        Type::Array(Box::new(inner), size)
    }

    pub fn template_type(name: &str, arguments: Vec<Type>) -> Self {
        Type::Template {
            name: name.to_string(),
            arguments,
        }
    }

    pub fn vector(inner: Type) -> Self {
        Self::template_type("std::vector", vec![inner])
    }

    pub fn map(key: Type, value: Type) -> Self {
        Self::template_type("std::map", vec![key, value])
    }

    pub fn set(inner: Type) -> Self {
        Self::template_type("std::set", vec![inner])
    }

    pub fn optional(inner: Type) -> Self {
        Self::template_type("std::optional", vec![inner])
    }

    pub fn unique_ptr(inner: Type) -> Self {
        Self::template_type("std::unique_ptr", vec![inner])
    }

    pub fn shared_ptr(inner: Type) -> Self {
        Self::template_type("std::shared_ptr", vec![inner])
    }
}

impl Expression {
    pub fn ident(s: &str) -> Self {
        Expression::Identifier(s.to_string())
    }

    pub fn call(func: Expression, args: Vec<Expression>) -> Self {
        Expression::Call {
            callee: Box::new(func),
            arguments: args,
        }
    }

    pub fn method_call(receiver: Expression, member: &str, args: Vec<Expression>) -> Self {
        Expression::Call {
            callee: Box::new(Expression::MemberAccess {
                object: Box::new(receiver),
                member: member.to_string(),
                is_pointer: false,
            }),
            arguments: args,
        }
    }

    pub fn ptr_method_call(receiver: Expression, member: &str, args: Vec<Expression>) -> Self {
        Expression::Call {
            callee: Box::new(Expression::MemberAccess {
                object: Box::new(receiver),
                member: member.to_string(),
                is_pointer: true,
            }),
            arguments: args,
        }
    }

    pub fn member(object: Expression, member: &str) -> Self {
        Expression::MemberAccess {
            object: Box::new(object),
            member: member.to_string(),
            is_pointer: false,
        }
    }

    pub fn ptr_member(object: Expression, member: &str) -> Self {
        Expression::MemberAccess {
            object: Box::new(object),
            member: member.to_string(),
            is_pointer: true,
        }
    }

    pub fn array_access(array: Expression, index: Expression) -> Self {
        Expression::ArrayAccess {
            array: Box::new(array),
            index: Box::new(index),
        }
    }

    pub fn cast(target_type: Type, expr: Expression) -> Self {
        Expression::Cast {
            target_type,
            expr: Box::new(expr),
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

    pub fn char_lit(c: char) -> Self {
        Expression::Literal(Literal::Character(c))
    }

    pub fn nullptr() -> Self {
        Expression::Literal(Literal::Null)
    }

    pub fn new_expr(type_name: &str, args: Vec<Expression>) -> Self {
        Expression::Call {
            callee: Box::new(Expression::Raw(format!("new {type_name}"))),
            arguments: args,
        }
    }
}

pub mod directive {
    use super::*;

    pub fn include_system(name: &str) -> Directive {
        Directive::Include(Include::System(name.to_string()))
    }

    pub fn include_local(name: &str) -> Directive {
        Directive::Include(Include::Local(name.to_string()))
    }

    pub fn define(name: &str) -> Directive {
        Directive::Define {
            name: name.to_string(),
            value: None,
        }
    }

    pub fn define_value(name: &str, value: &str) -> Directive {
        Directive::Define {
            name: name.to_string(),
            value: Some(value.to_string()),
        }
    }

    pub fn undef(name: &str) -> Directive {
        Directive::Undef(name.to_string())
    }

    pub fn ifdef(name: &str) -> Directive {
        Directive::Ifdef(name.to_string())
    }

    pub fn ifndef(name: &str) -> Directive {
        Directive::Ifndef(name.to_string())
    }

    pub fn pragma(text: &str) -> Directive {
        Directive::Pragma(text.to_string())
    }

    pub fn error(msg: &str) -> Directive {
        Directive::Error(msg.to_string())
    }
}

pub mod stmt {
    use super::*;

    pub fn declaration(var_type: Type, name: &str, initializer: Option<Expression>) -> Statement {
        Statement::VariableDeclaration(LocalVariable {
            name: name.to_string(),
            var_type,
            initializer,
            is_const: false,
            is_static: false,
            is_thread_local: false,
        })
    }

    pub fn const_declaration(
        var_type: Type,
        name: &str,
        initializer: Option<Expression>,
    ) -> Statement {
        Statement::VariableDeclaration(LocalVariable {
            name: name.to_string(),
            var_type,
            initializer,
            is_const: true,
            is_static: false,
            is_thread_local: false,
        })
    }

    pub fn assignment(target: Expression, value: Expression) -> Statement {
        Statement::Expression(Expression::BinaryOp {
            left: Box::new(target),
            op: BinaryOperator::Assign,
            right: Box::new(value),
        })
    }

    pub fn assign_ident(name: &str, value: Expression) -> Statement {
        assignment(Expression::Identifier(name.to_string()), value)
    }

    pub fn return_expr(expr: Expression) -> Statement {
        Statement::Return(Some(expr))
    }

    pub fn return_void() -> Statement {
        Statement::Return(None)
    }

    pub fn if_simple(cond: Expression, body: Vec<Statement>) -> Statement {
        Statement::If(Box::new(IfStatement {
            condition: cond,
            then_block: Block { statements: body },
            else_block: None,
        }))
    }

    pub fn if_else(
        cond: Expression,
        then_body: Vec<Statement>,
        else_body: Vec<Statement>,
    ) -> Statement {
        Statement::If(Box::new(IfStatement {
            condition: cond,
            then_block: Block {
                statements: then_body,
            },
            else_block: Some(Block {
                statements: else_body,
            }),
        }))
    }

    pub fn for_simple(
        initializer: Statement,
        condition: Expression,
        update: Expression,
        body: Vec<Statement>,
    ) -> Statement {
        Statement::For(Box::new(ForStatement {
            initializer: Some(Box::new(initializer)),
            condition: Some(condition),
            update: Some(update),
            body: Block { statements: body },
        }))
    }

    pub fn while_simple(cond: Expression, body: Vec<Statement>) -> Statement {
        Statement::While(Box::new(WhileStatement {
            condition: cond,
            body: Block { statements: body },
        }))
    }

    pub fn expr_stmt(expr: Expression) -> Statement {
        Statement::Expression(expr)
    }

    pub fn break_stmt() -> Statement {
        Statement::Break
    }

    pub fn continue_stmt() -> Statement {
        Statement::Continue
    }

    pub fn comment(text: &str) -> Statement {
        Statement::Comment(text.to_string())
    }

    pub fn raw(text: &str) -> Statement {
        Statement::Raw(text.to_string())
    }
}
