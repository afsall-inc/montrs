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

//! Pre-built UI components for MontRS applications.
//!
//! Each component uses the `variants!` macro for type-safe props,
//! `cn!()` for class merging, and themed CSS variables.

pub mod accordion;
pub mod action_bar;
pub mod alert;
pub mod alert_dialog;
pub mod animate;
pub mod aspect_ratio;
pub mod attachment;
pub mod auto_form;
pub mod avatar;
pub mod badge;
pub mod bento_grid;
pub mod bottom_nav;
pub mod breadcrumb;
pub mod bubble;
pub mod button;
pub mod button_action;
pub mod button_group;
pub mod callout;
pub mod card;
pub mod card_carousel;
pub mod carousel;
pub mod charts;
pub mod chat;
pub mod checkbox;
pub mod chips;
pub mod collapsible;
pub mod command;
pub mod context_menu;
pub mod data_grid;
pub mod data_table;
pub mod date_picker;
pub mod date_picker_dual_state;
pub mod date_picker_state;
pub mod dialog;
pub mod direction_provider;
pub mod drag_and_drop;
pub mod drawer;
pub mod dropdown_menu;
pub mod empty;
pub mod expandable;
pub mod faq_transition;
pub mod field;
pub mod footer;
pub mod form;
pub mod header;
pub mod hover_card;
pub mod image;
pub mod input;
pub mod input_group;
pub mod input_otp;
pub mod input_phone;
pub mod input_prompt;
pub mod item;
pub mod kbd;
pub mod label;
pub mod link;
pub mod marker;
pub mod marquee;
pub mod mask;
pub mod mask_2;
pub mod menubar;
pub mod message;
pub mod multi_select;
pub mod navigation_menu;
pub mod pagination;
pub mod popover;
pub mod pressable;
pub mod progress;
pub mod radio_button;
pub mod radio_button_group;
pub mod scroll_area;
pub mod select;
pub mod select_native;
pub mod separator;
pub mod sheet;
pub mod shimmer;
pub mod sidenav;
pub mod skeleton;
pub mod slider;
pub mod sonner;
pub mod spinner;
pub mod status;
pub mod stepper;
pub mod switch;
pub mod table;
pub mod tabs;
pub mod textarea;
pub mod theme_toggle;
pub mod toggle_group;
pub mod tooltip;
pub mod toaster;
