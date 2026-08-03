use crate::{copy_dir, run_cargo, run_tailwind};
use anyhow::{Result, anyhow};
use montrs_metadata::MontrsMetadata;
use std::path::{Path, PathBuf};
use std::process::Command;

/// The MontRS build pipeline.
pub struct Pipeline {
    pub meta: MontrsMetadata,
    pub project_root: PathBuf,
    pub site_root: PathBuf,
    pub pkg_dir: PathBuf,
    pub server_bin_path: PathBuf,
    pub workspace_target_dir: PathBuf,
}

impl Pipeline {
    pub fn from_root(root: &Path) -> Result<Self> {
        let meta = MontrsMetadata::from_file(root.join("montrs.toml"))?;
        let site_root = root.join(&meta.serve.site_root);
        let pkg_dir = site_root.join(&meta.serve.site_pkg_dir);
        let workspace_target = find_workspace_target_dir(root)?;
        let server_bin_path = workspace_target.join("debug").join(
            meta.serve.bin_package.as_deref().unwrap_or("montrs-server")
        );

        println!(
            "DEBUG: bin_package={:?}, server_bin_path={}",
            meta.serve.bin_package, server_bin_path.display()
        );

        Ok(Self {
            meta,
            project_root: root.to_path_buf(),
            site_root,
            pkg_dir,
            server_bin_path,
            workspace_target_dir: workspace_target,
        })
    }

    pub fn build_server(&self) -> Result<()> {
        println!(" Building server...");
        run_cargo(&self.build_server_args())?;
        println!(" Server built successfully");
        Ok(())
    }

    fn build_server_args(&self) -> Vec<String> {
        let mut args = vec!["build".to_string(), "--package".to_string()];
        if let Some(bin) = &self.meta.serve.bin_package {
            args.push(bin.clone());
        }
        if !self.meta.serve.bin_features.is_empty() {
            args.push("--features".to_string());
            args.push(self.meta.serve.bin_features.join(","));
        }
        if !self.meta.serve.bin_default_features {
            args.push("--no-default-features".to_string());
        }
        args
    }

    pub fn build_frontend(&self) -> Result<()> {
        println!(" Building frontend (WASM)...");
        run_cargo(&self.build_frontend_args())?;

        println!(" Bundling WASM with wasm-bindgen...");
        self.bundle_wasm()?;

        println!(" Frontend built successfully");
        Ok(())
    }

    fn build_frontend_args(&self) -> Vec<String> {
        let mut args = vec![
            "build".to_string(),
            "--target".to_string(),
            "wasm32-unknown-unknown".to_string(),
        ];
        if let Some(lib) = &self.meta.serve.lib_package {
            args.push("--package".to_string());
            args.push(lib.clone());
        }
        if !self.meta.serve.lib_features.is_empty() {
            args.push("--features".to_string());
            args.push(self.meta.serve.lib_features.join(","));
        }
        if !self.meta.serve.lib_default_features {
            args.push("--no-default-features".to_string());
        }
        args
    }

    fn bundle_wasm(&self) -> Result<()> {
        std::fs::create_dir_all(&self.pkg_dir)?;

        let lib_name = self.meta.serve.lib_package.as_deref()
            .unwrap_or("app")
            .replace('-', "_");

        let wasm_target_dir = self.workspace_target_dir
            .join("wasm32-unknown-unknown")
            .join("debug");

        let wasm_file = wasm_target_dir.join(format!("{}.wasm", lib_name));

        if !wasm_file.exists() {
            return Err(anyhow!(
                "WASM file not found at {}. Did the wasm32-unknown-unknown build succeed?",
                wasm_file.display()
            ));
        }

        // Run wasm-bindgen to produce the JS glue + bundled WASM
        let status = Command::new("wasm-bindgen")
            .arg("--target")
            .arg("web")
            .arg("--no-typescript")
            .arg("--out-dir")
            .arg(&self.pkg_dir)
            .arg("--out-name")
            .arg("front")
            .arg(&wasm_file)
            .status();

        match status {
            Ok(s) if s.success() => {
                println!(" wasm-bindgen completed successfully");
            }
            Ok(_) => {
                println!(" wasm-bindgen failed — falling back to manual copy");
                self.fallback_copy_wasm(&wasm_file, &lib_name)?;
            }
            Err(_e) => {
                println!(" wasm-bindgen not found — falling back to manual copy");
                self.fallback_copy_wasm(&wasm_file, &lib_name)?;
            }
        }

        Ok(())
    }

    fn fallback_copy_wasm(&self, wasm_file: &Path, lib_name: &str) -> Result<()> {
        std::fs::copy(wasm_file, self.pkg_dir.join("front.wasm"))?;
        let wasm_target_dir = self.workspace_target_dir
            .join("wasm32-unknown-unknown")
            .join("debug");
        let js_bindings = wasm_target_dir.join(format!("{}.js", lib_name));
        if js_bindings.exists() {
            std::fs::copy(&js_bindings, self.pkg_dir.join("front.js"))?;
        }
        Ok(())
    }

    pub fn process_tailwind(&self) -> Result<()> {
        if let Some(tw_input) = &self.meta.serve.tailwind_input_file {
            let input = self.project_root.join(tw_input);
            let output = self.site_root.join("main.css");
            if input.exists() {
                println!(" Processing Tailwind CSS...");
                std::fs::create_dir_all(&self.site_root)?;
                run_tailwind(&input, &output)?;
                println!(" Tailwind CSS processed");
            }
        }
        Ok(())
    }

    pub fn copy_assets(&self) -> Result<()> {
        if let Some(assets) = &self.meta.serve.assets_dir {
            let src = self.project_root.join(assets);
            if src.exists() {
                println!(" Copying assets...");
                copy_dir(&src, &self.site_root)?;
                println!(" Assets copied");
            }
        }
        Ok(())
    }

    pub fn build_all(&self) -> Result<()> {
        std::fs::create_dir_all(&self.site_root)?;
        std::fs::create_dir_all(&self.pkg_dir)?;
        if self.meta.serve.bin_package.is_some() {
            self.build_server()?;
        }
        if self.meta.serve.lib_package.is_some() {
            self.build_frontend()?;
        }
        self.process_tailwind()?;
        self.copy_assets()?;

        self.generate_fallback_html()?;

        println!(" Build complete");
        Ok(())
    }

    fn generate_fallback_html(&self) -> Result<()> {
        let index_path = self.site_root.join("index.html");
        if index_path.exists() {
            return Ok(());
        }
        let project_name = self.meta.project.name.as_deref().unwrap_or("MontRS App");
        let html = format!(
            r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="utf-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    <title>{}</title>
    <link rel="stylesheet" href="/main.css" />
</head>
<body>
    <div id="app">
        <h1 style="text-align:center;margin-top:20vh;font-family:sans-serif">{}</h1>
        <p style="text-align:center;font-family:sans-serif;color:#666">Static dev server — build the WASM frontend or SSR server to see full content.</p>
    </div>
</body>
</html>"#,
            project_name, project_name
        );
        std::fs::write(&index_path, html)?;
        println!(" Generated fallback index.html");
        Ok(())
    }
}

fn find_workspace_target_dir(app_root: &Path) -> Result<PathBuf> {
    // Walk up from app root looking for the workspace root (has Cargo.toml with [workspace])
    let mut current = app_root.to_path_buf();
    loop {
        let cargo_toml = current.join("Cargo.toml");
        if cargo_toml.exists() {
            if let Ok(content) = std::fs::read_to_string(&cargo_toml) {
                if content.contains("[workspace]") {
                    return Ok(current.join("target"));
                }
            }
        }
        if !current.pop() {
            break;
        }
    }
    // Fallback to app's own target
    Ok(app_root.join("target"))
}
