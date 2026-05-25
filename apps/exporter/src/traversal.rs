use anyhow::{Context, Result};
use indicatif::ProgressBar;
use reqwest::{Client, StatusCode};
use serde::Deserialize;
use std::collections::{HashSet, VecDeque};
use tokio::time::{sleep, Duration};

use crate::constants::API_URL;

#[derive(Debug, Deserialize, Clone)]
pub struct CategoryMember {
    pub title: String,
    #[serde(rename = "type")]
    pub cm_type: Option<String>,
    #[serde(default)]
    pub ns: Option<i32>,
}

pub async fn generate_page_list(
    client: &Client,
    root_category: &str,
    max_depth: usize,
    traversal_delay_ms: u64,
) -> Result<Vec<String>> {
    let mut visited_categories: HashSet<String> = HashSet::new();
    let mut pages: HashSet<String> = HashSet::new();
    let mut queue: VecDeque<(String, usize)> = VecDeque::new();
    queue.push_back((root_category.to_string(), 0));

    let bar = ProgressBar::new_spinner();
    bar.set_message("Traversing categories");
    let mut cat_counter: usize = 0;

    while let Some((category, depth)) = queue.pop_front() {
        if depth > max_depth {
            continue;
        }
        if !visited_categories.insert(category.clone()) {
            continue;
        }
        bar.set_message(format!("Category: {} (depth {})", category, depth));

        let members = fetch_category_members(client, &category).await?;
        cat_counter += 1;
        for m in members {
            let is_subcat = matches!(m.cm_type.as_deref(), Some("subcat")) || m.ns == Some(14);
            if is_subcat {
                if depth < max_depth {
                    queue.push_back((m.title.clone(), depth + 1));
                }
            } else {
                pages.insert(m.title.clone());
            }
        }
        if traversal_delay_ms > 0 {
            sleep(Duration::from_millis(traversal_delay_ms)).await;
        }
    }
    bar.finish_with_message("Category traversal complete");
    println!(
        "Traversal summary: {} categories visited, {} pages collected",
        cat_counter,
        pages.len()
    );
    Ok(pages.into_iter().collect())
}

async fn fetch_category_members(client: &Client, category: &str) -> Result<Vec<CategoryMember>> {
    let mut results = Vec::new();
    let mut cmcontinue: Option<String> = None;

    loop {
        let mut url = format!(
            "{API_URL}?action=query&format=json&list=categorymembers&cmtitle={}&cmlimit=500&cmtype=page|subcat",
            urlencoding::encode(category)
        );
        if let Some(ref cont) = cmcontinue {
            url.push_str(&format!("&cmcontinue={}", urlencoding::encode(cont)));
        }

        let resp = client.get(&url).send().await?;
        if resp.status() == StatusCode::TOO_MANY_REQUESTS {
            eprintln!("429 rate limited fetching category members; sleeping 10s");
            sleep(Duration::from_secs(10)).await;
            continue;
        }
        let text = resp.text().await?;
        let v: serde_json::Value =
            serde_json::from_str(&text).context("Parsing categorymembers json")?;

        if let Some(arr) = v
            .get("query")
            .and_then(|q| q.get("categorymembers"))
            .and_then(|cm| cm.as_array())
        {
            for item in arr {
                if let Some(title) = item.get("title").and_then(|t| t.as_str()) {
                    let ttype = item
                        .get("type")
                        .and_then(|t| t.as_str())
                        .map(|s| s.to_string());
                    results.push(CategoryMember {
                        title: title.to_string(),
                        cm_type: ttype,
                        ns: item.get("ns").and_then(|n| n.as_i64()).map(|v| v as i32),
                    });
                }
            }
        }
        if let Some(cont) = v
            .get("continue")
            .and_then(|c| c.get("cmcontinue"))
            .and_then(|c| c.as_str())
        {
            cmcontinue = Some(cont.to_string());
            sleep(Duration::from_millis(100)).await;
            continue;
        }
        break;
    }
    Ok(results)
}
