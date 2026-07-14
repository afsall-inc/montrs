use serde::{Deserialize, Serialize};

/// A skill definition loaded from a skill.toml manifest.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillManifest {
    pub skill: SkillMeta,
    pub workflow: SkillWorkflow,
    pub context: SkillContext,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillMeta {
    pub name: String,
    pub version: String,
    pub description: String,
    pub author: String,
    #[serde(default)]
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillWorkflow {
    pub steps: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillContext {
    #[serde(default)]
    pub prompts: Vec<String>,
    #[serde(default)]
    pub invariants: Vec<String>,
}

/// Discover all skills from the skills/ directory.
pub fn discover_skills(root_path: &std::path::Path) -> Vec<SkillManifest> {
    let skills_dir = root_path.join("skills");
    if !skills_dir.exists() {
        return Vec::new();
    }

    let mut skills = Vec::new();
    if let Ok(entries) = std::fs::read_dir(&skills_dir) {
        for entry in entries.flatten() {
            if entry.path().is_dir() {
                let manifest_path = entry.path().join("skill.toml");
                if manifest_path.exists()
                    && let Ok(content) = std::fs::read_to_string(&manifest_path)
                    && let Ok(manifest) =
                        toml::from_str::<SkillManifest>(&content)
                {
                    skills.push(manifest);
                }
            }
        }
    }
    skills
}

/// Convert discovered skills into the tools.json format.
pub fn skills_to_tools_json(
    skills: &[SkillManifest],
) -> Vec<serde_json::Value> {
    skills
        .iter()
        .map(|s| {
            serde_json::json!({
                "name": format!("skill_{}", s.skill.name.replace('-', "_")),
                "description": s.skill.description,
                "parameters": {
                    "type": "object",
                    "properties": {
                        "workflow": {
                            "type": "array",
                            "items": { "type": "string" },
                            "description": "Steps to follow for this skill"
                        }
                    }
                },
                "skill_meta": {
                    "name": s.skill.name,
                    "version": s.skill.version,
                    "tags": s.skill.tags,
                    "workflow_steps": s.workflow.steps,
                    "context_prompts": s.context.prompts,
                    "context_invariants": s.context.invariants,
                }
            })
        })
        .collect()
}
