use crate::ast::*;
use codeforge_emit::{CodeWriter, Emit};

impl Emit for Module {
    fn emit(&self, w: &mut CodeWriter) {
        for attr in &self.attributes {
            attr.emit(w);
        }
        if !self.attributes.is_empty() && !self.items.is_empty() {
            w.blank();
        }
        for (i, item) in self.items.iter().enumerate() {
            item.emit(w);
            if i < self.items.len() - 1 {
                let next = &self.items[i + 1];
                let needs_blank = !matches!((item, next), (Item::Use(_), Item::Use(_)));
                if needs_blank {
                    w.blank();
                }
            }
        }
    }
}

impl Emit for Attribute {
    fn emit(&self, w: &mut CodeWriter) {
        let prefix = if self.is_inner { "#!" } else { "#" };
        match &self.tokens {
            Some(tokens) => w.line(&format!("{}[{}({})]", prefix, self.path, tokens)),
            None => w.line(&format!("{}[{}]", prefix, self.path)),
        }
    }
}

impl Emit for Item {
    fn emit(&self, w: &mut CodeWriter) {
        match self {
            Item::Use(u) => emit_use(w, u),
            Item::Function(f) => f.emit(w),
            Item::Struct(s) => s.emit(w),
            Item::Enum(e) => e.emit(w),
            Item::Trait(t) => t.emit(w),
            Item::Impl(i) => i.emit(w),
            Item::TypeAlias(t) => t.emit(w),
            Item::Const(c) => c.emit(w),
            Item::Static(s) => s.emit(w),
            Item::Mod(m) => m.emit(w),
            Item::Raw(text) => w.line(text),
        }
    }
}

fn emit_visibility(vis: &Visibility, w: &mut CodeWriter) {
    match vis {
        Visibility::Private => {}
        Visibility::Public => w.write("pub "),
        Visibility::Crate => w.write("pub(crate) "),
        Visibility::Super => w.write("pub(super) "),
        Visibility::Restricted(path) => w.write(&format!("pub(in {}) ", path)),
    }
}

fn emit_visibility_str(vis: &Visibility) -> String {
    match vis {
        Visibility::Private => String::new(),
        Visibility::Public => "pub ".to_string(),
        Visibility::Crate => "pub(crate) ".to_string(),
        Visibility::Super => "pub(super) ".to_string(),
        Visibility::Restricted(path) => format!("pub(in {}) ", path),
    }
}

fn emit_attributes(attrs: &[Attribute], w: &mut CodeWriter) {
    for attr in attrs {
        attr.emit(w);
    }
}

fn emit_use(w: &mut CodeWriter, u: &Use) {
    w.write_indent();
    emit_visibility(&u.visibility, w);
    w.write("use ");
    emit_use_tree(w, &u.tree);
    w.writeln(";");
}

fn emit_use_tree(w: &mut CodeWriter, tree: &UseTree) {
    match tree {
        UseTree::Path(p) => w.write(p),
        UseTree::Alias { path, alias } => w.write(&format!("{} as {}", path, alias)),
        UseTree::Glob(prefix) => w.write(&format!("{}::*", prefix)),
        UseTree::Group { prefix, items } => {
            let trees: Vec<String> = items.iter().map(use_tree_to_string).collect();
            w.write(&format!("{}::{{{}}}", prefix, trees.join(", ")))
        }
    }
}

fn use_tree_to_string(tree: &UseTree) -> String {
    match tree {
        UseTree::Path(p) => p.clone(),
        UseTree::Alias { path, alias } => format!("{} as {}", path, alias),
        UseTree::Glob(prefix) => format!("{}::*", prefix),
        UseTree::Group { prefix, items } => {
            let trees: Vec<String> = items.iter().map(use_tree_to_string).collect();
            format!("{}::{{{}}}", prefix, trees.join(", "))
        }
    }
}

fn emit_generics_decl(w: &mut CodeWriter, generics: &Generics) {
    if generics.params.is_empty() {
        return;
    }
    w.write("<");
    let params: Vec<String> = generics
        .params
        .iter()
        .map(generic_param_to_string)
        .collect();
    w.write(&params.join(", "));
    w.write(">");
}

fn generic_param_to_string(param: &GenericParam) -> String {
    match param {
        GenericParam::Lifetime { name, bounds } => {
            if bounds.is_empty() {
                format!("'{}", name)
            } else {
                format!("'{}: {}", name, bounds.join(" + "))
            }
        }
        GenericParam::Type {
            name,
            bounds,
            default,
        } => {
            let mut s = name.clone();
            if !bounds.is_empty() {
                let bounds_str: Vec<String> = bounds.iter().map(|b| b.to_rust()).collect();
                s.push_str(&format!(": {}", bounds_str.join(" + ")));
            }
            if let Some(def) = default {
                s.push_str(&format!(" = {}", def.to_rust()));
            }
            s
        }
        GenericParam::Const { name, ty, default } => {
            let mut s = format!("const {}: {}", name, ty.to_rust());
            if let Some(def) = default {
                s.push_str(&format!(" = {}", def.to_rust()));
            }
            s
        }
    }
}

fn emit_where_clause(w: &mut CodeWriter, generics: &Generics) {
    if generics.where_clause.is_empty() {
        return;
    }
    w.newline();
    w.writeln("where");
    for (i, pred) in generics.where_clause.iter().enumerate() {
        w.indent();
        let bounds_str: Vec<String> = pred.bounds.iter().map(|b| b.to_rust()).collect();
        w.write_indent();
        w.write(&format!(
            "{}: {}",
            pred.ty.to_rust(),
            bounds_str.join(" + ")
        ));
        w.dedent();
        if i < generics.where_clause.len() - 1 {
            w.writeln(",");
        } else {
            w.newline();
        }
    }
}

impl Emit for Function {
    fn emit(&self, w: &mut CodeWriter) {
        emit_attributes(&self.attributes, w);
        w.write_indent();
        emit_visibility(&self.visibility, w);
        if self.is_unsafe {
            w.write("unsafe ");
        }
        if self.is_async {
            w.write("async ");
        }
        if self.is_const {
            w.write("const ");
        }
        if let Some(abi) = &self.abi {
            let escaped_abi = abi.replace('\\', "\\\\").replace('"', "\\\"");
            w.write(&format!("extern \"{}\" ", escaped_abi));
        }
        w.write("fn ");
        w.write(&self.name);
        emit_generics_decl(w, &self.generics);
        w.write("(");
        for (i, param) in self.parameters.iter().enumerate() {
            if i > 0 {
                w.write(", ");
            }
            match param {
                Parameter::Receiver(r) => {
                    if r.is_ref {
                        w.write("&");
                        if let Some(lt) = &r.lifetime {
                            w.write(&format!("'{} ", lt));
                        }
                        if r.is_mut {
                            w.write("mut ");
                        }
                        w.write("self");
                    } else if r.is_mut {
                        w.write("mut self");
                    } else {
                        w.write("self");
                    }
                }
                Parameter::Typed { pattern, ty } => {
                    w.write(&format!("{}: {}", pattern_to_rust(pattern), ty.to_rust()));
                }
            }
        }
        w.write(")");
        if let Some(ret) = &self.return_type {
            w.write(&format!(" -> {}", ret.to_rust()));
        }
        if !self.generics.where_clause.is_empty() {
            emit_where_clause(w, &self.generics);
        }
        if let Some(body) = &self.body {
            if self.generics.where_clause.is_empty() {
                w.writeln(" {");
            } else {
                w.writeln("{");
            }
            w.indent();
            emit_block_contents(w, body);
            w.dedent();
            w.line("}");
        } else {
            w.writeln(";");
        }
    }
}

fn emit_block_contents(w: &mut CodeWriter, block: &Block) {
    for stmt in &block.statements {
        stmt.emit(w);
    }
    if let Some(expr) = &block.trailing_expr {
        emit_trailing_expr(w, expr);
    }
}

fn emit_trailing_expr(w: &mut CodeWriter, expr: &Expression) {
    match expr {
        Expression::If(_)
        | Expression::Match { .. }
        | Expression::Loop { .. }
        | Expression::While { .. }
        | Expression::For { .. }
        | Expression::Block(_) => {
            emit_expr_stmt(w, expr);
        }
        Expression::Return(_) | Expression::Break { .. } | Expression::Continue { .. } => {
            w.line(&format!("{};", expr.to_rust()));
        }
        _ => {
            w.line(&expr.to_rust());
        }
    }
}

fn emit_expr_stmt(w: &mut CodeWriter, expr: &Expression) {
    match expr {
        Expression::If(_)
        | Expression::Match { .. }
        | Expression::Loop { .. }
        | Expression::While { .. }
        | Expression::For { .. }
        | Expression::Block(_) => match expr {
            Expression::If(ifexpr) => ifexpr.emit(w),
            Expression::Match { scrutinee, arms } => emit_match(w, scrutinee, arms),
            Expression::Loop { label, body } => emit_loop(w, label.as_deref(), body),
            Expression::While {
                label,
                condition,
                body,
            } => emit_while(w, label.as_deref(), condition, body),
            Expression::For {
                label,
                pattern,
                iter,
                body,
            } => emit_for(w, label.as_deref(), pattern, iter, body),
            Expression::Block(block) => {
                emit_block_expr(w, block);
            }
            _ => unreachable!(),
        },
        _ => {
            w.line(&format!("{};", expr.to_rust()));
        }
    }
}

impl Emit for Statement {
    fn emit(&self, w: &mut CodeWriter) {
        match self {
            Statement::Let(l) => {
                w.write_indent();
                w.write("let ");
                if l.is_mut {
                    w.write("mut ");
                }
                w.write(&pattern_to_rust(&l.pattern));
                if let Some(ty) = &l.ty {
                    w.write(&format!(": {}", ty.to_rust()));
                }
                if let Some(val) = &l.value {
                    w.write(&format!(" = {}", val.to_rust()));
                }
                if let Some(else_block) = &l.else_block {
                    w.writeln(" else {");
                    w.indent();
                    emit_block_contents(w, else_block);
                    w.dedent();
                    w.line("};");
                } else {
                    w.writeln(";");
                }
            }
            Statement::Expression(expr) => emit_expr_stmt(w, expr),
            Statement::Item(item) => {
                item.emit(w);
            }
            Statement::Comment(text) => {
                for line in text.lines() {
                    w.line(&format!("// {}", line));
                }
            }
            Statement::Raw(text) => {
                w.line(text);
            }
        }
    }
}

impl Emit for Struct {
    fn emit(&self, w: &mut CodeWriter) {
        emit_attributes(&self.attributes, w);
        w.write_indent();
        emit_visibility(&self.visibility, w);
        w.write(&format!("struct {}", self.name));
        emit_generics_decl(w, &self.generics);
        let has_where = !self.generics.where_clause.is_empty();
        if has_where {
            emit_where_clause(w, &self.generics);
        }
        match &self.kind {
            StructKind::Unit => {
                w.writeln(";");
            }
            StructKind::Tuple(fields) => {
                w.write("(");
                for (i, f) in fields.iter().enumerate() {
                    if i > 0 {
                        w.write(", ");
                    }
                    for attr in &f.attributes {
                        attr.emit(w);
                    }
                    w.write(&emit_visibility_str(&f.visibility));
                    w.write(&f.ty.to_rust());
                }
                w.writeln(");");
            }
            StructKind::Named(fields) => {
                if has_where {
                    w.writeln("{");
                } else {
                    w.writeln(" {");
                }
                w.indent();
                for field in fields {
                    for attr in &field.attributes {
                        attr.emit(w);
                    }
                    w.write_indent();
                    emit_visibility(&field.visibility, w);
                    w.writeln(&format!("{}: {},", field.name, field.ty.to_rust()));
                }
                w.dedent();
                w.line("}");
            }
        }
    }
}

impl Emit for Enum {
    fn emit(&self, w: &mut CodeWriter) {
        emit_attributes(&self.attributes, w);
        w.write_indent();
        emit_visibility(&self.visibility, w);
        w.write(&format!("enum {}", self.name));
        emit_generics_decl(w, &self.generics);
        if !self.generics.where_clause.is_empty() {
            emit_where_clause(w, &self.generics);
        }
        w.writeln(" {");
        w.indent();
        for variant in &self.variants {
            for attr in &variant.attributes {
                attr.emit(w);
            }
            w.write_indent();
            w.write(&variant.name);
            match &variant.kind {
                VariantKind::Unit => {}
                VariantKind::Tuple(types) => {
                    w.write("(");
                    for (i, ty) in types.iter().enumerate() {
                        if i > 0 {
                            w.write(", ");
                        }
                        w.write(&ty.to_rust());
                    }
                    w.write(")");
                }
                VariantKind::Named(fields) => {
                    w.writeln(" {");
                    w.indent();
                    for field in fields {
                        for attr in &field.attributes {
                            attr.emit(w);
                        }
                        w.write_indent();
                        w.writeln(&format!("{}: {},", field.name, field.ty.to_rust()));
                    }
                    w.dedent();
                    w.write_indent();
                    w.write("}");
                }
            }
            if let Some(disc) = &variant.discriminant {
                w.write(&format!(" = {}", disc.to_rust()));
            }
            w.writeln(",");
        }
        w.dedent();
        w.line("}");
    }
}

impl Emit for Trait {
    fn emit(&self, w: &mut CodeWriter) {
        emit_attributes(&self.attributes, w);
        w.write_indent();
        emit_visibility(&self.visibility, w);
        w.write(&format!("trait {}", self.name));
        emit_generics_decl(w, &self.generics);
        if !self.supertraits.is_empty() {
            w.write(": ");
            let bounds: Vec<String> = self.supertraits.iter().map(|t| t.to_rust()).collect();
            w.write(&bounds.join(" + "));
        }
        if !self.generics.where_clause.is_empty() {
            emit_where_clause(w, &self.generics);
        }
        w.writeln(" {");
        w.indent();
        for (i, item) in self.items.iter().enumerate() {
            if i > 0 {
                w.blank();
            }
            emit_assoc_item(w, item);
        }
        w.dedent();
        w.line("}");
    }
}

fn emit_assoc_item(w: &mut CodeWriter, item: &AssocItem) {
    match item {
        AssocItem::Function(f) => f.emit(w),
        AssocItem::Const(c) => {
            w.write_indent();
            emit_visibility(&c.visibility, w);
            w.write(&format!("const {}: {}", c.name, c.ty.to_rust()));
            if let Some(val) = &c.value {
                w.write(&format!(" = {}", val.to_rust()));
            }
            w.writeln(";");
        }
        AssocItem::Type(at) => {
            for attr in &at.attributes {
                attr.emit(w);
            }
            w.write_indent();
            w.write(&format!("type {}", at.name));
            emit_generics_decl(w, &at.generics);
            if !at.bounds.is_empty() {
                w.write(": ");
                let bounds: Vec<String> = at.bounds.iter().map(|t| t.to_rust()).collect();
                w.write(&bounds.join(" + "));
            }
            if let Some(val) = &at.value {
                w.write(&format!(" = {}", val.to_rust()));
            }
            w.writeln(";");
        }
        AssocItem::Raw(text) => w.line(text),
    }
}

impl Emit for Impl {
    fn emit(&self, w: &mut CodeWriter) {
        emit_attributes(&self.attributes, w);
        w.write_indent();
        if self.is_unsafe {
            w.write("unsafe ");
        }
        w.write("impl");
        emit_generics_decl(w, &self.generics);
        w.write(" ");
        if let Some(trait_) = &self.trait_ {
            w.write(&format!("{} for ", trait_.to_rust()));
        }
        w.write(&self.self_ty.to_rust());
        if !self.generics.where_clause.is_empty() {
            emit_where_clause(w, &self.generics);
        }
        if self.items.is_empty() {
            if self.generics.where_clause.is_empty() {
                w.writeln(" {}");
            } else {
                w.writeln("{}");
            }
        } else {
            if self.generics.where_clause.is_empty() {
                w.writeln(" {");
            } else {
                w.writeln("{");
            }
            w.indent();
            for (i, item) in self.items.iter().enumerate() {
                if i > 0 {
                    w.blank();
                }
                emit_assoc_item(w, item);
            }
            w.dedent();
            w.line("}");
        }
    }
}

impl Emit for TypeAlias {
    fn emit(&self, w: &mut CodeWriter) {
        emit_attributes(&self.attributes, w);
        w.write_indent();
        emit_visibility(&self.visibility, w);
        w.write(&format!("type {}", self.name));
        emit_generics_decl(w, &self.generics);
        w.writeln(&format!(" = {};", self.ty.to_rust()));
    }
}

impl Emit for Const {
    fn emit(&self, w: &mut CodeWriter) {
        emit_attributes(&self.attributes, w);
        w.write_indent();
        emit_visibility(&self.visibility, w);
        w.write(&format!("const {}: {}", self.name, self.ty.to_rust()));
        if let Some(val) = &self.value {
            w.write(&format!(" = {}", val.to_rust()));
        }
        w.writeln(";");
    }
}

impl Emit for Static {
    fn emit(&self, w: &mut CodeWriter) {
        emit_attributes(&self.attributes, w);
        w.write_indent();
        emit_visibility(&self.visibility, w);
        w.write(&format!(
            "static {}{}: {}",
            if self.is_mut { "mut " } else { "" },
            self.name,
            self.ty.to_rust()
        ));
        w.writeln(&format!(" = {};", self.value.to_rust()));
    }
}

impl Emit for Mod {
    fn emit(&self, w: &mut CodeWriter) {
        emit_attributes(&self.attributes, w);
        w.write_indent();
        emit_visibility(&self.visibility, w);
        w.write(&format!("mod {}", self.name));
        match &self.items {
            Some(items) => {
                w.writeln(" {");
                w.indent();
                for (i, item) in items.iter().enumerate() {
                    item.emit(w);
                    if i < items.len() - 1 {
                        let next = &items[i + 1];
                        let needs_blank = !matches!((item, next), (Item::Use(_), Item::Use(_)));
                        if needs_blank {
                            w.blank();
                        }
                    }
                }
                w.dedent();
                w.line("}");
            }
            None => {
                w.writeln(";");
            }
        }
    }
}

fn emit_block_expr(w: &mut CodeWriter, block: &Block) {
    if block.statements.is_empty() && block.trailing_expr.is_none() {
        w.write("{}");
        return;
    }
    w.writeln("{");
    w.indent();
    emit_block_contents(w, block);
    w.dedent();
    w.write_indent();
    w.write("}");
}

fn emit_match(w: &mut CodeWriter, scrutinee: &Expression, arms: &[MatchArm]) {
    w.write_indent();
    w.write(&format!("match {} {{", scrutinee.to_rust()));
    w.newline();
    w.indent();
    for arm in arms {
        w.write_indent();
        w.write(&pattern_to_rust(&arm.pattern));
        if let Some(guard) = &arm.guard {
            w.write(&format!(" if {}", guard.to_rust()));
        }
        w.write(" => ");
        match &arm.body {
            Expression::Block(block) => {
                emit_block_expr(w, block);
                w.write(",");
            }
            expr => {
                w.write(&format!("{},", expr.to_rust()));
            }
        }
        w.newline();
    }
    w.dedent();
    w.line("}");
}

fn emit_loop(w: &mut CodeWriter, label: Option<&str>, body: &Block) {
    w.write_indent();
    if let Some(l) = label {
        w.write(&format!("'{}: ", l));
    }
    w.writeln("loop {");
    w.indent();
    emit_block_contents(w, body);
    w.dedent();
    w.line("}");
}

fn emit_while(w: &mut CodeWriter, label: Option<&str>, condition: &IfCondition, body: &Block) {
    w.write_indent();
    if let Some(l) = label {
        w.write(&format!("'{}: ", l));
    }
    w.write("while ");
    match condition {
        IfCondition::Expr(expr) => {
            w.write(&expr.to_rust());
            w.writeln(" {");
        }
        IfCondition::Let { pattern, value } => {
            w.writeln(&format!(
                "let {} = {} {{",
                pattern_to_rust(pattern),
                value.to_rust()
            ));
        }
    }
    w.indent();
    emit_block_contents(w, body);
    w.dedent();
    w.line("}");
}

fn emit_for(
    w: &mut CodeWriter,
    label: Option<&str>,
    pattern: &Pattern,
    iter: &Expression,
    body: &Block,
) {
    w.write_indent();
    if let Some(l) = label {
        w.write(&format!("'{}: ", l));
    }
    w.writeln(&format!(
        "for {} in {} {{",
        pattern_to_rust(pattern),
        iter.to_rust()
    ));
    w.indent();
    emit_block_contents(w, body);
    w.dedent();
    w.line("}");
}

impl Pattern {
    pub fn to_rust_str(&self) -> String {
        pattern_to_rust(self)
    }
}

impl Emit for IfExpr {
    fn emit(&self, w: &mut CodeWriter) {
        w.write_indent();
        emit_if_core(w, self);
    }
}

fn emit_if_core(w: &mut CodeWriter, if_expr: &IfExpr) {
    w.write("if ");
    match &if_expr.condition {
        IfCondition::Expr(expr) => {
            w.write(&expr.to_rust());
        }
        IfCondition::Let { pattern, value } => {
            w.write(&format!(
                "let {} = {}",
                pattern_to_rust(pattern),
                value.to_rust()
            ));
        }
    }
    w.writeln(" {");
    w.indent();
    emit_block_contents(w, &if_expr.then_block);
    w.dedent();
    if let Some(else_branch) = &if_expr.else_branch {
        w.write_indent();
        match else_branch {
            ElseBranch::Block(block) => {
                w.writeln("} else {");
                w.indent();
                emit_block_contents(w, block);
                w.dedent();
                w.line("}");
            }
            ElseBranch::If(if_expr) => {
                w.write("} else ");
                emit_if_core(w, if_expr);
            }
        }
    } else {
        w.line("}");
    }
}

impl Type {
    pub fn to_rust(&self) -> String {
        match self {
            Type::Unit => "()".to_string(),
            Type::Bool => "bool".to_string(),
            Type::Char => "char".to_string(),
            Type::Str => "str".to_string(),
            Type::I8 => "i8".to_string(),
            Type::I16 => "i16".to_string(),
            Type::I32 => "i32".to_string(),
            Type::I64 => "i64".to_string(),
            Type::I128 => "i128".to_string(),
            Type::Isize => "isize".to_string(),
            Type::U8 => "u8".to_string(),
            Type::U16 => "u16".to_string(),
            Type::U32 => "u32".to_string(),
            Type::U64 => "u64".to_string(),
            Type::U128 => "u128".to_string(),
            Type::Usize => "usize".to_string(),
            Type::F32 => "f32".to_string(),
            Type::F64 => "f64".to_string(),
            Type::Path(p) => path_to_rust(p),
            Type::Reference {
                lifetime,
                is_mut,
                inner,
            } => {
                let mut s = "&".to_string();
                if let Some(lt) = lifetime {
                    s.push_str(&format!("'{} ", lt));
                }
                if *is_mut {
                    s.push_str("mut ");
                }
                s.push_str(&inner.to_rust());
                s
            }
            Type::Pointer { is_mut, inner } => {
                let kw = if *is_mut { "mut" } else { "const" };
                format!("*{} {}", kw, inner.to_rust())
            }
            Type::Tuple(types) => {
                let parts: Vec<String> = types.iter().map(|t| t.to_rust()).collect();
                format!("({})", parts.join(", "))
            }
            Type::Slice(inner) => format!("[{}]", inner.to_rust()),
            Type::Array(inner, count) => format!("[{}; {}]", inner.to_rust(), count.to_rust()),
            Type::TraitObject(bounds) => {
                let parts: Vec<String> = bounds.iter().map(|t| t.to_rust()).collect();
                format!("dyn {}", parts.join(" + "))
            }
            Type::ImplTrait(bounds) => {
                let parts: Vec<String> = bounds.iter().map(|t| t.to_rust()).collect();
                format!("impl {}", parts.join(" + "))
            }
            Type::Fn {
                params,
                return_type,
            } => {
                let params_str: Vec<String> = params.iter().map(|t| t.to_rust()).collect();
                let mut s = format!("fn({})", params_str.join(", "));
                if let Some(ret) = return_type {
                    s.push_str(&format!(" -> {}", ret.to_rust()));
                }
                s
            }
            Type::Infer => "_".to_string(),
            Type::SelfType => "Self".to_string(),
            Type::Raw(s) => s.clone(),
        }
    }
}

fn path_to_rust(path: &Path) -> String {
    let segments: Vec<String> = path
        .segments
        .iter()
        .map(|seg| {
            let mut s = seg.name.clone();
            if !seg.args.is_empty() {
                let args: Vec<String> = seg.args.iter().map(generic_arg_to_rust).collect();
                s.push_str(&format!("<{}>", args.join(", ")));
            }
            s
        })
        .collect();
    segments.join("::")
}

fn generic_arg_to_rust(arg: &GenericArg) -> String {
    match arg {
        GenericArg::Lifetime(lt) => format!("'{}", lt),
        GenericArg::Type(ty) => ty.to_rust(),
        GenericArg::Binding { name, ty } => format!("{} = {}", name, ty.to_rust()),
    }
}

impl Expression {
    pub fn to_rust(&self) -> String {
        match self {
            Expression::Literal(lit) => lit_to_rust(lit),
            Expression::Path(p) => path_to_rust(p),
            Expression::Binary { left, op, right } => {
                format!(
                    "{} {} {}",
                    left.to_rust(),
                    op_to_rust_str(op),
                    right.to_rust()
                )
            }
            Expression::Unary { op, operand } => {
                format!("{}{}", op_to_rust_str_unary(op), operand.to_rust())
            }
            Expression::Call { callee, args } => {
                let args_str: Vec<String> = args.iter().map(|a| a.to_rust()).collect();
                format!("{}({})", callee.to_rust(), args_str.join(", "))
            }
            Expression::MethodCall {
                receiver,
                method,
                turbofish,
                args,
            } => {
                let mut s = format!("{}.", receiver.to_rust());
                s.push_str(method);
                if !turbofish.is_empty() {
                    let args: Vec<String> = turbofish.iter().map(generic_arg_to_rust).collect();
                    s.push_str(&format!("::<{}>", args.join(", ")));
                }
                let meth_args: Vec<String> = args.iter().map(|a| a.to_rust()).collect();
                s.push_str(&format!("({})", meth_args.join(", ")));
                s
            }
            Expression::Field { base, name } => {
                format!("{}.{}", base.to_rust(), name)
            }
            Expression::Index { base, index } => {
                format!("{}[{}]", base.to_rust(), index.to_rust())
            }
            Expression::Reference { is_mut, inner } => {
                if *is_mut {
                    format!("&mut {}", inner.to_rust())
                } else {
                    format!("&{}", inner.to_rust())
                }
            }
            Expression::Deref(inner) => {
                format!("*{}", inner.to_rust())
            }
            Expression::Try(inner) => {
                format!("{}?", inner.to_rust())
            }
            Expression::Cast { expr, ty } => {
                format!("{} as {}", expr.to_rust(), ty.to_rust())
            }
            Expression::Tuple(exprs) => {
                let parts: Vec<String> = exprs.iter().map(|e| e.to_rust()).collect();
                format!("({})", parts.join(", "))
            }
            Expression::Array(exprs) => {
                let parts: Vec<String> = exprs.iter().map(|e| e.to_rust()).collect();
                format!("[{}]", parts.join(", "))
            }
            Expression::Repeat { value, count } => {
                format!("[{}; {}]", value.to_rust(), count.to_rust())
            }
            Expression::StructLiteral { path, fields, rest } => {
                let mut parts: Vec<String> = fields
                    .iter()
                    .map(|f| match &f.value {
                        Some(val) => format!("{}: {}", f.name, val.to_rust()),
                        None => f.name.clone(),
                    })
                    .collect();
                if let Some(r) = rest {
                    parts.push(format!("..{}", r.to_rust()));
                }
                format!("{} {{ {} }}", path_to_rust(path), parts.join(", "))
            }
            Expression::Closure {
                is_move,
                params,
                return_type,
                body,
            } => {
                let mut s = String::new();
                if *is_move {
                    s.push_str("move ");
                }
                let params_str: Vec<String> = params
                    .iter()
                    .map(|p| {
                        let mut ps = pattern_to_rust(&p.pattern);
                        if let Some(ty) = &p.ty {
                            ps.push_str(&format!(": {}", ty.to_rust()));
                        }
                        ps
                    })
                    .collect();
                s.push_str(&format!("|{}|", params_str.join(", ")));
                if let Some(ret) = return_type {
                    s.push_str(&format!(" -> {}", ret.to_rust()));
                }
                s.push_str(&format!(" {}", body.to_rust()));
                s
            }
            Expression::If(_) => "// multi-line if expression — use Emit".to_string(),
            Expression::Match { .. } => "// multi-line match expression — use Emit".to_string(),
            Expression::Loop { .. } => "// multi-line loop expression — use Emit".to_string(),
            Expression::While { .. } => "// multi-line while expression — use Emit".to_string(),
            Expression::For { .. } => "// multi-line for expression — use Emit".to_string(),
            Expression::Block(_) => "// multi-line block expression — use Emit".to_string(),
            Expression::Return(None) => "return".to_string(),
            Expression::Return(Some(val)) => format!("return {}", val.to_rust()),
            Expression::Break { label, value } => {
                let mut s = "break".to_string();
                if let Some(l) = label {
                    s.push_str(&format!(" '{}", l));
                }
                if let Some(v) = value {
                    s.push_str(&format!(" {}", v.to_rust()));
                }
                s
            }
            Expression::Continue { label } => {
                let mut s = "continue".to_string();
                if let Some(l) = label {
                    s.push_str(&format!(" '{}", l));
                }
                s
            }
            Expression::Range {
                start,
                end,
                inclusive,
            } => {
                let mut s = String::new();
                if let Some(start_expr) = start {
                    s.push_str(&start_expr.to_rust());
                }
                s.push_str(if *inclusive { "..=" } else { ".." });
                if let Some(end_expr) = end {
                    s.push_str(&end_expr.to_rust());
                }
                s
            }
            Expression::MacroCall { path, tokens } => {
                if path.ends_with('!') {
                    format!("{}({})", path, tokens)
                } else {
                    format!("{}!({})", path, tokens)
                }
            }
            Expression::Raw(text) => text.clone(),
        }
    }
}

fn lit_to_rust(lit: &Literal) -> String {
    match lit {
        Literal::Integer(i) => i.to_string(),
        Literal::UnsignedInteger(u) => format!("{}u64", u),
        Literal::Float(f) => {
            let v = f.0;
            if v.is_nan() {
                "f64::NAN".to_string()
            } else if v.is_infinite() {
                if v.is_sign_positive() {
                    "f64::INFINITY".to_string()
                } else {
                    "f64::NEG_INFINITY".to_string()
                }
            } else {
                let mut s = v.to_string();
                if !s.contains('.') && !s.contains('e') && !s.contains('E') {
                    s.push_str(".0");
                }
                s
            }
        }
        Literal::Boolean(true) => "true".to_string(),
        Literal::Boolean(false) => "false".to_string(),
        Literal::String(s) => {
            let escaped: String = s
                .chars()
                .flat_map(|c| match c {
                    '\\' => "\\\\".chars().collect::<Vec<_>>(),
                    '"' => "\\\"".chars().collect::<Vec<_>>(),
                    '\n' => "\\n".chars().collect::<Vec<_>>(),
                    '\r' => "\\r".chars().collect::<Vec<_>>(),
                    '\t' => "\\t".chars().collect::<Vec<_>>(),
                    '\0' => "\\0".chars().collect::<Vec<_>>(),
                    c if c.is_control() => {
                        format!("\\u{{{:x}}}", c as u32).chars().collect::<Vec<_>>()
                    }
                    c => vec![c],
                })
                .collect();
            format!("\"{}\"", escaped)
        }
        Literal::Char(c) => {
            let escaped = match c {
                '\\' => "\\\\".to_string(),
                '\'' => "\\'".to_string(),
                '\n' => "\\n".to_string(),
                '\r' => "\\r".to_string(),
                '\t' => "\\t".to_string(),
                '\0' => "\\0".to_string(),
                c if c.is_control() => format!("\\u{{{:x}}}", *c as u32),
                c => c.to_string(),
            };
            format!("\'{}\'", escaped)
        }
        Literal::ByteString(bytes) => {
            let escaped: String = bytes
                .iter()
                .map(|&b| match b {
                    b'\\' => "\\\\".to_string(),
                    b'"' => "\\\"".to_string(),
                    b'\n' => "\\n".to_string(),
                    b'\r' => "\\r".to_string(),
                    b'\t' => "\\t".to_string(),
                    0x20..=0x7e => (b as char).to_string(),
                    _ => format!("\\x{:02x}", b),
                })
                .collect();
            format!("b\"{}\"", escaped)
        }
        Literal::Raw(text) => text.clone(),
    }
}

fn op_to_rust_str(op: &BinaryOperator) -> &'static str {
    match op {
        BinaryOperator::Add => "+",
        BinaryOperator::Sub => "-",
        BinaryOperator::Mul => "*",
        BinaryOperator::Div => "/",
        BinaryOperator::Rem => "%",
        BinaryOperator::And => "&&",
        BinaryOperator::Or => "||",
        BinaryOperator::BitAnd => "&",
        BinaryOperator::BitOr => "|",
        BinaryOperator::BitXor => "^",
        BinaryOperator::Shl => "<<",
        BinaryOperator::Shr => ">>",
        BinaryOperator::Eq => "==",
        BinaryOperator::Ne => "!=",
        BinaryOperator::Lt => "<",
        BinaryOperator::Le => "<=",
        BinaryOperator::Gt => ">",
        BinaryOperator::Ge => ">=",
        BinaryOperator::Assign => "=",
        BinaryOperator::AddAssign => "+=",
        BinaryOperator::SubAssign => "-=",
        BinaryOperator::MulAssign => "*=",
        BinaryOperator::DivAssign => "/=",
        BinaryOperator::RemAssign => "%=",
        BinaryOperator::BitAndAssign => "&=",
        BinaryOperator::BitOrAssign => "|=",
        BinaryOperator::BitXorAssign => "^=",
        BinaryOperator::ShlAssign => "<<=",
        BinaryOperator::ShrAssign => ">>=",
    }
}

fn op_to_rust_str_unary(op: &UnaryOperator) -> &'static str {
    match op {
        UnaryOperator::Neg => "-",
        UnaryOperator::Not => "!",
    }
}

fn pattern_to_rust(pattern: &Pattern) -> String {
    match pattern {
        Pattern::Wildcard => "_".to_string(),
        Pattern::Rest => "..".to_string(),
        Pattern::Ident {
            name,
            is_ref,
            is_mut,
            subpattern,
        } => {
            let mut s = String::new();
            if *is_ref {
                s.push_str("ref ");
            }
            if *is_mut {
                s.push_str("mut ");
            }
            s.push_str(name);
            if let Some(sub) = subpattern {
                s.push_str(&format!(" @ {}", pattern_to_rust(sub)));
            }
            s
        }
        Pattern::Literal(lit) => lit_to_rust(lit),
        Pattern::Tuple(pats) => {
            let parts: Vec<String> = pats.iter().map(pattern_to_rust).collect();
            format!("({})", parts.join(", "))
        }
        Pattern::Slice(pats) => {
            let parts: Vec<String> = pats.iter().map(pattern_to_rust).collect();
            format!("[{}]", parts.join(", "))
        }
        Pattern::TupleStruct { path, elems } => {
            let parts: Vec<String> = elems.iter().map(pattern_to_rust).collect();
            format!("{}({})", path_to_rust(path), parts.join(", "))
        }
        Pattern::Struct {
            path,
            fields,
            has_rest,
        } => {
            let parts: Vec<String> = fields
                .iter()
                .map(|f| match &f.pattern {
                    Some(p) => format!("{}: {}", f.name, pattern_to_rust(p)),
                    None => f.name.clone(),
                })
                .collect();
            if *has_rest {
                let mut s = format!("{} {{ ", path_to_rust(path));
                s.push_str(&parts.join(", "));
                if !parts.is_empty() {
                    s.push_str(", ");
                }
                s.push_str(".. }");
                s
            } else {
                format!("{} {{ {} }}", path_to_rust(path), parts.join(", "))
            }
        }
        Pattern::Or(pats) => {
            let parts: Vec<String> = pats.iter().map(pattern_to_rust).collect();
            parts.join(" | ")
        }
        Pattern::Reference { is_mut, inner } => {
            if *is_mut {
                format!("&mut {}", pattern_to_rust(inner))
            } else {
                format!("&{}", pattern_to_rust(inner))
            }
        }
        Pattern::Path(path) => path_to_rust(path),
        Pattern::Raw(text) => text.clone(),
    }
}

pub fn emit_module(module: &Module) -> String {
    let mut w = CodeWriter::new();
    module.emit(&mut w);
    w.into_string()
}
