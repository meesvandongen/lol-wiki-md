#!/usr/bin/env python3
"""
Scan markdown output for pipe-param artifacts that shouldn't appear in prose:
- raw:    |[a-z]
- escaped: \|[a-z]

The checker tries to ignore legitimate markdown tables (header + separator + rows).
Exit code: 0 if clean, 1 if any suspicious lines are found.
"""
import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
MD_DIR = ROOT / "markdown"

TABLE_HEADER_RE = re.compile(r"^\|(?:[^|]+\|)+$")
TABLE_SEP_RE = re.compile(r"^\|\s*-+\s*(\|\s*-+\s*)+\|?\s*$")
PARAM_LINE_RE = re.compile(r"^\s*(?:\\\|\s*|\|\s*)[a-z][^=]{0,40}=", re.IGNORECASE)
RAW_PIPE_WORD_RE = re.compile(r"\|(?!\s*---)([a-z])")
ESCAPED_PIPE_WORD_RE = re.compile(r"\\\|([a-z])")


def is_table_block(lines, idx):
    if idx + 1 < len(lines) and TABLE_HEADER_RE.match(lines[idx]) and TABLE_SEP_RE.match(lines[idx+1]):
        return True
    return False


def scan_file(path: Path) -> list[str]:
    issues = []
    text = path.read_text(encoding='utf-8', errors='ignore')
    lines = text.splitlines()
    i = 0
    while i < len(lines):
        ln = lines[i]
        if ln.strip().startswith('|') and is_table_block(lines, i):
            # skip table header + sep and subsequent data rows
            i += 2
            while i < len(lines) and lines[i].strip().startswith('|'):
                i += 1
            continue
        # flag param-like lines
        if PARAM_LINE_RE.match(ln):
            issues.append(f"{path.name}:{i+1}: suspicious param line: {ln.strip()[:120]}")
        else:
            # flag raw/escaped pipe-word if not obviously in table
            if RAW_PIPE_WORD_RE.search(ln) or ESCAPED_PIPE_WORD_RE.search(ln):
                # Heuristic: ignore if line looks like a markdown table row with multiple pipes and no '='
                if not (ln.strip().startswith('|') and ln.count('|') >= 2 and '=' not in ln):
                    issues.append(f"{path.name}:{i+1}: pipe artifact: {ln.strip()[:120]}")
        i += 1
    return issues


def main():
    if not MD_DIR.exists():
        print(f"No markdown directory found at {MD_DIR}")
        sys.exit(0)
    problems: list[str] = []
    for path in MD_DIR.glob('*.md'):
        problems.extend(scan_file(path))
    if problems:
        print("Found potential artifacts:")
        for p in problems:
            print("- ", p)
        sys.exit(1)
    print("No pipe artifacts found outside tables.")


if __name__ == '__main__':
    main()
