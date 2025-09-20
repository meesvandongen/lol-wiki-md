# League of Legends Wiki to Markdown Converter

## Project Overview

This project provides a comprehensive solution for converting League of Legends champion data from MediaWiki format to clean, GitHub Flavored Markdown while preserving all information including formulas, abilities, stats, and patch history.

## Features

✅ **Complete Data Extraction**

- Champion basic information and stats
- All abilities with detailed descriptions
- Patch history with version tracking
- Trivia and lore information
- Cross-references and links

✅ **Formula Conversion**

- MediaWiki formula syntax (`{{ap|60 to 140}}`) → Markdown math (`$60-140$`)
- Complex formulas (`{{pp|...}}`) → KaTeX/LaTeX expressions
- Scaling data preservation

✅ **Dynamic Content**

- Template redirects resolution
- Ability data aggregation from multiple files
- Cross-reference linking
- Image and media handling

✅ **Clean Markdown Output**

- GitHub Flavored Markdown compatible
- Structured sections with table of contents
- Proper tables for stats and attributes
- Consistent formatting
- Preserved nested bullets for Patch History and Trivia
- Safer post-processing for common wiki template artifacts
- Smarter unit suffixes (avoids appending on 'Global', 'Instant', etc.)

## Files Created

### Core Scripts

1. **`lol_wiki_converter.py`** - Full-featured converter with external dependencies
2. **`simple_converter.py`** - Simplified version using only built-in Python libraries
3. **`config.yaml`** - Configuration file for customizing output
4. **`requirements.txt`** - Python package dependencies

### Templates

- **`templates/champion.md.j2`** - Jinja2 template for markdown generation

## Installation and Usage

### Option 1: Full-Featured Version

```bash
# Install dependencies
pip install -r requirements.txt

# Convert specific champions
python lol_wiki_converter.py --champions "Azir,Yasuo,Jinx" --output ./markdown_output

# Convert from file list
python lol_wiki_converter.py --champions-file champions_list.txt --output ./docs

# Convert all champions
python lol_wiki_converter.py --all-champions --output ./all_champions_md
```

### Option 2: Simple Version (No Dependencies)

```bash
# Convert single champion
python simple_converter.py --champion Azir --output ./markdown

# Test with different champions
python simple_converter.py --champion "Yasuo" --wiki-root ./out --output ./test_output
```

- For titled pages in non-main namespaces, the `Namespace:` prefix is removed from the folder path.
- Title subpages split on `/` becoming nested folders.
- File content is saved as UTF-8 with LF newlines.

## Requirements

- Python 3.8+

## Usage (PowerShell)

Run from the workspace root (`lol-wiki-md-3`):

```pwsh
python .\extract_pages.py .\runes.xml .\items.xml .\champions.xml --out .\out --ext txt --verbose
```

### Convert champions (PowerShell)

```pwsh
# Typical champion
python .\simple_converter.py --champion Azir --wiki-root .\out --output .\markdown

# Champion with an apostrophe in the name
python .\simple_converter.py --champion \`"Bel'Veth\`" --wiki-root .\out --output .\markdown

# Batch convert a small set
$names = @('Azir','Yasuo','LeBlanc','Zed','Rakan','Riven','Kayle','Hwei','Aurora','Aphelios','Cassiopeia','Kai\'Sa')
foreach ($n in $names) { python .\simple_converter.py --champion $n --wiki-root .\out --output .\markdown }
```

Tips:

- Use `\`"Name With 'Apostrophe'\`"` for PowerShell to avoid parsing issues.
- Some older or less common champions may be missing in your local `out/` depending on input XML dumps.

- `--out` sets output directory (default `out`).
- `--ext` changes the output file extension (default `txt`).
- `--dry-run` prints what would be written without creating files.
- `--verbose` prints progress and counts.

Example dry run:

```pwsh
python .\extract_pages.py .\runes.xml --dry-run --verbose
```

## Notes

- Windows reserved names (e.g., `CON`, `PRN`, `AUX`, `NUL`, `COM1`) are avoided by appending an underscore.
- Invalid path characters (`<>:"/\\|?*` and control characters) are replaced with underscores.
- Trailing spaces/dots are removed from folders and file names.

## Output structure example

```
out/
  File/
    Dark_Harvest_rune.png/
      page.txt
  Main/
    Some_Page/
      Subpage/
        page.txt
```
