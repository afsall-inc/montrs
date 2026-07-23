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
