pub struct CodeWriter {
    buf: String,
    indent_level: usize,
    indent_str: String,
}

impl CodeWriter {
    pub fn new() -> Self {
        Self {
            buf: String::new(),
            indent_level: 0,
            indent_str: String::from("    "),
        }
    }

    pub fn with_indent(indent: &str) -> Self {
        Self {
            buf: String::new(),
            indent_level: 0,
            indent_str: indent.to_string(),
        }
    }

    pub fn indent(&mut self) {
        self.indent_level += 1;
    }

    pub fn dedent(&mut self) {
        self.indent_level = self.indent_level.saturating_sub(1);
    }

    pub fn write_indent(&mut self) {
        for _ in 0..self.indent_level {
            self.buf.push_str(&self.indent_str);
        }
    }

    pub fn write(&mut self, s: &str) {
        self.buf.push_str(s);
    }

    pub fn writeln(&mut self, s: &str) {
        self.buf.push_str(s);
        self.buf.push('\n');
    }

    pub fn line(&mut self, s: &str) {
        self.write_indent();
        self.writeln(s);
    }

    pub fn blank(&mut self) {
        self.buf.push('\n');
    }

    pub fn newline(&mut self) {
        self.buf.push('\n');
    }

    pub fn into_string(self) -> String {
        self.buf
    }

    pub fn as_str(&self) -> &str {
        &self.buf
    }
}

impl Default for CodeWriter {
    fn default() -> Self {
        Self::new()
    }
}

pub trait Emit {
    fn emit(&self, w: &mut CodeWriter);
}
