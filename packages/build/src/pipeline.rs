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

use crate::{copy_dir, run_cargo, run_tailwind};
use anyhow::{Result, anyhow};
use montrs_build_core::{BuildPipeline, find_workspace_target_dir};
use montrs_cache::{Cache, CacheKey, Input};
use montrs_metadata::MontrsMetadata;
use std::{
    path::{Path, PathBuf},
    process::Command,
};

/// The MontRS build pipeline.
pub struct Pipeline {
    pub meta: MontrsMetadata,
    pub project_root: PathBuf,
    pub site_root: PathBuf,
    pub pkg_dir: PathBuf,
    pub server_bin_name: String,
    pub workspace_target_dir: PathBuf,
    /// Whether to build optimized (--release) artifacts.
    pub release: bool,
    /// Build both halves with `debug_assertions` + `LEPTOS_WATCH` so the
    /// hot-reload markers match between SSR and the WASM client (dev only).
    pub hot_reload: bool,
    /// Path to the Tailwind CSS binary (managed install override).
    pub tailwind_bin: Option<PathBuf>,
    /// Path to the wasm-bindgen binary (managed install override).
    pub wasm_bindgen_bin: Option<PathBuf>,
    /// Content-addressed cache; disabled when `MONTRS_NO_CACHE` is set.
    pub cache: Cache,
}

impl Pipeline {
    pub fn from_root(root: &Path) -> Result<Self> {
        let root = root.canonicalize()?;
        let meta = MontrsMetadata::from_file(root.join("montrs.toml"))?;
        let site_root = root.join(&meta.serve.site_root);
        let pkg_dir = site_root.join(&meta.serve.site_pkg_dir);
        let workspace_target = find_workspace_target_dir(&root)?;
        let release = meta.serve.release;
        let server_bin_name = meta
            .serve
            .package
            .as_deref()
            .unwrap_or("app")
            .replace('-', "_")
            + "-ssr";

        Ok(Self {
            meta,
            project_root: root.to_path_buf(),
            site_root,
            pkg_dir,
            server_bin_name,
            workspace_target_dir: workspace_target,
            release,
            hot_reload: false,
            tailwind_bin: None,
            wasm_bindgen_bin: None,
            cache: if std::env::var_os("MONTRS_NO_CACHE").is_some() {
                Cache::disabled()
            } else {
                Cache::local(&root)
            },
        })
    }

    /// Disable the incremental cache for this pipeline.
    pub fn without_cache(mut self) -> Self {
        self.cache = Cache::disabled();
        self
    }

    /// Run a step through the cache: skip when fresh, otherwise run and record.
    ///
    /// Returns `Ok(true)` on a cache hit.
    fn cached_step<F>(
        &self,
        namespace: &str,
        inputs: Vec<Input>,
        outputs: Vec<PathBuf>,
        label: &str,
        run: F,
    ) -> Result<bool>
    where
        F: FnOnce() -> Result<()>,
    {
        if !self.cache.is_enabled() {
            run()?;
            return Ok(false);
        }
        let key = CacheKey::new(namespace, &inputs).map_err(|e| anyhow!(e))?;
        if self.cache.is_fresh(&key) {
            println!(" {label}: up to date (cached)");
            return Ok(true);
        }
        run()?;
        if let Err(e) = self.cache.record(&key, &outputs) {
            // A cache write failure must never fail the build.
            eprintln!(" warning: could not record {namespace} in cache: {e}");
        }
        Ok(false)
    }

    /// Inputs shared by every Rust build step: the app sources, manifests, and
    /// the environment/profile that select the output.
    fn rust_inputs(&self) -> Vec<Input> {
        let mut inputs = vec![
            Input::dir(self.project_root.join("app").join("src")),
            Input::file(self.project_root.join("Cargo.toml")),
            Input::file(self.project_root.join("montrs.toml")),
            Input::env("PROFILE"),
            Input::env("LEPTOS_WATCH"),
            Input::env("RUSTFLAGS"),
            Input::value(self.release.to_string()),
            Input::value(self.hot_reload.to_string()),
        ];
        // Workspace manifests that change what gets compiled.
        if let Some(ws_root) = self.workspace_target_dir.parent() {
            let packages = ws_root.join("packages");
            if packages.exists() {
                inputs.push(Input::glob(&packages, "*/src/**/*.rs"));
                inputs.push(Input::glob(&packages, "*/Cargo.toml"));
            }
        }
        inputs
    }

    /// The compiled WASM artifact for the active profile.
    fn wasm_artifact(&self) -> PathBuf {
        let lib_name = self
            .meta
            .serve
            .package
            .as_deref()
            .unwrap_or("app")
            .replace('-', "_");
        let wasm_profile = if self.hot_reload { "hot" } else { "release" };
        self.workspace_target_dir
            .join("wasm32-unknown-unknown")
            .join(wasm_profile)
            .join(format!("{lib_name}.wasm"))
    }

    /// The compiled SSR server artifact for the active profile.
    fn server_artifact(&self) -> PathBuf {
        self.server_bin_path()
    }

    fn tool_version(name: &str, bin: Option<&Path>) -> String {
        let program = match bin {
            Some(path) => path.as_os_str(),
            None => std::ffi::OsStr::new(name),
        };
        let mut cmd = Command::new(program);
        cmd.arg("--version");
        match cmd.output() {
            Ok(out) if out.status.success() => {
                String::from_utf8_lossy(&out.stdout).trim().to_string()
            }
            _ => "unknown".to_string(),
        }
    }

    /// `cargo build` of the WASM frontend, cached by sources + tool version.
    fn step_wasm(&self) -> Result<bool> {
        let mut inputs = self.rust_inputs();
        inputs.push(Input::tool("rustc", Self::tool_version("rustc", None)));
        let out = self.wasm_artifact();
        self.cached_step(
            "wasm",
            inputs,
            vec![out],
            "Building frontend (WASM)",
            || {
                println!(" Building frontend (WASM)...");
                run_cargo(&self.build_frontend_args(), self.hot_reload)
            },
        )
    }

    /// `wasm-bindgen` bundling, cached by the wasm artifact + tool version.
    fn step_bindgen(&self) -> Result<bool> {
        let inputs = vec![
            Input::file(self.wasm_artifact()),
            Input::tool(
                "wasm-bindgen",
                Self::tool_version(
                    "wasm-bindgen",
                    self.wasm_bindgen_bin.as_deref(),
                ),
            ),
            Input::value("front"),
        ];
        self.cached_step(
            "bindgen",
            inputs,
            vec![
                self.pkg_dir.join("front.js"),
                self.pkg_dir.join("front_bg.wasm"),
            ],
            "Bundling WASM with wasm-bindgen",
            || self.bundle_wasm(),
        )
    }

    /// Tailwind CSS, cached by the input stylesheet, scanned sources, and the
    /// Tailwind binary version.
    fn step_tailwind(&self) -> Result<bool> {
        let Some(tw_input) = &self.meta.serve.tailwind_input_file else {
            return Ok(false);
        };
        let input = self.project_root.join(tw_input);
        if !input.exists() {
            return Ok(false);
        }
        let mut inputs = vec![
            Input::file(&input),
            Input::tool(
                "tailwindcss",
                Self::tool_version("tailwindcss", self.tailwind_bin.as_deref()),
            ),
        ];
        if let Some(ws_root) = self.workspace_target_dir.parent() {
            let ui = ws_root.join("packages").join("ui").join("src");
            if ui.exists() {
                inputs.push(Input::dir(ui));
            }
            let icons = ws_root.join("packages").join("icons").join("src");
            if icons.exists() {
                inputs.push(Input::dir(icons));
            }
        }
        let app = self.project_root.join("app").join("src");
        if app.exists() {
            inputs.push(Input::dir(app));
        }
        let output = self.site_root.join("main.css");
        self.cached_step(
            "tailwind",
            inputs,
            vec![output],
            "Processing Tailwind CSS",
            || self.process_tailwind(),
        )
    }

    /// Static assets, cached by the assets directory contents.
    fn step_assets(&self) -> Result<bool> {
        let Some(assets) = &self.meta.serve.assets_dir else {
            return Ok(false);
        };
        let src = self.project_root.join(assets);
        if !src.exists() {
            return Ok(false);
        }
        let outputs: Vec<PathBuf> = walkdir::WalkDir::new(&src)
            .into_iter()
            .filter_map(Result::ok)
            .filter(|e| e.file_type().is_file())
            .map(|e| {
                let rel = e.path().strip_prefix(&src).unwrap_or(e.path());
                self.site_root.join(rel)
            })
            .collect();
        self.cached_step(
            "assets",
            vec![Input::dir(&src)],
            outputs,
            "Copying assets",
            || self.copy_assets(),
        )
    }

    /// `index.html`, cached by its render inputs.
    fn step_index_html(&self) -> Result<bool> {
        let inputs = vec![
            Input::value(format!(
                "{:?}",
                self.meta.project.name.as_deref().unwrap_or("MontRS App")
            )),
            Input::value(
                self.meta
                    .serve
                    .output_name
                    .clone()
                    .unwrap_or_else(|| "website".to_string()),
            ),
        ];
        self.cached_step(
            "index-html",
            inputs,
            vec![self.site_root.join("index.html")],
            "Generating index.html",
            || self.generate_index_html(),
        )
    }

    /// SSR server, cached by sources + rustc version + profile.
    fn step_server(&self) -> Result<bool> {
        let mut inputs = self.rust_inputs();
        inputs.push(Input::tool("rustc", Self::tool_version("rustc", None)));
        let out = self.server_artifact();
        self.cached_step(
            "server",
            inputs,
            vec![out],
            "Building SSR server",
            || {
                println!(" Building SSR server...");
                run_cargo(&self.server_args(), self.hot_reload)
            },
        )
    }

    /// Directory that cargo builds artifacts into for the current profile.
    fn profile_dir(&self) -> &'static str {
        if self.release { "release" } else { "debug" }
    }

    /// Path to the compiled SSR server binary, with `.exe` on Windows.
    pub fn server_bin_path(&self) -> PathBuf {
        let mut name = self.server_bin_name.clone();
        if cfg!(windows) && !name.ends_with(".exe") {
            name.push_str(".exe");
        }
        self.workspace_target_dir
            .join(self.profile_dir())
            .join(name)
    }

    /// Args for `cargo build` of the SSR server binary.
    ///
    /// Order matters: the `--features` value must immediately follow the
    /// `--features` flag, other flags come after.
    pub fn server_args(&self) -> Vec<String> {
        server_build_args(
            self.meta.serve.package.as_deref().unwrap_or("app"),
            &self.meta.serve.bin_features,
            self.meta.serve.bin_default_features,
            self.release,
        )
    }

    fn build_frontend_args(&self) -> Vec<String> {
        let pkg = self.meta.serve.package.as_deref().unwrap_or("app");
        frontend_build_args(
            pkg,
            &self.meta.serve.lib_features,
            self.meta.serve.lib_default_features,
            self.hot_reload,
        )
    }

    fn bundle_wasm(&self) -> Result<()> {
        std::fs::create_dir_all(&self.pkg_dir)?;

        let lib_name = self
            .meta
            .serve
            .package
            .as_deref()
            .unwrap_or("app")
            .replace('-', "_");

        // Matches the profile chosen in `frontend_build_args`: `hot` during
        // hot-reload dev builds, otherwise `release`.
        let wasm_profile = if self.hot_reload { "hot" } else { "release" };
        let wasm_target_dir = self
            .workspace_target_dir
            .join("wasm32-unknown-unknown")
            .join(wasm_profile);

        let wasm_file = wasm_target_dir.join(format!("{}.wasm", lib_name));

        if !wasm_file.exists() {
            return Err(anyhow!(
                "WASM file not found at {}. Did the wasm32-unknown-unknown \
                 build succeed?",
                wasm_file.display()
            ));
        }

        let mut cmd = match &self.wasm_bindgen_bin {
            Some(path) => Command::new(path),
            None => Command::new("wasm-bindgen"),
        };
        let output = cmd
            .arg("--target")
            .arg("web")
            .arg("--no-typescript")
            .arg("--out-dir")
            .arg(&self.pkg_dir)
            .arg("--out-name")
            .arg("front")
            .arg(&wasm_file)
            .output();

        // Streaming `wasm-bindgen` is mandatory: without the JS bindings shim
        // (`front.js`) the hydration bootstrap 404s and the app silently stays
        // static SSR — no clicks, no hover, no theme toggle. Never fall back to
        // copying the raw `.wasm`; fail loudly with an actionable message.
        match output {
            Ok(o) if o.status.success() => {
                println!(" wasm-bindgen completed successfully");
            }
            Ok(o) => {
                let stderr = String::from_utf8_lossy(&o.stderr);
                return Err(anyhow!(
                    "wasm-bindgen failed to process the WASM \
                     bundle.\n{}\nHint: run `montrs install` to install a \
                     wasm-bindgen CLI that matches the `wasm-bindgen` crate \
                     version used by the app (a version mismatch produces \
                     exactly this error).",
                    stderr.trim()
                ));
            }
            Err(e) => {
                return Err(anyhow!(
                    "could not run wasm-bindgen: {e}.\nHint: run `montrs \
                     install` to install wasm-bindgen."
                ));
            }
        }

        for artifact in ["front.js", "front_bg.wasm"] {
            let path = self.pkg_dir.join(artifact);
            if !path.exists() {
                return Err(anyhow!(
                    "wasm-bindgen reported success but did not produce {}",
                    path.display()
                ));
            }
        }

        // `--out-name front` produces `front.js` + `front_bg.wasm`. Remove any
        // stale `front.wasm` (a leftover from older raw-copy fallbacks) so the
        // browser never downloads the giant unprocessed build.
        let stale = self.pkg_dir.join("front.wasm");
        if stale.exists() {
            let _ = std::fs::remove_file(&stale);
        }

        Ok(())
    }
}

impl BuildPipeline for Pipeline {
    fn build_server(&self) -> Result<()> {
        self.step_server().map(|_| ())
    }

    fn build_frontend(&self) -> Result<()> {
        self.step_wasm()?;
        self.step_bindgen()?;
        Ok(())
    }

    fn process_tailwind(&self) -> Result<()> {
        if let Some(tw_input) = &self.meta.serve.tailwind_input_file {
            let input = self.project_root.join(tw_input);
            let output = self.site_root.join("main.css");
            if input.exists() {
                println!(" Processing Tailwind CSS...");
                std::fs::create_dir_all(&self.site_root)?;
                run_tailwind(self.tailwind_bin.as_deref(), &input, &output)?;
                println!(" Tailwind CSS processed");
            }
        }
        Ok(())
    }

    fn copy_assets(&self) -> Result<()> {
        if let Some(assets) = &self.meta.serve.assets_dir {
            let src = self.project_root.join(assets);
            if src.exists() {
                println!(" Copying assets...");
                std::fs::create_dir_all(&self.site_root)?;
                copy_dir(&src, &self.site_root)?;
                println!(" Assets copied");
            }
        }
        Ok(())
    }

    fn generate_index_html(&self) -> Result<()> {
        let index_path = self.site_root.join("index.html");
        let project_name =
            self.meta.project.name.as_deref().unwrap_or("MontRS App");

        let html = format!(
            r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="utf-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    <title>{project_name}</title>
    <link rel="icon" type="image/svg+xml" href="/favicon.svg" />
    <link rel="icon" type="image/png" sizes="32x32" href="/favicon-32.png" />
    <link rel="apple-touch-icon" href="/favicon-180.png" />
    <link rel="stylesheet" href="/main.css" />
    <link rel="modulepreload" href="/pkg/front.js" />
    <script type="module">
        import init, {{ hydrate }} from '/pkg/front.js';
        init('/pkg/front_bg.wasm').then(() => hydrate());
    </script>
</head>
<body>
    <div id="app"></div>
</body>
</html>"#,
        );
        std::fs::write(&index_path, html)?;
        println!(" Generated index.html");
        Ok(())
    }

    fn build_all(&self) -> Result<()> {
        std::fs::create_dir_all(&self.site_root)?;
        std::fs::create_dir_all(&self.pkg_dir)?;

        // Steps whose inputs are unrelated run concurrently: Tailwind and asset
        // copying overlap the Rust build chain. `cargo` serializes itself via
        // its own build-directory lock, so the WASM → bindgen → server chain
        // stays ordered on this thread while the CSS/asset steps run alongside.
        std::thread::scope(|scope| -> Result<()> {
            let tailwind = scope.spawn(|| self.step_tailwind());
            let assets = scope.spawn(|| self.step_assets());

            self.step_wasm()?;
            self.step_bindgen()?;
            self.step_server()?;

            tailwind
                .join()
                .map_err(|_| anyhow!("tailwind step panicked"))??;
            assets
                .join()
                .map_err(|_| anyhow!("assets step panicked"))??;

            self.step_index_html()?;
            Ok(())
        })?;

        println!(" Build complete");
        Ok(())
    }

    /// Rebuild only the SSR server — the WASM client is already patched live
    /// by the `view!` hot-reload watcher, so it stays valid. This is the fast
    /// path for non-view `.rs` edits (server logic, comments, non-markup code).
    fn build_server_only(&self) -> Result<()> {
        std::fs::create_dir_all(&self.site_root)?;
        let hit = self.step_server()?;
        if hit {
            println!(" Server unchanged (cached)");
        } else {
            println!(" Server rebuilt (client unchanged)");
        }
        Ok(())
    }

    fn metadata(&self) -> &MontrsMetadata {
        &self.meta
    }

    fn project_root(&self) -> &Path {
        &self.project_root
    }

    fn site_root(&self) -> &Path {
        &self.site_root
    }

    fn pkg_dir(&self) -> &Path {
        &self.pkg_dir
    }
}

/// Args for `cargo build` of the WASM frontend (hydrate client).
///
/// Order matters: the `--features` value must immediately follow the
/// `--features` flag; other flags come after.
fn frontend_build_args(
    pkg: &str,
    lib_features: &[String],
    lib_default_features: bool,
    hot_reload: bool,
) -> Vec<String> {
    let mut args = vec![
        "build".to_string(),
        "--target".to_string(),
        "wasm32-unknown-unknown".to_string(),
        "--package".to_string(),
        pkg.to_string(),
        "--features".to_string(),
    ];
    let features = if lib_features.is_empty() {
        "hydrate".to_string()
    } else {
        lib_features.join(",")
    };
    // `--features` value must immediately follow the `--features` flag.
    args.push(features);
    if !lib_default_features {
        args.push("--no-default-features".to_string());
    }
    // A debug (unoptimized) WASM client is unusably large and slow in the
    // browser, so the frontend is always built with optimization. In dev with
    // hot reload the `hot` profile keeps optimization but enables
    // `debug_assertions`, so the client emits the same hot-reload markers as
    // the SSR server.
    if hot_reload {
        args.push("--profile".to_string());
        args.push("hot".to_string());
    } else {
        args.push("--release".to_string());
    }
    args
}

/// Args for `cargo build` of the SSR server binary.
///
/// Order matters: the `--features` value must immediately follow the
/// `--features` flag; other flags come after.
fn server_build_args(
    pkg: &str,
    bin_features: &[String],
    bin_default_features: bool,
    release: bool,
) -> Vec<String> {
    let mut args = vec![
        "build".to_string(),
        "--package".to_string(),
        pkg.to_string(),
        "--features".to_string(),
    ];
    let features = if bin_features.is_empty() {
        "ssr".to_string()
    } else {
        bin_features.join(",")
    };
    // `--features` value must immediately follow the `--features` flag.
    args.push(features);
    if !bin_default_features {
        args.push("--no-default-features".to_string());
    }
    if release {
        args.push("--release".to_string());
    }
    args
}

#[cfg(test)]
mod tests {
    use super::*;

    fn write_min_project(dir: &Path) {
        std::fs::write(
            dir.join("montrs.toml"),
            "[project]\nname = \"t\"\n\n[serve]\npackage = \"t\"\nassets-dir \
             = \"assets\"\n",
        )
        .unwrap();
    }

    #[test]
    fn asset_and_index_steps_are_cached_by_content() {
        let dir = tempfile::tempdir().unwrap();
        write_min_project(dir.path());
        let assets = dir.path().join("assets");
        std::fs::create_dir_all(&assets).unwrap();
        std::fs::write(assets.join("logo.svg"), "<svg/>").unwrap();

        let pipeline = Pipeline::from_root(dir.path()).unwrap();

        // First run: misses and copies.
        assert!(!pipeline.step_assets().unwrap());
        assert!(pipeline.step_assets().unwrap(), "second run is a cache hit");

        // Editing an asset invalidates the step.
        std::fs::write(assets.join("logo.svg"), "<svg id='x'/>").unwrap();
        assert!(!pipeline.step_assets().unwrap());

        // index.html is cached too.
        assert!(!pipeline.step_index_html().unwrap());
        assert!(pipeline.step_index_html().unwrap());
    }

    #[test]
    fn disabling_cache_forces_work() {
        let dir = tempfile::tempdir().unwrap();
        write_min_project(dir.path());
        let assets = dir.path().join("assets");
        std::fs::create_dir_all(&assets).unwrap();
        std::fs::write(assets.join("a.txt"), "x").unwrap();

        let pipeline = Pipeline::from_root(dir.path()).unwrap().without_cache();
        assert!(!pipeline.step_assets().unwrap());
        // Without a cache, the step always runs.
        assert!(!pipeline.step_assets().unwrap());
    }

    #[test]
    fn frontend_args_keep_features_flag_with_value() {
        let args = frontend_build_args("website", &[], true, false);
        let joined = args.join(" ");
        assert!(
            joined.contains("--features hydrate --release"),
            "unexpected frontend args: {joined}"
        );
        // The `--features` flag must be immediately followed by its value.
        let idx = args
            .iter()
            .position(|a| a == "--features")
            .expect("--features flag present");
        assert_eq!(args[idx + 1], "hydrate", "args: {joined}");
    }

    #[test]
    fn frontend_args_respect_configured_features() {
        let args = frontend_build_args(
            "website",
            &["hydrate".to_string(), "foo".to_string()],
            false,
            false,
        );
        let joined = args.join(" ");
        assert!(
            joined.contains(
                "--features hydrate,foo --no-default-features --release"
            ),
            "unexpected frontend args: {joined}"
        );
    }

    #[test]
    fn frontend_args_use_hot_profile_when_hot_reloading() {
        let args = frontend_build_args("website", &[], true, true);
        let joined = args.join(" ");
        assert!(
            joined.contains("--features hydrate --profile hot"),
            "unexpected frontend args: {joined}"
        );
        assert!(!joined.contains("--release"), "args: {joined}");
    }

    #[test]
    fn server_args_keep_features_flag_with_value() {
        let args = server_build_args("website", &[], true, false);
        let joined = args.join(" ");
        assert_eq!(
            joined, "build --package website --features ssr",
            "unexpected server args"
        );
        let idx = args
            .iter()
            .position(|a| a == "--features")
            .expect("--features flag present");
        assert_eq!(args[idx + 1], "ssr", "args: {joined}");
    }

    #[test]
    fn server_args_append_release_after_features() {
        let args = server_build_args("website", &[], true, true);
        let joined = args.join(" ");
        assert_eq!(
            joined, "build --package website --features ssr --release",
            "unexpected server args"
        );
        let idx = args
            .iter()
            .position(|a| a == "--features")
            .expect("--features flag present");
        assert_eq!(args[idx + 1], "ssr", "args: {joined}");
    }
}
