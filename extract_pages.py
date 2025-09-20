#!/usr/bin/env python3
"""
Extract each <page> from MediaWiki XML export files into individual text files.

- Parses <siteinfo><namespaces> to map ns ids to names (e.g., 6 -> File).
- Builds output path as: out/<Namespace>/<Title as folders>/page.txt
  - Namespace '0' becomes 'Main'.
  - Title slashes become nested folders (MediaWiki subpages).
  - For non-main namespaces, the leading 'Namespace:' in <title> is removed.
- Writes the raw wikitext from <revision><text> into page.txt using UTF-8.

Usage:
  python extract_pages.py runes.xml items.xml champions.xml

Optional flags:
  --out OUT_DIR           Output directory (default: out)
  --ext EXT               Output file extension without dot (default: txt)
  --dry-run               Do not write files, just log actions
  --verbose               Print more progress information

This script uses xml.etree.ElementTree.iterparse for streaming large files.
"""
from __future__ import annotations

import argparse
import os
import re
import sys
import xml.etree.ElementTree as ET
from typing import Dict, Optional

MW_NS = {"mw": "http://www.mediawiki.org/xml/export-0.11/"}

# Windows reserved names (case-insensitive)
WINDOWS_RESERVED = {
    "CON", "PRN", "AUX", "NUL",
    *{f"COM{i}" for i in range(1, 10)},
    *{f"LPT{i}" for i in range(1, 10)},
}

INVALID_CHARS_RE = re.compile(r"[<>:\\/\|\?\*\x00-\x1F]")
MULTI_SEP_RE = re.compile(r"[_\s]{2,}")
TRIM_END_RE = re.compile(r"[\s\.]+$")


def sanitize_segment(name: str) -> str:
    """Make a safe filesystem segment for Windows/macOS/Linux.
    - Remove invalid characters
    - Replace spaces with underscore
    - Collapse multiple underscores/spaces
    - Strip trailing spaces/dots
    - Avoid reserved device names (append underscore)
    """
    # Keep unicode, but strip control chars and invalids
    cleaned = INVALID_CHARS_RE.sub("_", name)
    # Replace remaining spaces with underscore
    cleaned = cleaned.replace(" ", "_")
    cleaned = MULTI_SEP_RE.sub("_", cleaned)
    cleaned = TRIM_END_RE.sub("", cleaned)
    if not cleaned:
        cleaned = "untitled"
    if cleaned.upper() in WINDOWS_RESERVED:
        cleaned = cleaned + "_"
    return cleaned


def get_namespace_map(root: ET.Element) -> Dict[str, str]:
    """Extract ns id -> name from <siteinfo><namespaces>.
    Returns names with spaces as-is; caller may sanitize for paths.
    For ns id '0' (main), returns empty string "" (as MediaWiki does).
    """
    ns_map: Dict[str, str] = {}
    siteinfo = root.find("mw:siteinfo", MW_NS)
    if siteinfo is None:
        return ns_map
    namespaces = siteinfo.find("mw:namespaces", MW_NS)
    if namespaces is None:
        return ns_map
    for ns in namespaces.findall("mw:namespace", MW_NS):
        key = ns.get("key") or ""
        name = (ns.text or "").strip()
        ns_map[key] = name
    return ns_map


def derive_paths(title: str, ns_id: str, ns_map: Dict[str, str]) -> tuple[list[str], str]:
    """Derive (folder_segments, filename) from title and namespace.
    - folder_segments: [NamespaceFolder, ...title parts...]
    - filename: final file name (e.g., 'page') without extension
    """
    ns_name = ns_map.get(ns_id, "")
    ns_folder = sanitize_segment(ns_name) if ns_name else "Main"

    # Remove leading "NsName:" from title if it matches the declared ns
    local_title = title
    if ns_name:
        prefix = f"{ns_name}:"
        # Compare case-insensitively on the prefix portion only (MediaWiki is case-sensitive after first char rules, but this is good enough)
        if title.startswith(prefix):
            local_title = title[len(prefix):]
    
    # Split on MediaWiki subpage delimiter
    parts = [p for p in local_title.split("/") if p]
    safe_parts = [sanitize_segment(p) for p in parts]

    # Use 'page' as content file name; write content under a folder representing title structure
    folder_segments = [ns_folder] + safe_parts
    filename = "page"
    return folder_segments, filename


def write_page(out_dir: str, folder_segments: list[str], filename_no_ext: str, ext: str, content: str, dry_run: bool, verbose: bool) -> None:
    rel_dir = os.path.join(*folder_segments) if folder_segments else ""
    full_dir = os.path.join(out_dir, rel_dir)
    full_path = os.path.join(full_dir, f"{filename_no_ext}.{ext}")
    if verbose:
        print(f"-> {os.path.normpath(full_path)}")
    if dry_run:
        return
    os.makedirs(full_dir, exist_ok=True)
    with open(full_path, "w", encoding="utf-8", newline="\n") as f:
        f.write(content or "")


def process_file(xml_path: str, out_dir: str, ext: str, dry_run: bool, verbose: bool) -> int:
    """Stream-parse a MediaWiki XML file and write out each page. Returns count."""
    if verbose:
        print(f"Processing: {xml_path}")

    # We want siteinfo first. For simplicity, parse root shallowly to get ns map, then iterparse for pages.
    try:
        # Parse just enough to get siteinfo
        root = ET.parse(xml_path).getroot()
    except ET.ParseError as e:
        print(f"ERROR: Failed to parse '{xml_path}': {e}", file=sys.stderr)
        return 0

    ns_map = get_namespace_map(root)

    # Iterparse pages to keep memory in check
    count = 0
    context = ET.iterparse(xml_path, events=("end",))
    for event, elem in context:
        if elem.tag == f"{{{MW_NS['mw']}}}page":
            # Extract fields
            title_el = elem.find("mw:title", MW_NS)
            ns_el = elem.find("mw:ns", MW_NS)
            title = (title_el.text or "").strip() if title_el is not None else ""
            ns_id = (ns_el.text or "0").strip() if ns_el is not None else "0"

            # Find latest revision text
            text_content: Optional[str] = None
            # Prefer last <revision>/<text> if multiple
            revisions = elem.findall("mw:revision", MW_NS)
            if revisions:
                last_rev = revisions[-1]
                text_el = last_rev.find("mw:text", MW_NS)
                if text_el is not None and text_el.text is not None:
                    text_content = text_el.text
                else:
                    text_content = ""
            else:
                text_content = ""

            folder_segments, filename = derive_paths(title, ns_id, ns_map)
            write_page(out_dir, folder_segments, filename, ext, text_content or "", dry_run, verbose)
            count += 1

            # Clear the element to free memory
            elem.clear()
    if verbose:
        print(f"Done: {xml_path} ({count} pages)")
    return count


def main(argv: Optional[list[str]] = None) -> int:
    parser = argparse.ArgumentParser(description="Extract MediaWiki pages from XML to files.")
    parser.add_argument("xml", nargs="+", help="Input MediaWiki XML file(s)")
    parser.add_argument("--out", dest="out_dir", default="out", help="Output directory (default: out)")
    parser.add_argument("--ext", dest="ext", default="txt", help="Output file extension without dot (default: txt)")
    parser.add_argument("--dry-run", action="store_true", help="Do not write files, only log")
    parser.add_argument("--verbose", action="store_true", help="Verbose logging")

    args = parser.parse_args(argv)

    total = 0
    for path in args.xml:
        if not os.path.isfile(path):
            print(f"WARNING: Not a file, skipping: {path}", file=sys.stderr)
            continue
        total += process_file(path, args.out_dir, args.ext, args.dry_run, args.verbose)

    if args.verbose or args.dry_run:
        print(f"Total pages processed: {total}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
