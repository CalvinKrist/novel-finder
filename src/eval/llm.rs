//! LLM-based novel evaluator.
//!
//! Uses an external LLM API (e.g., Anthropic, OpenAI) to evaluate
//! how well a novel matches natural language criteria. Provides richer
//! semantic understanding than keyword matching.

use crate::eval::filter::passes_hard_filters;
use crate::eval::Evaluator;
use crate::models::{Criteria, Novel, NovelScore, Review};
use anyhow::Result;

/// An evaluator that uses an LLM API for semantic evaluation.
///
/// Constructs prompts from the novel's metadata, description, and reviews,
/// then sends them to an LLM to get nuanced scoring and reasoning.
pub struct LlmEvaluator {
    /// API key for authentication.
    #[allow(dead_code)]
    api_key: String,
    /// Model identifier (e.g., "claude-sonnet-4-5-20250929").
    #[allow(dead_code)]
    model: String,
    /// API endpoint URL.
    #[allow(dead_code)]
    endpoint: String,
}

impl LlmEvaluator {
    /// Create a new LLM evaluator with the given API configuration.
    pub fn new(api_key: String, model: String, endpoint: String) -> Self {
        Self {
            api_key,
            model,
            endpoint,
        }
    }
}

impl Evaluator for LlmEvaluator {
    fn evaluate(
        &self,
        novel: &Novel,
        reviews: &[Review],
        criteria: &Criteria,
    ) -> Result<NovelScore> {
        let _ = (novel, reviews, criteria);

        // TODO: Implement LLM-based evaluation
        //
        // The approach:
        // 1. Construct a system prompt explaining the scoring task
        // 2. Build a user prompt containing:
        //    - The user's criteria (natural language prompt + filters)
        //    - The novel's description
        //    - A selection of reviews (truncated to fit context window)
        //    - Chapter titles as structural signal
        //    - Metadata (rating, pages, status, tags)
        // 3. Ask the LLM to return a JSON object with:
        //    - overall_score (0.0 - 1.0)
        //    - sub_scores (map of criteria dimension to score)
        //    - reasoning (string explanation)
        // 4. Parse the LLM response and construct a NovelScore
        //
        // API call will use reqwest to POST to self.endpoint with:
        // - Authorization header using self.api_key
        // - Model set to self.model
        // - Structured output format to ensure valid JSON response

        todo!("Implement LLM-based evaluation via API call")
    }

    fn pre_filter(&self, novel: &Novel, criteria: &Criteria) -> bool {
        // Use the same hard filters as local mode to avoid wasting API calls
        passes_hard_filters(novel, criteria)
    }
}
