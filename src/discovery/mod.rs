//! Discovery module for finding new novels to evaluate.
//!
//! Defines the `DiscoverySource` trait and provides implementations
//! for discovering related novels through RoyalRoad's recommendation features.

pub mod also_liked;

use crate::models::Novel;
use anyhow::Result;

/// Trait for discovering new novels related to a given novel.
///
/// Implementations use different strategies to find novels that
/// the user might be interested in based on a novel they're already
/// evaluating.
pub trait DiscoverySource: Send + Sync {
    /// Discover novels related to the given novel.
    ///
    /// Returns a list of novel stubs (may have partial metadata)
    /// that should be added to the processing queue for full evaluation.
    fn discover(&self, novel: &Novel) -> Result<Vec<Novel>>;
}
