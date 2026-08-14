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

//! Email provider abstraction for verification, magic links, and OTPs.

use async_trait::async_trait;

/// An email message to send.
#[derive(Debug, Clone)]
pub struct EmailMessage {
    pub to: String,
    pub subject: String,
    pub body_text: String,
    pub body_html: Option<String>,
}

/// Trait for sending transactional auth emails.
#[async_trait]
pub trait EmailProvider: Send + Sync + 'static {
    /// Send an email. Implementations may log, SMTP, or queue.
    async fn send(&self, message: EmailMessage) -> anyhow::Result<()>;
}

/// Development provider that logs emails to tracing / stdout.
#[derive(Debug, Default, Clone)]
pub struct ConsoleEmailProvider;

impl ConsoleEmailProvider {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl EmailProvider for ConsoleEmailProvider {
    async fn send(&self, message: EmailMessage) -> anyhow::Result<()> {
        tracing::info!(
            to = %message.to,
            subject = %message.subject,
            body = %message.body_text,
            "auth email (console)"
        );
        println!(
            "[montrs-auth email] to={} subject={}\n{}",
            message.to, message.subject, message.body_text
        );
        Ok(())
    }
}

/// No-op email provider for tests.
#[derive(Debug, Default, Clone)]
pub struct NullEmailProvider;

#[async_trait]
impl EmailProvider for NullEmailProvider {
    async fn send(&self, _message: EmailMessage) -> anyhow::Result<()> {
        Ok(())
    }
}
