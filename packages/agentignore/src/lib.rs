use ignore::gitignore::{Gitignore, GitignoreBuilder};
use std::path::{Path, PathBuf};

pub struct AgentIgnore {
    root: PathBuf,
    matcher: Gitignore,
    patterns: Vec<String>,
}

impl AgentIgnore {
    pub fn load(root: &Path) -> Result<Self, String> {
        let path = root.join(".agentignore");
        let (m, p) = if path.exists() {
            let c = std::fs::read_to_string(&path)
                .map_err(|e| format!("read error: {}", e))?;
            let p: Vec<String> = c
                .lines()
                .map(|l| l.trim().to_string())
                .filter(|l| !l.is_empty() && !l.starts_with("#"))
                .collect();
            let mut b = GitignoreBuilder::new(root);
            for pat in &p {
                b.add_line(None, pat)
                    .map_err(|e| format!("pattern error: {}", e))?;
            }
            let m = b.build().map_err(|e| format!("build error: {}", e))?;
            (m, p)
        } else {
            let mut b = GitignoreBuilder::new(root);
            let _ = b.add_line(None, "");
            let m = b.build().map_err(|e| format!("build error: {}", e))?;
            (m, Vec::new())
        };
        Ok(Self {
            root: root.to_path_buf(),
            matcher: m,
            patterns: p,
        })
    }

    pub fn create_from_gitignore(root: &Path) -> Result<Vec<String>, String> {
        let git = root.join(".gitignore");
        let mut p: Vec<String> = vec!["target/", "*.rs.bk", ".references/"]
            .into_iter()
            .map(|s| s.to_string())
            .collect();
        if git.exists() {
            let c = std::fs::read_to_string(&git)
                .map_err(|e| format!("read error: {}", e))?;
            for line in c.lines() {
                let t = line.trim();
                if !t.is_empty()
                    && !t.starts_with("#")
                    && !p.contains(&t.to_string())
                {
                    p.push(t.to_string());
                }
            }
        }
        std::fs::write(root.join(".agentignore"), p.join("\n"))
            .map_err(|e| format!("write error: {}", e))?;
        Ok(p)
    }

    pub fn is_ignored(&self, path: &Path) -> bool {
        let r = path.strip_prefix(&self.root).unwrap_or(path);
        self.matcher.matched(r, path.is_dir()).is_ignore()
    }

    pub fn patterns(&self) -> &[String] {
        &self.patterns
    }

    pub fn export_for_ide(root: &Path, ide: &str) -> Result<String, String> {
        let ap = root.join(".agentignore");
        let c = if ap.exists() {
            std::fs::read_to_string(&ap)
                .map_err(|e| format!("read error: {}", e))?
        } else {
            Self::create_from_gitignore(root)?.join("\n")
        };
        match ide {
            "opencode" => {
                std::fs::write(root.join(".opencodeignore"), &c)
                    .map_err(|e| format!("write error: {}", e))?;
                Ok("Exported to .opencodeignore".to_string())
            }
            "cursor" => {
                std::fs::write(root.join(".cursorignore"), &c)
                    .map_err(|e| format!("write error: {}", e))?;
                Ok("Exported to .cursorignore".to_string())
            }
            "trae" => {
                let d = root.join(".trae");
                if !d.exists() {
                    std::fs::create_dir_all(&d)
                        .map_err(|e| format!("mkdir error: {}", e))?;
                }
                std::fs::write(d.join(".agentignore"), &c)
                    .map_err(|e| format!("write error: {}", e))?;
                Ok("Exported to .trae/.agentignore".to_string())
            }
            _ => {
                Err("Unknown IDE. Supported: opencode, cursor, trae"
                    .to_string())
            }
        }
    }

    pub fn check_path(root: &Path, s: &str) -> Result<bool, String> {
        Ok(Self::load(root)?.is_ignored(&root.join(s)))
    }
}
