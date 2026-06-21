use crate::ast::*;

pub mod decorator {
    use super::*;

    pub fn classmethod() -> Expression {
        Expression::Identifier("classmethod".to_string())
    }

    pub fn staticmethod() -> Expression {
        Expression::Identifier("staticmethod".to_string())
    }

    pub fn property() -> Expression {
        Expression::Identifier("property".to_string())
    }

    pub fn named(name: &str) -> Expression {
        Expression::Identifier(name.to_string())
    }

    pub fn call(name: &str, args: Vec<Expression>) -> Expression {
        Expression::Call {
            func: Box::new(Expression::Identifier(name.to_string())),
            arguments: args,
            keywords: vec![],
        }
    }

    pub fn call_with_keywords(
        name: &str,
        args: Vec<Expression>,
        keywords: Vec<Keyword>,
    ) -> Expression {
        Expression::Call {
            func: Box::new(Expression::Identifier(name.to_string())),
            arguments: args,
            keywords,
        }
    }
}

pub mod stmt {
    use super::*;

    pub fn assign(target: &str, value: Expression) -> Statement {
        Statement::Assign(Assign {
            target: Expression::Identifier(target.to_string()),
            value,
        })
    }

    pub fn assign_call(target: &str, func: &str, args: Vec<Expression>) -> Statement {
        Statement::Assign(Assign {
            target: Expression::Identifier(target.to_string()),
            value: Expression::Call {
                func: Box::new(Expression::Identifier(func.to_string())),
                arguments: args,
                keywords: vec![],
            },
        })
    }

    pub fn assign_method_call(
        target: &str,
        receiver: Expression,
        method: &str,
        args: Vec<Expression>,
    ) -> Statement {
        Statement::Assign(Assign {
            target: Expression::Identifier(target.to_string()),
            value: Expression::Call {
                func: Box::new(Expression::Attribute {
                    object: Box::new(receiver),
                    name: method.to_string(),
                }),
                arguments: args,
                keywords: vec![],
            },
        })
    }

    pub fn extend(list: Expression, item: Expression) -> Statement {
        Statement::Expression(Expression::Call {
            func: Box::new(Expression::Attribute {
                object: Box::new(list),
                name: "extend".to_string(),
            }),
            arguments: vec![item],
            keywords: vec![],
        })
    }

    pub fn method_call(receiver: Expression, method: &str, args: Vec<Expression>) -> Statement {
        Statement::Expression(Expression::Call {
            func: Box::new(Expression::Attribute {
                object: Box::new(receiver),
                name: method.to_string(),
            }),
            arguments: args,
            keywords: vec![],
        })
    }

    pub fn return_value(expr: Expression) -> Statement {
        Statement::Return(Some(expr))
    }

    pub fn return_none() -> Statement {
        Statement::Return(Some(Expression::Literal(Literal::None_)))
    }

    pub fn if_simple(cond: Expression, body: Vec<Statement>) -> Statement {
        Statement::If(Box::new(IfStatement {
            condition: cond,
            body,
            elif_clauses: vec![],
            else_body: None,
        }))
    }

    pub fn for_simple(target: Expression, iter: Expression, body: Vec<Statement>) -> Statement {
        Statement::For(Box::new(ForStatement {
            target,
            iter,
            body,
            else_body: None,
        }))
    }

    pub fn expr_stmt(expression: Expression) -> Statement {
        Statement::Expression(expression)
    }
}
