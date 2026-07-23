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

use cargo_generate::{GenerateArgs, TemplatePath, generate};
use console::style;
use std::env;

pub async fn run(name: String, template_url: String) -> anyhow::Result<()> {
    println!(
        "{} Creating new MontRS project: {}",
        style("🚀").bold(),
        style(&name).cyan().bold()
    );

    // Use local template path
    let template_path = format!("templates/{}", template_url);

    // In a real CLI, we might use include_dir! to embed templates
    // or look relative to the binary path. For now, we look in the CWD
    // assuming the user is in the montrs root.

    let args = GenerateArgs {
        name: Some(name.clone()),
        template_path: TemplatePath {
            path: Some(template_path),
            ..Default::default()
        },
        destination: Some(env::current_dir()?),
        force: false,
        verbose: true,
        ..Default::default()
    };

    generate(args).map_err(|e| anyhow::anyhow!("Scaffolding failed: {}", e))?;

    println!(
        "\n{} Project {} created successfully!",
        style("✨").green().bold(),
        style(&name).cyan().bold()
    );
    println!("Next steps:\n  cd {}\n  montrs build\n  montrs serve", name);

    Ok(())
}

