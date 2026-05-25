use crate::error::{ConvertError, Result};
use once_cell::sync::OnceCell;
use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Arc;

#[derive(Debug, Clone)]
struct PageIndexEntry {
    title: String,
    lower_title: String,
    path: PathBuf,
}

/// Reader for flat export_out layout with URL-encoded filenames.
#[derive(Debug, Clone)]
pub struct WikiExport {
    pub root: PathBuf,
    page_index: Arc<OnceCell<Vec<PageIndexEntry>>>,
}

impl WikiExport {
    pub fn new(root: &Path) -> Self {
        let candidate = root.join("export_out");
        if candidate.is_dir() {
            Self {
                root: candidate,
                page_index: Arc::new(OnceCell::new()),
            }
        } else {
            Self {
                root: root.to_path_buf(),
                page_index: Arc::new(OnceCell::new()),
            }
        }
    }

    /// Read a champion main page wikitext.
    pub fn read_champion_main(&self, name: &str) -> Result<String> {
        self.read_entity_page(name, || ConvertError::ChampionNotFound(name.to_string()))
    }

    /// Read an item page by title (flat export naming expected).
    pub fn read_item_main(&self, name: &str) -> Result<String> {
        self.read_entity_page(name, || ConvertError::ItemNotFound(name.to_string()))
    }

    /// Read a rune page by title (flat export naming expected).
    pub fn read_rune_main(&self, name: &str) -> Result<String> {
        self.read_entity_page(name, || ConvertError::RuneNotFound(name.to_string()))
    }

    /// Read Module/ChampionData/data/page.txt or a flat variant if shipped that way.
    pub fn read_champion_module_data(&self) -> Result<Option<String>> {
        // Try common encodings of Module:ChampionData and data subpage
        let candidates = [
            "Module%3AChampionData%2Fdata.txt",
            "Module%3AChampionData.txt",
            "Module%3AChampionData%2FData.txt",
        ];
        for c in candidates {
            let p = self.root.join(c);
            if p.exists() {
                return Ok(Some(fs::read_to_string(p)?));
            }
        }
        Ok(None)
    }

    /// Read Module:ItemData/data if present.
    pub fn read_item_module_data(&self) -> Result<Option<String>> {
        let candidates = [
            "Module%3AItemData%2Fdata.txt",
            "Module%3AItemData.txt",
            "Module%3AItemData%2FData.txt",
        ];
        for c in candidates {
            let p = self.root.join(c);
            if p.exists() {
                return Ok(Some(fs::read_to_string(p)?));
            }
        }
        Ok(None)
    }

    /// Read all available ItemData module sources that contribute to item conversion.
    pub fn read_item_module_data_sources(&self) -> Result<Vec<(String, String)>> {
        let candidates = [
            ("Module:ItemData/data", "Module%3AItemData%2Fdata.txt"),
            (
                "Module:ItemData/data/removed",
                "Module%3AItemData%2Fdata%2Fremoved.txt",
            ),
            ("Module:ItemData", "Module%3AItemData.txt"),
            ("Module:ItemData/Data", "Module%3AItemData%2FData.txt"),
        ];
        let mut seen = HashSet::new();
        let mut sources = Vec::new();
        for (title, encoded) in candidates {
            let path = self.root.join(encoded);
            if !path.exists() || !seen.insert(path.clone()) {
                continue;
            }
            sources.push((title.to_string(), fs::read_to_string(path)?));
        }
        Ok(sources)
    }

    /// Read a raw Template page by its full decoded title (e.g., "Template:Data Akshan/Q").
    /// Follows a single-level redirect if the file starts with "#REDIRECT [[...]]".
    pub fn read_template_page(&self, title: &str) -> Result<Option<String>> {
        let encoded = url_encode(title);
        let path = self.root.join(format!("{}.txt", encoded));
        if !path.exists() {
            return Ok(None);
        }
        let content = std::fs::read_to_string(&path).map_err(ConvertError::Io)?;
        // Handle redirects of the form: #REDIRECT [[Template:Data Akshan/Avengerang]]
        let trimmed = content.trim_start();
        if let Some(rest) = trimmed.strip_prefix("#REDIRECT [[") {
            if let Some(end) = rest.find("]]") {
                let target = &rest[..end];
                if let Some(t) = self.read_template_page(target)? {
                    return Ok(Some(t));
                }
            }
        }
        Ok(Some(content))
    }

    /// List all Template:Data <Champion> subpages available in the export (decoded titles).
    pub fn list_champion_ability_templates(&self, champ: &str) -> Result<Vec<String>> {
        let mut out = Vec::new();
        let prefix = format!("template:data {}/", champ.to_ascii_lowercase());
        for entry in self.page_index()? {
            if entry.lower_title.starts_with(&prefix) {
                out.push(entry.title.clone());
            }
        }
        out.sort();
        Ok(out)
    }
}

impl WikiExport {
    /// Read any page by its decoded title, returning `None` when the file does not exist.
    pub fn read_optional_page(&self, title: &str) -> Result<Option<String>> {
        let Some(path) = self.find_entity_page_path(title)? else {
            return Ok(None);
        };
        Ok(Some(fs::read_to_string(path)?))
    }

    pub fn list_titles_with_prefix(&self, prefix: &str) -> Result<Vec<String>> {
        let prefix_lower = prefix.to_ascii_lowercase();
        let mut titles = Vec::new();
        for entry in self.page_index()? {
            if entry.lower_title.starts_with(&prefix_lower) {
                titles.push(entry.title.clone());
            }
        }
        titles.sort();
        titles.dedup();
        Ok(titles)
    }

    fn read_entity_page<F>(&self, title: &str, missing: F) -> Result<String>
    where
        F: FnOnce() -> ConvertError,
    {
        if let Some(path) = self.find_entity_page_path(title)? {
            return Ok(fs::read_to_string(path)?);
        }
        Err(missing())
    }

    fn find_entity_page_path(&self, title: &str) -> Result<Option<PathBuf>> {
        let fname = format!("{}.txt", url_encode(title));
        let path = self.root.join(&fname);
        if path.exists() {
            return Ok(Some(path));
        }
        let title_lower = title.to_ascii_lowercase();
        for entry in self.page_index()? {
            if entry.lower_title == title_lower {
                return Ok(Some(entry.path.clone()));
            }
        }
        Ok(None)
    }

    fn page_index(&self) -> Result<&Vec<PageIndexEntry>> {
        self.page_index.get_or_try_init(|| {
            let mut entries = Vec::new();
            for entry in fs::read_dir(&self.root).map_err(ConvertError::Io)? {
                let entry = entry.map_err(ConvertError::Io)?;
                let path = entry.path();
                if path.extension().and_then(|e| e.to_str()) != Some("txt") {
                    continue;
                }
                let stem = path.file_stem().and_then(|s| s.to_str()).unwrap_or("");
                let title = url_decode(stem);
                entries.push(PageIndexEntry {
                    lower_title: title.to_ascii_lowercase(),
                    title,
                    path,
                });
            }
            Ok(entries)
        })
    }
}

/// Minimal URL encoding matching MediaWiki export file naming in export_out examples.
/// Only encodes spaces and a small set to align with observed dataset; extend if needed.
pub fn url_encode(s: &str) -> String {
    // Use percent-encoding for bytes outside unreserved + encode space as %20
    let mut out = String::new();
    for ch in s.chars() {
        match ch {
            'A'..='Z' | 'a'..='z' | '0'..='9' | '-' | '_' | '.' | '~' => out.push(ch),
            ' ' => out.push_str("%20"),
            '/' => out.push_str("%2F"),
            '(' => out.push_str("%28"),
            ')' => out.push_str("%29"),
            '\'' => out.push_str("%27"),
            '!' => out.push_str("%21"),
            '*' => out.push_str("%2A"),
            _ => {
                let mut buf = [0u8; 4];
                for b in ch.encode_utf8(&mut buf).as_bytes() {
                    out.push('%');
                    out.push_str(&format!("{:02X}", b));
                }
            }
        }
    }
    out
}

/// Percent-decoding for file names back to page titles.
pub fn url_decode(s: &str) -> String {
    let mut out = String::new();
    let bytes = s.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'%' && i + 2 < bytes.len() {
            let h1 = (bytes[i + 1] as char).to_digit(16);
            let h2 = (bytes[i + 2] as char).to_digit(16);
            if let (Some(a), Some(b)) = (h1, h2) {
                out.push(((a * 16 + b) as u8) as char);
                i += 3;
                continue;
            }
        }
        out.push(bytes[i] as char);
        i += 1;
    }
    out
}

impl WikiExport {
    /// List champion names available in the dataset by scanning either the exploded tree or flat files.
    pub fn list_champion_names(&self) -> Result<Vec<String>> {
        let mut names = Vec::new();
        for entry in self.page_index()? {
            let path = &entry.path;
            let title = entry.title.clone();
            if title.contains('/') || title.contains(':') {
                continue;
            }
            // Cheap content sniff: look for Champion info template in first 4KB
            if let Ok(mut file) = fs::File::open(path) {
                use std::io::Read;
                let mut buf = vec![0u8; 4096];
                let n = file.read(&mut buf).unwrap_or(0);
                let head = String::from_utf8_lossy(&buf[..n]);
                if head.contains("{{Champion info") {
                    names.push(title);
                }
            }
        }
        names.sort_unstable();
        Ok(names)
    }

    pub fn list_item_names(&self) -> Result<Vec<String>> {
        let mut names = Vec::new();
        for entry in self.page_index()? {
            let path = &entry.path;
            let title = entry.title.clone();
            if title.contains(':') || title.contains('/') {
                continue;
            }
            if let Ok(mut file) = fs::File::open(path) {
                use std::io::Read;
                let mut buf = vec![0u8; 8192];
                let n = file.read(&mut buf).unwrap_or(0);
                let head = String::from_utf8_lossy(&buf[..n]);
                let has_item_info = crate::parse::extract_balanced_templates(&head)
                    .unwrap_or_default()
                    .into_iter()
                    .any(|span| span.name.trim().eq_ignore_ascii_case("Item info"));
                if has_item_info {
                    names.push(title);
                }
            }
        }
        names.sort_unstable();
        Ok(names)
    }

    pub fn list_rune_names(&self) -> Result<Vec<String>> {
        let mut names = Vec::new();
        for entry in self.page_index()? {
            let path = &entry.path;
            let title = entry.title.clone();
            if title.contains(':') || title.contains('/') {
                continue;
            }
            if let Ok(mut file) = fs::File::open(path) {
                use std::io::Read;
                let mut buf = vec![0u8; 8192];
                let n = file.read(&mut buf).unwrap_or(0);
                let head = String::from_utf8_lossy(&buf[..n]);
                let has_rune_markup = crate::parse::extract_balanced_templates(&head)
                    .unwrap_or_default()
                    .into_iter()
                    .any(|span| {
                        matches!(
                            span.name.trim().to_ascii_lowercase().as_str(),
                            "rune" | "rune info" | "rune box" | "rune infobox" | "rune header"
                        )
                    });
                if has_rune_markup {
                    names.push(title);
                }
            }
        }
        names.sort_unstable();
        Ok(names)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    #[test]
    fn url_encoding_examples() {
        assert_eq!(url_encode("Aatrox"), "Aatrox");
        assert_eq!(url_encode("Aatrox/Audio"), "Aatrox%2FAudio");
        assert_eq!(
            url_encode("10th Anniversary Event"),
            "10th%20Anniversary%20Event"
        );
        assert_eq!(
            url_encode("Adaptive Armor (Season 2014 Mastery)"),
            "Adaptive%20Armor%20%28Season%202014%20Mastery%29"
        );
    }

    #[test]
    fn url_decoding_examples() {
        assert_eq!(url_decode("Aatrox"), "Aatrox");
        assert_eq!(url_decode("Aatrox%2FAudio"), "Aatrox/Audio");
        assert_eq!(
            url_decode("10th%20Anniversary%20Event"),
            "10th Anniversary Event"
        );
    }

    #[test]
    fn read_champion_from_flat_root_direct() {
        let td = tempdir().unwrap();
        std::fs::write(td.path().join("Ahri.txt"), "hello").unwrap();
        let exp = WikiExport::new(td.path());
        let s = exp.read_champion_main("Ahri").unwrap();
        assert_eq!(s, "hello");
    }

    #[test]
    fn read_champion_from_export_out_subdir() {
        let td = tempdir().unwrap();
        let flat = td.path().join("export_out");
        std::fs::create_dir_all(&flat).unwrap();
        std::fs::write(
            flat.join("Akshan.txt"),
            "{{Champion info|...}}\n== Abilities ==",
        )
        .unwrap();
        let exp = WikiExport::new(td.path());
        let s = exp.read_champion_main("Akshan").unwrap();
        assert!(s.contains("Abilities"));
    }

    #[test]
    fn read_optional_page_returns_none_for_missing_file() {
        let td = tempdir().unwrap();
        let exp = WikiExport::new(td.path());
        assert!(exp.read_optional_page("Missing/Page").unwrap().is_none());
    }

    #[test]
    fn list_titles_with_prefix_matches_subpages() {
        let td = tempdir().unwrap();
        let flat = td.path().join("export_out");
        std::fs::create_dir_all(&flat).unwrap();
        std::fs::write(flat.join("Akshan.txt"), "{{Champion info}}\n").unwrap();
        std::fs::write(flat.join("Akshan%2FHistory.txt"), "History").unwrap();
        std::fs::write(flat.join("Akshan%2FTrivia.txt"), "Trivia").unwrap();
        let exp = WikiExport::new(td.path());
        let titles = exp.list_titles_with_prefix("Akshan/").unwrap();
        assert_eq!(
            titles,
            vec!["Akshan/History".to_string(), "Akshan/Trivia".to_string()]
        );
    }

    #[test]
    fn list_item_names_detects_item_pages() {
        let td = tempdir().unwrap();
        let flat = td.path().join("export_out");
        std::fs::create_dir_all(&flat).unwrap();
        std::fs::write(flat.join("Infinity%20Edge.txt"), "{{Item info|tier=2}}\n").unwrap();
        std::fs::write(
            flat.join("Akshan.txt"),
            "{{Champion info|title=Rogue Sentinel}}\n",
        )
        .unwrap();
        let exp = WikiExport::new(td.path());
        let items = exp.list_item_names().unwrap();
        assert_eq!(items, vec!["Infinity Edge".to_string()]);
    }

    #[test]
    fn list_item_names_ignores_item_info_variants_and_templates() {
        let td = tempdir().unwrap();
        let flat = td.path().join("export_out");
        std::fs::create_dir_all(&flat).unwrap();
        std::fs::write(
            flat.join("Perfect%20Timing.txt"),
            "{{rune header}}\n{{Item info/var|Stopwatch}}\n",
        )
        .unwrap();
        std::fs::write(
            flat.join("Template%3AItem%20info.txt"),
            "{{Item info/doc}}\n",
        )
        .unwrap();
        std::fs::write(
            flat.join("Arcane%20Sweeper.txt"),
            "{{Item info|Arcane Sweeper (Trinket)}}\n",
        )
        .unwrap();
        let exp = WikiExport::new(td.path());
        let items = exp.list_item_names().unwrap();
        assert_eq!(items, vec!["Arcane Sweeper".to_string()]);
    }

    #[test]
    fn list_rune_names_detects_rune_pages_and_ignores_templates() {
        let td = tempdir().unwrap();
        let flat = td.path().join("export_out");
        std::fs::create_dir_all(&flat).unwrap();
        std::fs::write(
            flat.join("Electrocute.txt"),
            "{{rune header|Electrocute}}\n",
        )
        .unwrap();
        std::fs::write(
            flat.join("Perfect%20Timing.txt"),
            "{{rune header|Perfect Timing}}\n{{Item info/var|Stopwatch}}\n",
        )
        .unwrap();
        std::fs::write(
            flat.join("Template%3ARune%20data%20Electrocute.txt"),
            "{{Rune data|Electrocute|description}}\n",
        )
        .unwrap();
        std::fs::write(flat.join("Infinity%20Edge.txt"), "{{Item info|tier=2}}\n").unwrap();

        let exp = WikiExport::new(td.path());
        let runes = exp.list_rune_names().unwrap();
        assert_eq!(
            runes,
            vec!["Electrocute".to_string(), "Perfect Timing".to_string()]
        );
    }
}
