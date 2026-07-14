use crate::summary::SummaryContext;

pub trait LlmSummaryProvider {
    fn generate_summary(
        &self,
        rich: &str,
        ctx: &SummaryContext,
    ) -> Result<String, String>;

    fn provider_name(&self) -> &str;
}

#[derive(Debug, Clone)]
pub struct LlmConfig {
    pub provider: String,
    pub model: String,
    pub api_key: String,
}

impl Default for LlmConfig {
    fn default() -> Self {
        Self {
            provider: "none".to_string(),
            model: String::new(),
            api_key: String::new(),
        }
    }
}

pub struct GroqProvider {
    pub config: LlmConfig,
}

impl LlmSummaryProvider for GroqProvider {
    fn generate_summary(
        &self,
        rich: &str,
        ctx: &SummaryContext,
    ) -> Result<String, String> {
        let prompt = build_prompt(rich, ctx);
        let response = call_openai_api(
            "https://api.groq.com/openai/v1/chat/completions",
            &self.config.api_key,
            &self.config.model,
            &prompt,
        )?;
        Ok(response)
    }

    fn provider_name(&self) -> &str {
        "groq"
    }
}

pub struct OpenAiProvider {
    pub config: LlmConfig,
}

impl LlmSummaryProvider for OpenAiProvider {
    fn generate_summary(
        &self,
        rich: &str,
        ctx: &SummaryContext,
    ) -> Result<String, String> {
        let prompt = build_prompt(rich, ctx);
        let response = call_openai_api(
            "https://api.openai.com/v1/chat/completions",
            &self.config.api_key,
            &self.config.model,
            &prompt,
        )?;
        Ok(response)
    }

    fn provider_name(&self) -> &str {
        "openai"
    }
}

pub struct OllamaProvider {
    pub config: LlmConfig,
}

impl LlmSummaryProvider for OllamaProvider {
    fn generate_summary(
        &self,
        rich: &str,
        ctx: &SummaryContext,
    ) -> Result<String, String> {
        let prompt = build_prompt(rich, ctx);
        let response = call_ollama_api(&self.config.model, &prompt)?;
        Ok(response)
    }

    fn provider_name(&self) -> &str {
        "ollama"
    }
}

pub fn create_provider(
    config: &LlmConfig,
) -> Option<Box<dyn LlmSummaryProvider>> {
    match config.provider.as_str() {
        "groq" => Some(Box::new(GroqProvider {
            config: config.clone(),
        })),
        "openai" => Some(Box::new(OpenAiProvider {
            config: config.clone(),
        })),
        "ollama" => Some(Box::new(OllamaProvider {
            config: config.clone(),
        })),
        _ => None,
    }
}

pub fn enhance_summary(
    rich: &str,
    ctx: &SummaryContext,
    config: &LlmConfig,
) -> String {
    match create_provider(config) {
        Some(provider) => match provider.generate_summary(rich, ctx) {
            Ok(summary) => summary,
            Err(e) => {
                eprintln!(
                    "LLM enhancement failed ({}): {e}",
                    provider.provider_name()
                );
                rich.to_string()
            }
        },
        None => rich.to_string(),
    }
}

fn build_prompt(rich: &str, ctx: &SummaryContext) -> String {
    let pkg_count = ctx.analysis.packages.len();
    let breaking = if ctx.analysis.is_breaking {
        "This PR contains BREAKING CHANGES. "
    } else {
        ""
    };

    let commit_hint = ctx
        .context
        .map(|c| {
            if c.commit_messages.is_empty() {
                String::new()
            } else {
                format!(
                    "Commits: {}",
                    c.commit_messages
                        .iter()
                        .take(5)
                        .cloned()
                        .collect::<Vec<_>>()
                        .join("; ")
                )
            }
        })
        .unwrap_or_default();

    let api_count =
        ctx.public_api_additions.len() + ctx.public_api_removals.len();

    format!(
        "You are a technical writer generating a concise, expressive summary \
         for a Rust project's pull request documentation.\n\nTechnical \
         details:\n{rich}\n\n{breaking}Packages affected: {pkg_count}. Public \
         API changes: {api_count} items.\n{commit_hint}\n\nRewrite the above \
         into a single paragraph summary (3-5 sentences) that explains:\n1. \
         What changed and why (the purpose)\n2. Key new features or fixes\n3. \
         Any breaking changes and migration impact\n4. Which \
         packages/functions were affected\n\nKeep it factual, technical, and \
         concise. Do not use bullet points. Preserve function/struct names. \
         Respond with ONLY the summary text, no preamble."
    )
}

fn call_openai_api(
    url: &str,
    api_key: &str,
    model: &str,
    prompt: &str,
) -> Result<String, String> {
    let body = serde_json::json!({
        "model": model,
        "messages": [
            {
                "role": "user",
                "content": prompt
            }
        ],
        "temperature": 0.3,
        "max_tokens": 500
    });

    let client = reqwest::blocking::Client::new();
    let response = client
        .post(url)
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .json(&body)
        .send()
        .map_err(|e| format!("HTTP error: {e}"))?;

    if !response.status().is_success() {
        let status = response.status();
        let error_text = response
            .text()
            .unwrap_or_else(|_| "unknown error".to_string());
        return Err(format!("API error {status}: {error_text}"));
    }

    let json: serde_json::Value = response
        .json()
        .map_err(|e| format!("JSON parse error: {e}"))?;

    let text = json["choices"][0]["message"]["content"]
        .as_str()
        .unwrap_or_default()
        .trim()
        .to_string();

    if text.is_empty() {
        Err("Empty response from API".to_string())
    } else {
        Ok(text)
    }
}

fn call_ollama_api(model: &str, prompt: &str) -> Result<String, String> {
    let body = serde_json::json!({
        "model": model,
        "prompt": prompt,
        "stream": false,
        "options": {
            "temperature": 0.3,
            "num_predict": 500
        }
    });

    let client = reqwest::blocking::Client::new();
    let response = client
        .post("http://localhost:11434/api/generate")
        .json(&body)
        .send()
        .map_err(|e| format!("Ollama error: {e}"))?;

    if !response.status().is_success() {
        return Err(format!("Ollama status: {}", response.status()));
    }

    let json: serde_json::Value = response
        .json()
        .map_err(|e| format!("JSON parse error: {e}"))?;

    let text = json["response"]
        .as_str()
        .unwrap_or_default()
        .trim()
        .to_string();

    if text.is_empty() {
        Err("Empty response from Ollama".to_string())
    } else {
        Ok(text)
    }
}
