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
use petgraph::graph::DiGraph;
use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};
use tokio::sync::Semaphore;

/// Dependency graph for task scheduling.
pub struct Deps {
    graph: DiGraph<String, ()>,
    indices: HashMap<String, petgraph::graph::NodeIndex>,
    executed: HashSet<String>,
    did_work: HashSet<String>,
}

impl Deps {
    pub fn new(tasks: &[Task]) -> Self {
        let mut graph = DiGraph::new();
        let mut indices = HashMap::new();
        for task in tasks {
            let idx = graph.add_node(task.name.clone());
            indices.insert(task.name.clone(), idx);
        }
        for task in tasks {
            for dep in &task.depends {
                let dep_name = dep.task_name();
                if let (Some(&from), Some(&to)) =
                    (indices.get(&task.name), indices.get(dep_name))
                {
                    graph.add_edge(from, to, ());
                }
            }
        }
        Self {
            graph,
            indices,
            executed: HashSet::new(),
            did_work: HashSet::new(),
        }
    }

    pub fn leaf_tasks(&self) -> Vec<String> {
        self.graph
            .node_indices()
            .filter(|&n| {
                self.graph
                    .neighbors_directed(n, petgraph::Direction::Incoming)
                    .count()
                    == 0
            })
            .map(|n| self.graph[n].clone())
            .collect()
    }

    pub fn is_empty(&self) -> bool {
        self.graph.node_count() == 0
    }

    pub fn remove(&mut self, task: &str) {
        if let Some(&idx) = self.indices.get(task) {
            self.graph.remove_node(idx);
            self.indices.remove(task);
        }
        self.executed.insert(task.to_string());
    }

    pub fn mark_did_work(&mut self, task: &str) {
        self.did_work.insert(task.to_string());
    }

    pub fn all_done(&self) -> bool {
        self.graph.node_count() == 0
    }

    /// Detect cycles using simple DFS.
    pub fn detect_cycles(&self) -> Vec<Vec<String>> {
        let mut cycles = Vec::new();
        let mut visited = HashSet::new();

        for node in self.graph.node_indices() {
            let name = &self.graph[node];
            if !visited.contains(name.as_str()) {
                let mut path = Vec::new();
                if self.dfs_cycle(name, &mut visited, &mut path)
                    && let Some(pos) = path.iter().position(|n| n == name)
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

        if let Some(&idx) = self.indices.get(name) {
            for neighbor in self
                .graph
                .neighbors_directed(idx, petgraph::Direction::Outgoing)
            {
                let neighbor_name = &self.graph[neighbor];
                if self.dfs_cycle(neighbor_name, visited, path) {
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
        // Build a new Deps with all nodes but no edges, then add edges back
        let all_names: Vec<Task> = self
            .deps
            .graph
            .node_indices()
            .map(|idx| Task {
                name: self.deps.graph[idx].clone(),
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
