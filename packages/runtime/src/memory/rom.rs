//! ROM-resident standard library — compile-time embedded data.
//!
//! Inspired by MicroQuickJS's ROM-resident stdlib (compiled into the binary
//! to save RAM). MontRS embeds read-only data (bytecode, prelude source,
//! lookup tables) via the `include_rom!` macro.

/// Embed a file's bytes at compile time as a `RomData` blob.
///
/// ```ignore
/// static PRELUDE: RomData = include_rom!("prelude.mnt");
/// ```
#[macro_export]
macro_rules! include_rom {
    ($path:expr) => {{
        // Compile-time constant bytes; resolve relative to CARGO_MANIFEST_DIR.
        const B: &[u8] = if let Some(data) = option_env!("CARGO_MANIFEST_DIR") {
            &include_bytes!(concat!(data, "/", $path))[..]
        } else {
            &[]
        };
        $crate::memory::rom::RomData::from_static(B)
    }};
}

/// A read-only data blob. Stores a byte slice and exposes bounds-checked,
/// O(1) random access — no heap allocation required at runtime.
#[derive(Debug, Clone, Copy)]
pub struct RomData {
    data: &'static [u8],
}

impl RomData {
    /// Create from a static slice.
    pub const fn from_static(data: &'static [u8]) -> Self {
        Self { data }
    }

    /// Access a byte (bounds-checked).
    pub fn get(&self, index: usize) -> Option<u8> {
        self.data.get(index).copied()
    }

    /// Slice a region. Returns `None` if out of bounds (avoids panics
    /// on embedded targets).
    pub fn slice(&self, start: usize, end: usize) -> Option<&'static [u8]> {
        if start <= end && end <= self.data.len() {
            Some(&self.data[start..end])
        } else {
            None
        }
    }

    /// Interpret the blob as UTF-8.
    pub fn as_str(&self) -> &'static str {
        std::str::from_utf8(self.data).unwrap_or("")
    }

    /// Total length in bytes.
    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Raw access.
    pub fn as_bytes(&self) -> &'static [u8] {
        self.data
    }
}

impl From<&'static [u8]> for RomData {
    fn from(data: &'static [u8]) -> Self {
        Self { data }
    }
}

impl PartialEq<&str> for RomData {
    fn eq(&self, other: &&str) -> bool {
        self.as_str() == *other
    }
}

/// Build a static ROM table of strings (e.g. error codes, keywords).
/// Returns a `&'static [&'static str]`.
#[macro_export]
macro_rules! rom_table {
    ($($item:expr),* $(,)?) => {{
        const TABLE: &[&str] = &[$($item),*];
        TABLE
    }};
}

#[cfg(test)]
mod tests {
    use super::*;

    const HELLO: &[u8] = b"hello rom";

    #[test]
    fn test_rom_data_access() {
        let rom = RomData::from_static(HELLO);
        assert_eq!(rom.len(), 9);
        assert_eq!(rom.get(0), Some(b'h'));
        assert_eq!(rom.get(100), None);
        assert_eq!(rom.as_str(), "hello rom");
    }

    #[test]
    fn test_rom_slice() {
        let rom = RomData::from_static(HELLO);
        assert_eq!(rom.slice(0, 5), Some(&b"hello"[..]));
        assert!(rom.slice(5, 100).is_none());
    }

    #[test]
    fn test_rom_table() {
        let table = crate::rom_table!["one", "two", "three"];
        assert_eq!(table.len(), 3);
        assert_eq!(table[1], "two");
    }
}