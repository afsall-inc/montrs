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

use crate::{FormatError, FormatterSettings};
use crop::Rope;
use montrs_utils::to_kebab_case;
use quote::ToTokens;
use rstml::node::{Node, NodeAttribute, NodeElement};
use syn::{
    File, Macro,
    visit::{self, Visit},
};

#[derive(Debug)]
pub struct MacroEdit {
    pub start_line: usize,
    pub start_col: usize,
    pub end_line: usize,
    pub end_col: usize,
    pub new_content: String,
}

pub fn collect_and_format_macros(
    file: &File,
    _source: &Rope,
    settings: &FormatterSettings,
    edits: &mut Vec<MacroEdit>,
) -> Result<(), FormatError> {
    let mut visitor = MacroVisitor {
        settings,
        edits,
        errors: Vec::new(),
    };
    visitor.visit_file(file);

    if !visitor.errors.is_empty() {
        return Err(FormatError::Macro(visitor.errors.join(", ")));
    }

    Ok(())
}

struct MacroVisitor<'a> {
    settings: &'a FormatterSettings,
    edits: &'a mut Vec<MacroEdit>,
    errors: Vec<String>,
}

impl<'ast> Visit<'ast> for MacroVisitor<'_> {
    fn visit_macro(&mut self, i: &'ast Macro) {
        if self.is_view_macro(i) {
            match self.format_macro(i) {
                Ok(formatted) => {
                    let span = i.delimiter.span().join();

                    self.edits.push(MacroEdit {
                        start_line: span.start().line,
                        start_col: span.start().column,
                        end_line: span.end().line,
                        end_col: span.end().column,
                        new_content: formatted,
                    });
                }
                Err(e) => {
                    self.errors.push(e.to_string());
                }
            }
        }
        visit::visit_macro(self, i);
    }
}

impl MacroVisitor<'_> {
    fn is_view_macro(&self, mac: &Macro) -> bool {
        let name = mac
            .path
            .segments
            .last()
            .map(|s| s.ident.to_string())
            .unwrap_or_default();
        self.settings.view.macro_names.contains(&name)
    }

    fn format_macro(&self, mac: &Macro) -> Result<String, FormatError> {
        let tokens = mac.tokens.clone();

        // rstml 0.12.x provides a top-level parse2 function
        let nodes = rstml::parse2(tokens)
            .map_err(|e| FormatError::Macro(e.to_string()))?;

        let mut printer = RstmlPrinter {
            settings: self.settings,
            indent: self.settings.tab_spaces, // Start with one level of indentation
            result: String::new(),
        };

        printer.print_nodes(&nodes);

        let result = printer.result.trim_end();

        // Return only the contents of the braces, with the braces themselves
        Ok(format!("{{\n{result}\n}}"))
    }
}

struct RstmlPrinter<'a> {
    settings: &'a FormatterSettings,
    indent: usize,
    result: String,
}

impl RstmlPrinter<'_> {
    fn print_nodes<C>(&mut self, nodes: &[Node<C>])
    where
        C: rstml::node::CustomNode + ToTokens + std::fmt::Debug,
    {
        for node in nodes {
            self.print_node(node);
        }
    }

    fn print_node<C>(&mut self, node: &Node<C>)
    where
        C: rstml::node::CustomNode + ToTokens + std::fmt::Debug,
    {
        match node {
            Node::Element(el) => self.print_element(el),
            Node::Text(text) => {
                self.add_indent();
                self.result
                    .push_str(&text.value.to_token_stream().to_string());
                self.result.push('\n');
            }
            Node::Block(block) => {
                self.add_indent();
                self.result.push_str("{ ");
                self.result.push_str(&block.to_token_stream().to_string());
                self.result.push_str(" }\n");
            }
            _ => {} // Handle other nodes as needed
        }
    }

    fn print_element<C>(&mut self, el: &NodeElement<C>)
    where
        C: rstml::node::CustomNode + ToTokens + std::fmt::Debug,
    {
        self.add_indent();
        let original_name = el.name().to_string();
        let name = if original_name
            .chars()
            .next()
            .map(|c| c.is_uppercase())
            .unwrap_or(false)
        {
            // Component: Force PascalCase
            montrs_utils::to_pascal_case(&original_name)
        } else {
            // HTML Tag: Force lowercase
            original_name.to_lowercase()
        };
        self.result.push('<');
        self.result.push_str(&name);

        for attr in el.attributes() {
            self.result.push(' ');
            self.print_attribute(attr);
        }

        if el.children.is_empty()
            && self.settings.view.closing_tag_style
                == crate::config::ClosingTagStyle::SelfClosing
        {
            self.result.push_str(" />\n");
        } else {
            self.result.push_str(">\n");
            self.indent += self.settings.tab_spaces;
            self.print_nodes(&el.children);
            self.indent -= self.settings.tab_spaces;
            self.add_indent();
            self.result.push_str("</");
            self.result.push_str(&name);
            self.result.push_str(">\n");
        }
    }

    fn print_attribute(&mut self, attr: &NodeAttribute) {
        match attr {
            NodeAttribute::Block(block) => {
                self.result.push_str(&block.to_token_stream().to_string());
            }
            NodeAttribute::Attribute(a) => {
                self.result.push_str(&to_kebab_case(&a.key.to_string()));
                if let Some(value) = a.value() {
                    self.result.push('=');
                    self.result.push_str(&value.to_token_stream().to_string());
                }
            }
        }
    }

    fn add_indent(&mut self) {
        for _ in 0..self.indent {
            self.result.push(' ');
        }
    }
}

pub fn apply_edits(source: &mut Rope, edits: Vec<MacroEdit>) {
    let mut sorted_edits = edits;
    sorted_edits.sort_by(|a, b| {
        if a.start_line != b.start_line {
            b.start_line.cmp(&a.start_line)
        } else {
            b.start_col.cmp(&a.start_col)
        }
    });

    for edit in sorted_edits {
        let start_offset =
            line_col_to_byte_offset(source, edit.start_line, edit.start_col);
        let end_offset =
            line_col_to_byte_offset(source, edit.end_line, edit.end_col);

        if let (Some(start), Some(end)) = (start_offset, end_offset) {
            source.replace(start..end, &edit.new_content);
        }
    }
}

fn line_col_to_byte_offset(
    source: &Rope,
    line: usize,
    col: usize,
) -> Option<usize> {
    if line == 0 || line > source.line_len() {
        return None;
    }
    let line_start = source.byte_of_line(line - 1);
    Some(line_start + col)
}
