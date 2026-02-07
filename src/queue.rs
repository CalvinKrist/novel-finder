//! Novel processing queue with deduplication and priority management.
//!
//! Maintains a queue of novels to be evaluated, ensuring that each novel
//! is only processed once and providing basic priority ordering.

use crate::models::Novel;
use std::collections::{HashSet, VecDeque};

/// A queue for managing novels awaiting evaluation.
///
/// Provides deduplication via a set of seen novel IDs and a FIFO queue
/// for processing order. Can be extended with priority-based ordering.
pub struct NovelQueue {
    /// The queue of novels waiting to be processed.
    queue: VecDeque<Novel>,
    /// Set of novel IDs that have already been seen (queued or processed).
    seen: HashSet<u64>,
}

impl NovelQueue {
    /// Create a new empty queue.
    pub fn new() -> Self {
        Self {
            queue: VecDeque::new(),
            seen: HashSet::new(),
        }
    }

    /// Add a novel to the queue if it hasn't been seen before.
    ///
    /// Returns `true` if the novel was added, `false` if it was a duplicate.
    pub fn push(&mut self, novel: Novel) -> bool {
        if self.seen.contains(&novel.id) {
            tracing::debug!("Skipping duplicate novel: {} (ID: {})", novel.title, novel.id);
            return false;
        }
        self.seen.insert(novel.id);
        self.queue.push_back(novel);
        true
    }

    /// Remove and return the next novel from the queue.
    pub fn pop(&mut self) -> Option<Novel> {
        self.queue.pop_front()
    }

    /// Check whether the queue is empty.
    pub fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }

    /// Get the number of novels currently in the queue.
    pub fn len(&self) -> usize {
        self.queue.len()
    }

    /// Check whether a novel ID has already been seen.
    pub fn has_seen(&self, novel_id: u64) -> bool {
        self.seen.contains(&novel_id)
    }
}
