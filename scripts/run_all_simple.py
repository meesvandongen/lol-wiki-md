#!/usr/bin/env python3
"""
Batch runner for simple_converter.py

Discovers all champion pages under the extracted wiki root (./out by default)
and runs the SimpleLoLConverter for each one, writing output markdown files
to ./markdown by default.

Usage (from repo root):
  python scripts/run_all_simple.py --wiki-root ./out --output ./markdown
"""

from __future__ import annotations

import argparse
from pathlib import Path
from typing import List, Tuple

# Ensure we can import from repo root
import sys
ROOT = Path(__file__).resolve().parents[1]
if str(ROOT) not in sys.path:
    sys.path.insert(0, str(ROOT))

from simple_converter import SimpleLoLConverter  # type: ignore


def find_champions(wiki_root: Path) -> List[str]:
    main_dir = wiki_root / "Main"
    champs: List[str] = []
    if not main_dir.exists():
        return champs
    for child in main_dir.iterdir():
        if child.is_dir():
            page = child / "page.txt"
            if page.exists():
                champs.append(child.name)
    champs.sort()
    return champs


def run_all(wiki_root: Path, output: Path) -> Tuple[int, int, List[str]]:
    conv = SimpleLoLConverter(wiki_root, output)
    champs = find_champions(wiki_root)
    ok = 0
    fail = 0
    failed: List[str] = []
    print(f"Discovered {len(champs)} champions under {wiki_root / 'Main'}")
    for name in champs:
        res = conv.convert_champion(name)
        if res is None:
            fail += 1
            failed.append(name)
        else:
            ok += 1
    return ok, fail, failed


def main() -> int:
    ap = argparse.ArgumentParser(description="Run SimpleLoLConverter for all champions")
    ap.add_argument("--wiki-root", default="./out", help="Path to extracted wiki root (default: ./out)")
    ap.add_argument("--output", default="./markdown", help="Output directory for markdown files (default: ./markdown)")
    args = ap.parse_args()

    wiki_root = Path(args.wiki_root).resolve()
    output = Path(args.output).resolve()
    output.mkdir(parents=True, exist_ok=True)

    print(f"Wiki root: {wiki_root}")
    print(f"Output dir: {output}")

    ok, fail, failed = run_all(wiki_root, output)
    print("\n=== Summary ===")
    print(f"Succeeded: {ok}")
    print(f"Failed:    {fail}")
    if failed:
        print("Failures:")
        for name in failed:
            print(f" - {name}")
    return 0 if fail == 0 else 1


if __name__ == "__main__":
    raise SystemExit(main())
