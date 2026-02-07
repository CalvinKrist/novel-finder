//! Scrape individual novel pages from RoyalRoad.
//!
//! Extracts metadata, description, chapter list, and "also liked" novels
//! from a novel's main page.

use crate::models::Novel;
use crate::scraper::RoyalRoadClient;
use anyhow::Result;

/// Scrape a novel's full details from its RoyalRoad page.
///
/// # Arguments
/// * `client` - The HTTP client to use for requests.
/// * `novel_id` - The RoyalRoad fiction ID.
///
/// # Returns
/// A fully populated `Novel` struct.
pub fn scrape_novel(client: &RoyalRoadClient, novel_id: u64) -> Result<Novel> {
    let url = format!("https://www.royalroad.com/fiction/{}", novel_id);
    let _html = client.fetch(&url)?;

    // TODO: Parse the HTML to extract novel metadata
    // - Title, author from the page header
    // - Description from the fiction-info section
    // - Pages, rating, status from the stats sidebar
    // - Tags from the tag list
    // - Chapter count and titles from the chapter list
    // - Followers and favorites from the stats

    todo!("Parse novel page HTML into Novel struct")
}

/// Extract novel IDs from the "Others Also Liked" section of a novel page.
///
/// # Arguments
/// * `client` - The HTTP client to use for requests.
/// * `novel_id` - The RoyalRoad fiction ID whose page to check.
///
/// # Returns
/// A list of novel IDs found in the "also liked" section.
pub fn scrape_also_liked(client: &RoyalRoadClient, novel_id: u64) -> Result<Vec<u64>> {
    let url = format!("https://www.royalroad.com/fiction/{}", novel_id);
    let _html = client.fetch(&url)?;

    // TODO: Parse the "Others Also Liked" sidebar section
    // - Find the recommendation carousel/list
    // - Extract fiction IDs from the links

    todo!("Parse 'also liked' section from novel page")
}
