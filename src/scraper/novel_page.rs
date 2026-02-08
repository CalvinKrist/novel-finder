//! Scrape individual novel pages from RoyalRoad.
//!
//! Extracts metadata, description, chapter list, and "also liked" novels
//! from a novel's main page.

use crate::models::{Novel, NovelStatus};
use crate::scraper::RoyalRoadClient;
use anyhow::{Context, Result};
use scraper::{Html, Selector};

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
    let html = client.fetch(&url)?;
    parse_novel_from_html(&html, novel_id)
}

/// Extract novel IDs from the "Others Also Liked" recommendations via the API.
///
/// # Arguments
/// * `client` - The HTTP client to use for requests.
/// * `novel_id` - The RoyalRoad fiction ID whose recommendations to fetch.
///
/// # Returns
/// A list of novel IDs found in the recommendations.
pub fn scrape_also_liked(client: &RoyalRoadClient, novel_id: u64) -> Result<Vec<u64>> {
    let url = format!(
        "https://www.royalroad.com/fictions/similar?fictionId={}",
        novel_id
    );
    let json = client.fetch(&url)?;
    parse_also_liked_from_json(&json)
}

/// Parse a novel's metadata from the raw HTML of its RoyalRoad page.
///
/// This is separated from `scrape_novel` so it can be unit-tested against
/// an HTML snapshot without making HTTP requests.
pub(crate) fn parse_novel_from_html(html: &str, novel_id: u64) -> Result<Novel> {
    let document = Html::parse_document(html);

    // --- Extract from JSON-LD ---
    let ld_json = extract_ld_json(&document)?;

    let title = ld_json["name"]
        .as_str()
        .context("missing 'name' in JSON-LD")?
        .to_string();

    let author = ld_json["author"]["name"]
        .as_str()
        .context("missing 'author.name' in JSON-LD")?
        .to_string();

    let description_html = ld_json["description"]
        .as_str()
        .context("missing 'description' in JSON-LD")?;
    let description = strip_html_tags(description_html);

    let pages = ld_json["numberOfPages"]
        .as_u64()
        .context("missing 'numberOfPages' in JSON-LD")?;

    let rating = ld_json["aggregateRating"]["ratingValue"]
        .as_f64()
        .context("missing 'aggregateRating.ratingValue' in JSON-LD")?;

    let tags: Vec<String> = ld_json["genre"]
        .as_array()
        .context("missing 'genre' in JSON-LD")?
        .iter()
        .filter_map(|v| v.as_str().map(String::from))
        .collect();

    // --- Extract status from HTML ---
    let status = extract_status(&document)?;

    // --- Extract followers and favorites from HTML ---
    let (followers, favorites) = extract_stats(&document)?;

    // --- Extract chapter titles from window.chapters ---
    let chapter_titles = extract_chapter_titles(html)?;
    let chapter_count = chapter_titles.len() as u64;

    let url = format!("https://www.royalroad.com/fiction/{}", novel_id);

    Ok(Novel {
        id: novel_id,
        title,
        author,
        url,
        description,
        pages,
        rating,
        status,
        tags,
        chapter_count,
        chapter_titles,
        followers,
        favorites,
    })
}

/// Parse the "also liked" JSON API response into a list of novel IDs.
pub(crate) fn parse_also_liked_from_json(json: &str) -> Result<Vec<u64>> {
    let items: Vec<serde_json::Value> =
        serde_json::from_str(json).context("failed to parse similar fictions JSON")?;

    let ids: Vec<u64> = items
        .iter()
        .filter_map(|item| item["id"].as_u64())
        .collect();

    Ok(ids)
}

/// Extract the JSON-LD structured data from the page.
fn extract_ld_json(document: &Html) -> Result<serde_json::Value> {
    let selector =
        Selector::parse("script[type='application/ld+json']").expect("valid selector");

    let element = document
        .select(&selector)
        .next()
        .context("no JSON-LD script tag found")?;

    let text = element.text().collect::<String>();
    let value: serde_json::Value =
        serde_json::from_str(&text).context("failed to parse JSON-LD")?;

    Ok(value)
}

/// Extract the publication status from HTML label spans.
fn extract_status(document: &Html) -> Result<NovelStatus> {
    let selector = Selector::parse("span.label").expect("valid selector");

    for element in document.select(&selector) {
        let text = element.text().collect::<String>();
        let text = text.trim().to_uppercase();

        match text.as_str() {
            "ONGOING" => return Ok(NovelStatus::Ongoing),
            "COMPLETED" => return Ok(NovelStatus::Completed),
            "HIATUS" => return Ok(NovelStatus::Hiatus),
            "DROPPED" => return Ok(NovelStatus::Dropped),
            "STUB" => return Ok(NovelStatus::Stub),
            _ => continue,
        }
    }

    anyhow::bail!("could not find novel status in page")
}

/// Extract followers and favorites counts from the stats section.
fn extract_stats(document: &Html) -> Result<(u64, u64)> {
    let selector =
        Selector::parse("div.fiction-stats div.stats-content ul li").expect("valid selector");

    let items: Vec<String> = document
        .select(&selector)
        .map(|el| el.text().collect::<String>().trim().to_string())
        .collect();

    let mut followers: Option<u64> = None;
    let mut favorites: Option<u64> = None;

    for (i, item) in items.iter().enumerate() {
        if item.starts_with("Followers") {
            if let Some(next) = items.get(i + 1) {
                followers = Some(parse_stat_number(next)?);
            }
        } else if item.starts_with("Favorites") {
            if let Some(next) = items.get(i + 1) {
                favorites = Some(parse_stat_number(next)?);
            }
        }
    }

    Ok((
        followers.context("could not find followers count")?,
        favorites.context("could not find favorites count")?,
    ))
}

/// Parse a stat number that may contain commas (e.g., "6,475").
fn parse_stat_number(s: &str) -> Result<u64> {
    let cleaned: String = s.chars().filter(|c| c.is_ascii_digit()).collect();
    cleaned
        .parse::<u64>()
        .with_context(|| format!("failed to parse stat number: '{}'", s))
}

/// Extract chapter titles from the `window.chapters` JavaScript variable.
fn extract_chapter_titles(html: &str) -> Result<Vec<String>> {
    let re = regex::Regex::new(r"window\.chapters\s*=\s*(\[.*?\])\s*;")
        .expect("valid regex");

    let caps = re
        .captures(html)
        .context("could not find window.chapters in page")?;

    let json_str = &caps[1];

    let chapters: Vec<serde_json::Value> =
        serde_json::from_str(json_str).context("failed to parse window.chapters JSON")?;

    let titles: Vec<String> = chapters
        .iter()
        .filter_map(|ch| ch["title"].as_str().map(String::from))
        .collect();

    Ok(titles)
}

/// Strip HTML tags from a string, returning plain text.
fn strip_html_tags(html: &str) -> String {
    let fragment = Html::parse_fragment(html);
    fragment.root_element().text().collect::<String>()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn testdata_path(filename: &str) -> PathBuf {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("src");
        path.push("scraper");
        path.push("testdata");
        path.push(filename);
        path
    }

    #[test]
    fn test_parse_novel_from_html() {
        let html =
            std::fs::read_to_string(testdata_path("novel_page_90435.html")).unwrap();
        let novel = parse_novel_from_html(&html, 90435).unwrap();

        assert_eq!(novel.id, 90435);
        assert_eq!(novel.title, "Bunny Girl Evolution");
        assert_eq!(novel.author, "Bedivere the Mad");
        assert_eq!(novel.url, "https://www.royalroad.com/fiction/90435");
        assert_eq!(novel.pages, 391);
        assert!((novel.rating - 4.398).abs() < 0.01);
        assert_eq!(novel.status, NovelStatus::Stub);
        assert_eq!(novel.followers, 6475);
        assert_eq!(novel.favorites, 1808);
        assert_eq!(novel.chapter_count, 37);

        // Check some specific tags
        assert!(novel.tags.contains(&"LitRPG".to_string()));
        assert!(novel.tags.contains(&"Fantasy".to_string()));
        assert!(novel.tags.contains(&"Action".to_string()));

        // Check chapter titles
        assert!(novel.chapter_titles.contains(&"1 - Rabbit".to_string()));
        assert!(novel
            .chapter_titles
            .contains(&"Stub Announcement".to_string()));
    }

    #[test]
    fn test_parse_novel_description_is_plain_text() {
        let html =
            std::fs::read_to_string(testdata_path("novel_page_90435.html")).unwrap();
        let novel = parse_novel_from_html(&html, 90435).unwrap();

        assert!(!novel.description.is_empty());
        assert!(novel.description.contains("bunny"));
        // Should not contain HTML tags
        assert!(!novel.description.contains("<p>"));
        assert!(!novel.description.contains("<b>"));
        assert!(!novel.description.contains("<span"));
    }

    #[test]
    fn test_parse_also_liked_from_json() {
        let json =
            std::fs::read_to_string(testdata_path("similar_90435.json")).unwrap();
        let ids = parse_also_liked_from_json(&json).unwrap();

        assert!(!ids.is_empty());
        assert_eq!(ids.len(), 10);
        assert!(ids.contains(&89877)); // Cursed Explorer of the Arcana
        assert!(ids.contains(&115399)); // Death Healer
        assert!(ids.contains(&80744)); // Dungeon of Knowledge
        assert!(ids.contains(&129189)); // Chloe the Zombie
    }
}
