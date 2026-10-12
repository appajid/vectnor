//! English interface labels and message formatting.
//!
//! Vectnor ships one English interface. These helpers retain the shared widget API;
//! document text, Unicode input, script shaping and font selection are independent.

/// The single supported interface language.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Lang;

impl Lang {
    pub const EN: Self = Self;

    pub fn code(self) -> &'static str {
        "en"
    }
    pub fn name(self) -> &'static str {
        "English"
    }
    pub fn all() -> impl Iterator<Item = Self> {
        std::iter::once(Self::EN)
    }
}

pub fn current() -> Lang {
    Lang::EN
}
pub fn t(s: &str) -> &str {
    s
}
pub fn tr(_lang: Lang, s: &str) -> &str {
    s
}
pub fn tr_ctx<'a>(_lang: Lang, _context: &str, s: &'a str) -> &'a str {
    s
}
pub fn tr_id<'a>(_lang: Lang, _id: &str, label: &'a str) -> &'a str {
    label
}
pub fn t_id<'a>(_id: &str, label: &'a str) -> &'a str {
    label
}

/// Fill `{name}` placeholders. Unknown placeholders are left as written.
pub fn fmt(template: &str, args: &[(&str, &str)]) -> String {
    let mut out = template.to_string();
    for (k, v) in args {
        out = out.replace(&format!("{{{k}}}"), v);
    }
    out
}

/// English singular and plural forms, with `{n}` filled with the count.
pub fn trn(_lang: Lang, n: u64, one: &str, other: &str) -> String {
    fmt(if n == 1 { one } else { other }, &[("n", &n.to_string())])
}
pub fn tn(n: u64, one: &str, other: &str) -> String {
    trn(Lang::EN, n, one, other)
}

/// Preserve both English built-in labels and user-provided Unicode names.
pub fn label_or_name(_lang: Lang, s: &str, _built_in: bool) -> &str {
    s
}
pub fn message(_lang: Lang, s: &str) -> String {
    s.to_string()
}
pub fn msg(s: &str) -> String {
    s.to_string()
}

#[cfg(test)]
mod tests;
