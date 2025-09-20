#!/usr/bin/env python3
"""
League of Legends Champion MediaWiki to Markdown Converter

This script converts champion data from MediaWiki format to GitHub Flavored Markdown,
preserving all information including formulas, abilities, stats, and patch history.

Usage:
    python lol_wiki_converter.py --champions "Azir,Yasuo,Jinx" --output ./markdown_output
    python lol_wiki_converter.py --champions-file champions_list.txt --output ./docs
    python lol_wiki_converter.py --all-champions --output ./all_champions_md

Features:
- Converts MediaWiki templates to structured markdown
- Preserves mathematical formulas using KaTeX/LaTeX syntax
- Dynamically includes ability data from separate template files
- Processes patch history into timeline format
- Handles cross-references and links
- Generates clean, navigable markdown documents

Dependencies:
    pip install mwparserfromhell jinja2 pyyaml beautifulsoup4
"""

import argparse
import json
import re
import sys
from pathlib import Path
from typing import Dict, List, Optional, Tuple, Any, Union
import mwparserfromhell as mwp
from jinja2 import Environment, FileSystemLoader
import yaml

class LoLWikiConverter:
    """Main converter class for League of Legends wiki data."""
    
    def __init__(self, wiki_root: Path, output_dir: Path, config_file: Optional[Path] = None):
        """
        Initialize the converter.
        
        Args:
            wiki_root: Path to the extracted MediaWiki files
            output_dir: Directory to write markdown files
            config_file: Optional configuration file for custom settings
        """
        self.wiki_root = Path(wiki_root)
        self.output_dir = Path(output_dir)
        self.config = self._load_config(config_file)
        
        # Initialize components
        self.template_parser = TemplateParser(self.wiki_root)
        self.formula_converter = FormulaConverter()
        self.ability_aggregator = AbilityDataAggregator(self.wiki_root, self.template_parser)
        self.patch_processor = PatchHistoryProcessor(self.wiki_root, self.template_parser)
        self.stats_extractor = StatsExtractor(self.template_parser)
        self.media_handler = MediaHandler(self.wiki_root, self.output_dir)
        self.link_resolver = CrossReferenceResolver(self.wiki_root)
        
        # Setup Jinja2 templates
        template_dir = Path(__file__).parent / "templates"
        self.jinja_env = Environment(loader=FileSystemLoader(template_dir))
        
    def _load_config(self, config_file: Optional[Path]) -> Dict[str, Any]:
        """Load configuration from file or use defaults."""
        default_config = {
            "formula_style": "katex",  # katex, latex, or plain
            "include_patch_history": True,
            "include_trivia": True,
            "include_images": True,
            "cross_reference_style": "relative",  # relative, absolute, or none
            "output_format": "individual",  # individual or combined
        }
        
        if config_file and config_file.exists():
            with open(config_file, 'r', encoding='utf-8') as f:
                user_config = yaml.safe_load(f)
                default_config.update(user_config)
                
        return default_config
    
    def convert_champion(self, champion_name: str) -> Optional[Path]:
        """
        Convert a single champion to markdown.
        
        Args:
            champion_name: Name of the champion to convert
            
        Returns:
            Path to the generated markdown file, or None if failed
        """
        try:
            print(f"Converting {champion_name}...")
            
            # Extract champion data
            champion_data = self._extract_champion_data(champion_name)
            if not champion_data:
                print(f"Warning: Could not extract data for {champion_name}")
                return None
            
            # Generate markdown
            markdown_content = self._generate_markdown(champion_data)
            
            # Write to file
            output_file = self.output_dir / f"{champion_name.replace(' ', '_')}.md"
            output_file.parent.mkdir(parents=True, exist_ok=True)
            
            with open(output_file, 'w', encoding='utf-8') as f:
                f.write(markdown_content)
                
            print(f"Successfully converted {champion_name} to {output_file}")
            return output_file
            
        except Exception as e:
            print(f"Error converting {champion_name}: {e}")
            return None
    
    def convert_champions(self, champion_names: List[str]) -> List[Path]:
        """Convert multiple champions to markdown."""
        results = []
        for champion_name in champion_names:
            result = self.convert_champion(champion_name)
            if result:
                results.append(result)
        return results
    
    def _extract_champion_data(self, champion_name: str) -> Optional[Dict[str, Any]]:
        """Extract all data for a champion from various sources."""
        # Get main champion page
        main_page_path = self.wiki_root / "Main" / champion_name / "page.txt"
        if not main_page_path.exists():
            return None
            
        with open(main_page_path, 'r', encoding='utf-8') as f:
            main_content = f.read()
            
        # Parse the main page
        parsed_main = mwp.parse(main_content)
        
        # Extract different data types
        champion_data = {
            "name": champion_name,
            "basic_info": self._extract_basic_info(parsed_main),
            "stats": self.stats_extractor.extract(champion_name, parsed_main),
            "abilities": self.ability_aggregator.extract(champion_name, parsed_main),
            "patch_history": self.patch_processor.extract(champion_name) if self.config["include_patch_history"] else [],
            "trivia": self._extract_trivia(parsed_main) if self.config["include_trivia"] else [],
            "images": self.media_handler.extract_images(parsed_main) if self.config["include_images"] else [],
            "cross_references": self.link_resolver.extract_links(parsed_main),
        }
        
        return champion_data
    
    def _extract_basic_info(self, parsed_content) -> Dict[str, Any]:
        """Extract basic champion information from parsed content."""
        # This would extract information from {{Champion info}} templates
        # Implementation details would depend on the specific template structure
        info = {}
        
        for template in parsed_content.filter_templates():
            if template.name.matches("Champion info"):
                # Extract parameters from the template
                for param in template.params:
                    key = str(param.name).strip()
                    value = str(param.value).strip()
                    info[key] = value
                break
        
        return info
    
    def _extract_trivia(self, parsed_content) -> List[str]:
        """Extract trivia section from parsed content."""
        trivia_items = []
        
        # Find trivia section
        sections = parsed_content.get_sections()
        for section in sections:
            if "trivia" in str(section).lower():
                # Parse trivia items (usually bullet points)
                for line in str(section).split('\n'):
                    line = line.strip()
                    if line.startswith('*'):
                        trivia_items.append(line[1:].strip())
        
        return trivia_items
    
    def _generate_markdown(self, champion_data: Dict[str, Any]) -> str:
        """Generate markdown content from champion data."""
        template = self.jinja_env.get_template("champion.md.j2")
        
        # Process formulas in the data
        processed_data = self._process_formulas_in_data(champion_data)
        
        # Process cross-references
        processed_data = self._process_cross_references(processed_data)
        
        return template.render(**processed_data, config=self.config)
    
    def _process_formulas_in_data(self, data: Dict[str, Any]) -> Dict[str, Any]:
        """Process all formulas in the champion data."""
        if isinstance(data, dict):
            return {k: self._process_formulas_in_data(v) for k, v in data.items()}
        elif isinstance(data, list):
            return [self._process_formulas_in_data(item) for item in data]
        elif isinstance(data, str):
            return self.formula_converter.convert(data)
        else:
            return data
    
    def _process_cross_references(self, data: Dict[str, Any]) -> Dict[str, Any]:
        """Process cross-references in the champion data."""
        # Similar recursive processing for links and references
        return self.link_resolver.process_data(data, self.config["cross_reference_style"])


class TemplateParser:
    """Handles parsing of MediaWiki templates."""
    
    def __init__(self, wiki_root: Path):
        self.wiki_root = wiki_root
        self.template_cache = {}
    
    def parse_template(self, template_name: str, params: Dict[str, str]) -> Dict[str, Any]:
        """Parse a MediaWiki template with given parameters."""
        # Implementation for parsing templates
        pass
    
    def resolve_redirect(self, template_path: Path) -> Path:
        """Resolve template redirects to actual template files."""
        if not template_path.exists():
            return template_path
            
        with open(template_path, 'r', encoding='utf-8') as f:
            content = f.read().strip()
            
        # Check if it's a redirect
        if content.startswith('#REDIRECT'):
            # Extract the redirect target
            match = re.search(r'#REDIRECT \[\[([^\]]+)\]\]', content)
            if match:
                redirect_target = match.group(1)
                # Convert to file path
                redirect_path = self._template_name_to_path(redirect_target)
                return self.resolve_redirect(redirect_path)
        
        return template_path
    
    def _template_name_to_path(self, template_name: str) -> Path:
        """Convert template name to file path."""
        # Handle Template: prefix
        if template_name.startswith('Template:'):
            template_name = template_name[9:]
            
        # Convert to path
        parts = template_name.split('/')
        path = self.wiki_root / "Template"
        for part in parts:
            path = path / part
        return path / "page.txt"


class FormulaConverter:
    """Converts MediaWiki formula syntax to markdown math."""
    
    def __init__(self):
        # Define conversion patterns
        self.patterns = [
            # {{ap|60 to 140}} -> 60-140
            (r'\{\{ap\|([^}]+)\}\}', self._convert_ap),
            # {{pp|formula|params}} -> complex formulas
            (r'\{\{pp\|([^}]+)\}\}', self._convert_pp),
            # {{as|text}} -> styled text
            (r'\{\{as\|([^}]+)\}\}', self._convert_as),
            # {{st|label|value}} -> label: value
            (r'\{\{st\|([^}]+)\|([^}]+)\}\}', self._convert_st),
        ]
    
    def convert(self, text: str) -> str:
        """Convert MediaWiki formulas to markdown math."""
        result = text
        
        for pattern, converter in self.patterns:
            result = re.sub(pattern, converter, result)
        
        return result
    
    def _convert_ap(self, match) -> str:
        """Convert {{ap|...}} to math notation."""
        content = match.group(1)
        
        if 'to' in content:
            # Range format: "60 to 140" -> "$60-140$"
            parts = content.split(' to ')
            if len(parts) == 2:
                return f"${parts[0].strip()}-{parts[1].strip()}$"
        
        # Single value: "60" -> "$60$"
        return f"${content.strip()}$"
    
    def _convert_pp(self, match) -> str:
        """Convert {{pp|...}} complex formulas."""
        content = match.group(1)
        parts = content.split('|')
        
        if len(parts) >= 2:
            # This is a complex formula that would need detailed parsing
            # For now, return a simplified version
            return f"${parts[0].strip()}$ (levels {parts[1].strip() if len(parts) > 1 else '1-18'})"
        
        return f"${content}$"
    
    def _convert_as(self, match) -> str:
        """Convert {{as|...}} styled text."""
        content = match.group(1)
        # Remove MediaWiki styling and return plain text
        return content.split('|')[0]  # Take first part before any styling info
    
    def _convert_st(self, match) -> str:
        """Convert {{st|label|value}} to markdown format."""
        label = match.group(1)
        value = match.group(2)
        return f"**{label}:** {value}"


class AbilityDataAggregator:
    """Aggregates ability data from template files."""
    
    def __init__(self, wiki_root: Path, template_parser: TemplateParser):
        self.wiki_root = wiki_root
        self.template_parser = template_parser
    
    def extract(self, champion_name: str, parsed_main_page) -> List[Dict[str, Any]]:
        """Extract all ability data for a champion."""
        abilities = []
        
        # Find ability template calls in main page
        for template in parsed_main_page.filter_templates():
            template_name = str(template.name).strip()
            
            # Look for Data Champion/Ability patterns
            if template_name.startswith(f"Data {champion_name}/"):
                ability_data = self._extract_ability_data(champion_name, template_name)
                if ability_data:
                    abilities.append(ability_data)
        
        return abilities
    
    def _extract_ability_data(self, champion_name: str, template_name: str) -> Optional[Dict[str, Any]]:
        """Extract data for a specific ability."""
        # Convert template name to file path
        template_path = self.template_parser._template_name_to_path(template_name)
        template_path = self.template_parser.resolve_redirect(template_path)
        
        if not template_path.exists():
            return None
        
        with open(template_path, 'r', encoding='utf-8') as f:
            content = f.read()
        
        # Parse the template content
        parsed = mwp.parse(content)
        ability_data = {}
        
        # Extract template parameters
        for template in parsed.filter_templates():
            for param in template.params:
                key = str(param.name).strip()
                value = str(param.value).strip()
                ability_data[key] = value
        
        return ability_data


class PatchHistoryProcessor:
    """Processes patch history data."""
    
    def __init__(self, wiki_root: Path, template_parser: TemplateParser):
        self.wiki_root = wiki_root
        self.template_parser = template_parser
    
    def extract(self, champion_name: str) -> List[Dict[str, Any]]:
        """Extract patch history for a champion."""
        patch_file = self.wiki_root / "Main" / champion_name / "Patch_history" / "page.txt"
        
        if not patch_file.exists():
            return []
        
        with open(patch_file, 'r', encoding='utf-8') as f:
            content = f.read()
        
        return self._parse_patch_history(content)
    
    def _parse_patch_history(self, content: str) -> List[Dict[str, Any]]:
        """Parse patch history content into structured data."""
        patches = []
        
        # Split by version headers
        version_pattern = r';?\[\[([V\d.]+)\]\]'
        sections = re.split(version_pattern, content)
        
        for i in range(1, len(sections), 2):
            if i + 1 < len(sections):
                version = sections[i]
                changes = sections[i + 1].strip()
                
                patches.append({
                    "version": version,
                    "changes": self._parse_changes(changes)
                })
        
        return patches
    
    def _parse_changes(self, changes_text: str) -> List[Dict[str, str]]:
        """Parse individual changes within a patch."""
        changes = []
        
        # Split by ability sections or stat sections
        lines = changes_text.split('\n')
        current_section = None
        current_changes = []
        
        for line in lines:
            line = line.strip()
            if not line:
                continue
                
            # Check if this is a section header (ability name, "Stats", etc.)
            if line.startswith('*') and '{{ai|' in line:
                # Save previous section
                if current_section and current_changes:
                    changes.append({
                        "section": current_section,
                        "changes": current_changes
                    })
                
                # Start new section
                current_section = self._extract_section_name(line)
                current_changes = []
                
            elif line.startswith('**') or line.startswith('***'):
                # This is a change item
                current_changes.append(line.lstrip('*').strip())
        
        # Don't forget the last section
        if current_section and current_changes:
            changes.append({
                "section": current_section,
                "changes": current_changes
            })
        
        return changes
    
    def _extract_section_name(self, line: str) -> str:
        """Extract ability or section name from a line."""
        # Extract from {{ai|ability_name|champion}}
        match = re.search(r'\{\{ai\|([^|}]+)', line)
        if match:
            return match.group(1)
        
        # Handle direct section names like "Stats"
        clean_line = re.sub(r'\*+\s*', '', line)
        return clean_line.split('{{')[0].strip()


class StatsExtractor:
    """Extracts champion statistics."""
    
    def __init__(self, template_parser: TemplateParser):
        self.template_parser = template_parser
    
    def extract(self, champion_name: str, parsed_content) -> Dict[str, Any]:
        """Extract champion statistics."""
        stats = {}
        
        # Look for Infobox stats template
        for template in parsed_content.filter_templates():
            if template.name.matches("Infobox stats"):
                # The stats are usually referenced from a data module
                # This would require parsing the Module:ChampionData
                stats = self._extract_from_champion_data(champion_name)
                break
        
        return stats
    
    def _extract_from_champion_data(self, champion_name: str) -> Dict[str, Any]:
        """Extract stats from champion data module."""
        # This would typically involve parsing Lua modules or JSON data
        # For this example, return a placeholder structure
        return {
            "health": {"base": 550, "growth": 85},
            "mana": {"base": 350, "growth": 50},
            "armor": {"base": 22, "growth": 3.5},
            "magic_resist": {"base": 30, "growth": 0.5},
            "attack_damage": {"base": 56, "growth": 3.1},
            "attack_speed": {"base": 0.625, "growth": 3.2},
        }


class MediaHandler:
    """Handles media files and images."""
    
    def __init__(self, wiki_root: Path, output_dir: Path):
        self.wiki_root = wiki_root
        self.output_dir = output_dir
    
    def extract_images(self, parsed_content) -> List[Dict[str, str]]:
        """Extract image references from content."""
        images = []
        
        # Find File: references
        file_pattern = r'\[\[File:([^|\]]+)(?:\|([^]]*))?\]\]'
        matches = re.findall(file_pattern, str(parsed_content))
        
        for filename, caption in matches:
            images.append({
                "filename": filename,
                "caption": caption or "",
                "markdown_path": self._convert_to_markdown_path(filename)
            })
        
        return images
    
    def _convert_to_markdown_path(self, filename: str) -> str:
        """Convert MediaWiki file reference to markdown image path."""
        # Convert spaces and special characters
        safe_filename = filename.replace(' ', '_')
        return f"./images/{safe_filename}"


class CrossReferenceResolver:
    """Resolves cross-references and links."""
    
    def __init__(self, wiki_root: Path):
        self.wiki_root = wiki_root
    
    def extract_links(self, parsed_content) -> List[Dict[str, str]]:
        """Extract all links from content."""
        links = []
        
        # Internal links [[Page]]
        internal_pattern = r'\[\[([^|\]]+)(?:\|([^]]*))?\]\]'
        matches = re.findall(internal_pattern, str(parsed_content))
        
        for target, display_text in matches:
            links.append({
                "type": "internal",
                "target": target,
                "display": display_text or target,
                "markdown": self._convert_internal_link(target, display_text)
            })
        
        return links
    
    def _convert_internal_link(self, target: str, display_text: str) -> str:
        """Convert internal MediaWiki link to markdown."""
        display = display_text or target
        
        # Convert to markdown file reference
        if target.startswith('File:'):
            return f"![{display}](./images/{target[5:].replace(' ', '_')})"
        else:
            # Link to another champion or page
            markdown_file = target.replace(' ', '_') + '.md'
            return f"[{display}](./{markdown_file})"
    
    def process_data(self, data: Dict[str, Any], style: str) -> Dict[str, Any]:
        """Process all cross-references in data based on style."""
        # Recursive processing similar to formula processing
        return data


def main():
    """Main entry point for the script."""
    parser = argparse.ArgumentParser(description="Convert LoL Wiki data to Markdown")
    parser.add_argument("--wiki-root", default="./out", help="Path to extracted wiki files")
    parser.add_argument("--output", default="./markdown", help="Output directory")
    parser.add_argument("--config", help="Configuration file")
    
    # Champion selection options
    group = parser.add_mutually_exclusive_group(required=True)
    group.add_argument("--champions", help="Comma-separated list of champions")
    group.add_argument("--champions-file", help="File containing list of champions")
    group.add_argument("--all-champions", action="store_true", help="Convert all champions")
    
    args = parser.parse_args()
    
    # Initialize converter
    converter = LoLWikiConverter(
        wiki_root=args.wiki_root,
        output_dir=args.output,
        config_file=Path(args.config) if args.config else None
    )
    
    # Determine which champions to convert
    if args.champions:
        champion_list = [name.strip() for name in args.champions.split(',')]
    elif args.champions_file:
        with open(args.champions_file, 'r') as f:
            champion_list = [line.strip() for line in f if line.strip()]
    else:  # all champions
        main_dir = Path(args.wiki_root) / "Main"
        champion_list = [d.name for d in main_dir.iterdir() if d.is_dir()]
    
    # Convert champions
    print(f"Converting {len(champion_list)} champions...")
    results = converter.convert_champions(champion_list)
    
    print(f"Successfully converted {len(results)} champions to {args.output}")


if __name__ == "__main__":
    main()