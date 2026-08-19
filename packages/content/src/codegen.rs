//! Build-time code generation for content collections.
//! Use this in a build.rs to embed content at compile time.

use std::path::Path;

/// Generate a Rust module string from a content directory.
/// The generated code embeds all entries as static data for compile-time loading.
///
/// # Example (build.rs)
/// ```rust,ignore
/// use montrs_content::codegen;
/// fn main() {
///     codegen::generate("content/posts", "posts").unwrap();
/// }
/// ```
pub fn generate(content_dir: &str, collection_name: &str) -> Result<String, String> {
    let path = Path::new(content_dir);
    if !path.is_dir() {
        return Err(format!("content directory not found: {content_dir}"));
    }

    // Try to load as a generic collection using serde_json::Value for frontmatter
    // Generate the module code
    let mut code = String::new();
    code.push_str(&format!(
        "// Auto-generated content collection: {collection_name}\n"
    ));
code.push_str("#[allow(unused_imports)]\n");
    code.push_str("use serde::Deserialize;\n");
    code.push_str("use montrs_content::Collection;\n");
    code.push_str("use montrs_content::Entry;\n\n");

    let entries = match std::fs::read_dir(path) {
        Ok(entries) => {
            let mut files: Vec<_> = entries
                .filter_map(|e| e.ok())
                .filter(|e| e.file_type().map(|t| t.is_file()).unwrap_or(false))
                .filter(|e| e.path().extension().is_some_and(|ext| ext == "md"))
                .collect();
            files.sort_by_key(|e| e.file_name());

            let mut entry_code = String::new();
            let mut entry_names = Vec::new();

for (i, file) in files.iter().enumerate() {
                let content = std::fs::read_to_string(file.path()).unwrap_or_default();
                let slug = file.path().file_stem().and_then(|s| s.to_str()).unwrap_or("unknown").to_string();
                let var_name = format!("ENTRY_{i}");

                entry_code.push_str(&format!(
                    "const {var_name}: &str = r###\"{content}\"###;\n"
                ));
                entry_names.push((var_name, slug));
            }

            if entry_names.is_empty() {
                code.push_str(&format!(
                    "pub fn {}() -> Collection<serde_json::Value> {{\n    Collection::from_embedded(vec![])\n}}\n",
                    collection_name
                ));
                return Ok(code);
            }

            code.push_str(&entry_code);
            code.push_str(&format!(
                "pub fn {collection_name}() -> Collection<serde_json::Value> {{\n"
            ));
            code.push_str("    Collection::from_embedded(vec![\n");
            for (var_name, slug) in &entry_names {
                code.push_str(&format!(
                    "        montrs_content::Entry::from_embedded({var_name}, \"{slug}\"),\n"
                ));
            }
            code.push_str("    ])\n");
            code.push_str("}\n");

            Ok(())
        }
        Err(e) => Err(format!("failed to read content directory: {e}")),
    };

    entries.map(|_| code)
}

/// Generate and write the content module to OUT_DIR.
pub fn generate_to_out_dir(content_dir: &str, collection_name: &str) -> Result<(), String> {
    let code = generate(content_dir, collection_name)?;
    let out_dir = std::env::var("OUT_DIR").map_err(|e| format!("OUT_DIR not set: {e}"))?;
    let dest = Path::new(&out_dir).join(format!("{collection_name}_collection.rs"));
    std::fs::write(&dest, &code).map_err(|e| format!("failed to write: {e}"))?;
    Ok(())
}
