//! Main pipeline orchestration.
//!
//! Ties together seed gathering, the processing queue, evaluation,
//! discovery, and result collection into a single processing flow.

use crate::config::{AppConfig, EvalMode, SeedSource};
use crate::discovery::also_liked::AlsoLikedDiscovery;
use crate::discovery::DiscoverySource;
use crate::eval::llm::LlmEvaluator;
use crate::eval::local::LocalEvaluator;
use crate::eval::Evaluator;
use crate::models::{NovelScore, StopCondition};
use crate::queue::NovelQueue;
use crate::scraper::RoyalRoadClient;
use anyhow::Result;
use std::sync::Arc;
use std::time::{Duration, Instant};

/// The main processing pipeline that orchestrates the full novel-finding flow.
pub struct Pipeline {
    /// Application configuration.
    config: AppConfig,
    /// Shared HTTP client for RoyalRoad scraping.
    client: Arc<RoyalRoadClient>,
    /// The evaluator to use for scoring novels.
    evaluator: Box<dyn Evaluator>,
    /// Optional discovery source for finding related novels.
    discovery: Option<Box<dyn DiscoverySource>>,
    /// The processing queue.
    queue: NovelQueue,
}

impl Pipeline {
    /// Build a new pipeline from the given configuration.
    pub fn new(config: AppConfig) -> Result<Self> {
        let client = Arc::new(RoyalRoadClient::new(Duration::from_millis(1000))?);

        // Build the evaluator based on config
        let evaluator: Box<dyn Evaluator> = match &config.eval_mode {
            EvalMode::Local => Box::new(LocalEvaluator::new()),
            EvalMode::Llm {
                api_key,
                model,
                endpoint,
            } => Box::new(LlmEvaluator::new(
                api_key.clone(),
                model.clone(),
                endpoint.clone(),
            )),
        };

        // Build discovery source if enabled
        let discovery: Option<Box<dyn DiscoverySource>> = if config.discovery_enabled {
            Some(Box::new(AlsoLikedDiscovery::new(
                Arc::clone(&client),
                config.criteria.clone(),
            )))
        } else {
            None
        };

        Ok(Self {
            config,
            client,
            evaluator,
            discovery,
            queue: NovelQueue::new(),
        })
    }

    /// Run the full pipeline and return scored results.
    pub fn run(&mut self) -> Result<Vec<NovelScore>> {
        tracing::info!("Starting novel-finder pipeline");

        // Step 1: Gather seed novels
        self.gather_seeds()?;
        tracing::info!("Seeded queue with {} novels", self.queue.len());

        // Step 2: Process queue until stop condition
        let mut results: Vec<NovelScore> = Vec::new();
        let start_time = Instant::now();

        while let Some(novel) = self.queue.pop() {
            // Check stop condition
            if self.should_stop(&results, start_time) {
                tracing::info!("Stop condition reached, finishing pipeline");
                break;
            }

            tracing::info!("Processing novel: {} (ID: {})", novel.title, novel.id);

            // Pre-filter check
            if !self.evaluator.pre_filter(&novel, &self.config.criteria) {
                tracing::info!("Novel '{}' failed pre-filter, skipping", novel.title);
                continue;
            }

            // Scrape reviews for evaluation
            let reviews =
                crate::scraper::reviews::scrape_reviews(&self.client, novel.id, 10)?;

            // Evaluate
            let score =
                self.evaluator
                    .evaluate(&novel, &reviews, &self.config.criteria)?;
            tracing::info!(
                "Novel '{}' scored {:.2}",
                novel.title,
                score.overall_score
            );
            results.push(score);

            // Discover related novels
            if let Some(ref discovery) = self.discovery {
                match discovery.discover(&novel) {
                    Ok(discovered) => {
                        for discovered_novel in discovered {
                            self.queue.push(discovered_novel);
                        }
                    }
                    Err(e) => {
                        tracing::warn!(
                            "Discovery failed for novel '{}': {}",
                            novel.title,
                            e
                        );
                    }
                }
            }
        }

        // Sort results by score descending
        results.sort_by(|a, b| {
            b.overall_score
                .partial_cmp(&a.overall_score)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        tracing::info!("Pipeline complete. {} novels evaluated.", results.len());
        Ok(results)
    }

    /// Gather seed novels and add them to the queue.
    fn gather_seeds(&mut self) -> Result<()> {
        match &self.config.seed_source {
            SeedSource::Manual(urls) => {
                for url in urls {
                    let novel_id = parse_novel_id(url)?;
                    let novel =
                        crate::scraper::novel_page::scrape_novel(&self.client, novel_id)?;
                    self.queue.push(novel);
                }
            }
            SeedSource::Search { query, max_results } => {
                let results = crate::scraper::search::search_novels(
                    &self.client,
                    query,
                    *max_results,
                )?;
                for result in results {
                    let novel = crate::scraper::novel_page::scrape_novel(
                        &self.client,
                        result.id,
                    )?;
                    self.queue.push(novel);
                }
            }
        }
        Ok(())
    }

    /// Check whether the stop condition has been met.
    fn should_stop(&self, results: &[NovelScore], start_time: Instant) -> bool {
        match &self.config.stop_condition {
            StopCondition::MaxNovels(max) => results.len() >= *max,
            StopCondition::MaxTime(duration) => start_time.elapsed() >= *duration,
            StopCondition::EmptyQueue => false, // Queue emptiness is handled by the while-let
        }
    }
}

/// Extract a RoyalRoad fiction ID from a URL or raw ID string.
fn parse_novel_id(url_or_id: &str) -> Result<u64> {
    // Try parsing as a plain number first
    if let Ok(id) = url_or_id.parse::<u64>() {
        return Ok(id);
    }

    // Try extracting from a RoyalRoad URL like:
    // https://www.royalroad.com/fiction/12345/some-title
    let parts: Vec<&str> = url_or_id.split('/').collect();
    for (i, part) in parts.iter().enumerate() {
        if *part == "fiction" {
            if let Some(id_str) = parts.get(i + 1) {
                if let Ok(id) = id_str.parse::<u64>() {
                    return Ok(id);
                }
            }
        }
    }

    anyhow::bail!(
        "Could not extract novel ID from: {}. Expected a numeric ID or RoyalRoad URL.",
        url_or_id
    )
}
