use codeforge_emit::{CodeWriter, Emit};

use crate::ast::*;

fn assert_no_newlines(s: &str, field: &str) {
    assert!(
        !s.contains('\n') && !s.contains('\r'),
        "Directive {field} must not contain newlines: {s:?}"
    );
}

fn assert_valid_include_system(s: &str) {
    assert_no_newlines(s, "Include::System");
    assert!(
        !s.contains('<') && !s.contains('>'),
        "Include::System name must not contain '<' or '>': {s:?}"
    );
}

fn assert_valid_include_local(s: &str) {
    assert_no_newlines(s, "Include::Local");
    assert!(
        !s.contains('"'),
        "Include::Local name must not contain '\"': {s:?}"
    );
}

fn assert_valid_ident(s: &str, field: &str) {
    assert_no_newlines(s, field);
    assert!(
        !s.is_empty()
            && s.chars().all(|c| c.is_ascii_alphanumeric() || c == '_')
            && !s.as_bytes()[0].is_ascii_digit(),
        "{field} must be a valid C identifier: {s:?}"
    );
}

impl Emit for Program {
    fn emit(&self, w: &mut CodeWriter) {
        for directive in &self.directives {
            directive.emit(w);
        }

        if !self.directives.is_empty() {
            w.blank();
        }

        let items: Vec<&dyn Emit> = {
            let mut v: Vec<&dyn Emit> = Vec::new();
            for d in &self.declarations {
                v.push(d);
            }
            for n in &self.namespaces {
                v.push(n);
            }
            v
        };
        for (i, item) in items.iter().enumerate() {
            item.emit(w);
            if i < items.len() - 1 {
                w.blank();
            }
        }
    }
}

impl Emit for Namespace {
    fn emit(&self, w: &mut CodeWriter) {
        w.line(&format!("namespace {} {{", self.name));
        w.indent();
        for (i, decl) in self.declarations.iter().enumerate() {
            decl.emit(w);
            if i < self.declarations.len() - 1 {
                w.blank();
            }
        }
        w.dedent();
        w.line(&format!("}} // namespace {}", self.name));
    }
}

impl Emit for Declaration {
    fn emit(&self, w: &mut CodeWriter) {
        match self {
            Declaration::Function(f) => f.emit(w),
            Declaration::Class(c) => c.emit(w),
            Declaration::Struct(s) => s.emit(w),
            Declaration::Variable(v) => {
                w.write_indent();
                v.emit(w);
                w.write(";");
                w.newline();
            }
            Declaration::Enum(e) => e.emit(w),
            Declaration::Typedef(t) => t.emit(w),
            Declaration::Template(t) => t.emit(w),
            Declaration::Conditional(c) => c.emit(w),
        }
    }
}

impl Emit for Include {
    fn emit(&self, w: &mut CodeWriter) {
        match self {
            Include::System(name) => {
                assert_valid_include_system(name);
                w.writeln(&format!("#include <{}>", name));
            }
            Include::Local(name) => {
                assert_valid_include_local(name);
                w.writeln(&format!("#include \"{}\"", name));
            }
        }
    }
}

impl Emit for Directive {
    fn emit(&self, w: &mut CodeWriter) {
        match self {
            Directive::Include(include) => include.emit(w),
            Directive::Define { name, value } => {
                assert_valid_ident(name, "Directive::Define name");
                if let Some(val) = value {
                    assert_no_newlines(val, "Directive::Define value");
                    w.writeln(&format!("#define {} {}", name, val));
                } else {
                    w.writeln(&format!("#define {}", name));
                }
            }
            Directive::Undef(name) => {
                assert_valid_ident(name, "Directive::Undef");
                w.writeln(&format!("#undef {}", name));
            }
            Directive::Ifdef(name) => {
                assert_valid_ident(name, "Directive::Ifdef");
                w.writeln(&format!("#ifdef {}", name));
            }
            Directive::Ifndef(name) => {
                assert_valid_ident(name, "Directive::Ifndef");
                w.writeln(&format!("#ifndef {}", name));
            }
            Directive::Error(msg) => {
                assert_no_newlines(msg, "Directive::Error");
                w.writeln(&format!("#error {}", msg));
            }
            Directive::Pragma(pragma) => {
                assert_no_newlines(pragma, "Directive::Pragma");
                w.writeln(&format!("#pragma {}", pragma));
            }
            Directive::Conditional(c) => c.emit(w),
        }
    }
}

impl<T: Emit> Emit for Conditional<T> {
    fn emit(&self, w: &mut CodeWriter) {
        emit_conditional_block(self, w, |item, w| item.emit(w));
    }
}

fn emit_conditional_block<T, F>(c: &Conditional<T>, w: &mut CodeWriter, mut emit_item: F)
where
    F: FnMut(&T, &mut CodeWriter),
{
    assert_no_newlines(&c.condition, "Conditional::condition");
    w.writeln(&format!("#if {}", c.condition));
    for item in &c.body {
        emit_item(item, w);
    }
    for (elif_cond, elif_body) in &c.elif_branches {
        assert_no_newlines(elif_cond, "Conditional::elif_branches condition");
        w.writeln(&format!("#elif {}", elif_cond));
        for item in elif_body {
            emit_item(item, w);
        }
    }
    if let Some(else_body) = &c.else_body {
        w.writeln("#else");
        for item in else_body {
            emit_item(item, w);
        }
    }
    w.writeln("#endif");
}

impl Emit for Template {
    fn emit(&self, w: &mut CodeWriter) {
        let params: Vec<String> = self.parameters.iter().map(|p| p.to_cpp()).collect();
        w.write_indent();
        w.write(&format!("template <{}>", params.join(", ")));
        w.newline();
        self.declaration.emit(w);
    }
}

impl TemplateParameter {
    pub fn to_cpp(&self) -> String {
        match self {
            TemplateParameter::Type { name, default } => {
                let mut s = format!("typename {}", name);
                if let Some(def) = default {
                    s.push_str(&format!(" = {}", def.to_cpp()));
                }
                s
            }
            TemplateParameter::NonType {
                param_type,
                name,
                default,
            } => {
                let mut s = format!("{} {}", param_type.to_cpp(), name);
                if let Some(def) = default {
                    s.push_str(&format!(" = {}", def.to_cpp()));
                }
                s
            }
            TemplateParameter::Template {
                parameters,
                name,
                default,
            } => {
                let params: Vec<String> = parameters.iter().map(|p| p.to_cpp()).collect();
                let mut s = format!("template <{}> typename {}", params.join(", "), name);
                if let Some(def) = default {
                    s.push_str(&format!(" = {}", def.to_cpp()));
                }
                s
            }
        }
    }
}

impl Emit for Function {
    fn emit(&self, w: &mut CodeWriter) {
        w.write_indent();

        if self.is_inline {
            w.write("inline ");
        }
        if self.is_static {
            w.write("static ");
        }
        if self.is_virtual {
            w.write("virtual ");
        }

        w.write(&format!("{} {}(", self.return_type.to_cpp(), self.name));

        let params: Vec<String> = self
            .parameters
            .iter()
            .map(|p| {
                let mut s = format!("{} {}", p.param_type.to_cpp(), p.name);
                if let Some(def) = &p.default_value {
                    s.push_str(&format!(" = {}", def.to_cpp()));
                }
                s
            })
            .collect();

        w.write(&params.join(", "));
        w.write(")");

        if self.is_const {
            w.write(" const");
        }
        if self.is_override {
            w.write(" override");
        }
        if self.is_noexcept {
            w.write(" noexcept");
        }

        if self.is_pure_virtual {
            w.writeln(" = 0;");
        } else if let Some(body) = &self.body {
            w.writeln(" {");
            w.indent();
            for stmt in &body.statements {
                stmt.emit(w);
            }
            w.dedent();
            w.line("}");
        } else {
            w.writeln(";");
        }
    }
}

impl Emit for Class {
    fn emit(&self, w: &mut CodeWriter) {
        w.write_indent();
        w.write(&format!("class {}", self.name));

        if !self.base_classes.is_empty() {
            w.write(" : ");
            let bases: Vec<String> = self
                .base_classes
                .iter()
                .map(|b| {
                    let mut s = b.access.to_cpp().to_lowercase();
                    if b.is_virtual {
                        s.push_str(" virtual");
                    }
                    s.push(' ');
                    s.push_str(&b.name);
                    s
                })
                .collect();
            w.write(&bases.join(", "));
        }

        w.writeln(" {");
        w.indent();

        for member in &self.members {
            emit_class_member(w, &self.name, member);
        }

        w.dedent();
        w.write_indent();
        w.write("}");
        if self.is_final {
            w.write(" final");
        }
        w.writeln(";");
    }
}

fn emit_class_member(w: &mut CodeWriter, class_name: &str, member: &ClassMember) {
    match member {
        ClassMember::Access(access) => {
            w.line(&format!("{}:", access.to_cpp().to_lowercase()));
        }
        ClassMember::Field(f) => {
            f.emit(w);
            w.write(";");
            w.newline();
        }
        ClassMember::Method(f) => {
            f.emit(w);
        }
        ClassMember::Constructor(ctor) => {
            emit_constructor(w, class_name, ctor);
        }
        ClassMember::Destructor(dt) => {
            emit_destructor(w, class_name, dt);
        }
        ClassMember::Conditional(c) => {
            emit_conditional_class_members(w, class_name, c);
        }
    }
}

fn emit_conditional_class_members(
    w: &mut CodeWriter,
    class_name: &str,
    c: &Conditional<ClassMember>,
) {
    emit_conditional_block(c, w, |item, w| emit_class_member(w, class_name, item));
}

fn emit_constructor(w: &mut CodeWriter, class_name: &str, ctor: &Constructor) {
    w.write_indent();
    w.write(&format!("{}(", class_name));

    let params: Vec<String> = ctor
        .parameters
        .iter()
        .map(|p| format!("{} {}", p.param_type.to_cpp(), p.name))
        .collect();
    w.write(&params.join(", "));
    w.write(")");

    if !ctor.initializer_list.is_empty() {
        w.writeln(" :");
        w.indent();
        for (i, init) in ctor.initializer_list.iter().enumerate() {
            w.write_indent();
            w.write(&format!("{}({})", init.member_name, init.value.to_cpp()));
            if i < ctor.initializer_list.len() - 1 {
                w.write(",");
            }
            w.newline();
        }
        w.dedent();
    }

    w.write_indent();
    w.writeln("{");
    w.indent();
    for stmt in &ctor.body.statements {
        stmt.emit(w);
    }
    w.dedent();
    w.line("}");
}

fn emit_destructor(w: &mut CodeWriter, class_name: &str, dt: &Destructor) {
    w.write_indent();
    if dt.is_virtual {
        w.write("virtual ");
    }
    w.write(&format!("~{}()", class_name));
    if dt.is_defaulted {
        w.writeln(" = default;");
    } else if dt.is_deleted {
        w.writeln(" = delete;");
    } else {
        w.writeln(" {}");
    }
}

impl Emit for Struct {
    fn emit(&self, w: &mut CodeWriter) {
        w.line(&format!("struct {} {{", self.name));
        w.indent();
        for field in &self.fields {
            field.emit(w);
            w.write(";");
            w.newline();
        }
        w.dedent();
        w.line("};");
    }
}

impl Emit for Field {
    fn emit(&self, w: &mut CodeWriter) {
        w.write_indent();
        w.write(&format!("{} ", self.access.to_cpp().to_lowercase()));
        if self.is_static {
            w.write("static ");
        }
        if self.is_thread_local {
            w.write("thread_local ");
        }
        if self.is_const {
            w.write("const ");
        }
        w.write(&format!("{} {}", self.var_type.to_cpp(), self.name));
        if let Some(init) = &self.initializer {
            w.write(&format!(" = {}", init.to_cpp()));
        }
    }
}

impl Emit for LocalVariable {
    fn emit(&self, w: &mut CodeWriter) {
        if self.is_static {
            w.write("static ");
        }
        if self.is_thread_local {
            w.write("thread_local ");
        }
        if self.is_const {
            w.write("const ");
        }
        w.write(&format!("{} {}", self.var_type.to_cpp(), self.name));
        if let Some(init) = &self.initializer {
            w.write(&format!(" = {}", init.to_cpp()));
        }
    }
}

impl Emit for Enum {
    fn emit(&self, w: &mut CodeWriter) {
        w.write_indent();
        if self.is_scoped {
            w.write(&format!("enum class {}", self.name));
        } else {
            w.write(&format!("enum {}", self.name));
        }

        if let Some(ut) = &self.underlying_type {
            w.write(&format!(" : {}", ut.to_cpp()));
        }

        w.writeln(" {");
        w.indent();
        for (i, variant) in self.variants.iter().enumerate() {
            w.write_indent();
            w.write(&variant.name);
            if let Some(val) = &variant.value {
                w.write(&format!(" = {}", val.to_cpp()));
            }
            if i < self.variants.len() - 1 {
                w.writeln(",");
            } else {
                w.newline();
            }
        }
        w.dedent();
        w.line("};");
    }
}

impl Emit for Typedef {
    fn emit(&self, w: &mut CodeWriter) {
        w.line(&format!("using {} = {};", self.name, self.alias.to_cpp()));
    }
}

impl Emit for Statement {
    fn emit(&self, w: &mut CodeWriter) {
        match self {
            Statement::Expression(expr) => {
                w.line(&format!("{};", expr.to_cpp()));
            }
            Statement::Return(None) => {
                w.line("return;");
            }
            Statement::Return(Some(expr)) => {
                w.line(&format!("return {};", expr.to_cpp()));
            }
            Statement::If(if_stmt) => if_stmt.emit(w),
            Statement::While(while_stmt) => while_stmt.emit(w),
            Statement::For(for_stmt) => for_stmt.emit(w),
            Statement::VariableDeclaration(v) => {
                w.write_indent();
                v.emit(w);
                w.write(";");
                w.newline();
            }
            Statement::Break => {
                w.line("break;");
            }
            Statement::Continue => {
                w.line("continue;");
            }
            Statement::Comment(text) => {
                w.line(&format!("// {}", text));
            }
            Statement::Raw(text) => {
                w.line(text);
            }
            Statement::Conditional(c) => {
                c.emit(w);
            }
        }
    }
}

impl Emit for IfStatement {
    fn emit(&self, w: &mut CodeWriter) {
        w.write_indent();
        w.write(&format!("if ({}) {{", self.condition.to_cpp()));
        w.newline();
        w.indent();
        for s in &self.then_block.statements {
            s.emit(w);
        }
        w.dedent();
        if let Some(else_block) = &self.else_block {
            w.line("} else {");
            w.indent();
            for s in &else_block.statements {
                s.emit(w);
            }
            w.dedent();
            w.line("}");
        } else {
            w.line("}");
        }
    }
}

impl Emit for WhileStatement {
    fn emit(&self, w: &mut CodeWriter) {
        w.write_indent();
        w.write(&format!("while ({}) {{", self.condition.to_cpp()));
        w.newline();
        w.indent();
        for s in &self.body.statements {
            s.emit(w);
        }
        w.dedent();
        w.line("}");
    }
}

impl Emit for ForStatement {
    fn emit(&self, w: &mut CodeWriter) {
        w.write_indent();
        w.write("for (");
        if let Some(init) = &self.initializer {
            match init.as_ref() {
                Statement::VariableDeclaration(v) => {
                    v.emit(w);
                }
                other => {
                    other.emit(w);
                }
            }
        }
        w.write("; ");
        if let Some(cond) = &self.condition {
            w.write(&cond.to_cpp());
        }
        w.write("; ");
        if let Some(upd) = &self.update {
            w.write(&upd.to_cpp());
        }
        w.writeln(") {");
        w.indent();
        for s in &self.body.statements {
            s.emit(w);
        }
        w.dedent();
        w.line("}");
    }
}

impl Type {
    pub fn to_cpp(&self) -> String {
        match self {
            Type::Void => "void".to_string(),
            Type::Bool => "bool".to_string(),
            Type::Int8 => "int8_t".to_string(),
            Type::Int16 => "int16_t".to_string(),
            Type::Int32 => "int32_t".to_string(),
            Type::Int64 => "int64_t".to_string(),
            Type::UInt8 => "uint8_t".to_string(),
            Type::UInt16 => "uint16_t".to_string(),
            Type::UInt32 => "uint32_t".to_string(),
            Type::UInt64 => "uint64_t".to_string(),
            Type::Float32 => "float".to_string(),
            Type::Float64 => "double".to_string(),
            Type::Char => "char".to_string(),
            Type::String => "std::string".to_string(),
            Type::Custom(name) => name.clone(),
            Type::Pointer(inner) => format!("{}*", inner.to_cpp()),
            Type::Reference(inner) => format!("{}&", inner.to_cpp()),
            Type::ConstReference(inner) => format!("const {}&", inner.to_cpp()),
            Type::Array(inner, size) => match size {
                Some(s) => format!("{}[{}]", inner.to_cpp(), s),
                None => format!("{}[]", inner.to_cpp()),
            },
            Type::Template { name, arguments } => {
                let args: Vec<String> = arguments.iter().map(|a| a.to_cpp()).collect();
                format!("{}<{}>", name, args.join(", "))
            }
            Type::Auto => "auto".to_string(),
        }
    }
}

impl Expression {
    pub fn to_cpp(&self) -> String {
        match self {
            Expression::Literal(lit) => lit.to_cpp(),
            Expression::Identifier(name) => name.clone(),
            Expression::BinaryOp { left, op, right } => {
                format!("{} {} {}", left.to_cpp(), op.to_cpp(), right.to_cpp())
            }
            Expression::UnaryOp { op, operand } => {
                let op_str = op.to_cpp();
                if matches!(op, UnaryOperator::PostInc | UnaryOperator::PostDec) {
                    format!("{}{}", operand.to_cpp(), op_str)
                } else {
                    format!("{}{}", op_str, operand.to_cpp())
                }
            }
            Expression::Call { callee, arguments } => {
                let args: Vec<String> = arguments.iter().map(|a| a.to_cpp()).collect();
                format!("{}({})", callee.to_cpp(), args.join(", "))
            }
            Expression::MemberAccess {
                object,
                member,
                is_pointer,
            } => {
                let op = if *is_pointer { "->" } else { "." };
                format!("{}{}{}", object.to_cpp(), op, member)
            }
            Expression::ArrayAccess { array, index } => {
                format!("{}[{}]", array.to_cpp(), index.to_cpp())
            }
            Expression::Cast { target_type, expr } => {
                format!("static_cast<{}>({})", target_type.to_cpp(), expr.to_cpp())
            }
            Expression::Ternary {
                condition,
                then_expr,
                else_expr,
            } => {
                format!(
                    "{} ? {} : {}",
                    condition.to_cpp(),
                    then_expr.to_cpp(),
                    else_expr.to_cpp()
                )
            }
            Expression::Sizeof(t) => format!("sizeof({})", t.to_cpp()),
            Expression::Raw(text) => text.clone(),
        }
    }
}

impl Literal {
    pub fn to_cpp(&self) -> String {
        match self {
            Literal::Integer(i) => i.to_string(),
            Literal::Float(f) => f.0.to_string(),
            Literal::Boolean(b) => {
                if *b {
                    "true".to_string()
                } else {
                    "false".to_string()
                }
            }
            Literal::String(s) => {
                format!("\"{}\"", s.replace('\\', "\\\\").replace('\"', "\\\""))
            }
            Literal::Character(c) => format!("'{}'", c),
            Literal::Null => "nullptr".to_string(),
        }
    }
}

impl AccessSpecifier {
    pub fn to_cpp(&self) -> &'static str {
        match self {
            AccessSpecifier::Public => "public",
            AccessSpecifier::Protected => "protected",
            AccessSpecifier::Private => "private",
        }
    }
}

impl BinaryOperator {
    pub fn to_cpp(&self) -> &'static str {
        match self {
            BinaryOperator::Add => "+",
            BinaryOperator::Sub => "-",
            BinaryOperator::Mul => "*",
            BinaryOperator::Div => "/",
            BinaryOperator::Rem => "%",
            BinaryOperator::Eq => "==",
            BinaryOperator::Ne => "!=",
            BinaryOperator::Lt => "<",
            BinaryOperator::Le => "<=",
            BinaryOperator::Gt => ">",
            BinaryOperator::Ge => ">=",
            BinaryOperator::And => "&&",
            BinaryOperator::Or => "||",
            BinaryOperator::BitAnd => "&",
            BinaryOperator::BitOr => "|",
            BinaryOperator::BitXor => "^",
            BinaryOperator::ShiftLeft => "<<",
            BinaryOperator::ShiftRight => ">>",
            BinaryOperator::Assign => "=",
            BinaryOperator::AddAssign => "+=",
            BinaryOperator::SubAssign => "-=",
            BinaryOperator::MulAssign => "*=",
            BinaryOperator::DivAssign => "/=",
        }
    }
}

impl UnaryOperator {
    pub fn to_cpp(&self) -> &'static str {
        match self {
            UnaryOperator::Pos => "+",
            UnaryOperator::Neg => "-",
            UnaryOperator::Not => "!",
            UnaryOperator::BitNot => "~",
            UnaryOperator::PreInc => "++",
            UnaryOperator::PreDec => "--",
            UnaryOperator::PostInc => "++",
            UnaryOperator::PostDec => "--",
            UnaryOperator::Deref => "*",
            UnaryOperator::AddressOf => "&",
        }
    }
}

pub fn emit_program(program: &Program) -> String {
    let mut w = CodeWriter::new();
    program.emit(&mut w);
    w.into_string()
}
