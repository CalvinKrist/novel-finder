//! Configuration loading and types for the novel-finder application.
//!
//! Handles parsing the TOML configuration file that defines criteria,
//! evaluation mode, seed sources, and run parameters.

use crate::models::{Criteria, NovelStatus, StopCondition};
use anyhow::{Context, Result};
use serde::Deserialize;
use std::path::Path;
use std::time::Duration;

/// The evaluation mode to use for scoring novels.
#[derive(Debug, Clone)]
pub enum EvalMode {
    /// Local evaluation using keyword matching and heuristics.
    Local,
    /// LLM-based evaluation using an external API.
    Llm {
        api_key: String,
        model: String,
        endpoint: String,
    },
}

/// How seed novels are sourced.
#[derive(Debug, Clone)]
pub enum SeedSource {
    /// Manually specified list of RoyalRoad URLs or IDs.
    Manual(Vec<String>),
    /// Scraped from RoyalRoad's advanced search.
    Search {
        query: String,
        max_results: usize,
    },
}

/// Top-level application configuration.
#[derive(Debug, Clone)]
pub struct AppConfig {
    /// User-defined evaluation criteria.
    pub criteria: Criteria,
    /// Which evaluation mode to use.
    pub eval_mode: EvalMode,
    /// How to obtain seed novels.
    pub seed_source: SeedSource,
    /// When to stop the pipeline.
    pub stop_condition: StopCondition,
    /// Whether to discover new novels via "also liked" sections.
    pub discovery_enabled: bool,
}

/// Raw TOML structure for deserialization.
#[derive(Debug, Deserialize)]
struct RawConfig {
    criteria: RawCriteria,
    eval: RawEval,
    seeds: RawSeeds,
    run: RawRun,
    logging: Option<RawLogging>,
}

#[derive(Debug, Deserialize)]
struct RawCriteria {
    prompt: Option<String>,
    min_pages: Option<u64>,
    max_pages: Option<u64>,
    min_rating: Option<f64>,
    allowed_statuses: Option<Vec<String>>,
    required_tags: Option<Vec<String>>,
    excluded_tags: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
struct RawEval {
    mode: String,
    llm_api_key: Option<String>,
    llm_model: Option<String>,
    llm_endpoint: Option<String>,
}

#[derive(Debug, Deserialize)]
struct RawSeeds {
    source: String,
    urls: Option<Vec<String>>,
    search_query: Option<String>,
    search_max_results: Option<usize>,
}

#[derive(Debug, Deserialize)]
struct RawRun {
    stop_condition: RawStopCondition,
    discovery_enabled: bool,
}

#[derive(Debug, Deserialize)]
struct RawStopCondition {
    #[serde(rename = "type")]
    kind: String,
    value: Option<u64>,
}

#[derive(Debug, Deserialize)]
struct RawLogging {
    #[allow(dead_code)]
    verbose: Option<bool>,
}

/// Parse a status string into a `NovelStatus`.
fn parse_status(s: &str) -> Result<NovelStatus> {
    match s.to_lowercase().as_str() {
        "ongoing" => Ok(NovelStatus::Ongoing),
        "completed" => Ok(NovelStatus::Completed),
        "hiatus" => Ok(NovelStatus::Hiatus),
        "dropped" => Ok(NovelStatus::Dropped),
        "stub" => Ok(NovelStatus::Stub),
        other => anyhow::bail!("Unknown novel status: {}", other),
    }
}

/// Load the application configuration from a TOML file at the given path.
pub fn load_config(path: &Path) -> Result<AppConfig> {
    let content = std::fs::read_to_string(path)
        .with_context(|| format!("Failed to read config file: {}", path.display()))?;

    let raw: RawConfig =
        toml::from_str(&content).with_context(|| "Failed to parse config TOML")?;

    // Build criteria
    let allowed_statuses = raw
        .criteria
        .allowed_statuses
        .map(|statuses| {
            statuses
                .iter()
                .map(|s| parse_status(s))
                .collect::<Result<Vec<_>>>()
        })
        .transpose()?;

    let criteria = Criteria {
        prompt: raw.criteria.prompt,
        min_pages: raw.criteria.min_pages,
        max_pages: raw.criteria.max_pages,
        min_rating: raw.criteria.min_rating,
        allowed_statuses,
        required_tags: raw.criteria.required_tags,
        excluded_tags: raw.criteria.excluded_tags,
    };

    // Build eval mode
    let eval_mode = match raw.eval.mode.as_str() {
        "local" => EvalMode::Local,
        "llm" => EvalMode::Llm {
            api_key: raw
                .eval
                .llm_api_key
                .context("LLM mode requires llm_api_key")?,
            model: raw
                .eval
                .llm_model
                .context("LLM mode requires llm_model")?,
            endpoint: raw
                .eval
                .llm_endpoint
                .context("LLM mode requires llm_endpoint")?,
        },
        other => anyhow::bail!("Unknown eval mode: {}", other),
    };

    // Build seed source
    let seed_source = match raw.seeds.source.as_str() {
        "manual" => {
            let urls = raw.seeds.urls.context("Manual seed source requires urls")?;
            SeedSource::Manual(urls)
        }
        "search" => {
            let query = raw
                .seeds
                .search_query
                .context("Search seed source requires search_query")?;
            let max_results = raw.seeds.search_max_results.unwrap_or(20);
            SeedSource::Search { query, max_results }
        }
        other => anyhow::bail!("Unknown seed source: {}", other),
    };

    // Build stop condition
    let stop_condition = match raw.run.stop_condition.kind.as_str() {
        "max_novels" => {
            let value = raw
                .run
                .stop_condition
                .value
                .context("max_novels stop condition requires a value")? as usize;
            StopCondition::MaxNovels(value)
        }
        "max_time" => {
            let value = raw
                .run
                .stop_condition
                .value
                .context("max_time stop condition requires a value (seconds)")?;
            StopCondition::MaxTime(Duration::from_secs(value))
        }
        "empty_queue" => StopCondition::EmptyQueue,
        other => anyhow::bail!("Unknown stop condition: {}", other),
    };

    Ok(AppConfig {
        criteria,
        eval_mode,
        seed_source,
        stop_condition,
        discovery_enabled: raw.run.discovery_enabled,
    })
}
