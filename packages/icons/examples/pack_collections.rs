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

//! Packs each non-Lucide collection into a DEFLATE-compressed binary blob so
//! the ~15 MB of inline SVG source does not inflate the compiled WASM.
//!
//! Run with every collection feature enabled:
//! `cargo run -p montrs-icons --example pack_collections --features animated,col-radix,col-tabler,col-iconoir,col-phosphor,col-mdi,col-bootstrap,col-simple-icons,col-cryptocurrency`
//!
//! The runtime side (`src/collections/data.rs`) decompresses these blobs
//! lazily and leaks the result to obtain `&'static` glyph tables.

use montrs_icons::collections::{CollectedGlyph, Collection};
use std::io::Write;
use std::path::Path;

fn write_u8(buf: &mut Vec<u8>, s: &str) {
    buf.push(s.len() as u8);
    buf.extend_from_slice(s.as_bytes());
}

fn write_u16(buf: &mut Vec<u8>, s: &str) {
    buf.extend_from_slice(&(s.len() as u16).to_le_bytes());
    buf.extend_from_slice(s.as_bytes());
}

fn write_u32(buf: &mut Vec<u8>, s: &str) {
    buf.extend_from_slice(&(s.len() as u32).to_le_bytes());
    buf.extend_from_slice(s.as_bytes());
}

fn pack(icons: &[CollectedGlyph]) -> Vec<u8> {
    let mut buf = Vec::new();
    buf.extend_from_slice(&(icons.len() as u32).to_le_bytes());
    for g in icons {
        write_u16(&mut buf, g.name);
        write_u32(&mut buf, g.svg);
        write_u8(&mut buf, g.viewbox);
        write_u8(&mut buf, g.fill);
        write_u8(&mut buf, g.stroke);
    }
    buf
}

fn main() -> std::io::Result<()> {
    let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("src/collections");
    let specs: &[(&str, Collection)] = &[
        ("radix", Collection::Radix),
        ("tabler", Collection::Tabler),
        ("iconoir", Collection::Iconoir),
        ("phosphor", Collection::Phosphor),
        ("mdi", Collection::Mdi),
        ("bootstrap", Collection::Bootstrap),
        ("simple-icons", Collection::SimpleIcons),
        ("cryptocurrency", Collection::Cryptocurrency),
    ];

    let mut total_raw = 0usize;
    let mut total_packed = 0usize;
    for (key, collection) in specs {
        let icons = collection.icons();
        let raw = pack(&icons);
        let mut encoder = flate2::write::DeflateEncoder::new(
            Vec::new(),
            flate2::Compression::best(),
        );
        encoder.write_all(&raw)?;
        let compressed = encoder.finish()?;
        std::fs::write(dir.join(format!("{key}.bin")), &compressed)?;
        total_raw += raw.len();
        total_packed += compressed.len();
        println!(
            "{key:<16} {:>6} icons  raw {:>9} B -> packed {:>9} B",
            icons.len(),
            raw.len(),
            compressed.len()
        );
    }
    println!(
        "total raw {} bytes -> packed {} bytes",
        total_raw, total_packed
    );
    Ok(())
}
