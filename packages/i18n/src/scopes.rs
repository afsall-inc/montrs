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

use crate::locale_traits::*;

/// Scope marker trait.
pub trait Scope<L: Locale>: 'static + Clone + Copy + Send + Sync {
    type Keys: LocaleKeys<Locale = L>;
    fn get_keys(locale: L) -> Self::Keys;
}

impl<L: Locale> Scope<L> for L::Keys {
    type Keys = L::Keys;
    fn get_keys(locale: L) -> Self::Keys {
        locale.get_keys()
    }
}

/// A locale scoped to a subset of keys.
#[derive(Debug, Clone, Copy)]
pub struct ScopedLocale<L: Locale> {
    inner: L,
}

impl<L: Locale> ScopedLocale<L> {
    pub fn new(locale: L) -> Self {
        Self { inner: locale }
    }
    pub fn inner(self) -> L {
        self.inner
    }
    pub fn as_str(self) -> &'static str {
        self.inner.as_str()
    }
    pub fn direction(self) -> Direction {
        self.inner.direction()
    }
    pub fn get_keys(self) -> L::Keys {
        self.inner.get_keys()
    }
}

impl<L: Locale + Default> Default for ScopedLocale<L> {
    fn default() -> Self {
        Self::new(L::default())
    }
}
impl<L: Locale> AsRef<str> for ScopedLocale<L> {
    fn as_ref(&self) -> &str {
        self.inner.as_ref()
    }
}
impl<L: Locale> AsRef<L> for ScopedLocale<L> {
    fn as_ref(&self) -> &L {
        &self.inner
    }
}
impl<L: Locale> std::fmt::Display for ScopedLocale<L> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.inner)
    }
}
impl<L: Locale + PartialEq> PartialEq for ScopedLocale<L> {
    fn eq(&self, other: &Self) -> bool {
        self.inner == other.inner
    }
}
impl<L: Locale + Eq> Eq for ScopedLocale<L> {}
impl<L: Locale + std::hash::Hash> std::hash::Hash for ScopedLocale<L> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.inner.hash(state);
    }
}
impl<L: Locale + serde::Serialize> serde::Serialize for ScopedLocale<L> {
    fn serialize<Ser: serde::Serializer>(
        &self,
        s: Ser,
    ) -> Result<Ser::Ok, Ser::Error> {
        self.inner.serialize(s)
    }
}
impl<'de, L: Locale + serde::de::DeserializeOwned> serde::de::Deserialize<'de>
    for ScopedLocale<L>
{
    fn deserialize<D: serde::de::Deserializer<'de>>(
        d: D,
    ) -> Result<Self, D::Error> {
        L::deserialize(d).map(Self::new)
    }
}
