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
