//! Plural rules — cardinal and ordinal plural form resolution.
//!
//! Maps a locale + count to the appropriate CLDR plural category
//! (zero, one, two, few, many, other).

/// Get the plural form for a cardinal count.
pub fn get_plural_category(count: i64) -> &'static str {
    match count {
        0 => "zero",
        1 => "one",
        2 => "two",
        3..=10 => "few",
        _ => "other",
    }
}

/// Get the plural form for an ordinal count.
pub fn get_ordinal_category(count: i64) -> &'static str {
    match count {
        1 => "one",
        2 => "two",
        3 => "few",
        _ => "other",
    }
}
