// بِسْمِ اللَّهِ الرَّحْمَنِ الرَّحِيم
// This file is part of montrs.
// Copyright (C) 2026-Present Afsall Inc.
// SPDX-License-Identifier: Apache-2.0 OR MIT
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
// http://www.apache.org/licenses/LICENSE-2.0
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
// Alternatively, this file is available under the MIT License:
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

//! Memory optimization — arena allocator, tagged values, bit fields, ROM data.

pub mod rom;
pub use rom::RomData;
use std::{
    alloc::{Layout, alloc, dealloc},
    ptr::NonNull,
    sync::atomic::Ordering,
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
    /// Uses a compare-and-swap loop so a failed allocation does NOT corrupt
    /// the cursor (B9 fix).
    pub fn alloc(&self, requested_size: usize) -> Option<(NonNull<u8>, usize)> {
        let align = 8usize;
        let aligned = (requested_size + align - 1) & !(align - 1);

        loop {
            let current = self.cursor.load(Ordering::Relaxed);
            let next = current + aligned;
            if next > self.size {
                return None;
            }
            match self.cursor.compare_exchange(
                current,
                next,
                Ordering::SeqCst,
                Ordering::SeqCst,
            ) {
                Ok(_) => {
                    let ptr = unsafe { self.buffer.add(current) };
                    return Some((ptr, aligned));
                }
                Err(_new) => {
                    // Another thread raced; retry with the new current value.
                    continue;
                }
            }
        }
    }

    /// Reset the arena — all allocations are invalidated.
    pub fn reset(&self) {
        self.cursor.store(0, Ordering::Relaxed);
    }

    pub fn used(&self) -> usize {
        self.cursor.load(Ordering::Relaxed)
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
///
/// Layout:
/// - Bits 52-63: marker `0xFFF` for tagged values (non-float)
/// - Bits 48-51: tag (int/bool/ptr/null/undef)
/// - Bits 0-47:  payload
///
/// Floats are stored as raw IEEE-754 bits. A float whose top 12 bits are
/// accidentally `0xFFF` (a rare quiet NaN) is rewritten to a non-colliding
/// quiet NaN (`0x7FF8_…`) so it is never misidentified as tagged.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TaggedValue(u64);

impl TaggedValue {
    /// Marker in bits 52-63 for tagged (non-float) values.
    const TAG_MARKER: u64 = 0xFFF0_0000_0000_0000;
    /// Mask for the marker bits (52-63).
    const MARKER_MASK: u64 = 0xFFF0_0000_0000_0000;

    // Tag constants (4 bits, bits 48-51).
    const TAG_INT: u64 = 1;
    const TAG_BOOL: u64 = 2;
    const TAG_PTR: u64 = 3;
    const TAG_NULL: u64 = 4;
    const TAG_UNDEF: u64 = 5;

    pub fn from_int(i: i64) -> Self {
        let payload = (i as u64) & 0x0000_FFFF_FFFF_FFFF;
        TaggedValue(Self::TAG_MARKER | (Self::TAG_INT << 48) | payload)
    }
    pub fn from_float(f: f64) -> Self {
        let bits = f.to_bits();
        // If the float collides with the tagged marker, rewrite to a
        // non-colliding quiet NaN.
        if (bits & Self::MARKER_MASK) == Self::TAG_MARKER {
            TaggedValue(0x7FF8_0000_0000_0000)
        } else {
            TaggedValue(bits)
        }
    }
    pub fn from_bool(b: bool) -> Self {
        TaggedValue(Self::TAG_MARKER | (Self::TAG_BOOL << 48) | (b as u64))
    }
    pub fn from_ptr<T>(ptr: *const T) -> Self {
        let payload = (ptr as u64) & 0x0000_FFFF_FFFF_FFFF;
        TaggedValue(Self::TAG_MARKER | (Self::TAG_PTR << 48) | payload)
    }
    pub fn null() -> Self {
        TaggedValue(Self::TAG_MARKER | (Self::TAG_NULL << 48))
    }
    pub fn undefined() -> Self {
        TaggedValue(Self::TAG_MARKER | (Self::TAG_UNDEF << 48))
    }

    /// Returns true if this is a tagged (non-float) value.
    fn is_tagged(&self) -> bool {
        (self.0 & Self::MARKER_MASK) == Self::TAG_MARKER
    }

    pub fn is_int(&self) -> bool {
        self.tag() == Some(Self::TAG_INT)
    }
    pub fn is_float(&self) -> bool {
        !self.is_tagged()
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
        if self.is_tagged() {
            Some((self.0 >> 48) & 0x0F)
        } else {
            None
        }
    }

    fn payload(&self) -> u64 {
        self.0 & 0x0000_FFFF_FFFF_FFFF
    }

    pub fn as_int(&self) -> Option<i64> {
        if self.is_int() {
            // Sign-extend 48-bit payload to i64.
            let p = self.payload();
            let sign_bit = 1u64 << 47;
            if p & sign_bit != 0 {
                Some((p | 0xFFFF_0000_0000_0000) as i64)
            } else {
                Some(p as i64)
            }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn arena_overflow_does_not_corrupt() {
        let arena = Arena::new(64);
        // Request more than capacity (aligned size > 64).
        assert!(arena.alloc(65).is_none());
        // Cursor should be unchanged (0) — a subsequent small alloc works.
        assert_eq!(arena.used(), 0);
        assert!(arena.alloc(32).is_some());
        assert_eq!(arena.used(), 32);
    }

    #[test]
    fn tagged_value_nan() {
        // B10 fix: NaN-boxing must not collide a real NaN float with ints/bools.
        let v = TaggedValue::from_float(f64::NAN);
        assert!(v.is_float());
        assert!(v.as_float().unwrap().is_nan());
        assert!(!v.is_int());
        assert!(!v.is_bool());
    }

    #[test]
    fn tagged_value_int_distinct_from_nan_float() {
        let int = TaggedValue::from_int(1);
        assert!(int.is_int());
        assert!(!int.is_float());
    }
}
