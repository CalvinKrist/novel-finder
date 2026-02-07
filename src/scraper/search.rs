//! Scrape RoyalRoad's advanced search results.
//!
//! Used to find seed novels when no manual URLs are provided.

use crate::scraper::RoyalRoadClient;
use anyhow::Result;

/// A minimal representation of a novel found in search results.
#[derive(Debug, Clone)]
pub struct SearchResult {
    /// The RoyalRoad fiction ID.
    pub id: u64,
    /// Title of the novel.
    pub title: String,
    /// URL to the novel's page.
    pub url: String,
}

/// Search RoyalRoad with the given query and return matching novel IDs.
///
/// # Arguments
/// * `client` - The HTTP client to use for requests.
/// * `query` - The search query string.
/// * `max_results` - Maximum number of results to return.
///
/// # Returns
/// A list of search results with basic novel info.
pub fn search_novels(
    client: &RoyalRoadClient,
    query: &str,
    max_results: usize,
) -> Result<Vec<SearchResult>> {
    let _url = format!(
        "https://www.royalroad.com/fictions/search?title={}",
        query
    );
    let _ = (client, max_results);

    // TODO: Implement search result scraping
    // - Build the search URL with proper query parameters
    // - Fetch and parse search result pages
    // - Handle pagination if max_results exceeds one page
    // - Extract fiction IDs, titles, and URLs from result entries

    todo!("Scrape RoyalRoad search results")
}
