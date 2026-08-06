use crate::{copy_dir, run_cargo, run_tailwind};
use anyhow::{Result, anyhow};
use montrs_build_core::{BuildPipeline, find_workspace_target_dir};
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
}

impl Pipeline {
    pub fn from_root(root: &Path) -> Result<Self> {
        let root = root.canonicalize()?;
        let meta = MontrsMetadata::from_file(root.join("montrs.toml"))?;
        let site_root = root.join(&meta.serve.site_root);
        let pkg_dir = site_root.join(&meta.serve.site_pkg_dir);
        let workspace_target = find_workspace_target_dir(&root)?;
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
        })
    }

    fn build_frontend_args(&self) -> Vec<String> {
        let pkg = self.meta.serve.package.as_deref().unwrap_or("app");
        let mut args = vec![
            "build".to_string(),
            "--target".to_string(),
            "wasm32-unknown-unknown".to_string(),
            "--package".to_string(),
            pkg.to_string(),
            "--features".to_string(),
        ];
        let features = if self.meta.serve.lib_features.is_empty() {
            "hydrate".to_string()
        } else {
            self.meta.serve.lib_features.join(",")
        };
        args.push(features);
        if !self.meta.serve.lib_default_features {
            args.push("--no-default-features".to_string());
        }
        args
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

        let wasm_target_dir = self
            .workspace_target_dir
            .join("wasm32-unknown-unknown")
            .join("debug");

        let wasm_file = wasm_target_dir.join(format!("{}.wasm", lib_name));

        if !wasm_file.exists() {
            return Err(anyhow!(
                "WASM file not found at {}. Did the wasm32-unknown-unknown \
                 build succeed?",
                wasm_file.display()
            ));
        }

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
                println!(
                    " wasm-bindgen not found — falling back to manual copy"
                );
                self.fallback_copy_wasm(&wasm_file, &lib_name)?;
            }
        }

        Ok(())
    }

    fn fallback_copy_wasm(
        &self,
        wasm_file: &Path,
        lib_name: &str,
    ) -> Result<()> {
        std::fs::copy(wasm_file, self.pkg_dir.join("front.wasm"))?;
        let wasm_target_dir = self
            .workspace_target_dir
            .join("wasm32-unknown-unknown")
            .join("debug");
        let js_bindings = wasm_target_dir.join(format!("{}.js", lib_name));
        if js_bindings.exists() {
            std::fs::copy(&js_bindings, self.pkg_dir.join("front.js"))?;
        }
        Ok(())
    }
}

impl BuildPipeline for Pipeline {
    fn build_server(&self) -> Result<()> {
        println!(" Building SSR server...");
        let pkg = self.meta.serve.package.as_deref().unwrap_or("app");
        let mut args = vec![
            "build".to_string(),
            "--package".to_string(),
            pkg.to_string(),
            "--features".to_string(),
        ];
        let features = if self.meta.serve.bin_features.is_empty() {
            "ssr".to_string()
        } else {
            self.meta.serve.bin_features.join(",")
        };
        args.push(features);
        if !self.meta.serve.bin_default_features {
            args.push("--no-default-features".to_string());
        }
        run_cargo(&args)?;
        println!(" SSR server built successfully");
        Ok(())
    }

    fn build_frontend(&self) -> Result<()> {
        println!(" Building frontend (WASM)...");
        run_cargo(&self.build_frontend_args())?;
        println!(" Bundling WASM with wasm-bindgen...");
        self.bundle_wasm()?;
        println!(" Frontend built successfully");
        Ok(())
    }

    fn process_tailwind(&self) -> Result<()> {
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

    fn copy_assets(&self) -> Result<()> {
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
    <link rel="stylesheet" href="/main.css" />
    <link rel="modulepreload" href="/pkg/front.js" />
    <script type="module">
        import init, {{ hydrate }} from '/pkg/front.js';
        init('/pkg/front.wasm').then(() => hydrate());
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

        self.build_server()?;
        self.build_frontend()?;
        self.process_tailwind()?;
        self.copy_assets()?;
        self.generate_index_html()?;

        println!(" Build complete");
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