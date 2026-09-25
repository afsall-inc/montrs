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
        // Edges are `task -> dependency`. A task is ready once every one of its
        // dependencies has been emitted. Each round emits one level; ties are
        // sorted for determinism. Cycles are handled by the builder/validator,
        // but a cycle here simply leaves the remaining nodes unemitted.
        let mut remaining = self.deps.graph.clone();
        let mut done: HashSet<String> = HashSet::new();
        let mut levels = Vec::new();

        while !remaining.is_empty() {
            let mut ready: Vec<String> = remaining
                .iter()
                .filter(|(_, deps)| deps.iter().all(|d| done.contains(d)))
                .map(|(name, _)| name.clone())
                .collect();
            if ready.is_empty() {
                // Only a cycle (or a dependency on an unknown task) remains.
                break;
            }
            ready.sort();
            for name in &ready {
                remaining.remove(name);
                done.insert(name.clone());
            }
            levels.push(ready);
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::TaskDep;

    fn task(name: &str, depends: &[&str]) -> Task {
        Task {
            name: name.to_string(),
            depends: depends
                .iter()
                .map(|d| TaskDep::Simple(d.to_string()))
                .collect(),
            ..Default::default()
        }
    }

    #[test]
    fn topological_sort_respects_dependencies() {
        let tasks = vec![
            task("ship", &["test", "build"]),
            task("test", &["build"]),
            task("build", &[]),
            task("lint", &[]),
        ];
        let scheduler = Scheduler::new(&tasks, 4);
        let levels = scheduler.topological_sort();

        assert_eq!(levels.len(), 3, "levels: {levels:?}");
        // Level 0: no dependencies.
        assert_eq!(levels[0], vec!["build".to_string(), "lint".to_string()]);
        // Level 1: test depends on build.
        assert_eq!(levels[1], vec!["test".to_string()]);
        // Level 2: ship depends on test + build.
        assert_eq!(levels[2], vec!["ship".to_string()]);
    }

    #[test]
    fn topological_sort_handles_missing_dependency() {
        // A dependency on an unregistered task is ignored (no edge added), so
        // the task is a root.
        let tasks = vec![task("a", &["does-not-exist"])];
        let levels = Scheduler::new(&tasks, 2).topological_sort();
        assert_eq!(levels, vec![vec!["a".to_string()]]);
    }

    #[test]
    fn topological_sort_stops_on_cycle() {
        let tasks = vec![task("a", &["b"]), task("b", &["a"])];
        let levels = Scheduler::new(&tasks, 2).topological_sort();
        assert!(
            levels.iter().flatten().all(|n| n != "a" && n != "b"),
            "a cycle must emit nothing: {levels:?}"
        );
    }
}
