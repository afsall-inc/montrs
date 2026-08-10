//! Memory optimization — arena allocator, tagged values, bit fields.
//!
//! Provides fast bump allocation via `Arena`, compact 64-bit tagged
//! value representation, and packed bit fields.

use std::{
    alloc::{Layout, alloc, dealloc},
    ptr::NonNull,
};

/// A bump allocator arena. Pre-allocates a contiguous block of memory
/// and hands out allocations with O(1) bump pointer advancement.
/// Reset as a whole when the arena is cleared — deterministic and fast.
pub struct Arena {
    buffer: NonNull<u8>,
    size: usize,
    cursor: std::sync::atomic::AtomicUsize,
}

unsafe impl Send for Arena {}
unsafe impl Sync for Arena {}

impl Arena {
    /// Create a new arena with the given size in bytes.
    pub fn new(size: usize) -> Self {
        let layout = Layout::from_size_align(size, 64).unwrap();
        let buffer = unsafe { NonNull::new(alloc(layout)).unwrap() };
        Self {
            buffer,
            size,
            cursor: std::sync::atomic::AtomicUsize::new(0),
        }
    }

    /// Allocate bytes from the arena. Returns a pointer and the size used.
    pub fn alloc(&self, requested_size: usize) -> Option<(NonNull<u8>, usize)> {
        let align = 8usize;
        let aligned = (requested_size + align - 1) & !(align - 1);
        let current = self
            .cursor
            .fetch_add(aligned, std::sync::atomic::Ordering::Relaxed);
        if current + aligned > self.size {
            return None;
        }
        let ptr = unsafe { self.buffer.add(current) };
        Some((ptr, aligned))
    }

    /// Reset the arena — all allocations are invalidated.
    pub fn reset(&self) {
        self.cursor.store(0, std::sync::atomic::Ordering::Relaxed);
    }

    pub fn used(&self) -> usize {
        self.cursor.load(std::sync::atomic::Ordering::Relaxed)
    }
    pub fn remaining(&self) -> usize {
        self.size.saturating_sub(self.used())
    }
    pub fn total_size(&self) -> usize {
        self.size
    }
}

impl Drop for Arena {
    fn drop(&mut self) {
        let layout = Layout::from_size_align(self.size, 64).unwrap();
        unsafe {
            dealloc(self.buffer.as_ptr(), layout);
        }
    }
}

/// A 64-bit tagged value — stores a type tag and a payload in a single u64.
/// Uses NaN-boxing: real f64 values (exponent != all-1s) are stored directly;
/// the all-1s exponent (0x7FF) marks a tagged value, with the tag in the
/// low bits of the payload.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TaggedValue(u64);

impl TaggedValue {
    /// NaN-box marker: exponent all 1s, sign set.
    const NAN_BOX: u64 = 0x7FF0_0000_0000_0000;

    // Tag constants (stored in the payload).
    const TAG_INT: u64 = 1;
    const TAG_BOOL: u64 = 2;
    const TAG_PTR: u64 = 3;
    const TAG_NULL: u64 = 4;
    const TAG_UNDEF: u64 = 5;

    pub fn from_int(i: i64) -> Self {
        TaggedValue(
            Self::NAN_BOX
                | (Self::TAG_INT << 48)
                | ((i as u64) & 0xFFFF_FFFF_FFFF),
        )
    }
    pub fn from_float(f: f64) -> Self {
        TaggedValue(f.to_bits())
    }
    pub fn from_bool(b: bool) -> Self {
        TaggedValue(Self::NAN_BOX | (Self::TAG_BOOL << 48) | (b as u64))
    }
    pub fn from_ptr<T>(ptr: *const T) -> Self {
        TaggedValue(
            Self::NAN_BOX
                | (Self::TAG_PTR << 48)
                | ((ptr as u64) & 0xFFFF_FFFF_FFFF),
        )
    }
    pub fn null() -> Self {
        TaggedValue(Self::NAN_BOX | (Self::TAG_NULL << 48))
    }
    pub fn undefined() -> Self {
        TaggedValue(Self::NAN_BOX | (Self::TAG_UNDEF << 48))
    }

    pub fn is_int(&self) -> bool {
        self.tag() == Some(Self::TAG_INT)
    }
    pub fn is_float(&self) -> bool {
        self.0 & Self::NAN_BOX != Self::NAN_BOX
    }
    pub fn is_ptr(&self) -> bool {
        self.tag() == Some(Self::TAG_PTR)
    }
    pub fn is_bool(&self) -> bool {
        self.tag() == Some(Self::TAG_BOOL)
    }
    pub fn is_null(&self) -> bool {
        self.tag() == Some(Self::TAG_NULL)
    }
    pub fn is_undefined(&self) -> bool {
        self.tag() == Some(Self::TAG_UNDEF)
    }

    fn tag(&self) -> Option<u64> {
        if self.0 & 0x7FF0_0000_0000_0000 == 0x7FF0_0000_0000_0000 {
            Some((self.0 >> 48) & 0x0F)
        } else {
            None
        }
    }

    fn payload(&self) -> u64 {
        self.0 & 0xFFFF_FFFF_FFFF
    }

    pub fn as_int(&self) -> Option<i64> {
        if self.is_int() {
            Some(self.payload() as i64)
        } else {
            None
        }
    }
    pub fn as_float(&self) -> Option<f64> {
        if self.is_float() {
            Some(f64::from_bits(self.0))
        } else {
            None
        }
    }
    pub fn as_ptr<T>(&self) -> Option<*const T> {
        if self.is_ptr() {
            Some(self.payload() as *const T)
        } else {
            None
        }
    }
    pub fn as_bool(&self) -> Option<bool> {
        if self.is_bool() {
            Some(self.payload() != 0)
        } else {
            None
        }
    }
}

/// Packed struct with bitfields — groups multiple fields into a single u64.
#[derive(Default)]
pub struct BitField(u64);

impl BitField {
    pub fn new() -> Self {
        BitField(0)
    }
    pub fn get(&self, offset: u8, bits: u8) -> u64 {
        (self.0 >> offset) & ((1 << bits) - 1)
    }
    pub fn set(&mut self, offset: u8, bits: u8, value: u64) {
        let mask = (1 << bits) - 1;
        self.0 = (self.0 & !(mask << offset)) | ((value & mask) << offset);
    }
    pub fn as_u64(&self) -> u64 {
        self.0
    }
    pub fn from_u64(v: u64) -> Self {
        BitField(v)
    }
}
