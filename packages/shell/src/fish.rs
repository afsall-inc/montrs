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
pub struct Fish;

impl fmt::Display for Fish {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "fish")
    }
}

impl Shell for Fish {
    fn activate(&self, opts: &ActivateOptions) -> String {
        let exe = opts.exe.to_string_lossy();
        let shims = std::env::var("MONTRS_SHIMS_DIR").unwrap_or_else(|_| {
            montrs_tool::backend::default_shims_dir()
                .display()
                .to_string()
        });
        format!(
            r#"set -gx MONTRS_SHELL fish
function _montrs_hook --on-event fish_prompt
    set -l ret $status
    eval "$([string join ' ' {exe} hook-env fish])" 2>/dev/null
    return $ret
end
fish_add_path {shims}
"#,
        )
    }

    fn deactivate(&self) -> String {
        String::from("set -e MONTRS_SHELL\nfunctions -e _montrs_hook\n")
    }

    fn set_env(&self, k: &str, v: &str) -> String {
        format!("set -gx {k} \"{v}\"\n")
    }

    fn unset_env(&self, k: &str) -> String {
        format!("set -e {k}\n")
    }

    fn prepend_path(&self, dir: &str) -> String {
        format!("fish_add_path \"{dir}\"\n")
    }
}
