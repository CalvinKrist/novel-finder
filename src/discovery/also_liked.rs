//! "Others Also Liked" discovery source.
//!
//! Scrapes the "also liked" section from a novel's RoyalRoad page
//! to discover related novels, then applies lightweight pre-filtering
//! before adding them to the processing queue.

use crate::discovery::DiscoverySource;
use crate::models::{Criteria, Novel};
use crate::scraper::RoyalRoadClient;
use anyhow::Result;
use std::sync::Arc;

/// Discovers new novels via RoyalRoad's "Others Also Liked" recommendations.
///
/// For each evaluated novel, this source scrapes the recommendation sidebar
/// and performs lightweight filtering to avoid adding obviously irrelevant
/// novels to the queue.
pub struct AlsoLikedDiscovery {
    /// Shared HTTP client for making requests.
    #[allow(dead_code)]
    client: Arc<RoyalRoadClient>,
    /// Criteria used for lightweight pre-filtering of discovered novels.
    #[allow(dead_code)]
    criteria: Criteria,
}

impl AlsoLikedDiscovery {
    /// Create a new "also liked" discovery source.
    pub fn new(client: Arc<RoyalRoadClient>, criteria: Criteria) -> Self {
        Self { client, criteria }
    }
}

impl DiscoverySource for AlsoLikedDiscovery {
    fn discover(&self, novel: &Novel) -> Result<Vec<Novel>> {
        let _ = novel;

        // TODO: Implement "also liked" discovery
        //
        // Steps:
        // 1. Use scraper::novel_page::scrape_also_liked() to get related novel IDs
        // 2. For each discovered ID, scrape and parse the novel using scrape_novel or parse_novel_from_html
        // 3. Apply lightweight pre-filtering:
        //    - Check if status is in allowed_statuses
        //    - Check if rating meets min_rating
        //    - Check for excluded tags
        // 4. Return novels that pass the pre-filter

        todo!("Implement 'also liked' discovery with pre-filtering")
    }
}
