use std::path::{Path, PathBuf};
use std::fs;
use crate::error::{Result, ConvertError};

/// Reader for flat export_out layout with URL-encoded filenames.
#[derive(Debug, Clone)]
pub struct WikiExport {
	pub root: PathBuf,
}

impl WikiExport {
	pub fn new(root: &Path) -> Self {
		let candidate = root.join("export_out");
		if candidate.is_dir() { Self { root: candidate } } else { Self { root: root.to_path_buf() } }
	}

	/// Read a champion main page wikitext.
	pub fn read_champion_main(&self, name: &str) -> Result<String> {
		let fname = format!("{}.txt", url_encode(name));
		let f = self.root.join(fname);
		println!("[WikiExport] root={} candidate={} exists={}", self.root.display(), f.display(), f.exists());
		if f.exists() { return Ok(fs::read_to_string(f)?); }
		// Fallback: scan all txt files and compare decoded stems
		for entry in fs::read_dir(&self.root).map_err(|e| ConvertError::Io(e))? {
			let entry = entry.map_err(|e| ConvertError::Io(e))?; let path = entry.path();
			if path.extension().and_then(|e| e.to_str()) != Some("txt") { continue; }
			let stem = path.file_stem().and_then(|s| s.to_str()).unwrap_or("");
			println!("[WikiExport] saw file stem={} decoded={} target={}", stem, url_decode(stem), name);
			if url_decode(stem).eq_ignore_ascii_case(name) { return Ok(fs::read_to_string(path)?); }
		}
		Err(ConvertError::ChampionNotFound(name.to_string()))
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
			if p.exists() { return Ok(Some(fs::read_to_string(p)?)); }
		}
		Ok(None)
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
				for b in ch.encode_utf8(&mut buf).as_bytes() { out.push('%'); out.push_str(&format!("{:02X}", b)); }
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
		for entry in fs::read_dir(&self.root)? {
			let entry = entry?; let path = entry.path();
			if path.extension().and_then(|e| e.to_str()) != Some("txt") { continue; }
			let fname = path.file_stem().and_then(|s| s.to_str()).unwrap_or("");
			let title = url_decode(fname);
			// Cheap content sniff: look for Champion info template in first 4KB
			if let Ok(mut file) = fs::File::open(&path) {
				use std::io::Read;
				let mut buf = vec![0u8; 4096];
				let n = file.read(&mut buf).unwrap_or(0);
				let head = String::from_utf8_lossy(&buf[..n]);
				if head.contains("{{Champion info") { names.push(title); }
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
		assert_eq!(url_encode("10th Anniversary Event"), "10th%20Anniversary%20Event");
		assert_eq!(url_encode("Adaptive Armor (Season 2014 Mastery)"), "Adaptive%20Armor%20%28Season%202014%20Mastery%29");
	}

	#[test]
	fn url_decoding_examples() {
		assert_eq!(url_decode("Aatrox"), "Aatrox");
		assert_eq!(url_decode("Aatrox%2FAudio"), "Aatrox/Audio");
		assert_eq!(url_decode("10th%20Anniversary%20Event"), "10th Anniversary Event");
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
		std::fs::write(flat.join("Akshan.txt"), "{{Champion info|...}}\n== Abilities ==").unwrap();
		let exp = WikiExport::new(td.path());
		let s = exp.read_champion_main("Akshan").unwrap();
		assert!(s.contains("Abilities"));
	}
}

