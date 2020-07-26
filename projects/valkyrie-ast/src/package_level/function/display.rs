use super::*;

impl AsRef<str> for FunctionType {
    fn as_ref(&self) -> &str {
        match self {
            FunctionType::Macro => "macro",
            FunctionType::Micro => "micro",
        }
    }
}

impl IndentDisplay for FunctionDeclarationNode {
    fn indent_fmt(&self, f: &mut IndentFormatter) -> core::fmt::Result {
        for m in &self.modifiers {
            write!(f, "{} ", m)?;
        }
        f.write_str(self.r#type.as_ref())?;
        write!(f, " {}", self.namepath)?;
        if let Some(s) = &self.generic {
            Display::fmt(s, f.borrow_mut())?
        }
        f.write_char('(')?;
        comma_terms(f.borrow_mut(), &self.arguments.terms)?;
        f.write_char(')')?;
        if let Some(ret) = &self.r#return {
            write!(f, ": {ret}")?
        }
        if let Some(s) = &self.body {
            f.write_str(" {")?;
            f.indent();
            let count = s.len();
            for (index, term) in s.iter().enumerate() {
                f.write_newline()?;
                term.r#type.indent_fmt(f)?;
                if index != count.saturating_sub(1) {
                    f.write_str(";")?;
                }
                else {
                    if term.end_semicolon {
                        f.write_str(";")?;
                    }
                }
            }
            f.dedent();
            f.write_newline()?;
            f.write_char('}')?;
        }
        Ok(())
    }
}

impl<K: Display, V: Display, D: Display> IndentDisplay for ArgumentTermNode<K, V, D> {
    fn indent_fmt(&self, f: &mut IndentFormatter) -> core::fmt::Result {
        write!(f, "{}", self.key)?;
        if let Some(value) = &self.value {
            write!(f, ": {}", value)?;
        }
        if let Some(default) = &self.default {
            write!(f, " = {}", default)?;
        }
        Ok(())
    }
}

impl Display for FunctionType {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            FunctionType::Macro => f.write_str("macro"),
            FunctionType::Micro => f.write_str("micro"),
        }
    }
}

impl Display for FunctionDeclarationNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        IndentFormatter::wrap(self, f)
    }
}
