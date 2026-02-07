//! Scrape novel reviews from RoyalRoad.
//!
//! Fetches user reviews for a given novel to use in evaluation.

use crate::models::Review;
use crate::scraper::RoyalRoadClient;
use anyhow::Result;

/// Scrape reviews for a novel from its RoyalRoad page.
///
/// # Arguments
/// * `client` - The HTTP client to use for requests.
/// * `novel_id` - The RoyalRoad fiction ID.
/// * `max_reviews` - Maximum number of reviews to fetch.
///
/// # Returns
/// A list of reviews for the novel.
pub fn scrape_reviews(
    client: &RoyalRoadClient,
    novel_id: u64,
    max_reviews: usize,
) -> Result<Vec<Review>> {
    let _url = format!(
        "https://www.royalroad.com/fiction/{}/reviews",
        novel_id
    );
    let _ = (client, max_reviews);

    // TODO: Implement review scraping
    // - Fetch the reviews page (may need pagination)
    // - Parse each review entry for author, rating, text, date
    // - Respect the max_reviews limit

    todo!("Scrape reviews from novel page")
}
