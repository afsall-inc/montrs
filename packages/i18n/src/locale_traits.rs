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

use std::{
    fmt::{Debug, Display},
    hash::Hash,
    str::FromStr,
};

/// Trait implemented by the enum representing supported locales.
pub trait Locale<L: Locale = Self>:
    'static
    + Default
    + Clone
    + Copy
    + FromStr
    + AsRef<str>
    + AsRef<L>
    + Display
    + Debug
    + PartialEq
    + Eq
    + Hash
    + Send
    + Sync
    + serde::Serialize
    + serde::de::DeserializeOwned
{
    type Keys: LocaleKeys<Locale = L>;

    fn as_str(self) -> &'static str;
    fn direction(self) -> Direction;
    fn get_all() -> &'static [L];

    fn find_locale<T: AsRef<[u8]>>(accepted_languages: &[T]) -> Self {
        // Search accepted languages for best match.
        for lang in accepted_languages {
            let s = std::str::from_utf8(lang.as_ref()).unwrap_or("");
            let base = s.split('-').next().unwrap_or(s);
            for loc in Self::get_all() {
                if loc.as_str() == base || loc.as_str() == s {
                    return Self::from_base_locale(*loc);
                }
            }
        }
        Self::default()
    }

    fn get_keys(self) -> Self::Keys {
        LocaleKeys::from_locale(self.to_base_locale())
    }
    fn to_base_locale(self) -> L;
    fn from_base_locale(locale: L) -> Self;
    fn map_locale(self, locale: L) -> Self {
        Self::from_base_locale(locale)
    }
}

/// Trait implemented by the struct representing translation keys.
pub trait LocaleKeys: 'static + Clone + Copy + Send + Sync {
    type Locale: Locale;
    fn from_locale(locale: Self::Locale) -> Self;
}

/// Trait for the type giving an ID to each section of translations.
pub trait TranslationUnitId:
    serde::Serialize
    + serde::de::DeserializeOwned
    + Copy
    + Debug
    + Send
    + Sync
    + Eq
    + Hash
    + 'static
{
    fn to_str(self) -> Option<&'static str>;
}

impl TranslationUnitId for () {
    fn to_str(self) -> Option<&'static str> {
        None
    }
}

/// Represents the direction of a script.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Direction {
    LeftToRight,
    RightToLeft,
    Auto,
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl Direction {
    pub const fn as_str(self) -> &'static str {
        match self {
            Direction::LeftToRight => "ltr",
            Direction::RightToLeft => "rtl",
            Direction::Auto => "auto",
        }
    }
}
