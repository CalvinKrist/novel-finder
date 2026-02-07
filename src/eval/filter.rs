//! Hard filter evaluation for novels.
//!
//! Applies strict pass/fail checks based on metadata thresholds.
//! Used as a pre-step by both Local and LLM evaluators to skip
//! novels that cannot possibly match the criteria.

use crate::models::{Criteria, Novel};

/// Check whether a novel passes all hard filters defined in the criteria.
///
/// Returns `true` if the novel meets all specified thresholds.
/// A filter that is `None` in the criteria is treated as "no constraint".
pub fn passes_hard_filters(novel: &Novel, criteria: &Criteria) -> bool {
    // Check minimum pages
    if let Some(min_pages) = criteria.min_pages {
        if novel.pages < min_pages {
            tracing::debug!(
                "Novel '{}' rejected: {} pages < min {}",
                novel.title,
                novel.pages,
                min_pages
            );
            return false;
        }
    }

    // Check maximum pages
    if let Some(max_pages) = criteria.max_pages {
        if novel.pages > max_pages {
            tracing::debug!(
                "Novel '{}' rejected: {} pages > max {}",
                novel.title,
                novel.pages,
                max_pages
            );
            return false;
        }
    }

    // Check minimum rating
    if let Some(min_rating) = criteria.min_rating {
        if novel.rating < min_rating {
            tracing::debug!(
                "Novel '{}' rejected: rating {:.2} < min {:.2}",
                novel.title,
                novel.rating,
                min_rating
            );
            return false;
        }
    }

    // Check allowed statuses
    if let Some(ref allowed) = criteria.allowed_statuses {
        if !allowed.is_empty() && !allowed.contains(&novel.status) {
            tracing::debug!(
                "Novel '{}' rejected: status {:?} not in allowed list",
                novel.title,
                novel.status
            );
            return false;
        }
    }

    // Check required tags
    if let Some(ref required) = criteria.required_tags {
        for tag in required {
            let tag_lower = tag.to_lowercase();
            if !novel.tags.iter().any(|t| t.to_lowercase() == tag_lower) {
                tracing::debug!(
                    "Novel '{}' rejected: missing required tag '{}'",
                    novel.title,
                    tag
                );
                return false;
            }
        }
    }

    // Check excluded tags
    if let Some(ref excluded) = criteria.excluded_tags {
        for tag in excluded {
            let tag_lower = tag.to_lowercase();
            if novel.tags.iter().any(|t| t.to_lowercase() == tag_lower) {
                tracing::debug!(
                    "Novel '{}' rejected: has excluded tag '{}'",
                    novel.title,
                    tag
                );
                return false;
            }
        }
    }

    true
}
