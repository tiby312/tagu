///
/// Writer adaptor that disallows escaping from xml.
///
pub fn escape_guard<T: std::fmt::Write>(a: T) -> EscapeGuard<T> {
    EscapeGuard::new(a)
}

/// Writer adaptor that replaces xml escaping characters with their encoded value.
///
/// Disallowed characters are `"` `'` `<` `>` `&`. characters are replaced with their equivalent from:
/// [https://dev.w3.org/html5/html-author/charref](https://dev.w3.org/html5/html-author/charref)
///
pub struct EscapeGuard<T> {
    writer: T,
}

impl<T: std::fmt::Write> EscapeGuard<T> {
    pub fn new(writer: T) -> EscapeGuard<T> {
        EscapeGuard { writer }
    }
}

impl<T: std::fmt::Write> std::fmt::Write for EscapeGuard<T> {
    fn write_str(&mut self, s: &str) -> Result<(), std::fmt::Error> {
        for c in s.chars() {
            let r = match c {
                '\"' => Some("&quot;"),
                '\'' => Some("&apos;"),
                '<' => Some("&lt;"),
                '>' => Some("&gt;"),
                '&' => Some("&amp;"),
                _ => None,
            };

            if let Some(r) = r {
                self.writer.write_str(r)?;
            } else {
                self.writer.write_char(c)?;
            }
        }
        Ok(())
    }
}

///
/// Used to wrap a `std::io::Write` to have `std::io::Write`.
/// The underlying error can be extracted through the error field.
///
pub struct Adaptor<T> {
    pub inner: T,
    pub error: Result<(), std::io::Error>,
}

///Update a `std::io::Write` to be a `std::fmt::Write`
pub fn upgrade_write<T: std::io::Write>(inner: T) -> Adaptor<T> {
    Adaptor {
        inner,
        error: Ok(()),
    }
}

impl<T: std::io::Write> std::fmt::Write for Adaptor<T> {
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        match self.inner.write_all(s.as_bytes()) {
            Ok(()) => Ok(()),
            Err(e) => {
                self.error = Err(e);
                Err(std::fmt::Error)
            }
        }
    }
}
