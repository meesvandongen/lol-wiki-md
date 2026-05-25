pub fn encode_title_filename(title: &str) -> String {
    let encoded = urlencoding::encode(title).to_string();
    if encoded.len() > 240 {
        encoded[..240].to_string()
    } else {
        encoded
    }
}

pub fn extract_tag_value(block: &str, tag: &str) -> Option<String> {
    let open_prefix = format!("<{tag}");
    let start_tag_pos = block.find(&open_prefix)?;
    let after_start = &block[start_tag_pos + open_prefix.len()..];
    let gt_pos = after_start.find('>')?;
    let content_start = start_tag_pos + open_prefix.len() + gt_pos + 1;
    let close = format!("</{tag}>");
    let end_rel = block[content_start..].find(&close)?;
    Some(block[content_start..content_start + end_rel].to_string())
}

pub fn decode_xml_entities(input: &str) -> String {
    html_escape::decode_html_entities(input).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trip_filename_encoding() {
        let title = "Ahri/History:Test%Stuff?*";
        let encoded = encode_title_filename(title);
        let decoded = urlencoding::decode(&encoded).unwrap();
        assert_eq!(decoded, title[..decoded.len()]);
    }

    #[test]
    fn extract_tag_with_attributes() {
        let sample = r#"<page><title>Sample</title><text xml:space=\"preserve\" bytes=\"10\">Hello World</text></page>"#;
        assert_eq!(
            extract_tag_value(sample, "title"),
            Some("Sample".to_string())
        );
        assert_eq!(
            extract_tag_value(sample, "text"),
            Some("Hello World".to_string())
        );
    }

    #[test]
    fn decode_entities() {
        let src = "&lt;gallery&gt;Fish &amp; Chips &amp; Tea &#169; &#x1F600; &quot;Q&quot;&lt;/gallery&gt;";
        let decoded = decode_xml_entities(src);
        assert!(decoded.contains('<') && decoded.contains('>'));
        assert!(decoded.contains('©'));
        assert!(decoded.contains('😀'));
        assert!(decoded.contains("Fish & Chips & Tea"));
    }
}
