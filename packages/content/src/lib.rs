//! Typed, deterministic Markdown content collections.

pub mod codegen;

use pulldown_cmark::{Options, Parser, html};
use serde::de::DeserializeOwned;
use std::{fs, path::Path};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ContentError {
    #[error("failed to read content: {0}")]
    Io(#[from] std::io::Error),
    #[error("content file is missing YAML frontmatter")]
    MissingFrontmatter,
    #[error("invalid YAML frontmatter: {0}")]
    Frontmatter(#[from] serde_yml::Error),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Entry<T> {
    pub slug: String,
    pub data: T,
    pub body: String,
}

impl<T> Entry<T> {
    pub fn render(&self) -> String {
        let parser = Parser::new_ext(&self.body, Options::all());
        let mut rendered = String::new();
        html::push_html(&mut rendered, parser);
        rendered
    }
}

impl Entry<serde_json::Value> {
    /// Create an entry from an embedded raw markdown source string (build-time codegen).
    pub fn from_embedded(source: &str, slug: &str) -> Self {
        match parse::<serde_json::Value>(source, slug) {
            Ok(entry) => entry,
            // Fall back to a minimal entry when frontmatter is absent so that
            // embedded collections remain resilient at compile time.
            Err(_) => Self {
                slug: slug.to_string(),
                data: serde_json::Value::Null,
                body: source.to_string(),
            },
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Collection<T> {
    entries: Vec<Entry<T>>,
}

impl<T> Collection<T> {
    pub fn from_entries(mut entries: Vec<Entry<T>>) -> Self {
        entries.sort_by(|left, right| left.slug.cmp(&right.slug));
        Self { entries }
    }

    /// Build a collection from entries embedded at compile time (build.rs codegen).
    pub fn from_embedded(entries: Vec<Entry<T>>) -> Self {
        Self::from_entries(entries)
    }

    pub fn entries(&self) -> &[Entry<T>] {
        &self.entries
    }

    pub fn into_entries(self) -> Vec<Entry<T>> {
        self.entries
    }
}

impl<T> Collection<T>
where
    T: DeserializeOwned,
{
    /// Load a collection from a directory at runtime (SSR/file-system mode).
    pub fn load(path: impl AsRef<Path>) -> Result<Self, ContentError> {
        Self::load_directory(path)
    }

    pub fn load_directory(
        path: impl AsRef<Path>,
    ) -> Result<Self, ContentError> {
        let mut files = fs::read_dir(path)?.collect::<Result<Vec<_>, _>>()?;
        files.sort_by_key(|entry| entry.file_name());
        let mut entries = Vec::new();
        for file in files {
            if file.file_type()?.is_file()
                && file.path().extension().is_some_and(|ext| ext == "md")
            {
                entries.push(parse_file(file.path())?);
            }
        }
        Ok(Self::from_entries(entries))
    }
}

pub fn parse<T>(
    source: &str,
    slug: impl Into<String>,
) -> Result<Entry<T>, ContentError>
where
    T: DeserializeOwned,
{
    let (frontmatter, body) = split_frontmatter(source)?;
    Ok(Entry {
        slug: slug.into(),
        data: serde_yml::from_str(frontmatter)?,
        body: body.to_string(),
    })
}

fn parse_file<T>(path: impl AsRef<Path>) -> Result<Entry<T>, ContentError>
where
    T: DeserializeOwned,
{
    let path = path.as_ref();
    let slug = path
        .file_stem()
        .and_then(|value| value.to_str())
        .unwrap_or_default()
        .to_string();
    parse(&fs::read_to_string(path)?, slug)
}

fn split_frontmatter(source: &str) -> Result<(&str, &str), ContentError> {
    let source = source
        .strip_prefix("---\n")
        .ok_or(ContentError::MissingFrontmatter)?;
    let end = source
        .find("\n---")
        .ok_or(ContentError::MissingFrontmatter)?;
    let (frontmatter, body) = source.split_at(end);
    Ok((frontmatter, body.trim_start_matches("\n---").trim_start()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Deserialize;

    #[derive(Debug, Deserialize, PartialEq, Eq)]
    struct Frontmatter {
        title: String,
    }

    #[test]
    fn parses_and_renders_markdown() {
        let entry =
            parse::<Frontmatter>("---\ntitle: Hello\n---\n# Welcome", "hello")
                .unwrap();
        assert_eq!(entry.data.title, "Hello");
        assert!(entry.render().contains("<h1>Welcome</h1>"));
    }

    #[test]
    fn rejects_missing_frontmatter() {
        assert!(matches!(
            parse::<Frontmatter>("# no frontmatter", "no"),
            Err(ContentError::MissingFrontmatter)
        ));
    }
}
