#!/usr/bin/env python3
"""
Regenerate markdown for all champions using simple_converter.SimpleLoLConverter.
Usage:
  python scripts/rebuild_champions.py [--wiki-root ./out] [--output ./markdown]
"""
import argparse
from pathlib import Path
from typing import List

from simple_converter import SimpleLoLConverter


def list_main_subdirs(wiki_root: Path) -> List[str]:
    main = wiki_root / "Main"
    if not main.exists():
        return []
    out: List[str] = []
    for child in sorted(main.iterdir()):
        if child.is_dir():
            # Skip special or hidden dirs
            name = child.name.strip()
            if not name or name.startswith('.'):
                continue
            out.append(name)
    return out


def main():
    ap = argparse.ArgumentParser(description="Rebuild all champion markdown files")
    ap.add_argument("--wiki-root", default="./out", help="Path to extracted wiki files (default: ./out)")
    ap.add_argument("--output", default="./markdown", help="Output directory for markdown (default: ./markdown)")
    args = ap.parse_args()

    wiki_root = Path(args.wiki_root).resolve()
    output_dir = Path(args.output).resolve()
    output_dir.mkdir(parents=True, exist_ok=True)

    conv = SimpleLoLConverter(wiki_root, output_dir)

    names = list_main_subdirs(wiki_root)
    if not names:
        print(f"No subdirectories found under {wiki_root / 'Main'}. Did you run extraction?")
        return 1

    total = 0
    converted = 0
    skipped = 0
    failed = 0

    for name in names:
        total += 1
        try:
            # convert_champion does its own champion validation and will skip if not a champion
            res = conv.convert_champion(name)
            if res is None:
                skipped += 1
                print(f"[skip] {name}")
            else:
                converted += 1
                print(f"[ok]   {name} -> {res}")
        except Exception as e:
            failed += 1
            print(f"[fail] {name}: {e}")

    print("")
    print(f"Done. total={total}, converted={converted}, skipped={skipped}, failed={failed}")
    return 0 if failed == 0 else 2


if __name__ == "__main__":
    raise SystemExit(main())
