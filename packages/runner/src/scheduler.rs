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

use crate::types::{Task, TaskOutput};
use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};
use tokio::sync::Semaphore;

/// Dependency graph for task scheduling.
///
/// Edges go from a task to the tasks it depends on: `task -> dependency`.
/// Stored as an adjacency map: `task name -> set of dependency names`.
pub struct Deps {
    graph: HashMap<String, HashSet<String>>,
    executed: HashSet<String>,
    did_work: HashSet<String>,
}

impl Deps {
    pub fn new(tasks: &[Task]) -> Self {
        let mut graph: HashMap<String, HashSet<String>> = HashMap::new();
        for task in tasks {
            graph.entry(task.name.clone()).or_default();
        }
        for task in tasks {
            for dep in &task.depends {
                let dep_name = dep.task_name();
                // Only add an edge if the dependency is a known task node.
                if graph.contains_key(dep_name)
                    && let Some(edges) = graph.get_mut(&task.name)
                {
                    edges.insert(dep_name.to_string());
                }
            }
        }
        Self {
            graph,
            executed: HashSet::new(),
            did_work: HashSet::new(),
        }
    }

    pub fn leaf_tasks(&self) -> Vec<String> {
        // Leaf = node with no incoming edges. Incoming edges to `x` come from
        // tasks that depend on `x`, so `x` is not a leaf iff it appears in any
        // dependency set.
        let all_deps: HashSet<&String> =
            self.graph.values().flatten().collect();
        self.graph
            .keys()
            .filter(|name| !all_deps.contains(name))
            .cloned()
            .collect()
    }

    pub fn is_empty(&self) -> bool {
        self.graph.is_empty()
    }

    pub fn remove(&mut self, task: &str) {
        self.graph.remove(task);
        // Drop edges pointing to this task from remaining tasks.
        for edges in self.graph.values_mut() {
            edges.remove(task);
        }
        self.executed.insert(task.to_string());
    }

    pub fn mark_did_work(&mut self, task: &str) {
        self.did_work.insert(task.to_string());
    }

    pub fn all_done(&self) -> bool {
        self.graph.is_empty()
    }

    /// Detect cycles using simple DFS.
    pub fn detect_cycles(&self) -> Vec<Vec<String>> {
        let mut cycles = Vec::new();
        let mut visited = HashSet::new();

        // Snapshot the node names so we don't mutate while iterating.
        let nodes: Vec<String> = self.graph.keys().cloned().collect();
        for name in nodes {
            if !visited.contains(&name) {
                let mut path = Vec::new();
                if self.dfs_cycle(&name, &mut visited, &mut path)
                    && let Some(pos) = path.iter().position(|n| n == &name)
                {
                    cycles.push(path[pos..].to_vec());
                }
            }
        }
        cycles
    }

    fn dfs_cycle(
        &self,
        name: &str,
        visited: &mut HashSet<String>,
        path: &mut Vec<String>,
    ) -> bool {
        if path.contains(&name.to_string()) {
            return true;
        }
        if visited.contains(name) {
            return false;
        }
        visited.insert(name.to_string());
        path.push(name.to_string());

        // Follow outgoing edges: task -> its dependencies.
        if let Some(deps) = self.graph.get(name) {
            for dep_name in deps {
                if self.dfs_cycle(dep_name, visited, path) {
                    return true;
                }
            }
        }

        path.pop();
        false
    }
}

/// Scheduler for parallel task execution with concurrency control.
pub struct Scheduler {
    pub semaphore: Arc<Semaphore>,
    pub deps: Deps,
}

impl Scheduler {
    pub fn new(tasks: &[Task], jobs: usize) -> Self {
        Self {
            semaphore: Arc::new(Semaphore::new(jobs)),
            deps: Deps::new(tasks),
        }
    }

    pub fn detect_cycles(&self) -> Vec<Vec<String>> {
        self.deps.detect_cycles()
    }

    pub fn topological_sort(&self) -> Vec<Vec<String>> {
        let all_names: Vec<Task> = self
            .deps
            .graph
            .keys()
            .map(|name| Task {
                name: name.clone(),
                depends: vec![],
                ..Default::default()
            })
            .collect();
        let mut levels = Vec::new();
        let mut deps_copy = Deps::new(&all_names);
        // Rebuild edges
        // We can't easily iterate edges from DiGraph, so we skip for now
        while !deps_copy.is_empty() {
            let leaves = deps_copy.leaf_tasks();
            if leaves.is_empty() {
                break;
            }
            levels.push(leaves.clone());
            for leaf in &leaves {
                deps_copy.remove(leaf);
            }
        }
        levels
    }
}

/// Resolve the output style for a task.
pub fn resolve_output(
    task: &Task,
    global_output: Option<TaskOutput>,
) -> TaskOutput {
    task.output.or(global_output).unwrap_or(TaskOutput::Prefix)
}

/// Whether a task needs a semaphore permit.
pub fn task_needs_permit(task: &Task) -> bool {
    !task.command.is_empty() || task.file.is_some()
}
