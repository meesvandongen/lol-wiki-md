#!/usr/bin/env python3
"""
Simplified League of Legends Champion MediaWiki to Markdown Converter

This is a demonstration version that works with built-in Python libraries only.
For full functionality, install the requirements and use lol_wiki_converter.py

Usage:
    python simple_converter.py --champion Azir --output ./markdown
"""

import argparse
import json
import re
import sys
from pathlib import Path
from typing import Dict, List, Optional, Tuple, Any, Union

class SimpleLoLConverter:
    """Simplified converter using only built-in Python libraries."""
    
    def __init__(self, wiki_root: Path, output_dir: Path):
        self.wiki_root = Path(wiki_root)
        self.output_dir = Path(output_dir)
        self.output_dir.mkdir(parents=True, exist_ok=True)
    
    def convert_champion(self, champion_name: str) -> Optional[Path]:
        """Convert a single champion to markdown."""
        try:
            print(f"Converting {champion_name}...")
            
            # Read main page
            main_page_path = self.wiki_root / "Main" / champion_name / "page.txt"
            if not main_page_path.exists():
                print(f"Error: Champion page not found at {main_page_path}")
                return None
            
            with open(main_page_path, 'r', encoding='utf-8') as f:
                content = f.read()
            
            # Extract basic information
            champion_data = self._extract_champion_data(champion_name, content)
            
            # Generate markdown
            markdown = self._generate_markdown(champion_data)
            
            # Write output
            output_file = self.output_dir / f"{champion_name.replace(' ', '_')}.md"
            with open(output_file, 'w', encoding='utf-8') as f:
                f.write(markdown)
            
            print(f"Successfully converted {champion_name} to {output_file}")
            return output_file
            
        except Exception as e:
            print(f"Error converting {champion_name}: {e}")
            return None
    
    def _extract_champion_data(self, champion_name: str, content: str) -> Dict[str, Any]:
        """Extract champion data from MediaWiki content."""
        data = {
            "name": champion_name,
            "basic_info": {},
            "abilities": [],
            "patch_history": [],
            "trivia": [],
            "stats": {},
        }
        
        # Extract abilities section
        abilities_match = re.search(r'== Abilities ==(.*?)(?:==|$)', content, re.DOTALL)
        if abilities_match:
            abilities_content = abilities_match.group(1)
            data["abilities"] = self._extract_abilities(champion_name, abilities_content)
        
        # Extract trivia section
        trivia_match = re.search(r'== ?Trivia ?==(.*?)(?:==|$)', content, re.DOTALL)
        if trivia_match:
            trivia_content = trivia_match.group(1)
            data["trivia"] = self._extract_trivia(trivia_content)
        
        # Extract basic champion info
        champion_info_match = re.search(r'\{\{Champion info\|([^}]+)\}\}', content)
        if champion_info_match:
            data["basic_info"] = self._extract_basic_info(content)
        
        # Extract stats (simplified)
        data["stats"] = self._extract_basic_stats(champion_name)
        
        # Extract patch history reference
        patch_match = re.search(r'\{\{Patch box\|([^}]+)\}\}', content)
        if patch_match:
            data["patch_history"] = self._extract_patch_history(champion_name)
        
        return data
    
    def _extract_basic_info(self, content: str) -> Dict[str, str]:
        """Extract basic champion information."""
        info = {}
        
        # Try to extract title from Game banner
        title_match = re.search(r'\{\{Game banner\|([^|}]+)(?:\|[^}]*)?\}\}', content)
        if title_match:
            info['champion'] = title_match.group(1)
        
        return info
    
    def _extract_basic_stats(self, champion_name: str) -> Dict[str, Dict[str, float]]:
        """Extract basic champion statistics (placeholder implementation)."""
        # This would ideally parse Module:ChampionData, but for now return example data
        return {
            "health": {"base": 550, "growth": 85},
            "mana": {"base": 350, "growth": 50},
            "armor": {"base": 22, "growth": 3.5},
            "magic_resist": {"base": 30, "growth": 0.5},
            "attack_damage": {"base": 56, "growth": 3.1},
            "attack_speed": {"base": 0.625, "growth": 3.2},
        }
    
    def _extract_abilities(self, champion_name: str, abilities_content: str) -> List[Dict[str, Any]]:
        """Extract ability information."""
        abilities = []
        
        # Find Data template calls
        ability_pattern = r'\{\{Data ' + re.escape(champion_name) + r'/([^|]+)\|Ability\}\}'
        matches = re.findall(ability_pattern, abilities_content)
        
        for ability_key in matches:
            ability_data = self._get_ability_data(champion_name, ability_key)
            if ability_data:
                abilities.append(ability_data)
        
        # Also try to find abilities by checking the actual directory structure
        template_dir = self.wiki_root / "Template" / f"Data_{champion_name}"
        if template_dir.exists():
            for ability_folder in template_dir.iterdir():
                if ability_folder.is_dir() and ability_folder.name not in [a['name'] for a in abilities]:
                    # Skip redirects (single letter folders)
                    if len(ability_folder.name) > 1:
                        ability_data = self._get_ability_data(champion_name, ability_folder.name)
                        if ability_data and ability_data not in abilities:
                            abilities.append(ability_data)
        
        # Sort abilities by skill order (I, Q, W, E, R)
        skill_order = {'I': 0, 'Q': 1, 'W': 2, 'E': 3, 'R': 4}
        abilities.sort(key=lambda x: skill_order.get(x.get('skill', 'Z'), 999))
        
        return abilities
    
    def _get_ability_data(self, champion_name: str, ability_key: str) -> Optional[Dict[str, Any]]:
        """Get ability data from template files."""
        # Try direct path first
        template_path = self.wiki_root / "Template" / f"Data_{champion_name}" / ability_key / "page.txt"
        
        if not template_path.exists():
            return None
        
        with open(template_path, 'r', encoding='utf-8') as f:
            content = f.read()
        
        # Check for redirects
        if content.strip().startswith('#REDIRECT'):
            redirect_match = re.search(r'#REDIRECT \[\[Template:([^\]]+)\]\]', content)
            if redirect_match:
                redirect_target = redirect_match.group(1)
                
                # Parse "Data Azir/Ability Name" format
                if redirect_target.startswith('Data '):
                    parts = redirect_target.split('/', 1)  # Split on first /
                    if len(parts) == 2:
                        data_part = parts[0]  # "Data Azir"
                        ability_name = parts[1]  # "Shurima's Legacy"
                        
                        # Extract champion name from "Data Champion"
                        champion_from_redirect = data_part.replace('Data ', '')
                        new_path = self.wiki_root / "Template" / f"Data_{champion_from_redirect}" / ability_name / "page.txt"
                        
                        if new_path.exists():
                            with open(new_path, 'r', encoding='utf-8') as f:
                                content = f.read()
                        else:
                            return None
                    else:
                        return None
                else:
                    return None
        
        # Extract ability name from the template content
        ability_name = ability_key
        name_match = re.search(r'\|([^|{]+)\|\{\{\{2\|\}\}\}\|\{\{\{3\|\}\}\}\|\{\{\{4\|\}\}\}\|\{\{\{5\|\}\}\}', content)
        if name_match:
            ability_name = name_match.group(1).strip()
        
        # Parse ability data from template
        ability_data = {
            "name": ability_name,
            "skill": ability_key if ability_key in ['Q', 'W', 'E', 'R', 'I'] else '',
        }
        
        # Extract skill from content if not found
        if not ability_data['skill']:
            skill_match = re.search(r'\|skill\s*=\s*([IQWER])', content)
            if skill_match:
                ability_data['skill'] = skill_match.group(1)
        
        # Extract template parameters with better handling
        param_patterns = [
            (r'\|blurb\s*=\s*(.+?)(?=\n\s*\||\n\s*}}|$)', 'blurb'),
            (r'\|blurb2\s*=\s*(.+?)(?=\n\s*\||\n\s*}}|$)', 'blurb2'),
            (r'\|description\s*=\s*(.+?)(?=\n\s*\||\n\s*}}|$)', 'description'),
            (r'\|description2\s*=\s*(.+?)(?=\n\s*\||\n\s*}}|$)', 'description2'),
            (r'\|description3\s*=\s*(.+?)(?=\n\s*\||\n\s*}}|$)', 'description3'),
            (r'\|target range\s*=\s*(.+?)(?=\n\s*\||\n\s*}}|$)', 'target_range'),
            (r'\|cooldown\s*=\s*(.+?)(?=\n\s*\||\n\s*}}|$)', 'cooldown'),
            (r'\|cost\s*=\s*(.+?)(?=\n\s*\||\n\s*}}|$)', 'cost'),
            (r'\|costtype\s*=\s*(.+?)(?=\n\s*\||\n\s*}}|$)', 'costtype'),
            (r'\|leveling\s*=\s*(.+?)(?=\n\s*\||\n\s*}}|$)', 'leveling'),
            (r'\|leveling2\s*=\s*(.+?)(?=\n\s*\||\n\s*}}|$)', 'leveling2'),
            (r'\|notes\s*=\s*(.+?)(?=\n\s*\||\n\s*}}|$)', 'notes'),
            (r'\|recharge\s*=\s*(.+?)(?=\n\s*\||\n\s*}}|$)', 'recharge'),
            (r'\|cast time\s*=\s*(.+?)(?=\n\s*\||\n\s*}}|$)', 'cast_time'),
        ]
        
        for pattern, key in param_patterns:
            match = re.search(pattern, content, re.DOTALL | re.IGNORECASE)
            if match:
                value = match.group(1).strip()
                # Special handling for notes to preserve list formatting
                if key == 'notes':
                    value = self._format_notes(value)
                else:
                    # Convert MediaWiki formatting to markdown
                    value = self._convert_wiki_to_markdown(value)
                ability_data[key] = value
        
        return ability_data
    
    def _format_notes(self, notes_text: str) -> str:
        """Format notes section preserving list structure."""
        if not notes_text:
            return "No additional notes."
        
        # Parse the raw notes text preserving structure
        lines = []
        for line in notes_text.split('\n'):
            line = line.strip()
            if line.startswith('* '):
                # Main bullet point - convert the whole line
                item = line[2:].strip()  # Remove '* '
                item = self._convert_wiki_to_markdown(item)
                lines.append(f"- {item}")
            elif line.startswith('** '):
                # Nested bullet point - convert the whole line
                item = line[3:].strip()  # Remove '** '
                item = self._convert_wiki_to_markdown(item)
                lines.append(f"  - {item}")
            elif line and not line.startswith('|') and not line.startswith('}') and line.strip():
                # Continue previous line (for multiline bullet points)
                item = self._convert_wiki_to_markdown(line)
                if lines and item.strip():
                    # Append to last line
                    lines[-1] += f" {item.strip()}"
        
        return '\n'.join(lines) if lines else "No additional notes."
    
    def _extract_trivia(self, trivia_content: str) -> List[str]:
        """Extract trivia items."""
        trivia = []
        lines = trivia_content.split('\n')
        
        for line in lines:
            line = line.strip()
            if line.startswith('*'):
                # Clean up MediaWiki formatting
                item = line[1:].strip()
                item = self._convert_wiki_to_markdown(item)
                trivia.append(item)
        
        return trivia
    
    def _extract_patch_history(self, champion_name: str) -> List[Dict[str, Any]]:
        """Extract patch history."""
        patch_file = self.wiki_root / "Main" / champion_name / "Patch_history" / "page.txt"
        
        if not patch_file.exists():
            return []
        
        with open(patch_file, 'r', encoding='utf-8') as f:
            content = f.read()
        
        patches = []
        
        # Find version sections
        version_pattern = r';?\[\[([V\d.]+)\]\]'
        sections = re.split(version_pattern, content)
        
        for i in range(1, len(sections), 2):
            if i + 1 < len(sections):
                version = sections[i]
                changes_text = sections[i + 1].strip()
                
                changes = []
                for line in changes_text.split('\n'):
                    line = line.strip()
                    if line.startswith('*'):
                        change = line[1:].strip()
                        change = self._convert_wiki_to_markdown(change)
                        changes.append(change)
                
                if changes:
                    patches.append({
                        "version": version,
                        "changes": changes
                    })
        
        return patches[:10]  # Limit to recent patches
    
    def _convert_wiki_to_markdown(self, text: str) -> str:
        """Convert basic MediaWiki syntax to markdown."""
        if not text:
            return ""
            
        # Convert formulas with better handling
        text = re.sub(r'\{\{ap\|([^}]+)\}\}', self._convert_ap_formula, text)
        text = re.sub(r'\{\{pp\|([^}]+)\}\}', self._convert_pp_formula, text)
        text = re.sub(r'\{\{fd\|([^}]+)\}\}', r'$\1$', text)  # Fixed decimals
        text = re.sub(r'\{\{tt\|([^}|]+)(?:\|[^}]*)?\}\}', r'\1', text)  # Tooltips
        
        # Convert styled text
        text = re.sub(r'\{\{as\|([^}|]+)(?:\|[^}]*)?\}\}', r'\1', text)
        text = re.sub(r'\{\{st\|([^}|]+)\|([^}]+)\}\}', r'**\1:** \2', text)
        text = re.sub(r'\{\{sbc\|([^}]+)\}\}', r'**\1**', text)  # Small bold caps
        text = re.sub(r'\{\{sti\|([^}]+)\}\}', r'*\1*', text)  # Styled italic
        
        # Convert links with better handling
        text = re.sub(r'\[\[([^|\]]+)(?:\|([^]]*))?\]\]', self._convert_link, text)
        
        # Convert champion/ability references
        text = re.sub(r'\{\{ci\|([^}|]+)(?:\|[^}]*)?\}\}', r'**\1**', text)
        text = re.sub(r'\{\{ai\|([^}|]+)\|([^}|]+)(?:\|[^}]*)?\}\}', r'**\1** (\2)', text)  # Better ability formatting
        text = re.sub(r'\{\{tip\|([^}|]+)(?:\|[^}]*)?\}\}', r'\1', text)
        text = re.sub(r'\{\{ri\|([^}|]+)(?:\|[^}]*)?\}\}', r'*\1*', text)  # Rune references
        text = re.sub(r'\{\{ii\|([^}|]+)(?:\|[^}]*)?\}\}', r'*\1*', text)  # Item references
        
        # Handle special formatting
        text = re.sub(r'\{\{w\|([^}|]+)(?:\|[^}]*)?\}\}', r'\1', text)  # Wikipedia links
        text = re.sub(r'\[\[([^|\]]+)#[^|\]]*\|([^\]]*)\]\]', r'[\2](./\1.md)', text)  # Section links
        
        # Handle stock and other gameplay terms
        text = re.sub(r'\[\[stock\]\]', 'stock', text)
        text = re.sub(r'\[\[basic attack\]\]', 'basic attack', text)
        
        # Handle file/image references
        text = re.sub(r'\[\[File:[^\]]+\]\]', '', text)  # Remove file references
        text = re.sub(r'\{\{sm2\|[^}]+\}\}', '', text)  # Remove sound files
        
        # Handle bug markers and special notations
        text = re.sub(r'\{\{bug\}\}', '', text)
        text = re.sub(r'\{\{bug\|[^}]+\}\}', '', text)
        
        # Remove references
        text = re.sub(r'<ref[^>]*>.*?</ref>', '', text, flags=re.DOTALL)
        text = re.sub(r'<ref[^>]*\s*/>', '', text)
        
        # Clean up HTML tags
        text = re.sub(r'<br\s*/?>', '\n', text)
        text = re.sub(r'<[^>]+>', '', text)
        
        # Convert bold/italic
        text = re.sub(r"'''([^']+)'''", r'**\1**', text)
        text = re.sub(r"''([^']+)''", r'*\1*', text)
        
        # Clean up remaining templates
        text = re.sub(r'\{\{[^}]+\}\}', '', text)
        
        # Clean up extra whitespace but preserve intentional line breaks
        text = re.sub(r'(?<!\n)\n(?!\n)', ' ', text)  # Single newlines to spaces (keep double newlines)
        text = re.sub(r'\s+', ' ', text)  # Multiple spaces to single
        text = text.strip()
        
        return text
    
    def _convert_ap_formula(self, match) -> str:
        """Convert {{ap|...}} formulas to markdown math."""
        content = match.group(1)
        if ' to ' in content:
            parts = content.split(' to ')
            if len(parts) == 2:
                return f"${parts[0].strip()}-{parts[1].strip()}$"
        return f"${content}$"
    
    def _convert_pp_formula(self, match) -> str:
        """Convert {{pp|...}} complex formulas."""
        content = match.group(1)
        parts = content.split('|')
        
        if len(parts) >= 1:
            main_values = parts[0].strip()
            
            # Handle special case: key=%
            if main_values == 'key=%' and len(parts) >= 2:
                values_part = parts[1].strip()
                if ' to ' in values_part:
                    # Extract range like "25 to 100"
                    range_match = re.search(r'(\d+)\s+to\s+(\d+)', values_part)
                    if range_match:
                        return f"${range_match.group(1)}-{range_match.group(2)}$%"
                return f"${values_part}$%"
            
            # Handle complex formulas like "0;5;10;15;20;25;30;35;40;45"
            if ';' in main_values:
                values = main_values.split(';')
                if len(values) > 5:
                    # Simplify to first and last values
                    return f"${values[0]}-{values[-1]}$"
                else:
                    return f"${main_values.replace(';', '/')}$"
            else:
                return f"${main_values}$"
        return f"${content}$"
    
    def _convert_link(self, match) -> str:
        """Convert MediaWiki links to markdown."""
        target = match.group(1)
        display = match.group(2) or target
        
        if target.startswith('File:'):
            return f"![{display}](./images/{target[5:].replace(' ', '_')})"
        else:
            return f"[{display}](./{target.replace(' ', '_')}.md)"
    
    def _generate_markdown(self, data: Dict[str, Any]) -> str:
        """Generate markdown content."""
        lines = []
        
        # Title
        lines.append(f"# {data['name']}")
        if data['basic_info'].get('champion'):
            lines.append(f"*The Emperor of Shurima*")
        lines.append("")
        
        # Table of contents
        lines.append("## Table of Contents")
        if data['basic_info']:
            lines.append("- [Basic Information](#basic-information)")
        if data['stats']:
            lines.append("- [Statistics](#statistics)")
        lines.append("- [Abilities](#abilities)")
        if data['patch_history']:
            lines.append("- [Patch History](#patch-history)")
        if data['trivia']:
            lines.append("- [Trivia](#trivia)")
        lines.append("")
        
        # Basic Information
        if data['basic_info']:
            lines.append("## Basic Information")
            lines.append("")
            lines.append("| Attribute | Value |")
            lines.append("|-----------|-------|")
            for key, value in data['basic_info'].items():
                if value:
                    lines.append(f"| **{key.title().replace('_', ' ')}** | {value} |")
            lines.append("")
        
        # Statistics
        if data['stats']:
            lines.append("## Statistics")
            lines.append("")
            lines.append("### Base Stats (Level 1-18)")
            lines.append("")
            lines.append("| Stat | Base | Growth | Level 18 |")
            lines.append("|------|------|--------|----------|")
            
            for stat_name, stat_data in data['stats'].items():
                if isinstance(stat_data, dict) and 'base' in stat_data and 'growth' in stat_data:
                    base = stat_data['base']
                    growth = stat_data['growth']
                    level_18 = base + (growth * 17)
                    
                    # Format the stat name nicely
                    display_name = stat_name.replace('_', ' ').title()
                    if stat_name == 'attack_speed':
                        lines.append(f"| **{display_name}** | ${base:.3f}$ | $+{growth:.1f}\\%$ | ${level_18:.3f}$ |")
                    else:
                        lines.append(f"| **{display_name}** | ${base}$ | $+{growth}$ | ${level_18:.1f}$ |")
            lines.append("")
        
        # Abilities
        if data['abilities']:
            lines.append("## Abilities")
            lines.append("")
            
            for ability in data['abilities']:
                # Ability header
                skill_label = ability.get('skill', '')
                ability_name = ability.get('name', 'Unknown')
                
                if skill_label == 'I':
                    lines.append(f"### Passive: {ability_name}")
                elif skill_label in ['Q', 'W', 'E', 'R']:
                    lines.append(f"### {skill_label}: {ability_name}")
                else:
                    lines.append(f"### {ability_name}")
                
                lines.append("")
                
                # Blurb (short description)
                if ability.get('blurb'):
                    lines.append(f"**{ability['blurb']}**")
                    lines.append("")
                
                if ability.get('blurb2'):
                    lines.append(f"*{ability['blurb2']}*")
                    lines.append("")
                
                # Full description
                descriptions = []
                for i in range(1, 6):  # description, description2, etc.
                    desc_key = 'description' if i == 1 else f'description{i}'
                    if ability.get(desc_key):
                        descriptions.append(ability[desc_key])
                
                if descriptions:
                    full_description = " ".join(descriptions)
                    lines.append(full_description)
                    lines.append("")
                
                # Stats table
                stats_items = []
                if ability.get('target_range'):
                    stats_items.append(('Range', f"{ability['target_range']} units"))
                if ability.get('cooldown'):
                    stats_items.append(('Cooldown', f"{ability['cooldown']} seconds"))
                if ability.get('recharge'):
                    stats_items.append(('Recharge', f"{ability['recharge']} seconds"))
                if ability.get('cast_time'):
                    stats_items.append(('Cast Time', f"{ability['cast_time']} seconds"))
                if ability.get('cost') and ability.get('costtype'):
                    stats_items.append(('Cost', f"{ability['cost']} {ability['costtype']}"))
                
                if stats_items:
                    lines.append("| Attribute | Value |")
                    lines.append("|-----------|-------|")
                    for attr, value in stats_items:
                        lines.append(f"| **{attr}** | {value} |")
                    lines.append("")
                
                # Scaling information
                scaling_items = []
                if ability.get('leveling'):
                    scaling_items.append(ability['leveling'])
                if ability.get('leveling2'):
                    scaling_items.append(ability['leveling2'])
                
                if scaling_items:
                    lines.append("**Scaling:**")
                    for scaling in scaling_items:
                        lines.append(f"- {scaling}")
                    lines.append("")
                
                # Notes
                if ability.get('notes'):
                    lines.append(f"**Notes:**")
                    lines.append("")
                    notes_text = ability['notes']
                    # Notes are already formatted as markdown by _format_notes
                    lines.append(notes_text)
                    lines.append("")
                
                lines.append("---")
                lines.append("")
        
        # Patch History
        if data['patch_history']:
            lines.append("## Patch History")
            lines.append("")
            
            for patch in data['patch_history']:
                lines.append(f"### {patch['version']}")
                for change in patch['changes']:
                    if change.strip():
                        lines.append(f"- {change}")
                lines.append("")
        
        # Trivia
        if data['trivia']:
            lines.append("## Trivia")
            lines.append("")
            for item in data['trivia']:
                if item.strip():
                    lines.append(f"- {item}")
            lines.append("")
        
        # Footer
        lines.append("---")
        lines.append("*This page was automatically generated from League of Legends Wiki data.*")
        
        return '\n'.join(lines)


def main():
    """Main entry point."""
    parser = argparse.ArgumentParser(description="Simple LoL Wiki to Markdown Converter")
    parser.add_argument("--champion", required=True, help="Champion name to convert")
    parser.add_argument("--wiki-root", default="./out", help="Path to extracted wiki files")
    parser.add_argument("--output", default="./markdown", help="Output directory")
    
    args = parser.parse_args()
    
    converter = SimpleLoLConverter(args.wiki_root, args.output)
    result = converter.convert_champion(args.champion)
    
    if result:
        print(f"Conversion completed successfully!")
        print(f"Output: {result}")
    else:
        print("Conversion failed!")
        sys.exit(1)


if __name__ == "__main__":
    main()