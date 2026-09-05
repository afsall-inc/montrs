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

//! montrs-build: Facade crate for the MontRS build system.
//!
//! Re-exports `montrs-build-core`, `montrs-build-watch`, and `montrs-build-serve`
//! for convenience, and provides the concrete `Pipeline` struct that implements
//! `BuildPipeline`.

pub use montrs_build_core::*;
pub use montrs_build_serve::*;
pub use montrs_build_watch::*;

mod pipeline;

pub use pipeline::Pipeline;

/// Run a cargo command and stream output.
/// Automatically sets RUSTFLAGS to enable Leptos `erase_components`
/// for reduced type-depth and faster compiles.
pub fn run_cargo(args: &[String]) -> anyhow::Result<()> {
    let status = std::process::Command::new("cargo")
        .env("RUSTFLAGS", "--cfg erase_components")
        .args(args)
        .stdout(std::process::Stdio::inherit())
        .stderr(std::process::Stdio::inherit())
        .status()?;
    if !status.success() {
        anyhow::bail!("cargo command failed: cargo {}", args.join(" "));
    }
    Ok(())
}

/// Run tailwindcss CLI on the input file to produce the output file.
/// `bin` is an optional path to the tailwindcss binary (e.g. a managed install).
pub fn run_tailwind(
    bin: Option<&std::path::Path>,
    input: &std::path::Path,
    output: &std::path::Path,
) -> anyhow::Result<()> {
    let mut cmd = match bin {
        Some(path) => std::process::Command::new(path),
        None => std::process::Command::new("tailwindcss"),
    };
    let status = cmd
        .arg("-i")
        .arg(input)
        .arg("-o")
        .arg(output)
        .stdout(std::process::Stdio::inherit())
        .stderr(std::process::Stdio::inherit())
        .status()?;
    if !status.success() {
        anyhow::bail!(
            "tailwindcss failed. Run `montrs install` to ensure it is \
             installed."
        );
    }
    Ok(())
}

/// Copy a directory recursively.
pub fn copy_dir(
    src: &std::path::Path,
    dst: &std::path::Path,
) -> anyhow::Result<()> {
    if src.exists() {
        fs_extra::dir::copy(
            src,
            dst,
            &fs_extra::dir::CopyOptions::new()
                .overwrite(true)
                .content_only(true),
        )?;
    }
    Ok(())
}
