//! Local (CPU-only) novel evaluator.
//!
//! Scores novels using keyword matching against descriptions and reviews,
//! plus metadata alignment with criteria. No external API calls required.

use crate::eval::filter::passes_hard_filters;
use crate::eval::Evaluator;
use crate::models::{Criteria, Novel, NovelScore, Review};
use anyhow::Result;

/// An evaluator that uses local heuristics and keyword matching.
///
/// This evaluator works entirely offline and scores novels based on:
/// - Keyword overlap between the user's prompt and the novel's description/reviews
/// - Metadata alignment (rating closeness to maximum, page count, etc.)
/// - Tag relevance
pub struct LocalEvaluator;

impl LocalEvaluator {
    /// Create a new local evaluator.
    pub fn new() -> Self {
        Self
    }
}

impl Evaluator for LocalEvaluator {
    fn evaluate(
        &self,
        novel: &Novel,
        reviews: &[Review],
        criteria: &Criteria,
    ) -> Result<NovelScore> {
        let _ = (novel, reviews, criteria);

        // TODO: Implement local scoring logic
        // 1. Extract keywords from the criteria prompt (if any)
        // 2. Count keyword matches in novel description
        // 3. Count keyword matches across review texts
        // 4. Score metadata alignment:
        //    - Rating proximity to 5.0
        //    - Follower/favorite counts as popularity signal
        //    - Chapter count as a maturity signal
        // 5. Combine sub-scores into overall score with weights
        // 6. Generate human-readable reasoning string

        todo!("Implement local keyword-based scoring")
    }

    fn pre_filter(&self, novel: &Novel, criteria: &Criteria) -> bool {
        passes_hard_filters(novel, criteria)
    }
}
