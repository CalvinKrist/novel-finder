//! Web scraping module for RoyalRoad.
//!
//! Provides a shared HTTP client with rate limiting and submodules
//! for scraping novel pages, search results, and reviews.

pub mod novel_page;
pub mod reviews;
pub mod search;

use anyhow::Result;
use std::time::Duration;

/// A client for making rate-limited HTTP requests to RoyalRoad.
pub struct RoyalRoadClient {
    /// The underlying HTTP agent.
    agent: ureq::Agent,
    /// Delay between consecutive requests to avoid being rate-limited.
    request_delay: Duration,
}

impl RoyalRoadClient {
    /// Create a new client with the specified delay between requests.
    pub fn new(request_delay: Duration) -> Result<Self> {
        let agent = ureq::AgentBuilder::new()
            .timeout_read(Duration::from_secs(30))
            .timeout_write(Duration::from_secs(30))
            .user_agent("novel-finder/0.1.0")
            .build();

        Ok(Self {
            agent,
            request_delay,
        })
    }

    /// Fetch the HTML content of a URL, respecting rate limits.
    pub fn fetch(&self, url: &str) -> Result<String> {
        tracing::debug!("Fetching URL: {}", url);
        std::thread::sleep(self.request_delay);
        let response = self.agent.get(url).call()?;
        let text = response.into_string()?;
        Ok(text)
    }
}
