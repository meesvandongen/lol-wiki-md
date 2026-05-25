use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Debug, Serialize, Deserialize)]
pub struct ExportMetrics {
    pub total_source_titles: usize,
    pub total_batches: usize,
    pub successful_batches: usize,
    pub failed_batches: usize,
    pub failed_batch_numbers: Vec<usize>,
    pub pages_exported: usize,
    pub unique_pages_written: usize,
    pub duplicate_writes: usize,
    pub parse_errors: usize,
    pub bytes_downloaded: usize,
    pub duration_secs: f64,
    pub pages_per_sec: f64,
    pub average_batch_duration_secs: f64,
    pub concurrency: usize,
    pub started_at: String,
    pub finished_at: String,
}

#[derive(Debug)]
pub struct BatchMetrics {
    pub pages_written: usize,
    pub parse_errors: usize,
    pub bytes_downloaded: usize,
    pub elapsed: Duration,
    pub titles_written: Vec<String>,
}
