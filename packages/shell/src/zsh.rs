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

use crate::{ActivateOptions, Shell};
use std::fmt;

#[derive(Default)]
pub struct Zsh;

impl fmt::Display for Zsh {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "zsh")
    }
}

impl Shell for Zsh {
    fn activate(&self, opts: &ActivateOptions) -> String {
        let exe = opts.exe.to_string_lossy();
        let mut out = String::new();
        out.push_str("export MONTRS_SHELL=\"zsh\"\n");
        out.push_str(&format!(
            r#"_montrs_hook() {{
    local ret=$?
    eval "$("{exe}" hook-env zsh)" 2>/dev/null
    return $ret
}}
autoload -Uz add-zsh-hook
add-zsh-hook precmd _montrs_hook
"#,
        ));
        let shims = std::env::var("MONTRS_SHIMS_DIR").unwrap_or_else(|_| {
            montrs_tool::backend::default_shims_dir()
                .display()
                .to_string()
        });
        out.push_str(&format!("export PATH=\"{shims}\":$PATH\n"));
        out
    }

    fn deactivate(&self) -> String {
        String::from(
            "unset MONTRS_SHELL\nunfunction _montrs_hook\nadd-zsh-hook -d \
             precmd _montrs_hook\n",
        )
    }

    fn set_env(&self, k: &str, v: &str) -> String {
        format!("export {k}=\"{v}\"\n")
    }

    fn unset_env(&self, k: &str) -> String {
        format!("unset {k}\n")
    }

    fn prepend_path(&self, dir: &str) -> String {
        format!("export PATH=\"{dir}\":$PATH\n")
    }
}
