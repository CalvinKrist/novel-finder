//! Evaluation module for scoring novels against user criteria.
//!
//! Defines the `Evaluator` trait and provides implementations for
//! local (CPU-only) and LLM-based evaluation.

pub mod filter;
pub mod llm;
pub mod local;

use crate::models::{Criteria, Novel, NovelScore, Review};
use anyhow::Result;

/// Trait for evaluating how well a novel matches user criteria.
///
/// Implementations can use different strategies (local heuristics, LLM calls, etc.)
/// to produce a score indicating how well a novel fits the user's preferences.
pub trait Evaluator: Send + Sync {
    /// Perform a full evaluation of a novel against the criteria.
    ///
    /// Uses the novel's metadata, description, and reviews to produce
    /// a detailed score with sub-scores and reasoning.
    fn evaluate(
        &self,
        novel: &Novel,
        reviews: &[Review],
        criteria: &Criteria,
    ) -> Result<NovelScore>;

    /// Quick pre-filter check to determine if a novel is worth fully evaluating.
    ///
    /// Returns `true` if the novel passes basic checks (hard filters like
    /// page count, status, rating thresholds) and should proceed to full evaluation.
    fn pre_filter(&self, novel: &Novel, criteria: &Criteria) -> bool;
}
