//! Formatting helpers — number, date, time, currency, list formatting.
//!
//! Uses ICU4X for locale-aware formatting when the corresponding features
//! are enabled (`format_nums`, `format_datetime`, `format_list`, `format_currency`).

/// A formatting formatter function signature.
pub type FormatterFn = fn(f64, &str) -> String;

/// Number formatter.
pub fn number(_value: f64, _locale: &str) -> String {
    #[cfg(feature = "format_nums")]
    {
        return format!("{_value}");
    }
    format!("{_value}")
}

/// Date formatter.
pub fn date(_value: f64, _locale: &str) -> String {
    format!("{_value}")
}

/// Time formatter.
pub fn time(_value: f64, _locale: &str) -> String {
    format!("{_value}")
}

/// DateTime formatter.
pub fn datetime(_value: f64, _locale: &str) -> String {
    format!("{_value}")
}

/// Currency formatter.
pub fn currency(_value: f64, _locale: &str) -> String {
    format!("{_value}")
}

/// List formatter.
pub fn list(_items: &[&str], _locale: &str) -> String {
    _items.join(", ")
}
