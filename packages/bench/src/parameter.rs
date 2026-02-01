use crate::BenchCase;
use async_trait::async_trait;
use std::{future::Future, ops::RangeInclusive};

/// Represents a benchmark parameter with a range of values.
///
/// Inspired by Substrate's parameter-based benchmarking, this allows
/// measuring how performance scales with input size.
#[derive(Debug, Clone)]
pub struct Parameter {
    pub name: String,
    pub range: RangeInclusive<u32>,
    pub step: u32,
}

impl Parameter {
    pub fn new(name: impl Into<String>, range: RangeInclusive<u32>) -> Self {
        Self {
            name: name.into(),
            range,
            step: 1,
        }
    }

    pub fn with_step(mut self, step: u32) -> Self {
        self.step = step;
        self
    }

    pub fn values(&self) -> Vec<u32> {
        let mut vals = Vec::new();
        let mut curr = *self.range.start();
        while curr <= *self.range.end() {
            vals.push(curr);
            if self.step == 0 {
                break;
            }
            curr += self.step;
        }
        vals
    }
}

/// A benchmark that varies based on a parameter.
pub struct ParametricBench<F, Fut>
where
    F: Fn(u32) -> Fut + Send + Sync,
    Fut: Future<Output = anyhow::Result<()>> + Send,
{
    name: String,
    parameter: Parameter,
    func: F,
    current_param: std::sync::atomic::AtomicU32,
}

impl<F, Fut> ParametricBench<F, Fut>
where
    F: Fn(u32) -> Fut + Send + Sync,
    Fut: Future<Output = anyhow::Result<()>> + Send,
{
    pub fn new(name: impl Into<String>, parameter: Parameter, func: F) -> Self {
        let start = *parameter.range.start();
        Self {
            name: name.into(),
            parameter,
            func,
            current_param: std::sync::atomic::AtomicU32::new(start),
        }
    }

    /// Sets the current parameter value for the next run.
    pub fn set_param(&self, val: u32) {
        self.current_param
            .store(val, std::sync::atomic::Ordering::SeqCst);
    }
}

#[async_trait]
impl<F, Fut> BenchCase for ParametricBench<F, Fut>
where
    F: Fn(u32) -> Fut + Send + Sync,
    Fut: Future<Output = anyhow::Result<()>> + Send,
{
    fn name(&self) -> &str {
        &self.name
    }

    fn parameter(&self) -> Option<Parameter> {
        Some(self.parameter.clone())
    }

    fn set_parameter(&self, val: u32) {
        self.set_param(val);
    }

    async fn run(&self) -> anyhow::Result<()> {
        let p = self.current_param.load(std::sync::atomic::Ordering::SeqCst);
        (self.func)(p).await
    }
}
