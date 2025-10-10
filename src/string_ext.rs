/// Extension traits for `String` to truncate strings with an ellipsis.
pub trait StringExt {
    fn truncate_ellipsis(&self, max_len: usize) -> String;
}

/// Extension traits for `&str` to truncate strings with an ellipsis.
pub trait StrExt {
    fn truncate_ellipsis(&self, max_len: usize) -> String;
}

impl StringExt for String {
    /// Truncates the string to `max_len` characters, appending an ellipsis (`…`) if truncation occurs.
    /// If the string is shorter than or equal to `max_len`, it is returned unchanged
    fn truncate_ellipsis(&self, max_len: usize) -> String {
        if self.len() > max_len {
            format!("{}…", &self[..max_len.saturating_sub(1)])
        } else {
            self.clone()
        }
    }
}

impl StrExt for &str {
    /// Truncates the string to `max_len` characters, appending an ellipsis (`…`) if truncation occurs.
    /// If the string is shorter than or equal to `max_len`, it is returned unchanged
    /// Returns a String instead of &str to avoid lifetime issues.
    fn truncate_ellipsis(&self, max_len: usize) -> String {
        if self.len() > max_len {
            format!("{}…", &self[..max_len.saturating_sub(1)])
        } else {
            self.to_string()
        }
    }
}
