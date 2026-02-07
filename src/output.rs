//! Result formatting and table output.
//!
//! Formats the scored novel results as a readable table using the `tabled` crate.

use crate::models::NovelScore;
use tabled::{Table, Tabled};

/// A row in the output table, derived from a `NovelScore`.
#[derive(Tabled)]
struct ResultRow {
    /// Rank position (1-based).
    #[tabled(rename = "Rank")]
    rank: usize,
    /// Novel title.
    #[tabled(rename = "Title")]
    title: String,
    /// Overall score formatted as percentage.
    #[tabled(rename = "Score")]
    score: String,
    /// Novel rating on RoyalRoad.
    #[tabled(rename = "Rating")]
    rating: String,
    /// Page count.
    #[tabled(rename = "Pages")]
    pages: u64,
    /// Publication status.
    #[tabled(rename = "Status")]
    status: String,
    /// Brief reasoning summary.
    #[tabled(rename = "Reasoning")]
    reasoning: String,
}

/// Format scored results as a table and print to stdout.
///
/// Results should be pre-sorted by score descending.
pub fn print_results(results: &[NovelScore]) {
    if results.is_empty() {
        println!("No novels matched the criteria.");
        return;
    }

    let rows: Vec<ResultRow> = results
        .iter()
        .enumerate()
        .map(|(i, score)| {
            // Truncate reasoning to a reasonable length for the table
            let reasoning = if score.reasoning.len() > 80 {
                format!("{}...", &score.reasoning[..77])
            } else {
                score.reasoning.clone()
            };

            ResultRow {
                rank: i + 1,
                title: score.novel.title.clone(),
                score: format!("{:.0}%", score.overall_score * 100.0),
                rating: format!("{:.2}", score.novel.rating),
                pages: score.novel.pages,
                status: score.novel.status.to_string(),
                reasoning,
            }
        })
        .collect();

    let table = Table::new(rows).to_string();
    println!("\n{}\n", table);
    println!("Total novels evaluated: {}", results.len());
}

/// Print a detailed breakdown for a single novel score.
pub fn print_detailed_score(score: &NovelScore) {
    println!("=== {} ===", score.novel.title);
    println!("URL: {}", score.novel.url);
    println!("Author: {}", score.novel.author);
    println!("Rating: {:.2} | Pages: {} | Status: {}", score.novel.rating, score.novel.pages, score.novel.status);
    println!("Overall Score: {:.0}%", score.overall_score * 100.0);
    println!();
    println!("Sub-scores:");
    let mut sub_scores: Vec<_> = score.sub_scores.iter().collect();
    sub_scores.sort_by_key(|(k, _)| k.clone());
    for (criterion, sub_score) in &sub_scores {
        println!("  {}: {:.0}%", criterion, *sub_score * 100.0);    
    }
    println!();
    println!("Reasoning: {}", score.reasoning);
    println!();
}
