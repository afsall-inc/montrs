//! Local LLM inference via candle-transformers (not yet wired).
//!
//! For local inference in CI or development, use Ollama:
//!
//! ```bash
//! ollama pull qwen2:0.5b && ollama serve
//! ```
//!
//! Then configure `provider = "ollama"` or `provider = "local"` in
//! `prdoc.toml`. Both map to the Ollama API at `localhost:11434`.
//!
//! The candle path (feature `local-llm`) is reserved for future direct
//! in-process inference without a daemon.

use crate::{llm::LlmSummaryProvider, summary::SummaryContext};

pub struct LocalLlmProvider {
    #[allow(dead_code)]
    cache_dir: String,
}

impl LocalLlmProvider {
    pub fn new(_model: &str, cache_dir: &str) -> Self {
        Self {
            cache_dir: cache_dir.to_string(),
        }
    }
}

#[allow(unreachable_code)]
impl LlmSummaryProvider for LocalLlmProvider {
    fn generate_summary(
        &self,
        _rich: &str,
        _ctx: &SummaryContext,
    ) -> Result<String, String> {
        Err(
            "Local inference not yet wired. Use Ollama instead:\n  ollama \
             pull qwen2:0.5b && ollama serve\n  Set provider = 'ollama' or \
             'local' in prdoc.toml."
                .to_string(),
        )
    }

    fn provider_name(&self) -> &str {
        "local-candle"
    }
}
