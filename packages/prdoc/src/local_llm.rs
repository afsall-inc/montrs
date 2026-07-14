use crate::{llm::LlmSummaryProvider, summary::SummaryContext};

pub struct LocalLlmProvider {
    model: String,
    cache_dir: String,
}

impl LocalLlmProvider {
    pub fn new(model: &str, cache_dir: &str) -> Self {
        Self {
            model: model.to_string(),
            cache_dir: cache_dir.to_string(),
        }
    }

    fn ensure_model(&self) -> Result<std::path::PathBuf, String> {
        let home =
            dirs_next().or_else(|| Some(std::path::PathBuf::from("~/.cache")));
        let cache = home
            .as_ref()
            .map(|h| h.join("montrs-prdoc").join("models"))
            .unwrap_or_else(|| std::path::PathBuf::from(&self.cache_dir));

        if !cache.exists() {
            std::fs::create_dir_all(&cache)
                .map_err(|e| format!("Failed to create cache dir: {e}"))?;
        }

        let model_path = cache.join(&self.model);
        if !model_path.exists() {
            return Err(format!(
                "Model '{}' not found at {}. Download it first:\nhfdownloader \
                 {} {}",
                self.model,
                model_path.display(),
                self.model,
                cache.display(),
            ));
        }

        Ok(model_path)
    }
}

impl LlmSummaryProvider for LocalLlmProvider {
    fn generate_summary(
        &self,
        rich: &str,
        ctx: &SummaryContext,
    ) -> Result<String, String> {
        let _model_path = self.ensure_model()?;

        let prompt = build_local_prompt(rich, ctx);
        let _result = run_local_inference(&prompt)?;

        Err(
            "Local LLM inference requires the `local-llm` feature with candle \
             and the model downloaded. Install with: `cargo install \
             montrs-prdoc --features local-llm`. Then download the model: \
             `hfdownloader Qwen/Qwen2-0.5B-Instruct \
             ~/.cache/montrs-prdoc/models/`."
                .to_string(),
        )
    }

    fn provider_name(&self) -> &str {
        "local"
    }
}

fn build_local_prompt(rich: &str, _ctx: &SummaryContext) -> String {
    format!(
        "You are a technical writer. Condense this technical summary into 2-3 \
         concise sentences explaining what changed and \
         why:\n{rich}\n\nSummary:",
    )
}

fn run_local_inference(_prompt: &str) -> Result<String, String> {
    Err(
        "Local inference not yet wired. Requires candle-transformers pipeline \
         setup with a loaded model. Use `--llm` with a cloud provider for \
         now, or set up Ollama locally with `ollama serve` and set `provider \
         = 'ollama'` in prdoc.toml."
            .to_string(),
    )
}

fn dirs_next() -> Option<std::path::PathBuf> {
    std::env::var("HOME")
        .or_else(|_| std::env::var("USERPROFILE"))
        .ok()
        .map(std::path::PathBuf::from)
        .map(|h| h.join(".cache"))
}
