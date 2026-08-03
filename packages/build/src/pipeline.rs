use montrs_metadata::MontrsMetadata;
use crate::{run_cargo, run_tailwind, copy_dir};
use anyhow::Result;
use std::path::{Path, PathBuf};

/// The MontRS build pipeline.
pub struct Pipeline {
    pub meta: MontrsMetadata,
    pub project_root: PathBuf,
    pub site_root: PathBuf,
    pub pkg_dir: PathBuf,
}

impl Pipeline {
    pub fn from_root(root: &Path) -> Result<Self> {
        let meta = MontrsMetadata::from_file(root.join("montrs.toml"))?;
        let site_root = root.join(&meta.serve.site_root);
        let pkg_dir = site_root.join(&meta.serve.site_pkg_dir);

        Ok(Self {
            meta,
            project_root: root.to_path_buf(),
            site_root,
            pkg_dir,
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
        args
    }

    pub fn build_frontend(&self) -> Result<()> {
        println!(" Building frontend (WASM)...");
        run_cargo(&self.build_frontend_args())?;
        println!(" Frontend built successfully");
        Ok(())
    }

    fn build_frontend_args(&self) -> Vec<String> {
        let mut args = vec![
            "build".to_string(),
            "--package".to_string(),
            "--target".to_string(),
            "wasm32-unknown-unknown".to_string(),
        ];
        if let Some(lib) = &self.meta.serve.lib_package {
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

    pub fn copy_wasm_package(&self) -> Result<()> {
        let target_dir = self.project_root.join("target");
        let wasm_dir = target_dir.join("wasm32-unknown-unknown").join("debug");

        if let Some(lib) = &self.meta.serve.lib_package {
            let lib_name = lib.replace('-', "_");
            let wasm_file = wasm_dir.join(format!("{}.wasm", lib_name));
            let js_bindings = wasm_dir.join(format!("{}.js", lib_name));

            std::fs::create_dir_all(&self.pkg_dir)?;
            if wasm_file.exists() {
                std::fs::copy(&wasm_file, self.pkg_dir.join("front.wasm"))?;
            }
            if js_bindings.exists() {
                std::fs::copy(&js_bindings, self.pkg_dir.join("front.js"))?;
            }
        }
        Ok(())
    }

    pub fn build_all(&self) -> Result<()> {
        std::fs::create_dir_all(&self.site_root)?;
        std::fs::create_dir_all(&self.pkg_dir)?;
        self.build_server()?;
        self.build_frontend()?;
        self.process_tailwind()?;
        self.copy_assets()?;
        self.copy_wasm_package()?;
        println!(" Build complete");
        Ok(())
    }
}