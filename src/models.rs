//! Core data models for the novel-finder application.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

/// The publication status of a novel on RoyalRoad.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum NovelStatus {
    Ongoing,
    Completed,
    Hiatus,
    Dropped,
    Stub,
}

impl std::fmt::Display for NovelStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NovelStatus::Ongoing => write!(f, "Ongoing"),
            NovelStatus::Completed => write!(f, "Completed"),
            NovelStatus::Hiatus => write!(f, "Hiatus"),
            NovelStatus::Dropped => write!(f, "Dropped"),
            NovelStatus::Stub => write!(f, "Stub"),
        }
    }
}

/// A novel from RoyalRoad with all scraped metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Novel {
    /// The RoyalRoad fiction ID.
    pub id: u64,
    /// Title of the novel.
    pub title: String,
    /// Author name.
    pub author: String,
    /// Full URL to the novel page.
    pub url: String,
    /// Novel description/blurb.
    pub description: String,
    /// Total page count.
    pub pages: u64,
    /// Overall rating (0.0 - 5.0).
    pub rating: f64,
    /// Current publication status.
    pub status: NovelStatus,
    /// Tags associated with the novel.
    pub tags: Vec<String>,
    /// Total number of chapters.
    pub chapter_count: u64,
    /// List of chapter titles.
    pub chapter_titles: Vec<String>,
    /// Number of followers.
    pub followers: u64,
    /// Number of favorites.
    pub favorites: u64,
}

/// A user review of a novel.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Review {
    /// Review author username.
    pub author: String,
    /// Rating given by the reviewer (0.0 - 5.0).
    pub rating: f64,
    /// Full text of the review.
    pub text: String,
    /// Date the review was posted (as a string for simplicity).
    pub posted_date: String,
}

/// User-defined criteria for evaluating novels.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Criteria {
    /// A natural language description of what the user is looking for.
    pub prompt: Option<String>,
    /// Minimum number of pages required.
    pub min_pages: Option<u64>,
    /// Maximum number of pages allowed.
    pub max_pages: Option<u64>,
    /// Minimum overall rating required.
    pub min_rating: Option<f64>,
    /// Allowed publication statuses (empty means all are allowed).
    pub allowed_statuses: Option<Vec<NovelStatus>>,
    /// Tags that must be present on the novel.
    pub required_tags: Option<Vec<String>>,
    /// Tags that must NOT be present on the novel.
    pub excluded_tags: Option<Vec<String>>,
}

/// The result of evaluating a novel against the criteria.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NovelScore {
    /// The novel that was scored.
    pub novel: Novel,
    /// Overall score (0.0 - 1.0).
    pub overall_score: f64,
    /// Breakdown of scores by criteria dimension.
    pub sub_scores: HashMap<String, f64>,
    /// Human-readable reasoning for the score.
    pub reasoning: String,
}

/// Condition that determines when the pipeline should stop processing.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StopCondition {
    /// Stop after evaluating this many novels.
    MaxNovels(usize),
    /// Stop after this much time has elapsed.
    MaxTime(Duration),
    /// Stop when the queue is empty.
    EmptyQueue,
}
