use std::{path::PathBuf, sync::Arc, time::Duration};

use anyhow::Result;
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::{Client, StatusCode};
use tokio::time::sleep;

use crate::{
    args::Args,
    constants::EXPORT_URL,
    metrics::{BatchMetrics, ExportMetrics},
    parse::{decode_xml_entities, encode_title_filename, extract_tag_value},
};

pub async fn bulk_export(client: &Client, pages: &[String], args: &Args) -> Result<ExportMetrics> {
    println!(
        "Starting bulk export: {} source titles (batch size {}, concurrency {})",
        pages.len(),
        args.batch_size,
        args.concurrency
    );

    let total_batches = (pages.len() + args.batch_size - 1) / args.batch_size;
    let bar = ProgressBar::new(total_batches as u64);
    bar.set_style(
        ProgressStyle::with_template(
            "[{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} batches ({eta})",
        )?
        .progress_chars("=>-"),
    );

    let sem = Arc::new(tokio::sync::Semaphore::new(if args.concurrency == 0 {
        1
    } else {
        args.concurrency
    }));
    let mut handles = Vec::new();

    let start = std::time::Instant::now();
    let started_at = chrono::Utc::now();

    for (batch_index, chunk) in pages.chunks(args.batch_size).enumerate() {
        let permit = sem.clone().acquire_owned().await.expect("Semaphore closed");
        let titles: Vec<String> = chunk.to_vec();
        let client_clone = client.clone();
        let bar_clone = bar.clone();
        let out_dir = args.out_dir.clone();
        let batch_delay_ms = args.batch_delay_ms;
        let batch_num = batch_index + 1;
        handles.push(tokio::spawn(async move {
            let res = process_batch(batch_num, titles, client_clone, out_dir, batch_delay_ms).await;
            bar_clone.inc(1);
            drop(permit);
            res
        }));
    }

    let mut successful_batches = 0usize;
    let mut failed_batches: Vec<usize> = Vec::new();
    let mut pages_exported = 0usize;
    let mut parse_errors = 0usize;
    let mut bytes_downloaded = 0usize;
    let mut total_batch_time = Duration::from_secs(0);

    use std::collections::HashSet;
    let mut unique_titles: HashSet<String> = HashSet::new();
    for h in handles {
        match h.await {
            Ok((_, Ok(m))) => {
                successful_batches += 1;
                pages_exported += m.pages_written;
                parse_errors += m.parse_errors;
                bytes_downloaded += m.bytes_downloaded;
                total_batch_time += m.elapsed;
                for t in m.titles_written {
                    unique_titles.insert(t);
                }
            }
            Ok((batch_num, Err(e))) => {
                eprintln!("Batch {batch_num} failed: {e:?}");
                failed_batches.push(batch_num);
            }
            Err(e) => {
                eprintln!("Join error on batch task: {e:?}");
            }
        }
    }

    bar.finish_with_message("Export complete");

    let duration = start.elapsed();
    if !failed_batches.is_empty() {
        eprintln!(
            "{} batches had errors: {:?}",
            failed_batches.len(),
            failed_batches
        );
    }
    let duration_secs = duration.as_secs_f64();
    let pages_per_sec = if duration_secs > 0.0 {
        pages_exported as f64 / duration_secs
    } else {
        0.0
    };
    let average_batch_duration_secs = if successful_batches > 0 {
        total_batch_time.as_secs_f64() / successful_batches as f64
    } else {
        0.0
    };

    let unique_pages_written = unique_titles.len();
    let duplicate_writes = if pages_exported >= unique_pages_written {
        pages_exported - unique_pages_written
    } else {
        0
    };

    let metrics = ExportMetrics {
        total_source_titles: pages.len(),
        total_batches,
        successful_batches,
        failed_batches: failed_batches.len(),
        failed_batch_numbers: failed_batches,
        pages_exported,
        unique_pages_written,
        duplicate_writes,
        parse_errors,
        bytes_downloaded,
        duration_secs,
        pages_per_sec,
        average_batch_duration_secs,
        concurrency: args.concurrency,
        started_at: started_at.to_rfc3339(),
        finished_at: chrono::Utc::now().to_rfc3339(),
    };

    Ok(metrics)
}

async fn process_batch(
    batch_num: usize,
    titles: Vec<String>,
    client: Client,
    out_dir: PathBuf,
    batch_delay_ms: u64,
) -> (usize, Result<BatchMetrics>) {
    let batch_start = std::time::Instant::now();
    let joined = titles.join("\n");
    let form = [
        ("pages", joined.as_str()),
        ("wpDownload", "1"),
        ("templates", "1"),
        ("curonly", "1"),
        ("wpEditToken", "+\\"),
    ];

    let mut attempt = 0u32;
    let max_attempts = 5u32;
    let backoff_base = 5u64;
    let resp_bytes = loop {
        attempt += 1;
        match client.post(EXPORT_URL).form(&form).send().await {
            Ok(resp) => {
                if resp.status() == StatusCode::TOO_MANY_REQUESTS {
                    let sleep_for = backoff_base * attempt as u64 * 2;
                    eprintln!("429 on batch {batch_num} attempt {attempt}; sleeping {sleep_for}s");
                    sleep(Duration::from_secs(sleep_for)).await;
                    continue;
                }
                if !resp.status().is_success() {
                    let status = resp.status();
                    if attempt >= max_attempts {
                        return (
                            batch_num,
                            Err(anyhow::anyhow!(
                                "HTTP status {status} after {attempt} attempts on batch {batch_num}"
                            )),
                        );
                    }
                    eprintln!("HTTP {status} on batch {batch_num} attempt {attempt}; retrying");
                    sleep(Duration::from_secs(backoff_base * attempt as u64)).await;
                    continue;
                }
                match resp.bytes().await {
                    Ok(b) => break b,
                    Err(e) => {
                        if attempt >= max_attempts {
                            return (
                                batch_num,
                                Err(anyhow::anyhow!(
                                    "Bytes read failed after {attempt} attempts on batch {batch_num}: {e}"
                                )),
                            );
                        }
                        eprintln!(
                            "Read bytes error on batch {batch_num} attempt {attempt}: {e}; retrying"
                        );
                        sleep(Duration::from_secs(backoff_base * attempt as u64)).await;
                        continue;
                    }
                }
            }
            Err(e) => {
                if attempt >= max_attempts {
                    return (
                        batch_num,
                        Err(anyhow::anyhow!(
                            "Request error after {attempt} attempts on batch {batch_num}: {e}"
                        )),
                    );
                }
                eprintln!("Network error on batch {batch_num} attempt {attempt}: {e}; retrying");
                sleep(Duration::from_secs(backoff_base * attempt as u64)).await;
                continue;
            }
        }
    };

    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .subsec_nanos();
    let jitter_ms: u64 = nanos as u64 % 250;
    sleep(Duration::from_millis(jitter_ms)).await;

    let xml_str = String::from_utf8_lossy(&resp_bytes);
    let mut written = 0usize;
    let mut titles_written: Vec<String> = Vec::new();
    let mut parse_errors = 0usize;

    let mut search_start = 0usize;
    while let Some(page_start_rel) = xml_str[search_start..].find("<page>") {
        let page_start = search_start + page_start_rel;
        if let Some(page_end_rel) = xml_str[page_start..].find("</page>") {
            let page_end = page_start + page_end_rel + "</page>".len();
            let page_block = &xml_str[page_start..page_end];
            match (
                extract_tag_value(page_block, "title"),
                extract_tag_value(page_block, "text"),
            ) {
                (Some(title), Some(wikitext_raw)) => {
                    let wikitext = decode_xml_entities(&wikitext_raw);
                    let encoded_title = encode_title_filename(&title);
                    let path = out_dir.join(format!("{}.txt", encoded_title));
                    let tmp_path = out_dir.join(format!("{}.txt.tmp", encoded_title));
                    if let Err(e) = std::fs::write(&tmp_path, wikitext.as_bytes()) {
                        eprintln!("Write error (tmp) batch {batch_num} page '{title}': {e}");
                    } else if let Err(e) = std::fs::rename(&tmp_path, &path) {
                        if path.exists() {
                            if let Err(e2) = std::fs::write(&path, wikitext.as_bytes()) {
                                eprintln!(
                                    "Fallback write error batch {batch_num} page '{title}': {e2}"
                                );
                            }
                        } else {
                            eprintln!(
                                "Rename error batch {batch_num} page '{title}': {e}. Attempting direct write."
                            );
                            if let Err(e2) = std::fs::write(&path, wikitext.as_bytes()) {
                                eprintln!(
                                    "Direct write after rename fail batch {batch_num} page '{title}': {e2}"
                                );
                            }
                        }
                    } else {
                        written += 1;
                        titles_written.push(encoded_title);
                    }
                }
                _ => {
                    parse_errors += 1;
                }
            }
            search_start = page_end;
        } else {
            break;
        }
    }

    if batch_delay_ms > 0 {
        sleep(Duration::from_millis(batch_delay_ms)).await;
    }
    if written == 0 {
        eprintln!("Warning: batch {batch_num} produced no pages");
    }

    let metrics = BatchMetrics {
        pages_written: written,
        parse_errors,
        bytes_downloaded: resp_bytes.len(),
        elapsed: batch_start.elapsed(),
        titles_written,
    };

    (batch_num, Ok(metrics))
}
