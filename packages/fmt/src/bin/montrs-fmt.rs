// This file is part of MontRS.

// Copyright (C) 2025-Present Afsall Labs.
// SPDX-License-Identifier: Apache-2.0 OR MIT

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
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
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

use clap::Parser;
use montrs_fmt::FormatterSettings;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Files or directories to format
    #[arg(default_value = ".")]
    input: Vec<PathBuf>,

    /// Check if files are formatted without modifying them
    #[arg(long)]
    check: bool,

    /// Verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    // Load settings using the Cascade of Truth
    let settings = FormatterSettings::load();

    let mut exit_code = 0;

    for input_path in &args.input {
        if input_path.is_file() {
            if format_one_file(input_path, &settings, args.check, args.verbose)?
            {
                exit_code = 1;
            }
        } else {
            for entry in WalkDir::new(input_path)
                .into_iter()
                .filter_map(|e| e.ok())
                .filter(|e| e.path().extension().is_some_and(|ext| ext == "rs"))
            {
                if format_one_file(
                    entry.path(),
                    &settings,
                    args.check,
                    args.verbose,
                )? {
                    exit_code = 1;
                }
            }
        }
    }

    if exit_code != 0 {
        std::process::exit(exit_code);
    }

    Ok(())
}

fn format_one_file(
    path: &Path,
    settings: &FormatterSettings,
    check: bool,
    verbose: bool,
) -> anyhow::Result<bool> {
    let original = std::fs::read_to_string(path)?;
    let formatted = match montrs_fmt::format_source(&original, settings) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Error formatting {}: {}", path.display(), e);
            return Ok(true);
        }
    };

    if original != formatted {
        if check {
            println!("File {} is not formatted", path.display());
            return Ok(true);
        } else {
            std::fs::write(path, formatted)?;
            if verbose {
                println!("Formatted {}", path.display());
            }
        }
    }

    Ok(false)
}
