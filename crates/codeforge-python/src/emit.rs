use codeforge_emit::{CodeWriter, Emit};

use crate::ast::*;

impl Emit for Module {
    fn emit(&self, w: &mut CodeWriter) {
        for import in &self.imports {
            import.emit(w);
        }
        if !self.imports.is_empty() && !self.body.is_empty() {
            w.blank();
            w.blank();
        }
        for (i, stmt) in self.body.iter().enumerate() {
            stmt.emit(w);
            if i < self.body.len() - 1 {
                let next = &self.body[i + 1];
                if is_definition(stmt) || is_definition(next) {
                    w.blank();
                    w.blank();
                }
            }
        }
    }
}

impl Emit for Import {
    fn emit(&self, w: &mut CodeWriter) {
        match self {
            Import::Simple(imp) => {
                w.line(&format!("import {}", imp.names.join(", ")));
            }
            Import::From(imp) => {
                let names: Vec<String> = imp
                    .names
                    .iter()
                    .map(|n| match &n.alias {
                        Some(alias) => format!("{} as {}", n.name, alias),
                        None => n.name.clone(),
                    })
                    .collect();
                w.line(&format!("from {} import {}", imp.module, names.join(", ")));
            }
        }
    }
}

impl Emit for Statement {
    fn emit(&self, w: &mut CodeWriter) {
        match self {
            Statement::FunctionDef(f) => f.emit(w),
            Statement::ClassDef(c) => c.emit(w),
            Statement::Return(None) => {
                w.line("return");
            }
            Statement::Return(Some(expr)) => {
                w.line(&format!("return {}", expr.to_python()));
            }
            Statement::Assign(a) => {
                w.line(&format!(
                    "{} = {}",
                    a.target.to_python(),
                    a.value.to_python()
                ));
            }
            Statement::AugAssign(a) => {
                w.line(&format!(
                    "{} {}= {}",
                    a.target.to_python(),
                    a.op.to_python(),
                    a.value.to_python()
                ));
            }
            Statement::If(if_stmt) => if_stmt.emit(w),
            Statement::While(while_stmt) => while_stmt.emit(w),
            Statement::For(for_stmt) => for_stmt.emit(w),
            Statement::Expression(expr) => {
                w.line(&expr.to_python());
            }
            Statement::Pass => {
                w.line("pass");
            }
            Statement::Break => {
                w.line("break");
            }
            Statement::Continue => {
                w.line("continue");
            }
            Statement::Comment(text) => {
                for line in text.lines() {
                    w.line(&format!("# {}", line));
                }
            }
            Statement::Raw(text) => {
                w.line(text);
            }
        }
    }
}

impl Emit for FunctionDef {
    fn emit(&self, w: &mut CodeWriter) {
        for dec in &self.decorators {
            w.line(&format!("@{}", dec.to_python()));
        }
        w.write_indent();
        if self.is_async {
            w.write("async ");
        }
        w.write(&format!("def {}(", self.name));
        emit_params(
            w,
            &self.parameters,
            &self.vararg,
            &self.kw_only_params,
            &self.kwarg,
        );
        w.write(")");
        if let Some(ret) = &self.return_annotation {
            w.write(&format!(" -> {}", ret.to_python()));
        }
        w.writeln(":");
        emit_body(w, &self.body, self.docstring.as_deref(), 0);
    }
}

impl Emit for ClassDef {
    fn emit(&self, w: &mut CodeWriter) {
        for dec in &self.decorators {
            w.line(&format!("@{}", dec.to_python()));
        }
        w.write_indent();
        w.write(&format!("class {}", self.name));
        if !self.bases.is_empty() || !self.keywords.is_empty() {
            let parts = self
                .bases
                .iter()
                .map(|b| b.to_python())
                .chain(
                    self.keywords
                        .iter()
                        .map(|k| format!("{}={}", k.name, k.value.to_python())),
                )
                .collect::<Vec<_>>()
                .join(", ");
            w.write(&format!("({})", parts));
        }
        w.writeln(":");
        emit_body(w, &self.body, self.docstring.as_deref(), 1);
    }
}

impl Emit for IfStatement {
    fn emit(&self, w: &mut CodeWriter) {
        w.write_indent();
        w.write(&format!("if {}:", self.condition.to_python()));
        w.newline();
        emit_body(w, &self.body, None, 0);
        for elif in &self.elif_clauses {
            w.write_indent();
            w.write(&format!("elif {}:", elif.condition.to_python()));
            w.newline();
            emit_body(w, &elif.body, None, 0);
        }
        if let Some(else_body) = &self.else_body {
            w.line("else:");
            emit_body(w, else_body, None, 0);
        }
    }
}

impl Emit for WhileStatement {
    fn emit(&self, w: &mut CodeWriter) {
        w.write_indent();
        w.write(&format!("while {}:", self.condition.to_python()));
        w.newline();
        emit_body(w, &self.body, None, 0);
    }
}

impl Emit for ForStatement {
    fn emit(&self, w: &mut CodeWriter) {
        w.write_indent();
        w.write(&format!(
            "for {} in {}:",
            self.target.to_python(),
            self.iter.to_python()
        ));
        w.newline();
        emit_body(w, &self.body, None, 0);
        if let Some(else_body) = &self.else_body {
            w.line("else:");
            emit_body(w, else_body, None, 0);
        }
    }
}

fn emit_params(
    w: &mut CodeWriter,
    params: &[Parameter],
    vararg: &Option<Parameter>,
    kw_only_params: &[Parameter],
    kwarg: &Option<Parameter>,
) {
    let mut parts: Vec<String> = Vec::new();
    for p in params {
        parts.push(p.to_python());
    }
    if let Some(va) = vararg {
        let mut s = format!("*{}", va.name);
        if let Some(ann) = &va.annotation {
            s.push_str(&format!(": {}", ann.to_python()));
        }
        parts.push(s);
    } else if !kw_only_params.is_empty() {
        parts.push("*".to_string());
    }
    for p in kw_only_params {
        parts.push(p.to_python());
    }
    if let Some(kw) = kwarg {
        let mut s = format!("**{}", kw.name);
        if let Some(ann) = &kw.annotation {
            s.push_str(&format!(": {}", ann.to_python()));
        }
        parts.push(s);
    }
    w.write(&parts.join(", "));
}

fn emit_body(w: &mut CodeWriter, stmts: &[Statement], docstring: Option<&str>, def_spacing: usize) {
    w.indent();
    if let Some(doc) = docstring {
        let sanitized = doc.replace("\"\"\"", "\\\"\\\"\\\"");
        if sanitized.contains('\n') {
            w.line("\"\"\"");
            for line in sanitized.lines() {
                w.line(line);
            }
            w.line("\"\"\"");
        } else {
            w.line(&format!("\"\"\"{}\"\"\"", sanitized));
        }
    }
    if docstring.is_none() && stmts.is_empty() {
        w.line("pass");
    }
    for (i, stmt) in stmts.iter().enumerate() {
        stmt.emit(w);
        if def_spacing > 0 && i < stmts.len() - 1 {
            let next = &stmts[i + 1];
            if is_definition(stmt) || is_definition(next) {
                for _ in 0..def_spacing {
                    w.blank();
                }
            }
        }
    }
    w.dedent();
}

fn is_definition(stmt: &Statement) -> bool {
    matches!(stmt, Statement::FunctionDef(_) | Statement::ClassDef(_))
}

impl Parameter {
    pub fn to_python(&self) -> String {
        match (&self.annotation, &self.default) {
            (None, None) => self.name.clone(),
            (Some(ann), None) => format!("{}: {}", self.name, ann.to_python()),
            (None, Some(def)) => format!("{}={}", self.name, def.to_python()),
            (Some(ann), Some(def)) => {
                format!("{}: {} = {}", self.name, ann.to_python(), def.to_python())
            }
        }
    }
}

impl Type {
    pub fn to_python(&self) -> String {
        match self {
            Type::None_ => "None".to_string(),
            Type::Int => "int".to_string(),
            Type::Float => "float".to_string(),
            Type::Str => "str".to_string(),
            Type::Bool => "bool".to_string(),
            Type::Bytes => "bytes".to_string(),
            Type::Any => "Any".to_string(),
            Type::Custom(name) => name.clone(),
            Type::Generic(name, args) => {
                let args_str: Vec<String> = args.iter().map(|a| a.to_python()).collect();
                format!("{}[{}]", name, args_str.join(", "))
            }
            Type::Optional(inner) => format!("Optional[{}]", inner.to_python()),
            Type::Union(types) => {
                let parts: Vec<String> = types.iter().map(|t| t.to_python()).collect();
                format!("Union[{}]", parts.join(", "))
            }
            Type::Tuple(types) => {
                let parts: Vec<String> = types.iter().map(|t| t.to_python()).collect();
                format!("tuple[{}]", parts.join(", "))
            }
            Type::List(inner) => format!("list[{}]", inner.to_python()),
            Type::Dict(k, v) => format!("dict[{}, {}]", k.to_python(), v.to_python()),
            Type::Set(inner) => format!("set[{}]", inner.to_python()),
            Type::Callable(params, ret) => {
                let params_str: Vec<String> = params.iter().map(|p| p.to_python()).collect();
                format!("Callable[[{}], {}]", params_str.join(", "), ret.to_python())
            }
            Type::Self_ => "Self".to_string(),
            Type::Raw(s) => s.clone(),
        }
    }
}

impl Expression {
    pub fn to_python(&self) -> String {
        match self {
            Expression::Literal(lit) => lit.to_python(),
            Expression::Identifier(name) => name.clone(),
            Expression::BinaryOp { left, op, right } => {
                format!(
                    "{} {} {}",
                    left.to_python(),
                    op.to_python(),
                    right.to_python()
                )
            }
            Expression::UnaryOp { op, operand } => {
                let operand_str = operand.to_python();
                if matches!(op, UnaryOperator::Not) {
                    if matches!(
                        operand.as_ref(),
                        Expression::BinaryOp {
                            op: BinaryOperator::And | BinaryOperator::Or,
                            ..
                        }
                    ) {
                        format!("{}({})", op.to_python(), operand_str)
                    } else {
                        format!("{}{}", op.to_python(), operand_str)
                    }
                } else {
                    format!("{}{}", op.to_python(), operand_str)
                }
            }
            Expression::Call {
                func,
                arguments,
                keywords,
            } => {
                let mut parts: Vec<String> = arguments.iter().map(|a| a.to_python()).collect();
                for kw in keywords {
                    parts.push(format!("{}={}", kw.name, kw.value.to_python()));
                }
                format!("{}({})", func.to_python(), parts.join(", "))
            }
            Expression::Attribute { object, name } => {
                format!("{}.{}", object.to_python(), name)
            }
            Expression::Subscript { value, index } => {
                format!("{}[{}]", value.to_python(), index.to_python())
            }
            Expression::Starred(inner) => {
                format!("*{}", inner.to_python())
            }
            Expression::List(items) => {
                let parts: Vec<String> = items.iter().map(|i| i.to_python()).collect();
                format!("[{}]", parts.join(", "))
            }
            Expression::Tuple(items) => match items.len() {
                0 => "()".to_string(),
                1 => format!("({},)", items[0].to_python()),
                _ => {
                    let parts: Vec<String> = items.iter().map(|i| i.to_python()).collect();
                    format!("({})", parts.join(", "))
                }
            },
            Expression::Dict(entries) => {
                let parts: Vec<String> = entries
                    .iter()
                    .map(|(k, v)| format!("{}: {}", k.to_python(), v.to_python()))
                    .collect();
                format!("{{{}}}", parts.join(", "))
            }
            Expression::Set(items) => {
                if items.is_empty() {
                    "set()".to_string()
                } else {
                    let parts: Vec<String> = items.iter().map(|i| i.to_python()).collect();
                    format!("{{{}}}", parts.join(", "))
                }
            }
            Expression::Ternary {
                condition,
                then_expr,
                else_expr,
            } => format!(
                "{} if {} else {}",
                then_expr.to_python(),
                condition.to_python(),
                else_expr.to_python()
            ),
            Expression::Lambda { parameters, body } => {
                let params: Vec<String> = parameters.iter().map(|p| p.to_python()).collect();
                if params.is_empty() {
                    format!("lambda: {}", body.to_python())
                } else {
                    format!("lambda {}: {}", params.join(", "), body.to_python())
                }
            }
            Expression::Raw(text) => text.clone(),
        }
    }
}

impl Literal {
    pub fn to_python(&self) -> String {
        match self {
            Literal::Integer(i) => i.to_string(),
            Literal::Float(f) => {
                let v = f.0;
                if v.is_nan() {
                    "float('nan')".to_string()
                } else if v.is_infinite() {
                    if v.is_sign_positive() {
                        "float('inf')".to_string()
                    } else {
                        "float('-inf')".to_string()
                    }
                } else {
                    v.to_string()
                }
            }
            Literal::Boolean(true) => "True".to_string(),
            Literal::Boolean(false) => "False".to_string(),
            Literal::String(s) => {
                let escaped = s
                    .replace('\\', "\\\\")
                    .replace('\'', "\\'")
                    .replace('\n', "\\n")
                    .replace('\r', "\\r")
                    .replace('\t', "\\t");
                format!("'{}'", escaped)
            }
            Literal::None_ => "None".to_string(),
        }
    }
}

impl BinaryOperator {
    pub fn to_python(&self) -> &'static str {
        match self {
            BinaryOperator::Add => "+",
            BinaryOperator::Sub => "-",
            BinaryOperator::Mul => "*",
            BinaryOperator::Div => "/",
            BinaryOperator::FloorDiv => "//",
            BinaryOperator::Mod => "%",
            BinaryOperator::Pow => "**",
            BinaryOperator::Eq => "==",
            BinaryOperator::Ne => "!=",
            BinaryOperator::Lt => "<",
            BinaryOperator::Le => "<=",
            BinaryOperator::Gt => ">",
            BinaryOperator::Ge => ">=",
            BinaryOperator::And => "and",
            BinaryOperator::Or => "or",
            BinaryOperator::BitAnd => "&",
            BinaryOperator::BitOr => "|",
            BinaryOperator::BitXor => "^",
            BinaryOperator::ShiftLeft => "<<",
            BinaryOperator::ShiftRight => ">>",
            BinaryOperator::In => "in",
            BinaryOperator::NotIn => "not in",
            BinaryOperator::Is => "is",
            BinaryOperator::IsNot => "is not",
        }
    }
}

impl UnaryOperator {
    pub fn to_python(&self) -> &'static str {
        match self {
            UnaryOperator::Pos => "+",
            UnaryOperator::Neg => "-",
            UnaryOperator::Not => "not ",
            UnaryOperator::BitNot => "~",
        }
    }
}

pub fn emit_module(module: &Module) -> String {
    let mut w = CodeWriter::new();
    module.emit(&mut w);
    w.into_string()
}
