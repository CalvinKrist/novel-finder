//! Scrape novel reviews from RoyalRoad.
//!
//! Fetches user reviews for a given novel to use in evaluation.

use crate::models::Review;
use crate::scraper::RoyalRoadClient;
use anyhow::Result;
use scraper::{Html, Selector};

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
    let url = format!("https://www.royalroad.com/fiction/{}", novel_id);
    let html = client.fetch(&url)?;
    parse_reviews_from_html(&html, max_reviews)
}

/// Parse reviews from the raw HTML of a RoyalRoad novel page.
///
/// This is separated from `scrape_reviews` so it can be unit-tested against
/// an HTML snapshot without making HTTP requests.
pub(crate) fn parse_reviews_from_html(html: &str, max_reviews: usize) -> Result<Vec<Review>> {
    let document = Html::parse_document(html);

    let review_selector = Selector::parse("div.review").expect("valid selector");

    let mut reviews = Vec::new();

    for review_el in document.select(&review_selector) {
        if reviews.len() >= max_reviews {
            break;
        }

        let author = extract_review_author(&review_el);
        let rating = extract_review_rating(&review_el);
        let text = extract_review_text(&review_el);
        let posted_date = extract_review_date(&review_el);

        // Only include reviews where we could extract at minimum the text.
        if let (Some(author), Some(rating), Some(text), Some(posted_date)) =
            (author, rating, text, posted_date)
        {
            reviews.push(Review {
                author,
                rating,
                text,
                posted_date,
            });
        }
    }

    Ok(reviews)
}

/// Extract the review author username from a review element.
fn extract_review_author(review_el: &scraper::ElementRef) -> Option<String> {
    let selector = Selector::parse("div.review-meta a.small").expect("valid selector");
    review_el
        .select(&selector)
        .next()
        .map(|el| el.text().collect::<String>().trim().to_string())
}

/// Extract the overall rating from a review element.
///
/// The rating is stored in an `aria-label` attribute like "5 stars" or "4.5 stars"
/// on a div inside `div.overall-score-container`.
fn extract_review_rating(review_el: &scraper::ElementRef) -> Option<f64> {
    let selector =
        Selector::parse("div.overall-score-container div[aria-label]").expect("valid selector");

    // The second div[aria-label] inside overall-score-container has the star rating.
    // First is "Overall Score", second is "X stars".
    for el in review_el.select(&selector) {
        let aria_label = el.value().attr("aria-label")?;
        if aria_label.ends_with("stars") || aria_label.ends_with("star") {
            // Parse "5 stars" or "4.5 stars" -> f64
            let rating_str = aria_label
                .trim_end_matches(" stars")
                .trim_end_matches(" star");
            if let Ok(rating) = rating_str.parse::<f64>() {
                return Some(rating);
            }
        }
    }
    None
}

/// Extract the review text content from a review element.
///
/// Collects plain text from the `div.review-inner` element, stripping HTML tags.
fn extract_review_text(review_el: &scraper::ElementRef) -> Option<String> {
    let selector = Selector::parse("div.review-inner").expect("valid selector");
    review_el.select(&selector).next().map(|el| {
        let text = el.text().collect::<String>();
        // Collapse whitespace runs and trim
        let cleaned: String = text
            .split_whitespace()
            .collect::<Vec<&str>>()
            .join(" ");
        cleaned
    })
}

/// Extract the posted date from a review element.
///
/// The date is stored in the `datetime` attribute of a `<time>` element.
fn extract_review_date(review_el: &scraper::ElementRef) -> Option<String> {
    let selector = Selector::parse("div.review-meta time").expect("valid selector");
    review_el
        .select(&selector)
        .next()
        .and_then(|el| el.value().attr("datetime").map(String::from))
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
    fn test_parse_reviews_from_html() {
        let html =
            std::fs::read_to_string(testdata_path("novel_page_90435.html")).unwrap();
        let reviews = parse_reviews_from_html(&html, 100).unwrap();

        // The snapshot contains 10 reviews
        assert_eq!(reviews.len(), 10);

        // Check the first review
        let first = &reviews[0];
        assert_eq!(first.author, "PhantomBuni");
        assert!((first.rating - 5.0).abs() < 0.01);
        assert!(first.text.contains("I loved this book so much"));
        assert!(first.posted_date.contains("2025-01-07"));
    }

    #[test]
    fn test_parse_reviews_max_limit() {
        let html =
            std::fs::read_to_string(testdata_path("novel_page_90435.html")).unwrap();
        let reviews = parse_reviews_from_html(&html, 3).unwrap();

        assert_eq!(reviews.len(), 3);
    }

    #[test]
    fn test_parse_reviews_rating_extraction() {
        let html =
            std::fs::read_to_string(testdata_path("novel_page_90435.html")).unwrap();
        let reviews = parse_reviews_from_html(&html, 100).unwrap();

        // Most reviews are 5 stars
        let five_star: Vec<&Review> = reviews.iter().filter(|r| (r.rating - 5.0).abs() < 0.01).collect();
        assert!(!five_star.is_empty());

        // jumpsplat120 gave 4.5 stars (review index 6)
        let jumpsplat = reviews.iter().find(|r| r.author == "jumpsplat120").unwrap();
        assert!((jumpsplat.rating - 4.5).abs() < 0.01);

        // Kptn gave 3.0 stars overall (review index 9)
        let kptn = reviews.iter().find(|r| r.author == "Kptn").unwrap();
        assert!((kptn.rating - 3.0).abs() < 0.01);
    }

    #[test]
    fn test_parse_reviews_empty_html() {
        let html = "<html><body><div>No reviews here</div></body></html>";
        let reviews = parse_reviews_from_html(html, 10).unwrap();
        assert!(reviews.is_empty());
    }

    #[test]
    fn test_parse_reviews_text_is_plain() {
        let html =
            std::fs::read_to_string(testdata_path("novel_page_90435.html")).unwrap();
        let reviews = parse_reviews_from_html(&html, 100).unwrap();

        for review in &reviews {
            assert!(!review.text.is_empty());
            // Should not contain HTML tags
            assert!(!review.text.contains("<p>"), "Review text contains <p> tag");
            assert!(!review.text.contains("<div>"), "Review text contains <div> tag");
        }
    }

    #[test]
    fn test_parse_reviews_dates_are_iso() {
        let html =
            std::fs::read_to_string(testdata_path("novel_page_90435.html")).unwrap();
        let reviews = parse_reviews_from_html(&html, 100).unwrap();

        for review in &reviews {
            // datetime attributes are in ISO format like "2025-01-07T10:09:50.0000000"
            assert!(review.posted_date.contains("T"), "Date should be ISO format: {}", review.posted_date);
        }
    }

    #[test]
    fn test_parse_reviews_all_authors_present() {
        let html =
            std::fs::read_to_string(testdata_path("novel_page_90435.html")).unwrap();
        let reviews = parse_reviews_from_html(&html, 100).unwrap();

        let expected_authors = vec![
            "PhantomBuni",
            "vikray17",
            "bryanzero",
            "Some_Random_Cultivator",
            "7whitewolf7",
            "Detruejedi",
            "jumpsplat120",
            "Purplegriffin",
            "Evilish",
            "Kptn",
        ];

        let actual_authors: Vec<&str> = reviews.iter().map(|r| r.author.as_str()).collect();
        for expected in &expected_authors {
            assert!(
                actual_authors.contains(expected),
                "Missing author: {}",
                expected
            );
        }
    }
}
