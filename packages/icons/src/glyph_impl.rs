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

use crate::glyph::Glyph;
use convert_case::{Case, Casing};
use std::{collections::BTreeMap, str::FromStr, sync::OnceLock};
use strum::{EnumProperty, IntoEnumIterator};

fn split_csv(raw: &'static str) -> impl Iterator<Item = &'static str> {
    raw.split(',').map(str::trim).filter(|s| !s.is_empty())
}

struct SearchEntry {
    glyph: Glyph,
    text: String,
}

static SEARCH_INDEX: OnceLock<Vec<SearchEntry>> = OnceLock::new();
static CATEGORIES: OnceLock<BTreeMap<String, u16>> = OnceLock::new();
static COUNT: OnceLock<usize> = OnceLock::new();

fn build_search_index() -> Vec<SearchEntry> {
    Glyph::iter()
        .map(|glyph| {
            let name: &'static str = glyph.into();
            let name_lower = name.to_lowercase();
            let tags = glyph.get_str("tags").unwrap_or("").to_lowercase();
            let categories =
                glyph.get_str("categories").unwrap_or("").to_lowercase();
            let text = format!("{},{},{}", name_lower, tags, categories);
            SearchEntry { glyph, text }
        })
        .collect()
}

fn build_categories() -> BTreeMap<String, u16> {
    let mut categories: BTreeMap<String, u16> = BTreeMap::new();
    for icon in Glyph::iter() {
        let cats = icon.get_str("categories").unwrap_or("");
        for cat in cats.split(',') {
            let cat = cat.trim();
            if !cat.is_empty() {
                let count = categories
                    .entry(cat.to_case(Case::Title).to_string())
                    .or_insert(0);
                *count += 1;
            }
        }
    }
    categories
}

impl Glyph {
    pub fn svg(&self) -> &'static str {
        self.get_str("svg").unwrap_or("")
    }

    pub fn name(&self) -> &'static str {
        (*self).into()
    }

    pub fn kebab_name(&self) -> String {
        self.name().to_case(Case::Kebab)
    }

    pub fn by_name(name: &str) -> Option<Glyph> {
        Glyph::from_str(name)
            .ok()
            .or_else(|| Glyph::from_str(&name.to_case(Case::UpperCamel)).ok())
    }

    pub fn count() -> usize {
        *COUNT.get_or_init(|| Glyph::iter().count())
    }

    pub fn related(&self, limit: usize) -> Vec<Glyph> {
        let own_tags: std::collections::HashSet<&'static str> =
            self.tags().collect();
        if own_tags.is_empty() {
            return Vec::new();
        }
        let me = *self;
        Glyph::iter()
            .filter(|g| *g != me)
            .filter(|g| g.tags().any(|t| own_tags.contains(t)))
            .take(limit)
            .collect()
    }

    pub fn categories_str(&self) -> &'static str {
        self.get_str("categories").unwrap_or("")
    }

    pub fn tags_str(&self) -> &'static str {
        self.get_str("tags").unwrap_or("")
    }

    pub fn categories(&self) -> impl Iterator<Item = &'static str> {
        split_csv(self.categories_str())
    }

    pub fn tags(&self) -> impl Iterator<Item = &'static str> {
        split_csv(self.tags_str())
    }

    pub fn contributors(&self) -> impl Iterator<Item = &'static str> {
        split_csv(self.get_str("contributors").unwrap_or(""))
    }

    pub fn all_categories() -> &'static BTreeMap<String, u16> {
        CATEGORIES.get_or_init(build_categories)
    }

    pub fn find(filter: &str) -> Vec<Glyph> {
        if filter.is_empty() {
            return Glyph::iter().collect();
        }

        let index = SEARCH_INDEX.get_or_init(build_search_index);
        let filter_lower = filter.to_lowercase();
        let terms: Vec<&str> = filter_lower.split_whitespace().collect();

        index
            .iter()
            .filter(|entry| terms.iter().all(|term| entry.text.contains(term)))
            .map(|entry| entry.glyph)
            .collect()
    }
}
