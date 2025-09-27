#!/usr/bin/env python3
"""
Simplified League of Legends Champion MediaWiki to Markdown Converter

This is a demonstration version that works with built-in Python libraries only.
For full functionality, install the requirements and use lol_wiki_converter.py

Usage:
    python simple_converter.py --champion Azir --output ./markdown
    python simple_converter.py --item "Infinity Edge" --output ./markdown
    python simple_converter.py --rune Electrocute --output ./markdown
"""

import argparse
import json
import re
import sys
from pathlib import Path
from typing import Dict, List, Optional, Tuple, Any, Union
import ast
import math

class SimpleLoLConverter:
    """Simplified converter using only built-in Python libraries."""
    
    def __init__(self, wiki_root: Path, output_dir: Path):
        self.wiki_root = Path(wiki_root)
        self.output_dir = Path(output_dir)
        self.output_dir.mkdir(parents=True, exist_ok=True)
        self._current_page_basename: Optional[str] = None
    
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

            # Validate this is actually a champion page; otherwise fail fast
            if not self.is_champion(champion_name, content):
                print(f"Error: '{champion_name}' is not a champion page (missing Champion info/module data)")
                return None
            
            # Extract basic information
            champion_data = self._extract_champion_data(champion_name, content)
            
            # Generate markdown
            markdown = self._generate_markdown(champion_data)
            
            # Post-process and write output
            output_file = self.output_dir / f"{champion_name.replace(' ', '_')}.md"
            # Track current page basename to preserve same-page anchors
            self._current_page_basename = champion_name.replace(' ', '_')
            markdown = self._postprocess_markdown(markdown)
            with open(output_file, 'w', encoding='utf-8') as f:
                f.write(markdown)
            
            print(f"Successfully converted {champion_name} to {output_file}")
            return output_file
            
        except Exception as e:
            print(f"Error converting {champion_name}: {e}")
            return None

    # ---------------- Items ----------------
    def convert_item(self, item_name: str) -> Optional[Path]:
        """Convert a single item page to markdown using Module:ItemData and the item's Main page."""
        try:
            print(f"Converting item {item_name}...")
            # Read main page (may contain notes/patch history)
            main_page_path = self.wiki_root / "Main" / item_name.replace(' ', '_') / "page.txt"
            main_content: Optional[str] = None
            if main_page_path.exists():
                main_content = main_page_path.read_text(encoding='utf-8')

            item_data = self._load_item_module_data(item_name)
            if not item_data:
                print(f"Error: Item data not found for '{item_name}' in Module:ItemData")
                return None

            md = self._generate_item_markdown(item_name, item_data, main_content or "")
            output_file = self.output_dir / f"{item_name.replace(' ', '_')}.md"
            self._current_page_basename = item_name.replace(' ', '_')
            md = self._postprocess_markdown(md)
            output_file.write_text(md, encoding='utf-8')
            print(f"Successfully converted {item_name} to {output_file}")
            return output_file
        except Exception as e:
            print(f"Error converting item '{item_name}': {e}")
            return None

    # ---------------- Runes ----------------
    def convert_rune(self, rune_name: str) -> Optional[Path]:
        """Convert a single rune page to markdown (basic conversion of notes and patch history)."""
        try:
            print(f"Converting rune {rune_name}...")
            main_page_path = self.wiki_root / "Main" / rune_name.replace(' ', '_') / "page.txt"
            if not main_page_path.exists():
                print(f"Error: Rune page not found at {main_page_path}")
                return None
            content = main_page_path.read_text(encoding='utf-8')

            data = self._extract_rune_data(rune_name, content)
            md = self._generate_rune_markdown(data)
            output_file = self.output_dir / f"{rune_name.replace(' ', '_')}.md"
            self._current_page_basename = rune_name.replace(' ', '_')
            md = self._postprocess_markdown(md)
            output_file.write_text(md, encoding='utf-8')
            print(f"Successfully converted {rune_name} to {output_file}")
            return output_file
        except Exception as e:
            print(f"Error converting rune '{rune_name}': {e}")
            return None

    def is_champion(self, champion_name: str, content: Optional[str] = None) -> bool:
        """Heuristic to determine if the given page belongs to a champion.
        Checks for '{{Champion info' on the main page or presence in Module:ChampionData.
        """
        try:
            # Check Module:ChampionData
            mod = self._load_champion_module_data(champion_name)
            if isinstance(mod, dict) and mod:
                return True
        except Exception:
            pass
        # Check page content for Champion info template
        if content is None:
            page = self.wiki_root / "Main" / champion_name / "page.txt"
            if page.exists():
                try:
                    content = page.read_text(encoding='utf-8')
                except Exception:
                    content = None
        if content and re.search(r'\{\{\s*Champion info\b', content, flags=re.IGNORECASE):
            return True
        # As an additional weak signal, presence of an abilities section AND a template folder
        # Do not rely solely on this to avoid false positives; require at least one strong signal above
        return False
    
    def _extract_champion_data(self, champion_name: str, content: str) -> Dict[str, Any]:
        """Extract champion data from MediaWiki content."""
        data = {
            "name": champion_name,
            "basic_info": {},
            "abilities": [],
            "pets": [],
            "patch_history": [],
            "trivia": [],
            "stats": {},
            "advanced_stats": {},
            "map_stats": {},
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

        # Extract pets section
        # Capture until the next top-level heading starting at line-begin (avoid '===' inside the section)
        pets_match = re.search(r'^==\s*Pets\s*==\s*(.*?)(?=^==[^=])', content, re.DOTALL | re.IGNORECASE | re.MULTILINE)
        if pets_match:
            pets_content = pets_match.group(1)
            data["pets"] = self._extract_pets(pets_content)
        
        # Extract basic champion info
        champion_info_match = re.search(r'\{\{Champion info\|([^}]+)\}\}', content)
        if champion_info_match:
            data["basic_info"] = self._extract_basic_info(content)
        
        # Extract stats (from Module:ChampionData if available)
        data["stats"] = self._extract_basic_stats(champion_name)
        data["advanced_stats"] = self._extract_advanced_stats(champion_name)

        # Enrich basic info from Module:ChampionData when available
        try:
            module_info = self._load_champion_module_data(champion_name)
        except Exception:
            module_info = None
        if module_info:
            # Merge non-empty fields
            basic_info = data.get("basic_info", {}) or {}
            mapping = {
                "title": "title",
                "resource": "resource",
                "rangetype": "range_type",
                "date": "release_date",
                "patch": "release_patch",
                "changes": "latest_changes",
            }
            for src, dst in mapping.items():
                val = module_info.get(src)
                if isinstance(val, list):
                    val = ", ".join([str(x) for x in val if x])
                if val not in (None, "", []):
                    basic_info[dst] = val
            # Roles/positions
            for key, dst in [("role", "roles"), ("client_positions", "riot_positions"), ("external_positions", "external_positions")]:
                val = module_info.get(key)
                if isinstance(val, list) and val:
                    basic_info[dst] = ", ".join([str(x) for x in val if x])
            # Pricing and misc hero info
            if module_info.get("be") is not None:
                basic_info.setdefault("be", str(module_info.get("be")))
            if module_info.get("rp") is not None:
                basic_info.setdefault("rp", str(module_info.get("rp")))
            if module_info.get("difficulty") is not None:
                basic_info.setdefault("difficulty", str(module_info.get("difficulty")))
            if isinstance(module_info.get("herotype"), str) and module_info.get("herotype"):
                basic_info.setdefault("hero_type", module_info.get("herotype"))
            if isinstance(module_info.get("alttype"), str) and module_info.get("alttype"):
                basic_info.setdefault("alt_type", module_info.get("alttype"))
            if isinstance(module_info.get("adaptivetype"), str) and module_info.get("adaptivetype"):
                basic_info.setdefault("adaptive_type", module_info.get("adaptivetype"))
            # Style metrics if present
            for k in ("damage", "toughness", "control", "mobility", "utility", "style"):
                if module_info.get(k) is not None and str(module_info.get(k)) != "":
                    basic_info.setdefault(k, str(module_info.get(k)))
            data["basic_info"] = basic_info
            # Map-specific stats from module stats block
            if isinstance(module_info.get("stats"), dict):
                stats_block = module_info["stats"]
                maps: Dict[str, Dict[str, Any]] = {}
                for map_key in ("aram", "urf", "usb"):
                    mv = stats_block.get(map_key)
                    if isinstance(mv, dict):
                        entry: Dict[str, Any] = {}
                        for mk in ("dmg_dealt", "dmg_taken", "healing"):
                            if mk in mv:
                                entry[mk] = mv[mk]
                        if entry:
                            maps[map_key] = entry
                if maps:
                    data["map_stats"] = maps
        
        # Extract patch history reference
        patch_match = re.search(r'\{\{Patch box\|([^}]+)\}\}', content)
        if patch_match:
            data["patch_history"] = self._extract_patch_history(champion_name)
        
        return data

    def _extract_advanced_stats(self, champion_name: str) -> Dict[str, Any]:
        """Extract advanced champion stats from Module:ChampionData when available.
        Includes: base AS, AS ratio, bonus AS per level, windup% (if derivable), missile speed,
        selection/pathing/acquisition/gameplay radii and selection height; plus critical damage default.
        """
        adv: Dict[str, Any] = {}
        data = self._load_champion_module_data(champion_name)
        if not data or not isinstance(data.get("stats"), dict):
            return adv
        s = data["stats"]
        def getf(k: str) -> Optional[float]:
            v = s.get(k)
            try:
                if v is None:
                    return None
                if isinstance(v, (int, float)):
                    return float(v)
                v_str = str(v).strip()
                v_str = re.split(r"[^0-9.+-]", v_str)[0]
                return float(v_str) if v_str else None
            except Exception:
                return None
        # Attack speed details
        base_as = getf("as_base")
        as_ratio = getf("as_ratio")
        as_lvl = getf("as_lvl")  # percent per level
        if base_as is not None:
            adv["base_attack_speed"] = base_as
        if as_ratio is not None:
            adv["attack_speed_ratio"] = as_ratio
        if as_lvl is not None:
            adv["bonus_attack_speed_per_level_percent"] = as_lvl
        # Windup% if derivable from attack_cast_time / attack_total_time
        cast = getf("attack_cast_time")
        total = getf("attack_total_time")
        if cast is not None and total and total > 0:
            adv["attack_windup_percent"] = round((cast / total) * 100.0, 1)
        # Windup modifier (some champs specify this directly)
        windup_mod = getf("windup_modifier")
        if windup_mod is not None:
            adv["windup_modifier"] = windup_mod
        # Missile speed
        mspeed = getf("missile_speed")
        if mspeed is not None:
            adv["missile_speed"] = mspeed
        # Radii and selection metrics
        acq = getf("acquisition_radius")
        if acq is not None:
            adv["acquisition_radius"] = acq
        sel_r = getf("selection_radius")
        if sel_r is not None:
            adv["selection_radius"] = sel_r
        sel_h = getf("selection_height")
        if sel_h is not None:
            adv["selection_height"] = sel_h
        path_r = getf("pathing_radius")
        if path_r is not None:
            adv["pathing_radius"] = path_r
        gameplay_r = getf("gameplay_radius")
        if gameplay_r is not None:
            adv["gameplay_radius"] = gameplay_r
        # Critical damage: not in module data; use game default with simple overrides
        crit_default = 175.0
        low_name = champion_name.strip().lower()
        if low_name in ("yasuo", "yone"):
            adv["critical_damage_percent"] = 160.0
        else:
            adv["critical_damage_percent"] = crit_default
        return adv

    # ------ Pets parsing ------
    def _extract_pets(self, pets_content: str) -> List[Dict[str, Any]]:
        """Parse Pets section with <tabber> entries and Infobox/Pet templates."""
        pets: List[Dict[str, Any]] = []
        # Find tabber content
        m = re.search(r'<tabber>([\s\S]*?)</tabber>', pets_content, flags=re.IGNORECASE)
        if m:
            tab_body = m.group(1)
            # Split by tab delimiters `|-|`
            chunks = re.split(r'\|\-\|', tab_body)
            for chunk in chunks:
                # Extract tab title like `Name=`
                header_m = re.match(r'\s*([^\n=]+)=', chunk)
                tab_title = header_m.group(1).strip() if header_m else ""
                # Find Infobox/Pet block
                idx = chunk.find('{{Infobox/Pet')
                if idx == -1:
                    continue
                box = self._extract_balanced_braces(chunk, idx)
                if not box:
                    continue
                info = self._parse_infobox_pet(box)
                if tab_title and not info.get('name'):
                    info['name'] = tab_title
                # Abilities from infobox field or from following lines in the chunk
                abilities: List[Dict[str, str]] = []
                infobox_abilities = info.get('abilities')
                if isinstance(infobox_abilities, str) and infobox_abilities.strip():
                    text = infobox_abilities
                    hdr = re.search(r'(?m)^\s*={2,}[^\n]*', text)
                    if hdr:
                        text = text[:hdr.start()].rstrip()
                    for am in re.finditer(r'(?m)^;\s*(?P<title>[^\n]+)\n(?P<body>[\s\S]*?)(?=^\s*;|\Z)', text):
                        title = am.group('title').strip()
                        body = am.group('body').strip()
                        if title.startswith('='):
                            continue
                        if title or body:
                            abilities.append({
                                'title': self._convert_wiki_to_markdown(title),
                                'description': self._convert_wiki_to_markdown(body)
                            })
                if not abilities:
                    after = chunk[idx + len(box):]
                    for am in re.finditer(r'(?m)^;\s*(?P<title>[^\n]+)\n(?P<body>[\s\S]*?)(?=^\s*;|\Z)', after):
                        title = am.group('title').strip()
                        body = am.group('body').strip()
                        if title or body:
                            abilities.append({
                                'title': self._convert_wiki_to_markdown(title),
                                'description': self._convert_wiki_to_markdown(body)
                            })
                info['abilities'] = abilities
                pets.append(info)
            return pets

        # No <tabber>: parse one or more standalone Infobox/Pet blocks in the Pets section
        # Find all occurrences of '{{Infobox/Pet'
        starts = [m.start() for m in re.finditer(r'\{\{Infobox/Pet', pets_content)]
        if not starts:
            return pets
        # Build segments for each infobox and the text after it (up to next infobox or end)
        for idx_i, start in enumerate(starts):
            box = self._extract_balanced_braces(pets_content, start)
            if not box:
                continue
            end = start + len(box)
            next_start = starts[idx_i + 1] if idx_i + 1 < len(starts) else len(pets_content)
            segment_after = pets_content[end:next_start]
            info = self._parse_infobox_pet(box)
            # Abilities
            abilities: List[Dict[str, str]] = []
            infobox_abilities = info.get('abilities')
            if isinstance(infobox_abilities, str) and infobox_abilities.strip():
                text = infobox_abilities
                hdr = re.search(r'(?m)^\s*={2,}[^\n]*', text)
                if hdr:
                    text = text[:hdr.start()].rstrip()
                for am in re.finditer(r'(?m)^;\s*(?P<title>[^\n]+)\n(?P<body>[\s\S]*?)(?=^\s*;|\Z)', text):
                    title = am.group('title').strip()
                    body = am.group('body').strip()
                    if title.startswith('='):
                        continue
                    if title or body:
                        abilities.append({
                            'title': self._convert_wiki_to_markdown(title),
                            'description': self._convert_wiki_to_markdown(body)
                        })
            if not abilities:
                for am in re.finditer(r'(?m)^;\s*(?P<title>[^\n]+)\n(?P<body>[\s\S]*?)(?=^\s*;|\Z)', segment_after):
                    title = am.group('title').strip()
                    body = am.group('body').strip()
                    if title or body:
                        abilities.append({
                            'title': self._convert_wiki_to_markdown(title),
                            'description': self._convert_wiki_to_markdown(body)
                        })
            info['abilities'] = abilities
            pets.append(info)
        return pets

    def _extract_balanced_braces(self, text: str, start: int) -> Optional[str]:
        """Extract a balanced '{{ ... }}' template block starting at start index."""
        if start < 0 or start + 2 > len(text) or text[start:start+2] != '{{':
            return None
        i = start
        depth = 0
        while i < len(text) - 1:
            if text[i:i+2] == '{{':
                depth += 1
                i += 2
                continue
            if text[i:i+2] == '}}':
                depth -= 1
                i += 2
                if depth == 0:
                    return text[start:i]
                continue
            i += 1
        return None

    def _parse_infobox_pet(self, box: str) -> Dict[str, Any]:
        """Parse key/value pairs inside an Infobox/Pet template with nested templates support."""
        # Strip opening and closing
        inner = box
        if inner.startswith('{{Infobox/Pet'):
            inner = inner[len('{{Infobox/Pet'):]
        if inner.endswith('}}'):
            inner = inner[:-2]
        fields: Dict[str, Any] = {}
        key: Optional[str] = None
        val_lines: List[str] = []
        depth = 0
        # Utility to flush current field
        def flush():
            nonlocal key, val_lines
            if key is not None:
                val = '\n'.join(val_lines).strip()
                val = re.sub(r'<!--.*?-->', '', val, flags=re.DOTALL).strip()
                fields[key] = val
            key = None
            val_lines = []
        # Iterate lines, track depth of nested templates
        for raw_line in inner.splitlines():
            line = raw_line.rstrip('\r')
            # Update depth before testing field start? We need to consider the line context at top level
            # Determine if this line starts a new top-level field
            if depth == 0 and line.lstrip().startswith('|'):
                # New field starts; flush previous one
                flush()
                # Split at first '='
                try:
                    bar_idx = line.index('|')
                except ValueError:
                    bar_idx = 0
                after = line[bar_idx+1:]
                if '=' in after:
                    k, v = after.split('=', 1)
                    key = k.strip().lower().replace(' ', '_')
                    val_lines = [v.strip()]
                else:
                    # Field without '=' (rare); treat remainder as key, empty value
                    key = after.strip().lower().replace(' ', '_')
                    val_lines = []
            else:
                # Continuation of current value
                if key is not None:
                    val_lines.append(line)
            # Update depth counters for templates on this line
            # Count occurrences of '{{' and '}}'
            opens = len(re.findall(r'\{\{', line))
            closes = len(re.findall(r'\}\}', line))
            depth += opens - closes
            if depth < 0:
                depth = 0
        # Flush last field
        flush()
        return fields
    
    def _extract_basic_info(self, content: str) -> Dict[str, str]:
        """Extract basic champion information."""
        info = {}
        
        # Try to extract title from Game banner
        title_match = re.search(r'\{\{Game banner\|([^|}]+)(?:\|[^}]*)?\}\}', content)
        if title_match:
            info['champion'] = title_match.group(1)
        
        return info
    
    def _extract_basic_stats(self, champion_name: str) -> Dict[str, Dict[str, float]]:
        """Extract basic champion statistics from Module:ChampionData if available, else minimal fallback."""
        data = self._load_champion_module_data(champion_name)
        stats: Dict[str, Dict[str, float]] = {}
        if data and isinstance(data.get("stats"), dict):
            s = data["stats"]
            def num(v: Any, default: float = 0.0) -> float:
                try:
                    if v is None:
                        return default
                    if isinstance(v, (int, float)):
                        return float(v)
                    v_str = str(v).strip()
                    # Remove trailing comments or units if any
                    v_str = re.split(r"[^0-9.+-]", v_str)[0]
                    return float(v_str) if v_str else default
                except Exception:
                    return default
            stats = {
                "health": {"base": num(s.get("hp_base")), "growth": num(s.get("hp_lvl"))},
                "mana": {"base": num(s.get("mp_base")), "growth": num(s.get("mp_lvl"))},
                "health_regen": {"base": num(s.get("hp5_base")), "growth": num(s.get("hp5_lvl"))},
                "mana_regen": {"base": num(s.get("mp5_base")), "growth": num(s.get("mp5_lvl"))},
                "armor": {"base": num(s.get("arm_base")), "growth": num(s.get("arm_lvl"))},
                "magic_resist": {"base": num(s.get("mr_base")), "growth": num(s.get("mr_lvl"))},
                "attack_damage": {"base": num(s.get("dam_base")), "growth": num(s.get("dam_lvl"))},
                "attack_speed": {"base": num(s.get("as_base")), "growth": num(s.get("as_lvl"))},
                "movement_speed": {"base": num(s.get("ms")), "growth": 0.0},
                "attack_range": {"base": num(s.get("range")), "growth": 0.0},
            }
            # Drop zero rows to keep table compact but keep core six stats
            essential = {"health", "mana", "armor", "magic_resist", "attack_damage", "attack_speed"}
            for k in list(stats.keys()):
                v = stats[k]
                if k not in essential and (v.get("base", 0) == 0 and v.get("growth", 0) == 0):
                    del stats[k]
            return stats
        # No fallback: if module not found, leave stats empty
        return {}

    # ------ Module:ChampionData helpers ------
    def _module_data_path(self) -> Path:
        return self.wiki_root / "Module" / "ChampionData" / "data" / "page.txt"

    def _read_file_text(self, path: Path) -> Optional[str]:
        try:
            with open(path, 'r', encoding='utf-8') as f:
                return f.read()
        except Exception:
            return None

    def _find_champion_block(self, text: str, champion_name: str) -> Optional[str]:
        """Find and return the Lua table block for the given champion name, including nested { } balanced."""
        # Pattern like ["Azir"] = {
        pat = re.escape(f'[{chr(34)}{champion_name}{chr(34)}]') + r'\s*=\s*\{'
        m = re.search(pat, text)
        if not m:
            return None
        start = m.end() - 1  # position at '{'
        depth = 0
        i = start
        while i < len(text):
            ch = text[i]
            if ch == '{':
                depth += 1
            elif ch == '}':
                depth -= 1
                if depth == 0:
                    # include closing brace
                    return text[start:i+1]
            i += 1
        return None

    def _parse_lua_table_top(self, block: str) -> Dict[str, Any]:
        """Parse a top-level Lua table block with entries ["key"] = value, extracting simple numbers/strings and nested stats table."""
        res: Dict[str, Any] = {}
        # Extract ["key"] = "string" or number
        for m in re.finditer(r'\[\"([^\"]+)\"\]\s*=\s*(.+?)(?=,\s*\[\"|\}\s*$)', block, flags=re.DOTALL):
            key = m.group(1)
            val = m.group(2).strip()
            # If this is a nested table (like stats = { ... })
            if val.startswith('{'):
                # For stats subtable we want full balanced table
                if key == 'stats':
                    # Find the balanced stats block starting at this val position
                    sub_start = m.start(2)
                    # Walk from sub_start to match braces
                    depth = 0
                    i = sub_start
                    while i < len(block):
                        ch = block[i]
                        if ch == '{':
                            depth += 1
                        elif ch == '}':
                            depth -= 1
                            if depth == 0:
                                sub_block = block[sub_start:i+1]
                                res[key] = self._parse_lua_simple_kv(sub_block)
                                break
                        i += 1
                else:
                    # Attempt to parse as a simple array of strings
                    sub_start = m.start(2)
                    depth = 0
                    i = sub_start
                    while i < len(block):
                        ch = block[i]
                        if ch == '{':
                            depth += 1
                        elif ch == '}':
                            depth -= 1
                            if depth == 0:
                                sub_block = block[sub_start:i+1]
                                res[key] = self._parse_lua_string_array(sub_block)
                                break
                        i += 1
            else:
                # Try to parse number or quoted string
                str_m = re.match(r'^"([^"]*)"', val)
                if str_m:
                    res[key] = str_m.group(1)
                else:
                    try:
                        res[key] = float(val) if ('.' in val or 'e' in val.lower()) else int(val)
                    except Exception:
                        # Array style {"a","b"}
                        arr_m = re.match(r'^\{([^}]*)\}', val, flags=re.DOTALL)
                        if arr_m:
                            items = [x.strip().strip('"\'') for x in arr_m.group(1).split(',') if x.strip()]
                            res[key] = [x for x in items if x]
                        else:
                            res[key] = val
        return res

    def _parse_lua_string_array(self, table_text: str) -> List[str]:
        """Parse a Lua array-like table {"a", "b", ...} into a Python list of strings."""
        s = table_text.strip()
        if s.startswith('{') and s.endswith('}'):
            s = s[1:-1]
        # Extract quoted strings
        return [m.group(1) for m in re.finditer(r'"([^"]+)"', s)]

    def _parse_lua_simple_kv(self, table_text: str) -> Dict[str, Any]:
        """Parse a Lua table of form { ["k"]=v, ["k2"]=v2, ... } supporting nested subtables.
        Values can be numbers, quoted strings, or nested tables (recursively parsed).
        """
        out: Dict[str, Any] = {}
        s = table_text.strip()
        # Strip outer braces
        if s.startswith('{') and s.endswith('}'):
            s = s[1:-1]
        i = 0
        n = len(s)
        while i < n:
            # Skip whitespace and commas
            while i < n and s[i] in ' \t\r\n,':
                i += 1
            if i >= n:
                break
            # Expect key like ["key"]
            key_m = re.match(r'\[\"([^\"]+)\"\]\s*=\s*', s[i:])
            if key_m:
                key = key_m.group(1)
                i += key_m.end()
            else:
                key_m_num = re.match(r'\[(\d+)\]\s*=\s*', s[i:])
                if key_m_num:
                    # Numeric-indexed keys -> keep as stringified index for dict; callers may convert to list if desired
                    key = key_m_num.group(1)
                    i += key_m_num.end()
                else:
                    # Can't parse further
                    break
            if i >= n:
                break
            # Determine value type
            ch = s[i]
            # Nested table
            if ch == '{':
                depth = 0
                start = i
                while i < n:
                    if s[i] == '{':
                        depth += 1
                    elif s[i] == '}':
                        depth -= 1
                        if depth == 0:
                            i += 1  # include closing brace
                            break
                    i += 1
                sub = s[start:i]
                # First try as key-value style subtable
                parsed_sub: Union[Dict[str, Any], List[Any], str]
                parsed_sub = {}
                try:
                    parsed_sub = self._parse_lua_simple_kv(sub)
                except Exception:
                    parsed_sub = {}
                # If empty, try as array-like table
                if isinstance(parsed_sub, dict) and not parsed_sub:
                    arr = self._parse_lua_array(sub)
                    if arr:
                        out[key] = arr
                    else:
                        out[key] = sub
                else:
                    out[key] = parsed_sub
            # Quoted string
            elif ch == '"':
                i += 1
                start = i
                while i < n and s[i] != '"':
                    # naive string parse; escapes rare in this data
                    i += 1
                out[key] = s[start:i]
                i += 1  # skip closing quote
            else:
                # Bare value until comma or end or closing brace at top level
                start = i
                while i < n and s[i] not in ',}\n':
                    i += 1
                val = s[start:i].strip()
                # Try number
                num_m = re.match(r'^-?\d+(?:\.\d+)?$', val)
                if num_m:
                    out[key] = float(val) if '.' in val else int(val)
                else:
                    out[key] = val.strip('"')
            # Move past trailing spaces/commas before next pair
            while i < n and s[i] in ' \t\r\n,':
                i += 1
        return out

    def _parse_lua_array(self, table_text: str) -> List[Any]:
        """Parse a Lua array-like table {"a", "b", 3, 4} into a Python list.
        Supports simple quoted strings and numbers; ignores nested subtables.
        Returns [] if no array elements found.
        """
        s = table_text.strip()
        if s.startswith('{') and s.endswith('}'):
            s = s[1:-1]
        # Tokenize by commas at top-level (no nested braces handling needed for our use-cases)
        parts: List[str] = []
        buf: List[str] = []
        depth = 0
        i = 0
        n = len(s)
        while i < n:
            ch = s[i]
            if ch == '{':
                depth += 1
                buf.append(ch)
                i += 1
                continue
            if ch == '}':
                depth = max(0, depth - 1)
                buf.append(ch)
                i += 1
                continue
            if ch == ',' and depth == 0:
                parts.append(''.join(buf).strip())
                buf = []
                i += 1
                continue
            buf.append(ch)
            i += 1
        if buf:
            parts.append(''.join(buf).strip())
        out: List[Any] = []
        for p in parts:
            if not p:
                continue
            # Strip trailing comments or spaces
            p = re.sub(r'--.*$', '', p).strip()
            # Quoted string
            m = re.match(r'^"([^"]*)"$', p)
            if m:
                out.append(m.group(1))
                continue
            # Number
            if re.match(r'^-?\d+(?:\.\d+)?$', p):
                out.append(float(p) if '.' in p else int(p))
                continue
            # Fallback: raw token
            out.append(p.strip('"'))
        return out

    # ------ Module:ItemData helpers ------
    def _item_data_path(self) -> Path:
        return self.wiki_root / "Module" / "ItemData" / "data" / "page.txt"

    def _find_item_block(self, text: str, item_name: str) -> Optional[str]:
        """Find Lua table block for the given item name inside Module:ItemData/data/page.txt."""
        pat = re.escape(f'["{item_name}"]') + r'\s*=\s*\{'
        m = re.search(pat, text)
        if not m:
            return None
        start = m.end() - 1  # at '{'
        depth = 0
        i = start
        n = len(text)
        while i < n:
            ch = text[i]
            if ch == '{':
                depth += 1
            elif ch == '}':
                depth -= 1
                if depth == 0:
                    return text[start:i+1]
            i += 1
        return None

    def _load_item_module_data(self, item_name: str) -> Optional[Dict[str, Any]]:
        """Load item entry from Module:ItemData/data/page.txt as a Python dict."""
        path = self._item_data_path()
        if not path.exists():
            return None
        text = self._read_file_text(path)
        if not text:
            return None
        block = self._find_item_block(text, item_name)
        if not block:
            # Try alternative names with underscores replaced by spaces or vice versa
            alt = item_name.replace('_', ' ')
            if alt != item_name:
                block = self._find_item_block(text, alt)
                if block:
                    item_name = alt
            if not block:
                return None
        try:
            data = self._parse_lua_simple_kv(block)
        except Exception:
            data = None
        return data if data else None

    def _load_champion_module_data(self, champion_name: str) -> Optional[Dict[str, Any]]:
        """Load champion entry from Module:ChampionData/data/page.txt and return a dict with keys like stats/title/resource/etc."""
        path = self._module_data_path()
        if not path.exists():
            return None
        text = self._read_file_text(path)
        if not text:
            return None
        block = self._find_champion_block(text, champion_name)
        if not block:
            return None
        top = self._parse_lua_table_top(block)
        return top if top else None
    
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
    
    def _resolve_simple_data_templates(self, text: str, champion_name: Optional[str] = None) -> str:
        """Resolve a few simple data-bearing templates to numeric literals to enable formula evaluation.
        - {{ccd|Champion|crit_base}} -> champion critical damage percent (e.g., 175)
        - {{cid|ItemName|field}} -> item field numeric (e.g., Infinity Edge critdamage)
        Only resolves when a numeric value can be found; otherwise leaves the template intact.
        """
        if not text or ('{{ccd' not in text and '{{cid' not in text):
            return text

        out = text

        # Resolve ccd (Champion Constant Data) for crit_base
        def _ccd_repl(m: re.Match) -> str:
            champ = (m.group(1) or '').strip()
            key = (m.group(2) or '').strip().lower()
            # Prefer provided champion name context
            target = champion_name or champ
            val: Optional[float] = None
            try:
                if key in ('crit_base', 'critical_base', 'critdamage', 'crit_damage'):
                    # Use our advanced stats default: 175% normally, 160% for Yasuo/Yone
                    low = (target or '').strip().lower()
                    val = 160.0 if low in ('yasuo', 'yone') else 175.0
            except Exception:
                val = None
            return str(val) if val is not None else m.group(0)

        out = re.sub(r"\{\{\s*ccd\s*\|\s*([^|}]+)\s*\|\s*([^|}]+)\s*\}\}", _ccd_repl, out, flags=re.IGNORECASE)

        # Resolve cid (Cargo Item Data) for simple numeric fields
        def _cid_repl(m: re.Match) -> str:
            name = (m.group(1) or '').strip()
            field = (m.group(2) or '').strip()
            field_l = field.lower()
            data = None
            try:
                data = self._load_item_module_data(name)
            except Exception:
                data = None
            val: Optional[float] = None
            if isinstance(data, dict):
                # direct field
                if field_l in data and isinstance(data[field_l], (int, float)):
                    try:
                        val = float(data[field_l])
                    except Exception:
                        val = None
                # sometimes stats nested under 'stats'
                if val is None and isinstance(data.get('stats'), dict):
                    st = data['stats']
                    if field_l in st and isinstance(st[field_l], (int, float)):
                        try:
                            val = float(st[field_l])
                        except Exception:
                            val = None
            # One-off fallback for common field if module lacks it
            if val is None and name.lower() == 'infinity edge' and field_l in ('critdamage','crit_damage','critdmg','critdmg_bonus'):
                # Modern IE crit damage bonus typically around 45%; use 45 as a pragmatic default
                val = 45.0
            return str(val) if val is not None else m.group(0)

        out = re.sub(r"\{\{\s*cid\s*\|\s*([^|}]+)\s*\|\s*([^|}]+)\s*\}\}", _cid_repl, out, flags=re.IGNORECASE)
        return out

    # ---------------- Custom Table/Structural Template Expansion (Quote, Channel Type) ----------------
    def _expand_custom_templates(self, text: str, champion_name: Optional[str]) -> str:
        """Apply lightweight expansions for select high-value templates BEFORE general wiki->markdown conversion.
        Currently implemented:
          - {{Quote|text|author|source}}
          - {{ct|type|attack=...|move=...|cast=...|items=...|spells=...|consume=...|interrupts=...|damage=...}}
                    - Item Haste tabber ({{Item haste table}} or <tabber> with Item Haste section)
        The expansion is deliberately conservative: if parsing fails, the original template is left intact so the
        downstream generic cleaner can remove or simplify it without throwing exceptions.
        """
        if not text or '{{' not in text:
            return text
        # Order matters: expand Quote first (simple) then channel type (may contain nested templates preserved verbatim)
        text = self._expand_quote_templates(text)
        text = self._expand_channel_type_templates(text)
        text = self._expand_item_haste_tabber(text)
        # Light-weight ward icononly mapping (convert {{ii|Item|icononly=yes}} -> Item) prior to generic stripping
        if '{{ii|' in text.lower():
            text = re.sub(r"\{\{ii\|([^|}]+)\|icononly=yes[^}]*\}\}", lambda m: m.group(1).strip(), text, flags=re.IGNORECASE)
        return text

    def _expand_quote_templates(self, text: str) -> str:
        """Convert {{Quote|text|author|source}} to Markdown blockquote.
        Handles up to 3 positional args; nested templates inside params are preserved for later passes.
        """
        if 'Quote' not in text and 'quote' not in text:
            return text
        out: list[str] = []
        i = 0
        n = len(text)
        while i < n:
            j = text.lower().find('{{quote', i)
            if j == -1:
                out.append(text[i:])
                break
            # append preceding slice
            out.append(text[i:j])
            block = self._extract_balanced_braces(text, j)
            if not block:
                # fail-safe: append remainder and break
                out.append(text[j:])
                break
            # Parse inside
            inner = block[2:-2]  # strip {{ }}
            # remove template name (case-insensitive) + optional whitespace + leading pipe
            m = re.match(r'(?i)quote\s*\|?(.*)$', inner, flags=re.DOTALL)
            if not m:
                out.append(block)
                i = j + len(block)
                continue
            args_raw = m.group(1)
            parts = self._split_top_level_pipes(args_raw)
            # Positional: ignore key=value segments (robustness)
            pos: list[str] = []
            for seg in parts:
                seg_s = seg.strip()
                if not seg_s:
                    continue
                # treat as positional if no top-level '='
                depth = 0
                has_eq = False
                for ch in seg_s:
                    if ch == '{':
                        depth += 1
                    elif ch == '}':
                        depth = max(0, depth - 1)
                    elif ch == '=' and depth == 0:
                        has_eq = True
                        break
                if not has_eq:
                    pos.append(seg_s)
            quote_text = pos[0].strip() if len(pos) >= 1 else ''
            author = pos[1].strip() if len(pos) >= 2 else ''
            source = pos[2].strip() if len(pos) >= 3 else ''
            if not quote_text:
                # nothing meaningful; keep original
                out.append(block)
                i = j + len(block)
                continue
            # Assemble Markdown blockquote
            # Avoid adding smart quotes if already wrapped in any quote chars
            qt = quote_text
            lines = [f"> {qt}" ] if qt else []  # Avoid additional punctuation; user text may contain period
            attrib_line = ''
            if author:
                if source:
                    attrib_line = f"> — {author}, *{source}*"
                else:
                    attrib_line = f"> — {author}"
            elif source:  # source w/o author (rare) -> treat as source only
                attrib_line = f"> — *{source}*"
            if attrib_line:
                lines.append(attrib_line)
            md_block = '\n'.join(lines)
            # Ensure a blank line before if previous output doesn't end with newline (except at start)
            if out and not out[-1].endswith('\n'):
                md_block = '\n' + md_block
            out.append(md_block)
            i = j + len(block)
        return ''.join(out)

    def _expand_channel_type_templates(self, text: str) -> str:
        """Convert {{ct|...}} (channel type behavior) templates into a mini-markdown table.
        The template shape: {{ct|type|attack=false|move=true|cast=...|items=interrupts,true,,false|spells=true,true,interrupts|consume=true|interrupts=Silence,Death|damage=...}}
        Unrecognized / malformed instances are left unchanged.
        """
        # Quick exit
        if '{{ct' not in text.lower() and '{{channel type' not in text.lower():
            return text
        out: list[str] = []
        i = 0
        n = len(text)
        name_pattern = re.compile(r'(?i)^(ct|channel[_ ]type|ctable)')
        while i < n:
            j = text.lower().find('{{ct', i)
            j2 = text.lower().find('{{channel type', i)
            # pick earliest occurrence
            candidates = [pos for pos in [j, j2] if pos != -1]
            if not candidates:
                out.append(text[i:])
                break
            j = min(candidates)
            out.append(text[i:j])
            block = self._extract_balanced_braces(text, j)
            if not block:
                out.append(text[j:])
                break
            inner = block[2:-2]
            # Split at first '|' after template name to isolate remaining params
            # Extract name token
            mname = re.match(r'\s*([^|]+)\|?(.*)$', inner, flags=re.DOTALL)
            if not mname:
                out.append(block)
                i = j + len(block)
                continue
            raw_name = mname.group(1).strip().lower().replace('_', ' ')
            if not name_pattern.match(raw_name):
                # Not a ct variant
                out.append(block)
                i = j + len(block)
                continue
            rest = mname.group(2)
            parts = self._split_top_level_pipes(rest)
            pos: list[str] = []
            named: dict[str, str] = {}
            for seg in parts:
                seg_s = seg.strip()
                if not seg_s:
                    continue
                # detect k=v at top level
                depth = 0
                eq_index = -1
                for idx, ch in enumerate(seg_s):
                    if ch == '{':
                        depth += 1
                    elif ch == '}':
                        depth = max(0, depth - 1)
                    elif ch == '=' and depth == 0:
                        eq_index = idx
                        break
                if eq_index != -1:
                    k = seg_s[:eq_index].strip().lower()
                    v = seg_s[eq_index + 1:].strip()
                    named[k] = v
                else:
                    pos.append(seg_s)
            # Accept missing first positional arg if we clearly have ct named params; default to 'unknown'
            if not pos:
                ct_keys = {"attack","move","cast","items","spells","consume","interrupts","damage"}
                if any(k in named for k in ct_keys):
                    pos = ["unknown"]
                else:
                    out.append(block)
                    i = j + len(block)
                    continue
            ctype = pos[0].strip().lower() if pos and pos[0] else 'unknown'
            # Helper to tokenize comma-separated values (top-level only)
            def _tokens(val: str) -> list[str]:
                # Split on commas not inside nested templates (rare) – simple depth counter
                toks: list[str] = []
                buf: list[str] = []
                depth = 0
                for ch in val:
                    if ch == '{':
                        depth += 1
                    elif ch == '}':
                        depth = max(0, depth - 1)
                    if ch == ',' and depth == 0:
                        tok = ''.join(buf).strip()
                        if tok:
                            toks.append(tok)
                        buf = []
                    else:
                        buf.append(ch)
                last = ''.join(buf).strip()
                if last:
                    toks.append(last)
                return toks
            # Normalization map for simple boolean/keyword tokens
            def _classify_basic(val: str) -> str:
                v = val.strip().lower()
                if v in ('true', 'yes', 'allowed', 'allow'):
                    return 'allowed'
                if v in ('false', 'no', 'disabled', 'disable'):
                    return 'disabled'
                if v.startswith('interrupt'):
                    return 'interrupts'
                if v.startswith('recast') or v == 'recasts':
                    return 'recasts'
                return 'other'
            def _reduce_simple(tokens: list[str]) -> tuple[list[str], list[str], list[str], list[str]]:
                allowed: list[str] = []
                disabled: list[str] = []
                interrupts: list[str] = []
                others: list[str] = []
                for t in tokens:
                    cls = _classify_basic(t)
                    if cls == 'allowed':
                        allowed.append(t)
                    elif cls == 'disabled':
                        disabled.append(t)
                    elif cls == 'interrupts':
                        interrupts.append(t)
                    else:
                        others.append(t)
                return allowed, disabled, interrupts, others
            # Build rows
            rows: list[tuple[str, str]] = []
            notes: list[str] = []
            def add_row(label: str, value: str):
                rows.append((label, value if value else ''))
            # Attack / movement / abilities (uniform handling)
            for key, label in [('attack', 'Attacking'), ('move', 'Movement'), ('cast', 'Abilities')]:
                if key in named:
                    toks = _tokens(named[key])
                    allowed, disabled, interrupts, others = _reduce_simple(toks)
                    state_parts: list[str] = []
                    if interrupts:
                        state_parts.append('Interrupts')
                    if allowed and not disabled:
                        state_parts.append('Allowed')
                    if disabled and not allowed:
                        state_parts.append('Disabled')
                    if allowed and disabled:
                        state_parts.append('Partially Allowed')
                    # Recasts indicator if explicitly present
                    if any(_classify_basic(t) == 'recasts' for t in toks):
                        state_parts.append('Recasts')
                    # If no simple classification matched, keep the original narrative/text
                    add_row(label, ' / '.join(state_parts) if state_parts else (named[key] or ''))
            # Items / Summoner Spells token breakdown with positional semantics
            def _render_spells_summary(val: str) -> str:
                raw = val.strip()
                toks = _tokens(raw)
                # Mapping per Channel_type_table
                group0 = [
                    'Barrier','Clarity','Cleanse','Exhaust','Ghost','Heal','Ignite','Smite'
                ]
                idx_map = {
                    1: ['Flash'],
                    2: ['Teleport'],
                    3: ['Recall'],
                    4: ['Hexflash'],
                    5: ['Mark'],
                    6: ['Dash'],
                }
                def classify(v: str) -> str:
                    v2 = v.strip().lower()
                    if v2 in ('true','yes','allowed','allow'): return 'allowed'
                    if v2 in ('false','no','disabled','disable'): return 'disabled'
                    if v2.startswith('interrupt'): return 'interrupts'
                    if v2.startswith('recast'): return 'recasts'
                    if v2 == 'pft': return 'pft'
                    return 'other'
                # Single-token: applies to all spells
                if ',' not in raw and len(toks) == 1:
                    t = classify(toks[0])
                    if t == 'allowed':
                        return 'Usable: All summoner spells'
                    if t == 'disabled':
                        return 'Disabled: All summoner spells'
                    if t == 'interrupts':
                        return 'Interrupted by: All summoner spells'
                    if t == 'pft':
                        return 'Pending for Test: All summoner spells'
                    return self._convert_wiki_to_markdown_ctx(raw, None)
                allowed: list[str] = []
                disabled: list[str] = []
                interrupts: list[str] = []
                pft: list[str] = []
                # Expand index 0 (group of 8 spells)
                if len(toks) >= 1:
                    c = classify(toks[0])
                    if c == 'allowed':
                        allowed.extend(group0)
                    elif c == 'disabled':
                        disabled.extend(group0)
                    elif c == 'interrupts':
                        interrupts.extend(group0)
                    elif c == 'pft':
                        pft.extend(group0)
                # Indices 1..6
                for idx in range(1, 7):
                    if idx < len(toks):
                        c = classify(toks[idx])
                        if c == 'allowed':
                            allowed.extend(idx_map.get(idx, []))
                        elif c == 'disabled':
                            disabled.extend(idx_map.get(idx, []))
                        elif c == 'interrupts':
                            # Special: recasts marker may accompany Hexflash in template; treat as interrupt
                            lbls = idx_map.get(idx, [])
                            # If token literally 'recasts', handled below; else interrupts
                            interrupts.extend(lbls)
                        elif c == 'recasts':
                            # Append note to Hexflash if idx==4
                            names = idx_map.get(idx, [])
                            interrupts.extend([n + ' (recasts)' for n in names])
                        elif c == 'pft':
                            pft.extend(idx_map.get(idx, []))
                parts: list[str] = []
                def join_list(xs: list[str]) -> str:
                    # Keep order and deduplicate
                    seen: set[str] = set()
                    out: list[str] = []
                    for x in xs:
                        k = x.lower()
                        if k in seen: continue
                        seen.add(k)
                        out.append(x)
                    return ', '.join(out)
                if allowed:
                    parts.append(f"Usable: {join_list(allowed)}")
                if disabled:
                    parts.append(f"Disabled: {join_list(disabled)}")
                if interrupts:
                    parts.append(f"Interrupted by: {join_list(interrupts)}")
                if pft:
                    parts.append(f"Pending for Test: {join_list(pft)}")
                return '; '.join(parts) if parts else self._convert_wiki_to_markdown_ctx(raw, None)

            def _render_items_summary(val: str) -> str:
                raw = val.strip()
                toks = _tokens(raw)
                # Positional mapping from Channel_type_table
                idx_map = {
                    1: ["Shurelya's Battlesong", "Youmuu's Ghostblade", "Randuin's Omen"],
                    2: ["Zhonya's Hourglass"],
                    3: ["Hextech Rocketbelt"],
                    4: ["Stridebreaker"],
                }
                def classify(v: str) -> str:
                    v2 = v.strip().lower()
                    if v2 in ('true','yes','allowed','allow'): return 'allowed'
                    if v2 in ('false','no','disabled','disable'): return 'disabled'
                    if v2.startswith('interrupt'): return 'interrupts'
                    if v2 == 'pft': return 'pft'
                    return 'other'
                # Single-token applies to all items
                if ',' not in raw and len(toks) == 1:
                    t = classify(toks[0])
                    if t == 'allowed':
                        return 'Usable: All items'
                    if t == 'disabled':
                        return 'Disabled: All items'
                    if t == 'interrupts':
                        return 'Interrupted by: All items'
                    if t == 'pft':
                        return 'Pending for Test: All items'
                    return self._convert_wiki_to_markdown_ctx(raw, None)
                allowed: list[str] = []
                disabled: list[str] = []
                interrupts: list[str] = []
                pft: list[str] = []
                other_note: Optional[str] = None
                # Index 0 controls "All other item-actives ..."
                if len(toks) >= 1:
                    c0 = classify(toks[0])
                    if c0 == 'allowed':
                        other_note = 'Other items: Usable'
                    elif c0 == 'disabled':
                        other_note = 'Other items: Disabled'
                    elif c0 == 'interrupts':
                        other_note = 'Other items: Interrupt'
                    elif c0 == 'pft':
                        other_note = 'Other items: Pending for Test'
                # Specific items by indices
                for idx in range(1, 5):
                    if idx < len(toks):
                        c = classify(toks[idx])
                        names = idx_map.get(idx, [])
                        if c == 'allowed':
                            allowed.extend(names)
                        elif c == 'disabled':
                            disabled.extend(names)
                        elif c == 'interrupts':
                            interrupts.extend(names)
                        elif c == 'pft':
                            pft.extend(names)
                def join_list(xs: list[str]) -> str:
                    seen: set[str] = set(); out: list[str] = []
                    for x in xs:
                        k = x.lower()
                        if k in seen: continue
                        seen.add(k); out.append(x)
                    return ', '.join(out)
                parts: list[str] = []
                if allowed:
                    parts.append(f"Usable: {join_list(allowed)}")
                if disabled:
                    parts.append(f"Disabled: {join_list(disabled)}")
                if interrupts:
                    parts.append(f"Interrupted by: {join_list(interrupts)}")
                if pft:
                    parts.append(f"Pending for Test: {join_list(pft)}")
                if other_note:
                    parts.append(other_note)
                return '; '.join(parts) if parts else self._convert_wiki_to_markdown_ctx(raw, None)

            if 'items' in named:
                add_row('Items', _render_items_summary(named['items']))
            if 'spells' in named:
                add_row('Summoner Spells', _render_spells_summary(named['spells']))
            # Consumables
            if 'consume' in named:
                toks = _tokens(named['consume'])
                allowed, disabled, interrupts, others = _reduce_simple(toks)
                if allowed and not disabled:
                    add_row('Consumables', 'Usable')
                elif disabled and not allowed:
                    add_row('Consumables', 'Disabled')
                elif allowed and disabled:
                    add_row('Consumables', 'Partially Usable')
                if interrupts:
                    add_row('Consumables', 'Interrupts')
                if others:
                    notes.append(' '.join(others))
            # Human-friendly formatting for interrupt tokens
            def _format_interrupts(int_val: str, damage_val: Optional[str]) -> str:
                tokens = [t.strip() for t in _tokens(int_val)] if int_val else []
                out_parts: list[str] = []
                # Damage effects line (from separate 'damage' param in template)
                if damage_val is not None:
                    dv = damage_val.strip().lower()
                    if dv == 'true':
                        out_parts.append('Damaging effects')
                    elif dv and dv != 'false':
                        out_parts.append(self._convert_wiki_to_markdown_ctx(damage_val.strip(), None))
                # Map known interrupt keywords
                mapping = {
                    'death': 'Death',
                    'revival-false': 'Death (unless protected by Resurrection)',
                    'ground': 'Grounding effects',
                    'root': 'Immobilizing effects',
                    'silence': 'Cast-inhibiting effects',
                }
                seen: set[str] = set()
                for tok in tokens:
                    key = tok.lower()
                    label = mapping.get(key)
                    if not label:
                        # keep unknown token as-is
                        label = self._convert_wiki_to_markdown_ctx(tok, None)
                    k2 = label.lower()
                    if k2 not in seen:
                        seen.add(k2)
                        out_parts.append(label)
                return ', '.join(out_parts) if out_parts else (int_val or 'N/A')

            if 'interrupts' in named:
                add_row('Interrupted by', _format_interrupts(named['interrupts'], named.get('damage')))
            if 'damage' in named:
                add_row('Damage', named['damage'])
            # Do not duplicate cast narrative into notes; Abilities cell already conveys it.
            # Assemble markdown table
            if not rows:
                # Synthesize a minimal table from any known params rather than leaking raw template
                synth_order = [
                    ("items", "Items"), ("spells", "Summoner Spells"), ("consume", "Consumables"),
                    ("interrupts", "Interrupted by"), ("damage", "Damage"),
                ]
                for k, lab in synth_order:
                    if k in named and named[k].strip():
                        val = self._convert_wiki_to_markdown_ctx(named[k].strip(), None)
                        rows.append((lab, val))
                # If still empty, drop the template entirely
                if not rows:
                    i = j + len(block)
                    continue
            table_lines = ["| Aspect | State / Notes |", "|--------|---------------|"]
            # Deduplicate notes while preserving order
            dedup_notes: list[str] = []
            seen_notes: set[str] = set()
            for ntext in notes:
                key = ntext.strip().lower()
                if key and key not in seen_notes:
                    dedup_notes.append(ntext.strip())
                    seen_notes.add(key)
            # Optional first row with Type, if known
            def _display_ct_type(t: str) -> Optional[str]:
                if not t or t == 'unknown':
                    return None
                mapping = {
                    'channel': 'Charge channel',
                    'charge': 'Charge channel',
                    'charging': 'Charge channel',
                }
                return mapping.get(t, t.replace('_', ' ').title())
            type_disp = _display_ct_type(ctype)
            if type_disp:
                table_lines.append(f"| **Type** | {type_disp} |")
            for label, val in rows:
                safe_val = val.replace('\n', ' ').strip()
                # Convert any nested templates inside cells now to markdown (context-less)
                safe_val = self._convert_wiki_to_markdown_ctx(safe_val, None)
                table_lines.append(f"| **{label}** | {safe_val} |")
            if dedup_notes and any(n for n in dedup_notes if n.lower() not in {r[1].lower() for r in rows}):
                note_text = ' '.join(dedup_notes).strip()
                if note_text:
                    note_text = self._convert_wiki_to_markdown_ctx(note_text, None)
                    table_lines.append(f"| **Notes** | {note_text} |")
            table_md = '\n'.join(table_lines)
            # Ensure blank line separation
            if out and not out[-1].endswith('\n\n'):
                if not out[-1].endswith('\n'):
                    table_md = '\n' + table_md
                table_md = '\n' + table_md
            out.append(table_md)
            i = j + len(block)
        return ''.join(out)

    # ---------------- Item Haste Tabber ----------------
    def _expand_item_haste_tabber(self, text: str) -> str:
        """Parse the Item Haste tabber (three wikitext tables) into Markdown subsections.
        Detection:
          - '{{Item haste table' template (balanced braces)
          - <tabber> block containing 'Item Haste' header text and wikitext tables
        Limitations: best-effort; if parsing fails, original markup is preserved.
        """
        lowered = text.lower()
        if 'item haste' not in lowered or ('<tabber>' not in lowered and '{{item haste table' not in lowered):
            return text
        # Strategy: iterate over <tabber> blocks first
        out_parts: list[str] = []
        i = 0
        n = len(text)
        changed = False
        while i < n:
            tab_start = text.lower().find('<tabber>', i)
            if tab_start == -1:
                out_parts.append(text[i:])
                break
            tab_end = text.lower().find('</tabber>', tab_start)
            if tab_end == -1:
                # unmatched; append rest
                out_parts.append(text[i:])
                break
            block = text[tab_start: tab_end + 9]
            inner = block[8:-9]  # between <tabber> and </tabber>
            if 'item haste' not in block.lower():
                # keep as-is
                out_parts.append(text[i:tab_end + 9])
                i = tab_end + 9
                continue
            rendered = self._render_item_haste_tabber(inner)
            if rendered:
                # Ensure separation
                if out_parts and not out_parts[-1].endswith('\n'):
                    out_parts.append('\n')
                out_parts.append(rendered)
                changed = True
            else:
                out_parts.append(block)
            i = tab_end + 9
        result = ''.join(out_parts)
        return result if changed else text

    def _render_item_haste_tabber(self, tab_inner: str) -> Optional[str]:
        """Render the inner portion of an Item Haste <tabber> block into markdown.
        Splits on '|-|' to get individual labeled chunks, extracts the first wikitext table per chunk.
        """
        # Split segments
        chunks = re.split(r'\|\-\|', tab_inner)
        sections: list[tuple[str, str]] = []  # (title, table_md)
        for chunk in chunks:
            if not chunk.strip():
                continue
            # Title pattern: Some Title= followed by newline/table
            m = re.match(r'\s*([^=\n]{2,80})=(.*)$', chunk, flags=re.DOTALL)
            if not m:
                continue
            title = m.group(1).strip()
            body = m.group(2)
            # Find first wikitext table
            tbl_start = body.find('{|')
            if tbl_start == -1:
                continue
            tbl_end = body.find('|}', tbl_start)
            if tbl_end == -1:
                continue
            table_text = body[tbl_start: tbl_end + 2]
            md_table = self._wikitext_table_to_md(table_text)
            if md_table:
                sections.append((title, md_table))
        if not sections:
            return None
        # Normalize known section titles (fallback to provided)
        lines: list[str] = []
        title_map = {
            'item haste': 'Item Haste',
            'equivalent cdr': 'Equivalent CDR',
            'item cooldowns': 'Item Cooldowns',
        }
        for title, tbl in sections:
            key = title.lower()
            disp = title_map.get(key, title)
            lines.append(f"### {disp}\n\n{tbl}\n")
        return '\n'.join(lines).strip()

    def _wikitext_table_to_md(self, table_text: str) -> Optional[str]:
        """Convert a single wikitext table ({| ... |}) to a Markdown table.
        Basic support: header row beginning with '!' cells, data rows with '|' or '|-'.
        Strips most style attributes. Returns None on failure.
        """
        try:
            lines = table_text.splitlines()
            # Strip outer table markers until first line starting with '{|'
            # Already guaranteed pattern by caller
            rows: list[list[str]] = []
            header: list[str] = []
            cur_row: list[str] = []
            def flush():
                nonlocal cur_row
                if cur_row:
                    rows.append(cur_row)
                cur_row = []
            for raw in lines:
                line = raw.strip()
                if not line:
                    continue
                if line.startswith('{|'):
                    continue
                if line.startswith('|}'):
                    flush()
                    break
                if line.startswith('|-'):
                    flush()
                    continue
                # Header cells
                if line.startswith('!'):
                    # possible combined header row; split by '!!'
                    cells = [c.strip() for c in re.split(r'!!', line.lstrip('!'))]
                    clean_cells = [self._clean_wikitext_cell(c) for c in cells if c]
                    if not header:
                        header = clean_cells
                    else:
                        # additional header row -> treat as data
                        if clean_cells:
                            rows.append(clean_cells)
                    continue
                # Data cells: may use '||'
                if line.startswith('|'):
                    # remove leading '|'
                    content = line[1:]
                    cells = [c.strip() for c in re.split(r'\|\|', content)]
                    clean_cells = [self._clean_wikitext_cell(c) for c in cells]
                    cur_row.extend(clean_cells)
                    continue
            # If last row not flushed
            flush()
            if not header and rows:
                # Use first row as header if none defined
                header = rows.pop(0)
            if not header:
                return None
            # Normalize column lengths
            col_count = len(header)
            norm_rows = [r[:col_count] + [''] * (col_count - len(r)) if len(r) < col_count else r[:col_count] for r in rows]
            # Build markdown
            md_lines = [
                '| ' + ' | '.join(self._escape_md_cell(h) for h in header) + ' |',
                '| ' + ' | '.join(['---'] * col_count) + ' |'
            ]
            for r in norm_rows:
                md_lines.append('| ' + ' | '.join(self._escape_md_cell(c) for c in r) + ' |')
            return '\n'.join(md_lines)
        except Exception:
            return None

    def _clean_wikitext_cell(self, cell: str) -> str:
        # Remove style / scope / align fragments and leading parameters separated by '|'
        # Keep only the last part after style specs
        # Example: 'style="text-align:center;" 80 {{#expr: 10+5}}' -> '80 15'
        # Split by '|' only if looks like style tokens present
        if re.search(r'\bstyle=|scope=|align=', cell, flags=re.IGNORECASE):
            bits = [b.strip() for b in cell.split('|') if b.strip()]
            if bits:
                cell = bits[-1]
        # Evaluate expr if present
        if '{{#expr:' in cell.lower():
            cell = self._evaluate_expr_templates(cell)
        # Strip remaining curly braces for simple templates (non-nested) except math we already handle later
        cell = re.sub(r'\{\{([^{}]+)\}\}', lambda m: m.group(1).split('|')[-1], cell)
        return cell.strip()

    def _escape_md_cell(self, text_cell: str) -> str:
        return text_cell.replace('|', '\\|')

    def _convert_wiki_to_markdown_ctx(self, text: str, champion_name: Optional[str]) -> str:
        """Context-aware conversion: first resolve simple data templates with champion context, then run main converter."""
        if not text:
            return ""
        # Resolve simple data-bearing templates before formula conversion
        s = self._resolve_simple_data_templates(text, champion_name)
        # Expand select structural templates (Quote, Channel Type) prior to generic stripping
        s = self._expand_custom_templates(s, champion_name)
        return self._convert_wiki_to_markdown(s)

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
        # Try to capture the ability display name from the template header
        name_match = re.search(r'\|([^|\n]+)\|\{\{\{2\|', content)
        if name_match:
            ability_name = name_match.group(1).strip()
        else:
            # Fallback: prettify folder key
            ability_name = ability_key.replace('_', ' ').strip()
        
        # Extract page-level variables defined with #vardefine before parsing params
        vars_map = self._extract_vardefines(content)

        # Parse parameters robustly using top-level '|' separators (ignore nested templates)
        params = self._parse_template_params(content)

        # Parse ability data from template
        ability_data = {
            "name": ability_name,
            "skill": ability_key if ability_key in ['Q', 'W', 'E', 'R', 'I'] else '',
        }

        # Extract skill from content if not found
        if not ability_data['skill']:
            skill = params.get('skill')
            if skill and skill in 'IQWER':
                ability_data['skill'] = skill

        # Map of template keys to our fields
        key_map = {
            'blurb': 'blurb',
            'blurb2': 'blurb2',
            'blurb3': 'blurb3',
            'blurb4': 'blurb4',
            'description': 'description',
            'description2': 'description2',
            'description3': 'description3',
            'description4': 'description4',
            'description5': 'description5',
            'description6': 'description6',
            'target range': 'target_range',
            'range': 'range',
            'attack range': 'attack_range',
            'ai range': 'ai_range',
            'windup': 'windup',
            'collision radius': 'collision_radius',
            'cooldown': 'cooldown',
            'cooldown2': 'cooldown2',
            'cdstart': 'cdstart',
            'static': 'static',
            'cost': 'cost',
            'cost2': 'cost2',
            'costtype': 'costtype',
            'leveling': 'leveling',
            'leveling2': 'leveling2',
            'leveling3': 'leveling3',
            'leveling4': 'leveling4',
            'leveling5': 'leveling5',
            'leveling6': 'leveling6',
            'notes': 'notes',
            'recharge': 'recharge',
            'cast time': 'cast_time',
            'targeting': 'targeting',
            'affects': 'affects',
            'damagetype': 'damage_type',
            'speed': 'speed',
            'effect radius': 'effect_radius',
            'angle': 'angle',
            'inner radius': 'inner_radius',
            'tether radius': 'tether_radius',
            'width': 'width',
            'spellshield': 'spell_shield',
            'spelleffects': 'spell_effects',
            'outofrange': 'outofrange',
            'targetwarning': 'targetwarning',
            'terraingrace': 'terraingrace',
            'parry': 'parry',
            'projectile': 'projectile',
            'callforhelp': 'callforhelp',
            'grounded': 'grounded',
            'knockdown': 'knockdown',
            'silence': 'silence',
            'customlabel': 'customlabel',
            'custominfo': 'custominfo',
            'customlabel2': 'customlabel2',
            'custominfo2': 'custominfo2',
            'cdstart': 'cdstart',
            'static': 'static',
            'ontargetcdstatic': 'ontargetcdstatic',
            'ontargetcd': 'ontargetcd',
            'queue time': 'queue_time',
            'flavortext': 'flavortext',
            'flavorsound': 'flavorsound',
            'differences': 'differences',
            'tips': 'tips',
            'yvideo': 'yvideo',
            'yvideo2': 'yvideo2',
        }

        for src_key, dst_key in key_map.items():
            raw_val = params.get(src_key)
            if raw_val is None:
                continue
            # First, resolve any {{#var:...}} with values from #vardefine
            val = self._resolve_vars(raw_val.strip(), vars_map)
            if dst_key == 'notes':
                ability_data[dst_key] = self._format_notes(val, vars_map, champion_name)
            else:
                ability_data[dst_key] = self._convert_wiki_to_markdown_ctx(val, champion_name)

        # Collect any free-floating {{st|...}} blocks not tied to a parameter line
        # Resolve vars in full content before scanning for free st blocks
        content_resolved = self._resolve_vars(content, vars_map)
        # Pre-resolve simple data templates so nested st/ap/pp can evaluate better
        content_resolved = self._resolve_simple_data_templates(content_resolved, champion_name)
        extra_st = self._collect_free_st_blocks(content_resolved)
        if extra_st:
            # Render them to markdown tables now
            ability_data['extra_scaling'] = [self._render_st(inner) for inner in extra_st if inner.strip()]

        return ability_data

    def _parse_template_params(self, text: str) -> Dict[str, str]:
        """Parse top-level template parameters of the form '| key = value'.
        Uses brace-depth tracking to avoid splitting inside nested templates.
        Returns a dict with lowercase keys; spaces preserved in keys (e.g., 'cast time').
        """
        params: Dict[str, str] = {}
        key: Optional[str] = None
        val_lines: List[str] = []
        depth = 0
        def flush():
            nonlocal key, val_lines
            if key is not None:
                params[key] = '\n'.join(val_lines).strip()
            key = None
            val_lines = []
        for raw_line in text.splitlines():
            line = raw_line.rstrip('\r')
            stripped = line.lstrip()
            # Update depth counts BEFORE testing to reflect previous lines
            opens = len(re.findall(r'\{\{', line))
            closes = len(re.findall(r'\}\}', line))
            depth += opens - closes
            if depth < 0:
                depth = 0
            # Start of a new parameter at top-level of the main template (depth == 1)
            if depth == 1 and stripped.startswith('|'):
                flush()
                body = stripped[1:]
                if '=' in body:
                    k, v = body.split('=', 1)
                    key = k.strip().lower()
                    val_lines = [v.strip()]
                else:
                    key = body.strip().lower()
                    val_lines = []
            else:
                if key is not None:
                    val_lines.append(line)
        flush()
        return params

    def _collect_free_st_blocks(self, text: str) -> List[str]:
        """Collect brace-aware {{st|...}} blocks that appear outside key=value param lines.
        We'll scan all occurrences and then subtract those that clearly belong to known 'leveling*' param values.
        Returns list of inner strings for rendering.
        """
        inners: List[str] = []
        i = 0
        n = len(text)
        while i < n:
            j = text.find('{{st|', i)
            if j == -1:
                break
            # ensure not inside a param value line starting with '|'
            line_start = text.rfind('\n', 0, j) + 1
            if line_start < 0:
                line_start = 0
            if text[line_start:line_start+1] == '|':
                # belongs to a parameter; skip, handled via params
                # move past this '{{st|'
                i = j + 5
                continue
            # parse brace-aware until closing '}}'
            k = j + 5
            depth = 1
            buf: List[str] = []
            while k < n:
                if k + 1 < n and text[k] == '{' and text[k+1] == '{':
                    depth += 1
                    buf.append(text[k:k+2])
                    k += 2
                    continue
                if k + 1 < n and text[k] == '}' and text[k+1] == '}':
                    depth -= 1
                    if depth == 0:
                        k += 2
                        break
                    buf.append(text[k:k+2])
                    k += 2
                    continue
                buf.append(text[k])
                k += 1
            inner = ''.join(buf)
            if inner.startswith('st|'):
                inner = inner[3:]
            if inner:
                inners.append(inner)
            i = k
        return inners
    
    def _format_notes(self, notes_text: str, vars_map: Optional[Dict[str, str]] = None, champion_name: Optional[str] = None) -> str:
        """Format notes section preserving list structure and expanding select structural templates.
        Enhancements vs previous version:
          - Runs custom template expansion (Quote, Channel Type, Item Haste, ward icononly) inside notes so
            channel-type (ct) tables render properly instead of leaking raw template markup.
          - Treats block lines starting with '#### ' (headers) or '|' (markdown tables) as standalone lines
            rather than merging them into the preceding bullet.
        Supports MediaWiki list markers '*', '#', and mixed sequences like '*#' with multi-level indentation.
        The list type for each line is determined by the last marker in the prefix (e.g., '*#' -> ordered sublist).
        """
        if not notes_text:
            return "No additional notes."

        # Resolve variables if provided
        if vars_map:
            notes_text = self._resolve_vars(notes_text, vars_map)

        # EARLY: expand external info templates (cast time start/end, energized, spellblade, etc.)
        # before list parsing so any leading '*' inside their bodies becomes a proper bullet line.
        try:
            notes_text = self._expand_external_info_templates(notes_text)
        except Exception:
            pass

        # Run custom template expansion so that {{ct|...}} becomes a rendered block prior to list parsing
        try:
            notes_text = self._expand_custom_templates(notes_text, champion_name)
        except Exception:
            # Fail-safe: keep original notes_text if expansion raises
            pass

        # Rescue pass: detect inline ct parameter runs left in free text and normalize them to a small table
        try:
            notes_text = self._rescue_ct_param_runs(notes_text)
        except Exception:
            pass

        # Strip raw wiki template param lines (e.g., "|cooldown = ...") that leaked into notes
        cleaned_lines: List[str] = []
        for ln in notes_text.split('\n'):
            # Keep legitimate markdown tables (start with '|' but have many pipes)
            if ln.startswith('|') and ln.count('|') >= 2 and not re.match(r"^\|\s*[a-z][^=]{0,40}=", ln, flags=re.IGNORECASE):
                cleaned_lines.append(ln)
                continue
            # Drop bare wiki param lines
            if re.match(r"^\s*\|\s*[a-z][^=]{0,40}=", ln, flags=re.IGNORECASE):
                continue
            cleaned_lines.append(ln)
        notes_text = '\n'.join(cleaned_lines)

        out_lines: List[str] = []
        last_kind: str = 'none'  # 'bullet' | 'block' | 'blank' | 'heading'
        in_block_sequence = False
        for raw in notes_text.split('\n'):
            s = raw.rstrip('\r')
            if not s.strip():
                # Blank line resets block sequence but we don't emit multiple blanks
                if out_lines and out_lines[-1] != '':
                    out_lines.append('')
                last_kind = 'blank'
                in_block_sequence = False
                continue
            is_block_line = (
                s.startswith('#### ') or
                s.startswith('| Aspect ') or
                (s.startswith('|') and '|' in s and s.count('|') >= 2)
            )
            if is_block_line:
                if s.startswith('#### '):
                    # Heading: ensure exactly one blank line before if previous was bullet (handled by previous logic)
                    if last_kind == 'bullet' and out_lines and out_lines[-1] != '':
                        out_lines.append('')
                    out_lines.append(s)
                    # Always ensure a blank line AFTER heading unless next added line is already blank
                    if not (out_lines and out_lines[-1] == ''):
                        out_lines.append('')
                    last_kind = 'heading'
                    in_block_sequence = False
                else:
                    # Table or other block line
                    # If transitioning from bullet list (not already in block sequence) insert a single blank line
                    if last_kind in ('bullet', 'heading') and out_lines and out_lines[-1] != '':
                        # heading already added a blank line, so only add if prior not blank
                        out_lines.append('')
                    # Avoid accumulating multiple blank lines before table
                    if out_lines and out_lines[-1] == '' and len(out_lines) >= 2 and out_lines[-2] == '':
                        out_lines.pop()  # remove extra blank
                    out_lines.append(s)
                    last_kind = 'block'
                    in_block_sequence = True
                continue
            # Non-block line: treat as bullet or continuation
            m = re.match(r'^(?P<prefix>[\*\#]+)\s+(?P<body>.*)$', s)
            if m:
                prefix = m.group('prefix')
                body = m.group('body').strip()
                if not body:
                    continue
                level = len(prefix)
                marker = prefix[-1]
                indent = '  ' * (level - 1)
                bullet = '- ' if marker == '*' else '1. '
                item_md = self._convert_wiki_to_markdown_ctx(body, champion_name)
                out_lines.append(f"{indent}{bullet}{item_md}")
                last_kind = 'bullet'
                in_block_sequence = False
            else:
                # Continuation of previous bullet only if last was bullet
                cont = self._convert_wiki_to_markdown_ctx(s.strip(), champion_name)
                if last_kind == 'bullet' and out_lines and cont:
                    out_lines[-1] += f" {cont}"
                else:
                    # Standalone text line (treat like bullet but no marker)
                    out_lines.append(self._convert_wiki_to_markdown_ctx(s.strip(), champion_name))
                    last_kind = 'bullet'
                    in_block_sequence = False

        # Collapse any accidental double blanks (defensive)
        cleaned: List[str] = []
        prev_blank = False
        for ln in out_lines:
            if not ln.strip():
                if not prev_blank:
                    cleaned.append('')
                prev_blank = True
            else:
                cleaned.append(ln)
                prev_blank = False
        # Drop trailing blank line(s)
        while cleaned and cleaned[-1] == '':
            cleaned.pop()
        # Remove single blank lines between consecutive bullet lines (tighten list) while preserving
        # intentional blank lines before blocks/headings.
        tightened: List[str] = []
        for i, ln in enumerate(cleaned):
            if ln == '' and i > 0 and i + 1 < len(cleaned):
                prev_ln = cleaned[i-1]
                next_ln = cleaned[i+1]
                if prev_ln.startswith('- ') and next_ln.startswith('- '):
                    # skip this blank to keep bullets contiguous
                    continue
            tightened.append(ln)
        return '\n'.join([ln for ln in tightened if ln is not None]) if tightened else "No additional notes."

    def _rescue_ct_param_runs(self, text: str) -> str:
        """Detect sequences like '|items=false|consume=false|spells=true,...' outside ct templates
        and convert them into a compact table using the same rules as ct expansion.
        We conservatively trigger only when two or more ct-keys appear in a single run.
        """
        if not text or '|' not in text:
            return text
        ct_keys = ["attack","move","cast","items","spells","consume","interrupts","damage"]
        # Extract discrete segments like (\|)?key = value up to next pipe/newline
        seg_re = re.compile(r"(\\?\|)\s*(?:" + '|'.join(ct_keys) + r")\s*=\s*([^|\n]+)", re.IGNORECASE)
        lines = text.split('\n')
        out: List[str] = []
        for ln in lines:
            segs = []
            for m in seg_re.finditer(ln):
                # Normalize leading pipe to a real '|'
                val = m.group(2).strip()
                # Trim stray trailing backslashes (escaped pipes from earlier passes)
                val = re.sub(r"\\+$", "", val)
                key_match = re.search(r"(attack|move|cast|items|spells|consume|interrupts|damage)", m.group(0), re.IGNORECASE)
                if not key_match:
                    continue
                key = key_match.group(1)
                segs.append(f"|{key}={val}")
            if len(segs) >= 2:
                inner = 'ct|unknown' + ''.join(segs)
                rendered = self._expand_channel_type_templates('{{' + inner + '}}')
                if rendered and rendered != '{{' + inner + '}}':
                    out.append(rendered)
                    continue
            out.append(ln)
        return '\n'.join(out)

    def _extract_vardefines(self, text: str) -> Dict[str, str]:
        """Extract {{#vardefine:name|value}} variables from a template page content.
        Returns a mapping of variable name -> raw value string.
        """
        if not text:
            return {}
        # Remove HTML comments to avoid false positives inside comments
        cleaned = re.sub(r'<!--[\s\S]*?-->', '', text)
        vars_map: Dict[str, str] = {}
        # Pattern: {{#vardefine:var|value}}
        for m in re.finditer(r"\{\{\s*#vardefine\s*:\s*([^|}]+)\|([^}]*)\}\}", cleaned, flags=re.IGNORECASE):
            name = m.group(1).strip()
            value = m.group(2).strip()
            if name:
                # Strip enclosing quotes if any and whitespace
                value = value.strip().strip('"')
                vars_map[name] = value
        return vars_map

    def _resolve_vars(self, text: str, vars_map: Optional[Dict[str, str]]) -> str:
        """Replace {{#var:name}} with the corresponding value from vars_map."""
        if not text or not vars_map:
            return text
        def repl(m: re.Match) -> str:
            name = m.group(1).strip()
            return vars_map.get(name, m.group(0))
        return re.sub(r"\{\{\s*#var\s*:\s*([^}|]+)\s*\}\}", repl, text, flags=re.IGNORECASE)
    
    def _extract_trivia(self, trivia_content: str) -> List[str]:
        """Extract trivia items, preserving nested bullets and ordered sublists.
        Supports MediaWiki prefixes composed of '*' (unordered) and '#' (ordered),
        including mixed cases like '*#' which indicate an ordered sublist nested under an unordered item.
        """
        trivia: List[str] = []
        for raw in trivia_content.split('\n'):
            line = raw.rstrip()
            if not line.strip():
                continue
            m = re.match(r'^(?P<prefix>[\*\#]+)\s+(?P<body>.*)$', line)
            if not m:
                continue
            prefix = m.group('prefix')
            body = m.group('body').strip()
            if not body:
                continue
            level = len(prefix)
            marker = prefix[-1]  # decide list type by the last marker (handles mixed '*#')
            indent = '  ' * (level - 1)
            bullet = '- ' if marker == '*' else '1. '
            item_md = self._convert_wiki_to_markdown(body)
            trivia.append(f"{indent}{bullet}{item_md}")
        return trivia
    
    def _extract_patch_history(self, champion_name: str) -> List[Dict[str, Any]]:
        """Extract patch history."""
        patch_file = self.wiki_root / "Main" / champion_name / "Patch_history" / "page.txt"
        
        if not patch_file.exists():
            return []
        
        with open(patch_file, 'r', encoding='utf-8') as f:
            content = f.read()
        
        patches = []
        
        # Find version sections (allow non-digit sublabels e.g., V25.S1.3)
        version_pattern = r';?\[\[([^\]]+)\]\]'
        sections = re.split(version_pattern, content)
        
        for i in range(1, len(sections), 2):
            if i + 1 < len(sections):
                version = sections[i]
                changes_text = sections[i + 1].strip()
                
                changes: List[str] = []
                for raw in changes_text.split('\n'):
                    if not raw.strip():
                        continue
                    m = re.match(r'^(\*+)\s*(.*)$', raw.rstrip())
                    if not m:
                        continue
                    level = len(m.group(1))
                    line_content = m.group(2).strip()
                    if not line_content:
                        continue
                    content_md = self._convert_wiki_to_markdown(line_content)
                    indent = '  ' * (level - 1)
                    changes.append(f"{indent}- {content_md}")
                
                if changes:
                    patches.append({
                        "version": version,
                        "changes": changes
                    })
        
        return patches[:10]  # Limit to recent patches

    def _parse_patch_history_text(self, text: str) -> List[Dict[str, Any]]:
        """Parse a patch history block that uses ;[[V...]] headers and * bullets into structured data."""
        version_pattern = r';?\[\[([^\]]+)\]\]'
        sections = re.split(version_pattern, text)
        patches: List[Dict[str, Any]] = []
        for i in range(1, len(sections), 2):
            if i + 1 < len(sections):
                version = sections[i]
                changes_text = sections[i + 1].strip()
                changes: List[str] = []
                for raw in changes_text.split('\n'):
                    if not raw.strip():
                        continue
                    m = re.match(r'^(\*+)\s*(.*)$', raw.rstrip())
                    if not m:
                        continue
                    level = len(m.group(1))
                    content = m.group(2).strip()
                    if not content:
                        continue
                    content_md = self._convert_wiki_to_markdown(content)
                    indent = '  ' * (level - 1)
                    changes.append(f"{indent}- {content_md}")
                if changes:
                    patches.append({"version": version, "changes": changes})
        return patches
    
    def _convert_ft_blocks(self, s: str) -> str:
        """Convert FlipText templates to a static inline swap marker.
        Handles {{ft|text1|text2}} and {{FlipText|text1|text2}} with brace-aware parsing.
        Output format (always inline): 「 text1 ⟷ text2 」
        """
        out: List[str] = []
        i = 0
        n = len(s)
        while i < n:
            j = s.find('{{', i)
            if j == -1:
                out.append(s[i:])
                break
            out.append(s[i:j])
            # Attempt to read template name
            k = j + 2
            while k < n and s[k].isspace():
                k += 1
            name_start = k
            while k < n and s[k] not in '|}':
                k += 1
            name_raw = s[name_start:k].strip()
            name_norm = name_raw.lower().replace('_', ' ').strip()
            # Walk forward to find balanced end of this template
            t = j
            depth = 0
            while t < n - 1:
                if s[t:t+2] == '{{':
                    depth += 1
                    t += 2
                    continue
                if s[t:t+2] == '}}':
                    depth -= 1
                    t += 2
                    if depth == 0:
                        break
                    continue
                t += 1
            tpl = s[j:t] if t <= n else s[j:]
            if name_norm in ("ft", "fliptext", "flip text"):
                # Extract inner content after the name and first '|'
                inner = tpl[2:-2]
                rest = inner[len(name_raw):].lstrip()
                if rest.startswith('|'):
                    rest = rest[1:]
                # Split on top-level pipes to get args; ignore named params
                parts = self._split_top_level_pipes(rest)
                pos_args = [p.strip() for p in parts if p.strip() and '=' not in p]
                a = pos_args[0] if len(pos_args) >= 1 else ''
                b = pos_args[1] if len(pos_args) >= 2 else ''
                a_md = self._convert_wiki_to_markdown(a) if a else ''
                b_md = self._convert_wiki_to_markdown(b) if b else ''
                out.append(f"「 {a_md} ⟷ {b_md} 」")
                i = t
            else:
                # Not FlipText: keep template as-is for other passes
                out.append(tpl)
                i = t
        return ''.join(out)
    
    def _convert_wiki_to_markdown(self, text: str) -> str:
        """Convert basic MediaWiki syntax to markdown."""
        if not text:
            return ""

        # Helper: apply English possessive rules with typographic apostrophe
        # - If the last alphabetic character is 's' or 'S', append ’ (s-only plural possessive)
        # - Otherwise append ’s
        def _last_alpha(s: str) -> str:
            for ch in reversed(s):
                if ch.isalpha():
                    return ch
            return ''
        # Expose as an instance method for reuse in other handlers
        # Note: define on self dynamically if not already present (for single-file simplicity)
        if not hasattr(self, '_make_possessive'):
            def _make_possessive_impl(label: str) -> str:
                base = label or ''
                if not base:
                    return ''
                last = _last_alpha(base)
                if last.lower() == 's':
                    return base + '’'
                return base + '’s'
            setattr(self, '_make_possessive', _make_possessive_impl)

        # Evaluate simple arithmetic: {{#expr: ... }} -> computed value
        # This is done early so nested uses like {{as|{{#expr:175+40}}% AD}} resolve correctly
        text = self._evaluate_expr_templates(text)
        # Expand selected external info templates (e.g., Spellblade info, Energized info, etc.)
        # by loading their extracted includeonly content from the template dump directory.
        # We do this early so that list markers (*) are processed by list conversion later
        # and nested templates (tip, sbc, fd, etc.) are still available for downstream handlers.
        text = self._expand_external_info_templates(text)

        # NOTE: Do NOT try to handle bold/italic with naive regex; we run a robust apostrophe parser later.

        # Unwrap common icon/link templates to plain text only (no styling), preserving possessive for specific variants.
        # This pass is brace-aware and runs early so later specialized handlers won't restyle the content.
        def _unwrap_icon_like_templates(s: str) -> str:
            names_plain = {
                # champion/item/rune/unit/skill wrappers
                'ci', 'ii', 'iis', 'ri', 'ui', 'uis', 'si', 'sti', 'stil', 'wi', 'wri',
                # ability wrappers
                'ai',
                # misc link/icon wrappers
                'cid', 'csl', 'items', 'tft item', 'tip',
                # mastery icons common variants
                'mi', 'mi1', 'mi2', 'mi3', 'mi4', 'mi5', 'mi6', 'mi7',
            }
            names_possessive = {'cis', 'ais', 'cais', 'ris', 'sis'}
            # Process by scanning for '{{' and then extract balanced template when name matches
            out: List[str] = []
            i = 0
            n = len(s)
            while i < n:
                j = s.find('{{', i)
                if j == -1:
                    out.append(s[i:])
                    break
                out.append(s[i:j])
                # Try to read the template name token
                k = j + 2
                # Skip whitespace
                while k < n and s[k].isspace():
                    k += 1
                # Read name up to '|' or '}}'
                name_start = k
                while k < n and s[k] not in '|}':
                    k += 1
                name_raw = s[name_start:k].strip()
                name_lc = name_raw.lower()
                # Now extract balanced braces to get full template
                # Walk from j to matching '}}'
                t = j
                depth = 0
                while t < n - 1:
                    if s[t:t+2] == '{{':
                        depth += 1
                        t += 2
                        continue
                    if s[t:t+2] == '}}':
                        depth -= 1
                        t += 2
                        if depth == 0:
                            break
                        continue
                    t += 1
                tpl = s[j:t] if t <= n else s[j:]
                # Determine if this template should be unwrapped to plain text
                is_target = (name_lc in names_plain or name_lc in names_possessive)
                # Heuristic fallback: short icon-like wrappers (e.g., ci/ii/ri/ui/si/wi/sti/stil/mi1..), including mi with digits
                if not is_target:
                    if name_lc != 'bi' and (re.match(r'^(?:[a-z]{1,4}i|[a-z]{1,4}is)$', name_lc) or re.match(r'^mi\d+$', name_lc)):
                        is_target = True
                if not is_target:
                    out.append(tpl)
                    i = t
                    continue
                # Extract inner (drop '{{' and '}}')
                inner = tpl[2:-2]
                # Remove name and leading pipe if present
                rest = inner[len(name_raw):].lstrip()
                if rest.startswith('|'):
                    rest = rest[1:]
                # Split on top-level pipes to get args; ignore key=value args
                parts = self._split_top_level_pipes(rest)
                # Determine positional args vs named using top-level '=' only (ignore '=' inside nested templates)
                pos_args: List[str] = []
                for seg in parts:
                    seg_s = seg.strip()
                    if not seg_s:
                        continue
                    depth_eq = 0
                    has_top_eq = False
                    i2 = 0
                    L2 = len(seg_s)
                    while i2 < L2:
                        if i2 + 1 < L2 and seg_s[i2] == '{' and seg_s[i2+1] == '{':
                            depth_eq += 1
                            i2 += 2
                            continue
                        if i2 + 1 < L2 and seg_s[i2] == '}' and seg_s[i2+1] == '}':
                            depth_eq = max(0, depth_eq - 1)
                            i2 += 2
                            continue
                        if seg_s[i2] == '=' and depth_eq == 0:
                            has_top_eq = True
                            break
                        i2 += 1
                    if not has_top_eq:
                        pos_args.append(seg_s)
                # Special rule: champion ability link -> Champion’s Ability
                if name_lc == 'cai':
                    cur = getattr(self, '_current_page_basename', None)
                    ability = pos_args[0] if len(pos_args) >= 1 else ''
                    champion = pos_args[1] if len(pos_args) >= 2 else ''
                    display = pos_args[2] if len(pos_args) >= 3 and pos_args[2] else ability
                    champ_md = self._convert_wiki_to_markdown_ctx(champion, cur) if champion else ''
                    abil_md = self._convert_wiki_to_markdown_ctx(display, cur) if display else ''
                    # Build possessive champion label + ability label
                    possessive_champ = self._make_possessive(champ_md) if champ_md else ''
                    # Avoid stray leading/trailing spaces if any part is empty
                    combined = f"{possessive_champ} {abil_md}".strip()
                    out.append(combined)
                    i = t
                    continue
                # Special rule: sti with exactly two unnamed args -> (first) second
                if name_lc == 'sti' and len(pos_args) == 2:
                    left = self._convert_wiki_to_markdown_ctx(pos_args[0], getattr(self, '_current_page_basename', None)) if pos_args[0] else ''
                    right = self._convert_wiki_to_markdown_ctx(pos_args[1], getattr(self, '_current_page_basename', None)) if pos_args[1] else ''
                    out.append(f"({left}) {right}")
                    i = t
                    continue
                display = ''
                if pos_args:
                    if name_lc in ('ai', 'ais', 'cais'):
                        # {{ai|Ability|Champion|Display}} -> prefer 3rd arg (display) else 1st (ability)
                        display = (pos_args[2] if len(pos_args) >= 3 and pos_args[2] else pos_args[0])
                    elif name_lc == 'tip':
                        # {{tip|Subject|Display|...}} -> prefer 2nd positional (display) else 1st (subject)
                        display = (pos_args[1] if len(pos_args) >= 2 and pos_args[1] else pos_args[0])
                    else:
                        # For others (ci/ii/ri/ui/si/sti/stil/wi/wri/cai/cid/csl/items/mi*), prefer first arg
                        display = pos_args[0]
                # Recursively convert the extracted display text (so nested templates like sbc still apply)
                disp_md = self._convert_wiki_to_markdown(display) if display else ''
                if name_lc in names_possessive:
                    disp_md = self._make_possessive(disp_md) if disp_md else ''
                out.append(disp_md)
                i = t
            return ''.join(out)

        text = _unwrap_icon_like_templates(text)

        # Convert MediaWiki lists (*, **, ***, #, ##, ...) to Markdown nested lists.
        # Perform this BEFORE bold/italic conversions so that leading "** " isn't mistaken for bold.
        def _mw_list_repl(m: re.Match) -> str:
            prefix = m.group('prefix')
            level = len(prefix)
            # Use the LAST marker to determine the list type for this line.
            # This correctly maps mixed prefixes like '*#' to an ordered sublist.
            marker = prefix[-1]
            indent = '  ' * (level - 1)
            if marker == '*':
                return f"{indent}- "
            else:
                return f"{indent}1. "
        text = re.sub(r'(?m)^(?P<prefix>[\*\#]+)\s+', _mw_list_repl, text)

        # Convert formulas with better handling
        text = re.sub(r'\{\{ap\|([^}]+)\}\}', self._convert_ap_formula, text)
        text = re.sub(r'\{\{pp\|([^}]+)\}\}', self._convert_pp_formula, text)
        text = re.sub(r'\{\{pptooltip\|([^}]+)\}\}', self._convert_pptooltip, text)
        text = re.sub(r'\{\{fd\|([^}]+)\}\}', r'$\1$', text)  # Fixed decimals
        # Tooltips: {{tt|value|tooltip}} -> value (tooltip)
        text = re.sub(r'\{\{tt\|([^}]+)\}\}', self._convert_tt, text)

        # Convert styled text
        # Convert {{as|...}} (styled inline content) using a brace-aware parser to avoid breaking inner pipes
        def _convert_as_blocks(s: str) -> str:
            out: List[str] = []
            i = 0
            n = len(s)
            while i < n:
                j = s.find('{{as|', i)
                if j == -1:
                    out.append(s[i:])
                    break
                out.append(s[i:j])
                k = j + 5
                depth = 1
                buf: List[str] = []
                while k < n:
                    if k + 1 < n and s[k] == '{' and s[k+1] == '{':
                        depth += 1
                        buf.append(s[k:k+2])
                        k += 2
                        continue
                    if k + 1 < n and s[k] == '}' and s[k+1] == '}':
                        depth -= 1
                        if depth == 0:
                            k += 2
                            break
                        buf.append(s[k:k+2])
                        k += 2
                        continue
                    buf.append(s[k])
                    k += 1
                inner = ''.join(buf)
                # inner may contain parameters separated by top-level '|', we only need the first arg for display
                parts = self._split_top_level_pipes(inner)
                content = parts[0] if parts else ''
                out.append(self._convert_wiki_to_markdown(content))
                i = k
            return ''.join(out)
        text = _convert_as_blocks(text)
        # FlipText: {{ft|text1|text2}} -> 「 text1 ⟷ text2 」 (brace-aware)
        text = self._convert_ft_blocks(text)
        # Skill tabs: {{st|Label1|Value1|Label2|Value2|...}} -> table (brace-aware)
        text = self._convert_st_blocks(text)
        # Small bold caps: brace-aware, allow nested templates and wiki italics.
        # We recursively convert the inner content, strip inline italics to avoid
        # mixing '*' with outer bold, then uppercase and wrap in **...**.
        def _convert_sbc_blocks(s: str) -> str:
            out: List[str] = []
            i = 0
            n = len(s)
            while i < n:
                j = s.find('{{sbc|', i)
                if j == -1:
                    out.append(s[i:])
                    break
                out.append(s[i:j])
                k = j + 6  # position after '{{sbc|'
                depth = 1
                buf: List[str] = []
                while k < n:
                    if k + 1 < n and s[k] == '{' and s[k+1] == '{':
                        depth += 1
                        buf.append(s[k:k+2])
                        k += 2
                        continue
                    if k + 1 < n and s[k] == '}' and s[k+1] == '}':
                        depth -= 1
                        if depth == 0:
                            k += 2
                            break
                        buf.append(s[k:k+2])
                        k += 2
                        continue
                    buf.append(s[k])
                    k += 1
                inner = ''.join(buf)
                # inner starts with the remainder after 'sbc|', may contain pipes for other args
                parts = self._split_top_level_pipes(inner)
                raw_disp = parts[0] if parts else ''
                # Recursively convert nested templates and wiki markup within the sbc content
                disp_md = self._convert_wiki_to_markdown(raw_disp) if raw_disp else ''
                # Remove inline italics/bold markers inside sbc content to avoid '**' + '*' conflicts
                # Keep math ('$') and links intact.
                if disp_md:
                    disp_md = disp_md.replace('*', '')
                out.append(f"**{(disp_md or '').upper()}**")
                i = k
            return ''.join(out)
        text = _convert_sbc_blocks(text)

        # Fix double bold markers
        text = re.sub(r'\*\*\*\*([^*]+):\*\*', r'**\1:**', text)  # ****text:** -> **text:**

        # Convert section links BEFORE general links so they don't get captured by the general rule
        # [[Page#Anchor|Display]] -> [Display](./Page.md#Anchor)
        text = re.sub(r'\[\[([^|\]#]+)#([^|\]]+)(?:\|([^\]]*))?\]\]', self._convert_section_link, text)
        # Convert links with better handling (general case)
        text = re.sub(r'\[\[([^|\]]+)(?:\|([^]]*))?\]\]', self._convert_link, text)

        # Convert champion/ability references
        # Note: icon/link wrappers (ci/cis/ai/ais/cais/ri/ii/etc.) are handled earlier by
        # the brace-aware _unwrap_icon_like_templates, which outputs plain text and
        # adds possessive suffixes without italics/bold. Avoid reintroducing emphasis here.
        # bi: buff information; keep the most relevant text (prefer last non-empty arg, else first)
        def _bi_repl(m: re.Match) -> str:
            full = m.group(0)
            inner = full[2:-2]
            parts = inner.split('|')
            args = [a.strip() for a in parts[1:] if a.strip()]
            if not args:
                return ''
            # Heuristic: if multiple args, last is often the label/quoted text
            return args[-1]
        text = re.sub(r'\{\{bi\|[^}]+\}\}', _bi_repl, text)
        # ui: unit information; keep the primary text (first arg)
        def _ui_repl(m: re.Match) -> str:
            full = m.group(0)
            inner = full[2:-2]
            parts = inner.split('|')
            arg = parts[1].strip() if len(parts) > 1 else ''
            return arg
        text = re.sub(r'\{\{ui\|[^}]+\}\}', _ui_repl, text)
        text = re.sub(r'\{\{tip\|([^}|]+)(?:\|[^}]*)?\}\}', r'\1', text)

        # High-frequency templates from audit: provide safe text-only mappings
        # LoR card/link templates -> keep display text
        text = re.sub(r'\{\{tiplor\|([^}|]+)(?:\|[^}]*)?\}\}', r'\1', text, flags=re.IGNORECASE)
        text = re.sub(r'\{\{lor\|([^}|]+)(?:\|[^}]*)?\}\}', r'\1', text, flags=re.IGNORECASE)
        # Wild Rift / TFT tooltips -> keep display text
        text = re.sub(r'\{\{tiptft\|([^}|]+)(?:\|[^}]*)?\}\}', r'\1', text, flags=re.IGNORECASE)
        text = re.sub(r'\{\{wrtip\|([^}|]+)(?:\|[^}]*)?\}\}', r'\1', text, flags=re.IGNORECASE)
        # Styled/special links/icons -> prefer plain text
        text = re.sub(r'\{\{csl\|([^}|]+)(?:\|[^}]*)?\}\}', r'\1', text, flags=re.IGNORECASE)
        text = re.sub(r'\{\{si\|([^}|]+)(?:\|[^}]*)?\}\}', r'\1', text, flags=re.IGNORECASE)
        text = re.sub(r'\{\{cid\|([^}|]+)(?:\|[^}]*)?\}\}', r'\1', text, flags=re.IGNORECASE)
        # tip template: keep display name when present (2nd positional), otherwise subject (1st positional); drop named params
        def _tip_fallback(m: re.Match) -> str:
            inner = m.group(1)
            parts = [p.strip() for p in inner.split('|')]
            # parts[0] is 'tip'
            args = [p for p in parts[1:] if p and '=' not in p]
            if not args:
                return ''
            disp = args[1] if len(args) >= 2 and args[1] else args[0]
            return disp
        text = re.sub(r'\{\{tip\|([^}]+)\}\}', _tip_fallback, text, flags=re.IGNORECASE)
        # Items and styled-italic variants are handled upstream; do not italicize here.
        # Gold-related templates -> append unit
        text = re.sub(r'\{\{g\|([^}|]+)(?:\|[^}]*)?\}\}', r'\1 gold', text, flags=re.IGNORECASE)
        text = re.sub(r'\{\{\s*g\s*\}\}', 'gold', text, flags=re.IGNORECASE)
        text = re.sub(r'\{\{gold value\|([^}|]+)(?:\|[^}]*)?\}\}', r'\1 gold', text, flags=re.IGNORECASE)
        # Symbol helpers
        text = re.sub(r'\{\{times(?:\|[^}]*)?\}\}', '×', text, flags=re.IGNORECASE)
        text = re.sub(r'\{\{degree(?:\|[^}]*)?\}\}', '°', text, flags=re.IGNORECASE)
        text = re.sub(r'\{\{plus(?:\|[^}]*)?\}\}', '+', text, flags=re.IGNORECASE)
        text = re.sub(r'\{\{tftt\}\}', 'Teamfight Tactics', text, flags=re.IGNORECASE)
        # Mastery icon labels handled upstream; no italics here.
        # Structural/maintenance templates -> drop
        text = re.sub(r'\{\{(?:references|lol navigation|champions|champion categories|doc|fairuse|section top)\b[^}]*\}\}', '', text, flags=re.IGNORECASE)
        text = re.sub(r'\{\{(?:rune header|rune footer)\b[^}]*\}\}', '', text, flags=re.IGNORECASE)
        text = re.sub(r'\{\{(?:game banner|patch box)\b[^}]*\}\}', '', text, flags=re.IGNORECASE)
        text = re.sub(r'\{\{!\}\}', '', text)  # table/format helper
        # Scribunto/cargo variables or invocations -> drop
        text = re.sub(r'\{\{\s*#var:[^}]+\}\}', '', text, flags=re.IGNORECASE)
        text = re.sub(r'\{\{\s*#invoke:[^}]+\}\}', '', text, flags=re.IGNORECASE)
        # Numeric-only templates frequently used as layout helpers -> drop
        text = re.sub(r'\{\{\s*\d+\s*\}\}', '', text)
        # Misc pass-through (keep primary text)
        text = re.sub(r'\{\{(?:rd|nie|spells|recurring)\|([^}|]+)(?:\|[^}]*)?\}\}', r'\1', text, flags=re.IGNORECASE)
        # Specific label template seen in audit
        text = re.sub(r'\{\{\s*effect at cast time end\s*\}\}', 'Effect at cast time end', text, flags=re.IGNORECASE)
        text = re.sub(r'\{\{\s*effect at cast time start\s*\}\}', 'Effect at cast time start', text, flags=re.IGNORECASE)
        # Champion info/infobox occurrences in body -> drop
        text = re.sub(r'\{\{\s*champion info\b[^}]*\}\}', '', text, flags=re.IGNORECASE)
        # Champion color/style wrappers: keep content
        text = re.sub(r'\{\{cc[dsib]?\|([^}|]+)(?:\|[^}]*)?\}\}', r'\1', text, flags=re.IGNORECASE)
        # TFT helper templates (icons/names/categories) -> keep primary text
        text = re.sub(r'\{\{tft[inc]?\|([^}|]+)(?:\|[^}]*)?\}\}', r'\1', text, flags=re.IGNORECASE)
        # Wild Rift wrappers -> keep content plain (wr handled above already)
        text = re.sub(r'\{\{wr\|([^}|]+)(?:\|[^}]*)?\}\}', r'\1', text, flags=re.IGNORECASE)
        # Unit/item plural wrappers handled upstream; no italics here.
        # Generic wrappers: keep or drop
        text = re.sub(r'\{\{(?:builds|grouped ability|map changes|recipe/item|recipe|link|text)\|([^}|]+)(?:\|[^}]*)?\}\}', r'\1', text, flags=re.IGNORECASE)
        text = re.sub(r'\{\{(?:icononly|image|clear|width|alttext|documentation|border|class|iconclass|iconstyle|labelclass|labelstyle|style|display|label|height|pagename|variant|nolink)\b[^}]*\}\}', '', text, flags=re.IGNORECASE)
        # Simple keyword templates
        text = re.sub(r'\{\{\s*adaptive\s*\}\}', 'adaptive', text, flags=re.IGNORECASE)
        text = re.sub(r'\{\{\s*critical damage\s*\}\}', 'critical damage', text, flags=re.IGNORECASE)
        text = re.sub(r'\{\{\s*equals\s*\}\}', '=', text, flags=re.IGNORECASE)
        text = re.sub(r'\{\{\s*separator\s*\}\}', '•', text, flags=re.IGNORECASE)

        # Handle special formatting
        text = re.sub(r'\{\{w\|([^}|]+)(?:\|[^}]*)?\}\}', r'\1', text)  # Wikipedia links

        # Handle stock and other gameplay terms
        text = re.sub(r'\[\[stock\]\]', 'stock', text)
        text = re.sub(r'\[\[basic attack\]\]', 'basic attack', text)

        # Handle file/image references
        text = re.sub(r'\[\[File:[^\]]+\]\]', '', text)  # Remove file references
        text = re.sub(r'!\[[^\]]*\]\([^)]*\)', '', text)  # Remove broken image markdown
        text = re.sub(r'\{\{sm2\|[^}]+\}\}', '', text)  # Remove sound files

        # Handle bug markers and special notations
        text = re.sub(r'\{\{bug\}\}', '', text)
        text = re.sub(r'\{\{bug\|[^}]+\}\}', '', text)
        # Champion without ability power ratio -> hard-coded text
        text = re.sub(r'\{\{\s*Champion without ability power ratio\s*\|[^}]*\}\}', 'This champion has no ability power ratio.', text)
        text = re.sub(r'\{\{\s*Champion without ability power ratio\s*\}\}', 'This champion has no ability power ratio.', text)

        # Remove references and HTML comments
        text = re.sub(r'<ref[^>]*>.*?</ref>', '', text, flags=re.DOTALL)
        text = re.sub(r'<ref[^>]*\s*/>', '', text)
        text = re.sub(r'<!--([\s\S]*?)-->', '', text)

        # Clean up HTML tags
        text = re.sub(r'<br\s*/?>', '\n', text)
        text = re.sub(r'<[^>]+>', '', text)

        # Convert MediaWiki apostrophe styles ('' italic, ''' bold, ''''' both) with a robust parser
        text = self._convert_apostrophe_styles(text)

        # Rely on the robust apostrophe-run parser; avoid any bold/italic cleanup here.

        # Clean up remaining templates
        text = re.sub(r'\{\{[^}]+\}\}', '', text)
        # Replace digit-separated pipes with slashes (e.g., 120|105|90 -> 120/105/90)
        text = re.sub(r'(?<=\d)\|(?=\d)', '/', text)
        # Remove any stray template braces left over
        text = text.replace('{{', '').replace('}}', '')
        # Do not drop '|' globally; needed for markdown tables generated from templates
        # Strip leftover short template prefixes that may leak mid-sentence (safe, space-bounded)
        text = re.sub(r'(?<=\s)(?:si|ui|ais|ci|ai|ri|ii)[a-z]{0,2}(?=\s)', '', text)
        # Fix common pluralization glitches after link conversion
        text = re.sub(r"\[stock\]\(\./stock\.md\)s", 'stocks', text)
        text = re.sub(r"\[basic attack\]\(\./basic_attack\.md\)s", 'basic attacks', text)
        # Convert external links of the form [http://url Text] to markdown
        text = re.sub(r'\[(https?://[^\s\]]+)\s+([^\]]+)\]', r'[\2](\1)', text)
        # Remove stray short template tokens left after brace removal (common: 'iis' artifacts)
        text = re.sub(r'\biis(?=\*)', '', text)
        # Normalize common patch note labels
        text = re.sub(r'\*\*\s*Bug\s*Fix:\*', '**Bug Fix:**', text, flags=re.IGNORECASE)
        text = re.sub(r'\*\*\s*Undocumented:\*', '**Undocumented:**', text, flags=re.IGNORECASE)
        text = re.sub(r'\*\*\s*New\s*Effect:\*', '**New Effect:**', text, flags=re.IGNORECASE)
        # Normalize a few more frequent labels
        text = re.sub(r'\*\*\s*Bug\s*Fixes:\*', '**Bug Fixes:**', text, flags=re.IGNORECASE)
        text = re.sub(r'\*\*\s*New:\*', '**New:**', text, flags=re.IGNORECASE)
        text = re.sub(r'\*\*\s*Removed:\*', '**Removed:**', text, flags=re.IGNORECASE)
        text = re.sub(r'\*\*\s*Changed:\*', '**Changed:**', text, flags=re.IGNORECASE)
        # (Removed legacy bullet splitting for cast time notes; handled earlier by expanding templates before list parsing.)

        # Clean up extra whitespace while preserving indentation at line starts (for nested lists)
        # Collapse 2+ spaces that are NOT immediately after a newline into a single space
        text = re.sub(r'(?m)(?<!\n)[ \t]{2,}', ' ', text)
        text = text.strip()
        # Strip stray equals signs at line ends (artifact of template params)
        text = re.sub(r'(?m)=\s*$', '', text)

        # Final pass: evaluate any {{#expr:...}} that may have become evaluable after prior substitutions
        # Remove stray leading single quote left at start of lines (from mismatched wiki italics)
        text = re.sub(r"(?m)^'(?=[A-Za-z])", '', text)

        text = self._evaluate_expr_templates(text)
        return text

    def _convert_apostrophe_styles(self, s: str) -> str:
        """Convert MediaWiki apostrophe-based styles to Markdown safely.
        Handles:
        - ''italic'' -> *italic*
        - '''bold''' -> **bold**
        - '''''both''''' -> ***both***
        And mixed patterns like '''''X''' rest'' (open both, close bold, keep italic),
        while preserving single apostrophes inside words (e.g., Ambessa's).
        Strategy: scan runs of apostrophes, prefer 5, then 3, then 2; toggle states accordingly.
        """
        if not s or "'" not in s:
            return s
        out: List[str] = []
        i = 0
        n = len(s)
        italic_on = False
        bold_on = False
        while i < n:
            ch = s[i]
            if ch != "'":
                out.append(ch)
                i += 1
                continue
            # Count consecutive apostrophes
            j = i
            while j < n and s[j] == "'":
                j += 1
            count = j - i
            # Special-case: close italic + possessive written as '''s
            # MediaWiki authors often write ''Word'''s to mean: close italic (''), then apostrophe + s.
            # When we're currently italic and not bold, interpret '''s as: close italic + literal ’ + keep 's' as text.
            if count == 3 and italic_on and not bold_on and j < n and s[j].lower() == 's':
                # Close italic
                out.append('*')
                italic_on = False
                # Typographic apostrophe
                out.append('’')
                # Advance past the three apostrophes; the 's' will be processed next iteration
                i = j
                continue
            # If only a single apostrophe, it's not a style marker; emit as-is
            if count == 1:
                out.append("'")
                i = j
                continue
            # Process runs in priority: 5, then 3, then 2. Leftover singles are emitted.
            remaining = count
            def toggle_both():
                nonlocal italic_on, bold_on
                # Emit *** whether opening or closing combined
                out.append('***')
                italic_on = not italic_on
                bold_on = not bold_on
            def toggle_bold():
                nonlocal bold_on
                out.append('**')
                bold_on = not bold_on
            def toggle_italic():
                nonlocal italic_on
                out.append('*')
                italic_on = not italic_on

            # Handle groups of 5 first
            while remaining >= 5:
                toggle_both()
                remaining -= 5
            # Then bold (3)
            if remaining >= 3:
                toggle_bold()
                remaining -= 3
            # Then italic (2)
            if remaining >= 2:
                toggle_italic()
                remaining -= 2
            # Any leftover singles are literal apostrophes
            if remaining > 0:
                out.append("'" * remaining)
            i = j
        return ''.join(out)

    def _expand_external_info_templates(self, text: str) -> str:
        """Expand specific info-style templates by inlining their extracted includeonly content.

        Supported template names (case/space/underscore insensitive):
          - Spellblade info
          - Energized info
          - Diminishing gold info
          - Effect at cast time start
          - Effect at cast time end

        For each occurrence of {{Template Name}} we look for a corresponding
        extracted file at: wiki_root / 'Template' / CanonicalName / 'page.txt'
        where CanonicalName matches the exact capitalization used by the
        extracted dump (we attempt a few common variants). We then extract the
        <includeonly>...</includeonly> segment (or fall back to whole file) and
        substitute it inline. The inserted raw wikitext still flows through the
        remaining conversion pipeline (lists, templates, etc.).
        """
        if not text or '{{' not in text:
            return text

        # Map normalized template key -> list of possible directory names
        template_dir_variants: Dict[str, List[str]] = {
            'spellblade info': ['Spellblade_info'],
            'energized info': ['Energized_info'],
            'diminishing gold info': ['Diminishing_gold_info'],
            'effect at cast time start': ['Effect_at_cast_time_start'],
            'effect at cast time end': ['Effect_at_cast_time_end'],
        }

        # Preload contents lazily and cache per instance run to avoid repeated disk hits
        cache: Dict[str, str] = {}

        def load_template_body(norm_key: str) -> Optional[str]:
            if norm_key in cache:
                return cache[norm_key]
            dirs = template_dir_variants.get(norm_key)
            if not dirs:
                cache[norm_key] = ''
                return ''
            body: Optional[str] = None
            for variant in dirs:
                path = self.wiki_root / 'Template' / variant / 'page.txt'
                if path.exists():
                    try:
                        raw = path.read_text(encoding='utf-8', errors='ignore')
                    except Exception:
                        continue
                    m = re.search(r'<includeonly>([\s\S]*?)</includeonly>', raw, flags=re.IGNORECASE)
                    if m:
                        body = m.group(1).strip()
                    else:
                        body = raw.strip()
                    break
            cache[norm_key] = body or ''
            return cache[norm_key]

        # Brace-aware scan replacing only our target templates with their bodies
        out: List[str] = []
        i = 0
        s = text
        n = len(s)
        while i < n:
            j = s.find('{{', i)
            if j == -1:
                out.append(s[i:])
                break
            out.append(s[i:j])
            # Parse template name token
            k = j + 2
            while k < n and s[k].isspace():
                k += 1
            name_start = k
            while k < n and s[k] not in '|}':
                k += 1
            name_raw = s[name_start:k].strip()
            # Walk to balanced end
            t = j
            depth = 0
            while t < n - 1:
                if s[t:t+2] == '{{':
                    depth += 1
                    t += 2
                    continue
                if s[t:t+2] == '}}':
                    depth -= 1
                    t += 2
                    if depth == 0:
                        break
                    continue
                t += 1
            tpl = s[j:t] if t <= n else s[j:]
            norm = name_raw.lower().replace('_', ' ').strip()
            if norm in template_dir_variants:
                body = load_template_body(norm)
                if body:
                    # Ensure the inserted body starts on a fresh (blank) line so that leading
                    # list markers (*) are recognized. If previous chunk ends with non-newline
                    # and body begins with '*', add two newlines; otherwise at least one.
                    if out:
                        if not out[-1].endswith('\n'):
                            out.append('\n')
                        # If previous line has text and body starts with a bullet, add an extra blank line
                        if body.lstrip().startswith('*'):
                            # Ensure exactly one blank line separation
                            if not out[-1].endswith('\n\n'):
                                out.append('\n')
                    out.append(body)
                    if not body.endswith('\n'):
                        out.append('\n')
                # If no body, we drop the template silently.
                i = t
                continue
            # Not one of our targets: keep original template markup
            out.append(tpl)
            i = t
        return ''.join(out)

    def _evaluate_expr_templates(self, text: str) -> str:
        """Find and replace {{#expr: ...}} with a safely evaluated result.
        Supports basic arithmetic (+,-,*,/,^,mod), parentheses, and a few functions (round, floor, ceil, min, max, abs).
        """
        if not text or '{{#expr' not in text:
            return text

        # Pre-compile a simple non-greedy regex that does not support nested templates inside the expr body.
        # This covers the vast majority of cases found in item/champion notes.
        expr_re = re.compile(r"\{\{\s*#expr\s*:\s*([^{}]+?)\s*\}\}", flags=re.IGNORECASE | re.DOTALL)

        def repl(m: re.Match) -> str:
            expr = m.group(1)
            try:
                val = self._safe_eval_num_expr(expr)
                if val is None:
                    return m.group(0)  # leave unchanged if cannot evaluate
                # Format: integer if close to int, else concise decimal
                if abs(val - round(val)) < 1e-9:
                    return str(int(round(val)))
                s = f"{val:.3f}".rstrip('0').rstrip('.')
                return s
            except Exception:
                # On any failure, return the original template unchanged
                return m.group(0)

        # Iterate until no more changes (to catch multiple instances)
        prev = None
        cur = text
        # Cap iterations to avoid pathological loops
        for _ in range(10):
            prev = cur
            cur = expr_re.sub(repl, prev)
            if cur == prev:
                break
        return cur

    def _safe_eval_num_expr(self, expr: str) -> Optional[float]:
        """Safely evaluate a numeric expression subset used in {{#expr:...}}.
        Allowed:
          - Numbers (int/float), parentheses
          - Operators: + - * / ^ (as power), % (mod)
          - Functions: round(x[,ndigits]), floor, ceil, min, max, abs
          - Constants: pi, e
        Returns float or None if not evaluable.
        """
        if not expr:
            return None
        s = str(expr)
        # Normalize: replace caret power and 'mod' token
        s = s.replace('^', '**')
        s = re.sub(r'\bmod\b', '%', s, flags=re.IGNORECASE)
        # Remove thousands separators
        s = s.replace(',', '')
        # Trim spaces
        s = s.strip()

        # Parse with AST and evaluate with a whitelist
        try:
            node = ast.parse(s, mode='eval')
        except Exception:
            return None

        allowed_funcs = {
            'round': round,
            'floor': math.floor,
            'ceil': math.ceil,
            'min': min,
            'max': max,
            'abs': abs,
        }
        allowed_names = {'pi': math.pi, 'e': math.e}

        def _check(n: ast.AST) -> None:
            if isinstance(n, ast.Expression):
                _check(n.body)
            elif isinstance(n, ast.BinOp) and isinstance(n.op, (ast.Add, ast.Sub, ast.Mult, ast.Div, ast.Mod, ast.Pow)):
                _check(n.left); _check(n.right)
            elif isinstance(n, ast.UnaryOp) and isinstance(n.op, (ast.UAdd, ast.USub)):
                _check(n.operand)
            elif isinstance(n, ast.Num):  # py<3.8
                return
            elif isinstance(n, ast.Constant):
                if isinstance(n.value, (int, float)):
                    return
                raise ValueError('Invalid constant')
            elif isinstance(n, ast.Call):
                # Only allow simple function names from the whitelist
                if isinstance(n.func, ast.Name) and n.func.id in allowed_funcs:
                    for arg in n.args:
                        _check(arg)
                    # Disallow keywords for safety except round(x, ndigits)
                    for kw in n.keywords:
                        _check(kw.value)
                    return
                raise ValueError('Disallowed function')
            elif isinstance(n, ast.Name):
                if n.id in allowed_names:
                    return
                raise ValueError('Unknown name')
            else:
                raise ValueError('Disallowed expression')

        _check(node)
        try:
            val = eval(compile(node, '<expr>', 'eval'), {'__builtins__': {}}, {**allowed_funcs, **allowed_names})
        except Exception:
            return None
        try:
            return float(val)
        except Exception:
            return None

    def _normalize_anchor(self, anchor: str) -> str:
        """Normalize a wiki anchor for markdown link: keep case, replace spaces with underscores."""
        if not anchor:
            return ''
        # Replace spaces, remove parens, normalize dashes, and trim stray punctuation
        a = anchor.replace(' ', '_')
        a = a.replace('(', '').replace(')', '')
        a = a.replace('—', '-').replace('–', '-')
        a = a.replace(',', '')
        a = a.strip(' .')
        return a

    def _convert_section_link(self, match) -> str:
        """Convert MediaWiki section links [[Page#Anchor|Display]] to markdown."""
        page = match.group(1)
        anchor = match.group(2)
        display = match.group(3) or (anchor or page)
        page_md = page.replace(' ', '_')
        anchor_md = self._normalize_anchor(anchor)
        return f"[{display}](./{page_md}.md#{anchor_md})"

    def _convert_tt(self, match) -> str:
        """Convert {{tt|value|tooltip}} -> 'value (tooltip)'.
        If only one arg present, return it. Ignore extra pipes.
        """
        inner = match.group(1)
        parts = [p.strip() for p in inner.split('|')]
        if not parts:
            return ''
        if len(parts) == 1:
            return parts[0]
        value = parts[0]
        tip = parts[1]
        if not tip:
            return value
        return f"{value} ({tip})"

    def _convert_st_blocks(self, text: str) -> str:
        """Find and replace all top-level {{st|...}} blocks with markdown tables, preserving nested templates.
        This is brace-aware to avoid splitting on inner pipes.
        """
        out: List[str] = []
        i = 0
        n = len(text)
        while i < n:
            j = text.find('{{st|', i)
            if j == -1:
                out.append(text[i:])
                break
            # append text before match
            out.append(text[i:j])
            # parse from j
            k = j + 5  # position after '{{st|'
            depth = 1  # depth of nested templates after '{{st|'
            buf: List[str] = []
            while k < n:
                if k + 1 < n and text[k] == '{' and text[k+1] == '{':
                    depth += 1
                    buf.append(text[k:k+2])
                    k += 2
                    continue
                if k + 1 < n and text[k] == '}' and text[k+1] == '}':
                    depth -= 1
                    if depth == 0:
                        k += 2
                        break
                    buf.append(text[k:k+2])
                    k += 2
                    continue
                buf.append(text[k])
                k += 1
            # buf contains "Label1|Value1|..." plus possibly leading parameters including 'st|'
            inner = ''.join(buf)
            # Remove any leading template name remnants if present
            if inner.startswith('st|'):
                inner = inner[3:]
            table_md = self._render_st(inner) if inner else ''
            out.append(table_md)
            i = k
        return ''.join(out)

    def _convert_st(self, match) -> str:
        """Convert {{st|Label1|Value1|Label2|Value2|...}} to a markdown table with right-aligned values."""
        inner = match.group(1)
        return self._render_st(inner)

    def _split_top_level_pipes(self, s: str) -> List[str]:
        parts: List[str] = []
        buf: List[str] = []
        depth = 0
        i = 0
        n = len(s)
        while i < n:
            if i + 1 < n and s[i] == '{' and s[i+1] == '{':
                depth += 1
                buf.append(s[i]); buf.append(s[i+1])
                i += 2
                continue
            if i + 1 < n and s[i] == '}' and s[i+1] == '}':
                depth = max(0, depth - 1)
                buf.append(s[i]); buf.append(s[i+1])
                i += 2
                continue
            if s[i] == '|' and depth == 0:
                parts.append(''.join(buf))
                buf = []
                i += 1
                continue
            buf.append(s[i])
            i += 1
        parts.append(''.join(buf))
        return parts

    def _render_st(self, inner: str) -> str:
        parts = [p.strip() for p in self._split_top_level_pipes(inner)]
        if not parts:
            return ''
        rows: List[Tuple[str, str]] = []
        i = 0
        while i + 1 < len(parts):
            label = parts[i]
            value = parts[i+1]
            i += 2
            if not label:
                continue
            # Convert both label and value through the wiki->markdown pipeline so any templates
            # (tt, ai, ii, fd, as, etc.) inside labels/values are unwrapped and styled correctly.
            label_md = self._convert_wiki_to_markdown(label) if label else ''
            # Avoid double-bold on labels: if label is already wrapped in ** **, strip it before we bold below.
            label_md = re.sub(r'^\s*\*\*(.*?)\*\*\s*$', r'\1', label_md)
            value_md = self._convert_wiki_to_markdown(value) if value else ''
            rows.append((label_md, value_md))
        if not rows:
            return ''
        lines = ["| Attribute | Value |", "|-----------|------:|"]
        def _esc_pipes(s: str) -> str:
            return s.replace('|', '\\|') if s else s
        for label, value in rows:
            lines.append(f"| **{_esc_pipes(label)}** | {_esc_pipes(value)} |")
        return "\n".join(lines)
    
    def _convert_ap_formula(self, match) -> str:
        """Convert {{ap|...}} into a progression list matching Module:Ability_progression.

        Supported forms (examples):
        - {{ap|50|70|90}} -> 50 / 70 / 90
        - {{ap|2*x}} -> 5 values by default: 2 / 4 / 6 / 8 / 10
        - {{ap|60 to 200}} -> 5 values linearly spaced from start..finish inclusive
        - {{ap|2*x 3}} -> override count: 2 / 4 / 6
        - {{ap|60 to 200 4}} -> linear with 4 values
        - Nested 'to' in formulas is supported via expansion into a function of x and times.
        Named params supported: round=(number|ceil|floor|abs|trunc), skill=R (default values = 3).
        Up to 6 values total; display an error if exceeded.
        """
        content = match.group(1)

        # Split args on top-level '|' (no braces here; parentheses don't contain '|')
        parts = [p.strip() for p in content.split('|')]
        pos_args: List[str] = []
        named: Dict[str, str] = {}
        for p in parts:
            if not p:
                continue
            if '=' in p:
                k, v = p.split('=', 1)
                named[k.strip().lower()] = v.strip()
            else:
                pos_args.append(p)

        # Rounding settings
        round_param: Optional[str] = named.get('round')
        skill_param = named.get('skill')
        # Default fill count: 5 or 3 if ultimate (skill=R)
        fill = 3 if (skill_param and skill_param.strip().upper() == 'R') else 5

        # Max total values
        remaining = 6
        results: List[str] = []

        def half_up_round(val: float, decimals: int) -> float:
            if decimals <= 0:
                # emulate Lua floor(val + 0.5) for non-negative, ceil(val - 0.5) for negative
                return math.floor(val + 0.5) if val >= 0 else math.ceil(val - 0.5)
            scale = 10 ** decimals
            if val >= 0:
                return math.floor(val * scale + 0.5) / scale
            else:
                return math.ceil(val * scale - 0.5) / scale

        def apply_rounding(val: Union[float, int, str]) -> Union[float, int, str]:
            if round_param is None:
                return val
            rp = round_param.strip().lower()
            try:
                if rp == 'abs':
                    return abs(float(val))
                if rp == 'ceil':
                    return math.ceil(float(val))
                if rp == 'floor':
                    return math.floor(float(val))
                if rp == 'trunc':
                    return float(str(val)).split('.')[0]
                # numeric decimals
                dec = int(float(rp))
                return half_up_round(float(val), dec)
            except Exception:
                return val

        def fmt_number(v: Union[float, int, str]) -> str:
            if isinstance(v, (int,)):
                return str(v)
            if isinstance(v, float):
                # General format to avoid long floats; similar to Lua's display
                return format(v, 'g')
            return str(v)

        # Expand nested 'to' into function of x and times
        def expand_to(expr: str) -> str:
            s = expr
            # Keep expanding the first top-level 'to' found
            while True:
                # Scan for 'to' at top-level (depth 0)
                depth = 0
                idx = -1
                i = 0
                while i + 1 < len(s):
                    ch = s[i]
                    if ch == '(':
                        depth += 1
                    elif ch == ')':
                        depth = max(0, depth - 1)
                    # match 'to' when depth==0 and surrounded by non-word boundaries
                    if depth == 0 and s[i:i+2] == 'to':
                        # ensure token-ish boundaries
                        left_ok = (i == 0) or (s[i-1].isspace() or s[i-1] in '+-*/^,(')
                        right_ok = (i+2 >= len(s)) or (s[i+2].isspace() or s[i+2] in '+-*/^,)')
                        if left_ok and right_ok:
                            idx = i
                            break
                    i += 1
                if idx == -1:
                    return s

                # Find left operand bounds
                l = idx - 1
                while l >= 0 and s[l].isspace():
                    l -= 1
                if l >= 0 and s[l] == ')':
                    # find matching '('
                    bal = 1
                    l -= 1
                    while l >= 0 and bal > 0:
                        if s[l] == ')':
                            bal += 1
                        elif s[l] == '(':
                            bal -= 1
                        l -= 1
                    start_start = l + 1
                    start_end = idx - 1
                else:
                    # scan left over token chars
                    token_chars = set('._%')  # allow % in numbers like 10% (we'll strip later if needed)
                    while l >= 0 and (s[l].isalnum() or s[l] in token_chars):
                        l -= 1
                    start_start = l + 1
                    start_end = idx - 1

                # Find right operand bounds
                r = idx + 2
                while r < len(s) and s[r].isspace():
                    r += 1
                if r < len(s) and s[r] == '(':
                    bal = 1
                    r += 1
                    while r < len(s) and bal > 0:
                        if s[r] == '(':
                            bal += 1
                        elif s[r] == ')':
                            bal -= 1
                        r += 1
                    finish_start = idx + 2
                    finish_end = r
                else:
                    # scan right over token chars
                    token_chars_r = set('._%')
                    while r < len(s) and (s[r].isalnum() or s[r] in token_chars_r):
                        r += 1
                    finish_start = idx + 2
                    finish_end = r

                start = s[start_start:start_end].strip()
                finish = s[finish_start:finish_end].strip()
                # Recurse on finish for nested 'to'
                finish_expanded = expand_to(finish) if 'to' in finish else finish
                # Build replacement
                repl = f"({start})+((({finish_expanded})-({start}))/(times-1)*(x-1))"
                s = s[:start_start] + repl + s[finish_end:]
            # not reached

        def eval_expr(expr: str) -> Optional[float]:
            try:
                # Replace percent like '20%' -> '0.2'
                expr_pct = re.sub(r'(\d+(?:\.\d+)?)%', lambda m: str(float(m.group(1))/100.0), expr)
                return float(self._safe_eval_num_expr(expr_pct))
            except Exception:
                return None

        # Helper: append a single value (numeric or string)
        def append_value(v: Union[float, int, str]):
            nonlocal remaining
            if remaining <= 0:
                return
            if isinstance(v, (int, float)):
                v2 = apply_rounding(v)
                try:
                    v_num = float(v2)
                    results.append(fmt_number(v_num))
                except Exception:
                    results.append(str(v2))
            else:
                results.append(str(v))
            remaining -= 1

        # Process each positional argument
        for token in pos_args:
            if remaining <= 0:
                break
            if not token:
                continue
            t = token.strip()
            has_to = 'to' in t
            has_x = 'x' in t

            # Branch 1: simple linear range without x -> start to finish [times]
            if has_to and not has_x:
                m = re.match(r'^(.*)\bto\b(.*)$', t)
                if m:
                    start_str = m.group(1).strip()
                    right = m.group(2).strip()
                    m2 = re.match(r'^(.*?)(?:\s+(\S+))?$', right)
                    finish_str = (m2.group(1) if m2 else right).strip()
                    times_token = (m2.group(2) if m2 else '')

                    start_val = eval_expr(start_str)
                    finish_val = eval_expr(finish_str)
                    times_val: Optional[int] = None
                    if times_token:
                        tv = eval_expr(times_token)
                        if tv is not None:
                            times_val = int(tv)
                    elif fill:
                        times_val = fill
                        fill = 0

                    if start_val is not None and finish_val is not None and times_val and times_val > 0:
                        # Emit linear interpolation inclusive
                        if remaining - times_val < 0:
                            # exceed limit -> display error
                            return "[ap error: maximum size exceeded]"
                        if times_val == 1:
                            append_value(start_val)
                        else:
                            scale = (finish_val - start_val) / (times_val - 1)
                            for i in range(times_val):
                                val = start_val + scale * i
                                append_value(val)
                        continue
                # fallback to generic handling

            # Branch 2: formula (may include nested 'to') possibly with trailing times
            # Detect trailing times: only if last token is a number
            m = re.match(r'^(.*\S)\s+([\-+]?\d+(?:\.\d+)?)$', t)
            base_formula = t
            times_val: Optional[int] = None
            if m:
                base_formula = m.group(1).strip()
                tv = eval_expr(m.group(2))
                if tv is not None:
                    times_val = int(tv)
            if times_val is None and fill:
                times_val = fill
                fill = 0

            if has_to:
                base_formula = expand_to(base_formula)
                # replace whole-word 'times' with concrete value if known
                if times_val is not None:
                    base_formula = re.sub(r'\btimes\b', str(times_val), base_formula)

            # If we have times and either x present or 'to' expanded, try evaluating sequence
            if times_val and times_val > 0 and (has_x or has_to):
                if remaining - times_val < 0:
                    return "[ap error: maximum size exceeded]"
                ok_any = False
                for x in range(1, times_val + 1):
                    expr_x = re.sub(r'\bx\b', str(x), base_formula)
                    val = eval_expr(expr_x)
                    if val is None:
                        ok_any = False
                        break
                    ok_any = True
                    append_value(val)
                if ok_any:
                    continue

            # Branch 3: literal or single expression value
            val1 = eval_expr(t)
            if val1 is not None:
                append_value(val1)
            else:
                append_value(t)

        # If nothing produced, keep original content
        if not results:
            return content
        return " / ".join(results)
    
    def _convert_pp_formula(self, match) -> str:
        """Convert {{pp|...}} templates to a compact list of values aligned to level breakpoints.
        Expected common form: {{pp|<start> to <end> for <n>|<levels>|...}}
        - Values: generate n evenly spaced values from start..end inclusive
        - Levels: tokens separated by ';' with optional 'a to b' inclusive expansion
        Output (compact, level-annotated): e.g. '30@1; 35–90@7–18'. Falls back to endpoints when needed.
        """
        content = match.group(1)
        # Use brace-aware splitting to avoid breaking on nested templates
        raw_parts = [p.strip() for p in self._split_top_level_pipes(content) if p.strip()]
        if not raw_parts:
            return content
        # Separate positional vs named args
        pos_args: List[str] = []
        named: Dict[str, str] = {}
        for p in raw_parts:
            if '=' in p:
                k, v = p.split('=', 1)
                named[k.strip().lower()] = v.strip()
            else:
                pos_args.append(p)
        # Values spec is the first positional argument
        if not pos_args:
            # Nothing to compute; if a 'key' is present, drop it and return empty
            return ''
        values_spec = pos_args[0]
        # Levels spec may be provided as the next positional argument
        levels_spec = pos_args[1] if len(pos_args) > 1 else ''
        # Named extras
        key_suffix = (named.get('key') or '').strip()
        round_param = (named.get('round') or '').strip()
        # Optional key label for annotations
        key_label: Optional[str] = None
        if 'type' in named and named['type'].strip():
            key_label = named['type'].strip()
        else:
            for k in ('label', 'per', 'by', 'scale'):
                if k in named and named[k].strip():
                    key_label = named[k].strip()
                    break

        # Try to parse canonical 'start to end for n' form
        # Helper: rounding with default=2 decimals; clamp to 100 when key='%' and within a rounding epsilon
        def _round_value(v: float) -> float:
            if round_param:
                rp = round_param.lower()
                try:
                    if rp == 'abs':
                        v = abs(v)
                        # fall-through to no-decimal adjustment
                    elif rp == 'ceil':
                        return float(math.ceil(v))
                    elif rp == 'floor':
                        return float(math.floor(v))
                    elif rp == 'trunc':
                        return float(int(v))
                    else:
                        nd = int(float(rp))
                        v = float(f"{v:.{nd}f}")
                except Exception:
                    pass
            else:
                # default to 2 decimals like Lua default
                v = float(f"{v:.2f}")
            # Clamp near-100 for percentages to avoid 100.01%
            if key_suffix and '%' in key_suffix:
                if 99.995 <= v <= 100.05:
                    v = 100.0
            return v

        m = re.match(r'^(-?\d+(?:\.\d+)?)\s*to\s*(-?\d+(?:\.\d+)?)\s*for\s*(\d+)$', values_spec, flags=re.IGNORECASE)
        values: List[str] = []
        values_num: List[Optional[float]] = []
        levels: List[int] = []
        if m:
            start = float(m.group(1))
            end = float(m.group(2))
            count = int(m.group(3))
            if count <= 1:
                seq = [start]
            else:
                step = (end - start) / (count - 1)
                seq = [start + i * step for i in range(count)]
            # Apply rounding policy to sequence
            seq_r = [_round_value(x) for x in seq]
            values_num = seq_r
            values = [self._pp_num_fmt(x) + (key_suffix if key_suffix else '') for x in seq_r]
            if levels_spec:
                levels = self._pp_parse_levels(levels_spec)
        else:
            # Fallbacks: explicit semicolon list of values
            if ';' in values_spec:
                seq_raw = [s.strip() for s in values_spec.split(';') if s.strip()]
                values = []
                values_num = []
                for s in seq_raw:
                    if re.match(r'^-?\d+(?:\.\d+)?$', s):
                        f = _round_value(float(s))
                        values.append(self._pp_num_fmt(f) + (key_suffix if key_suffix else ''))
                        values_num.append(f)
                    else:
                        values.append(s + (key_suffix if (key_suffix and re.match(r'^-?\d', s)) else ''))
                        values_num.append(None)
                if levels_spec:
                    levels = self._pp_parse_levels(levels_spec)
            else:
                # Try to evaluate expression possibly containing 'x' by sampling endpoints (level 1 and 18)
                def _eval_expr(expr: str, x_val: int) -> Optional[float]:
                    # Replace percent literals like '10%' with 0.1
                    expr2 = re.sub(r'(\d+(?:\.\d+)?)%', lambda m: str(float(m.group(1))/100.0), expr)
                    # Replace whole-word 'x' with level value
                    expr2 = re.sub(r'\bx\b', str(x_val), expr2)
                    try:
                        val = self._safe_eval_num_expr(expr2)
                        return float(val) if val is not None else None
                    except Exception:
                        return None
                v1 = _eval_expr(values_spec, 1)
                v18 = _eval_expr(values_spec, 18)
                # Optionally apply rounding similar to Module: round defaults to 2
                def _apply_round(v: float) -> float:
                    if not round_param:
                        # default 2 decimals
                        return float(f"{v:.2f}")
                    rp = round_param.lower()
                    try:
                        if rp == 'abs':
                            return abs(v)
                        if rp == 'ceil':
                            return float(math.ceil(v))
                        if rp == 'floor':
                            return float(math.floor(v))
                        if rp == 'trunc':
                            return float(int(v))
                        nd = int(float(rp))
                        return float(f"{v:.{nd}f}")
                    except Exception:
                        return v
                if v1 is not None and v18 is not None:
                    a = _round_value(v1)
                    b = _round_value(v18)
                    # If equal after rounding, show single value; else show range
                    if abs(a - b) < 1e-9:
                        values = [self._pp_num_fmt(a) + (key_suffix if key_suffix else '')]
                    else:
                        values = [self._pp_num_fmt(a) + (key_suffix if key_suffix else ''), self._pp_num_fmt(b) + (key_suffix if key_suffix else '')]
                        values_num = [a, b]
                        levels = [1, 18]
                else:
                    # Plain value or unknown form: return the raw spec without the 'key=' noise
                    return values_spec

        # If we have a levels spec, produce a compact, annotated summary
        if levels_spec and levels:
            try:
                segs = self._pp_parse_level_segments(levels_spec)
                # If mismatch in lengths, summarize endpoints with any available levels
                if len(values) != len(levels):
                    first_v = values[0] if values else ''
                    last_v = values[-1] if values else ''
                    first_L = levels[0]
                    last_L = levels[-1]
                    if first_v and last_v:
                        annot = f" (@={key_label})" if key_label else ''
                        return f"{first_v}–{last_v}@{first_L}–{last_L}{annot}"
                    return '/'.join(values)
                # Build segments in order based on the flattened levels
                out_parts: List[str] = []
                for a, b in segs:
                    # find indices in flattened levels
                    try:
                        i0 = levels.index(a)
                        i1 = len(levels) - 1 - levels[::-1].index(b)
                    except ValueError:
                        # If a/b not found, skip
                        continue
                    if i1 < i0:
                        i0, i1 = i1, i0
                    seg_vals = values[i0:i1+1]
                    seg_nums = values_num[i0:i1+1] if values_num else [None] * len(seg_vals)
                    if len(seg_vals) == 1:
                        out_parts.append(f"{seg_vals[0]}@{a}")
                        continue
                    # Decide representation for segment
                    # If numeric and constant across, compress to v@a–b
                    if all((x is not None) for x in seg_nums):
                        nums = [x for x in seg_nums if x is not None]
                        first_n, last_n = nums[0], nums[-1]
                        # constant
                        if all(abs(nums[i] - nums[0]) < 1e-9 for i in range(len(nums))):
                            out_parts.append(f"{seg_vals[0]}@{a}–{b}")
                        else:
                            # arithmetic or monotonic -> show endpoints
                            out_parts.append(f"{seg_vals[0]}–{seg_vals[-1]}@{a}–{b}")
                    else:
                        # Non-numeric: show endpoints
                        out_parts.append(f"{seg_vals[0]}–{seg_vals[-1]}@{a}–{b}")
                if out_parts:
                    annot = f" (@={key_label})" if key_label else ''
                    return '; '.join(out_parts) + annot
            except Exception:
                # Fall through to simpler formatting
                pass

        # Without usable numeric levels, prefer full list to preserve distinct tiers (e.g., Boots tiers)
        if values:
            return " / ".join(values)
        return ''

    def _convert_pptooltip(self, match) -> str:
        """Convert {{pptooltip|...}} to a readable inline string.
        Supported keys (best-effort): bot_values, top_values, bot_label, top_label, bot_key, top_key, displayformula, start, finish.
        Output example: "140 / 153 / 166 / 175 (Boots Tier: None / Basic / Finished / Zephyr)".
        """
        inner = match.group(1)
        parts = [p.strip() for p in inner.split('|') if p.strip()]
        kv: Dict[str, str] = {}
        for p in parts:
            if '=' in p:
                k, v = p.split('=', 1)
                kv[k.strip().lower()] = v.strip()
        # Values
        bot_vals_raw = kv.get('bot_values', '')
        top_vals_raw = kv.get('top_values', '')
        bot_vals = [s.strip() for s in bot_vals_raw.split(';') if s.strip()]
        top_vals = [s.strip() for s in top_vals_raw.split(';') if s.strip()]
        bot_label = kv.get('bot_label') or kv.get('label') or 'value'
        top_label = kv.get('top_label') or 'level'
        bot_key = kv.get('bot_key', '')
        top_key = kv.get('top_key', '')
        displayformula = kv.get('displayformula', '')

        out_parts: List[str] = []
        if bot_vals:
            vals = ' / '.join(bot_vals)
            if bot_key:
                vals = f"{vals}{bot_key}"
            out_parts.append(vals)
        elif displayformula:
            out_parts.append(displayformula)
        else:
            # Try start/finish fallback
            start = kv.get('start', '').replace(';', ' / ')
            finish = kv.get('finish', '').replace(';', ' / ')
            if start or finish:
                if start and finish:
                    out_parts.append(f"{start} – {finish}")
                else:
                    out_parts.append(start or finish)

        anno_parts: List[str] = []
        if top_vals:
            tvals = ' / '.join(top_vals)
            if top_key:
                tvals = f"{tvals}{top_key}"
            anno_parts.append(f"{top_label}: {tvals}")
        # Combine
        if anno_parts:
            return f"{'; '.join(out_parts)} ({'; '.join(anno_parts)})"
        return '; '.join(out_parts)

    def _pp_parse_levels(self, spec: str) -> List[int]:
        """Parse a levels spec like '1;7 to 18' into a list of ints: [1,7,8,...,18]."""
        out: List[int] = []
        tokens = [t.strip() for t in spec.split(';') if t.strip()]
        for tok in tokens:
            m = re.match(r'^(\d+)\s*to\s*(\d+)$', tok, flags=re.IGNORECASE)
            if m:
                a, b = int(m.group(1)), int(m.group(2))
                if a <= b:
                    out.extend(list(range(a, b + 1)))
                else:
                    out.extend(list(range(a, b - 1, -1)))
            elif tok.isdigit():
                out.append(int(tok))
        return out

    def _pp_parse_level_segments(self, spec: str) -> List[Tuple[int, int]]:
        """Parse a levels spec into ordered inclusive segments: '1;7 to 18' -> [(1,1),(7,18)]."""
        segs: List[Tuple[int, int]] = []
        tokens = [t.strip() for t in spec.split(';') if t.strip()]
        for tok in tokens:
            m = re.match(r'^(\d+)\s*to\s*(\d+)$', tok, flags=re.IGNORECASE)
            if m:
                a, b = int(m.group(1)), int(m.group(2))
                segs.append((a, b))
            elif tok.isdigit():
                x = int(tok)
                segs.append((x, x))
        return segs

    def _pp_num_fmt(self, x: float) -> str:
        """Format a number for pp output: drop trailing .0, keep up to 3 decimals if needed."""
        if abs(x - round(x)) < 1e-9:
            return str(int(round(x)))
        # Keep a concise decimal representation
        s = f"{x:.3f}"
        s = s.rstrip('0').rstrip('.')
        return s
    
    def _convert_link(self, match) -> str:
        """Convert MediaWiki links to markdown."""
        target = match.group(1)
        display = match.group(2) or target
        
        if target.startswith('File:'):
            return f"![{display}](./images/{target[5:].replace(' ', '_')})"
        else:
            return f"[{display}](./{target.replace(' ', '_')}.md)"

    # ------ Item Markdown generation ------
    def _generate_item_markdown(self, name: str, item: Dict[str, Any], main_content: str) -> str:
        lines: List[str] = []
        lines.append(f"# {name}")
        lines.append("")
        lines.append("## Table of Contents")
        lines.append("- [Basic Information](#basic-information)")
        lines.append("- [Statistics](#statistics)")
        if item.get('effects'):
            lines.append("- [Effects](#effects)")
        # Recipe/builds
        if item.get('recipe'):
            lines.append("- [Build Path](#build-path)")
        # Parse Item info once for additional sections
        info = self._parse_item_info_block(main_content) or {}
        # Extra sections in TOC if present
        if info.get('notes'):
            lines.append("- [Notes](#notes)")
        # Role/category lists
        for sec_id, _label in [
            ('similaritems', 'Similar items'),
            ('assassinitems', 'Assassin items'),
            ('fighteritems', 'Fighter items'),
            ('mageitems', 'Mage items'),
            ('marksmanitems', 'Marksman items'),
            ('supportitems', 'Support items'),
            ('tankitems', 'Tank items'),
            ('prismaticitems', 'Prismatic items'),
        ]:
            if info.get(sec_id):
                anchor = sec_id.replace('items', '-items') if sec_id != 'similaritems' else 'similar-items'
                lines.append(f"- [{_label}](#{anchor})")
        if info.get('strategy'):
            lines.append("- [Strategy](#strategy)")
        if info.get('background'):
            lines.append("- [Background](#background)")
        if info.get('quotes'):
            lines.append("- [Quotes](#quotes)")
        if info.get('sfx'):
            lines.append("- [Sound Effects](#sound-effects)")
        if info.get('revisions'):
            lines.append("- [Revisions](#revisions)")
        # Patch history if we can find any
        if info.get('patchhistory'):
            lines.append("- [Patch History](#patch-history)")
        lines.append("")

        # Basic Information
        lines.append("## Basic Information")
        lines.append("")
        lines.append("| Attribute | Value |")
        lines.append("|-----------|------:|")
        # Type / Tier
        if isinstance(item.get('type'), dict) or isinstance(item.get('type'), list):
            types = []
            if isinstance(item['type'], dict):
                # parse_lua_simple_kv may parse array-like as dict with numeric keys; collect values
                for v in item['type'].values():
                    types.append(str(v))
            else:
                types = [str(x) for x in item['type']]
            if types:
                lines.append(f"| **Type** | {', '.join(types)} |")
        if item.get('tier') is not None:
            lines.append(f"| **Tier** | {item['tier']} |")
        # Mode availability
        modes = item.get('modes')
        if isinstance(modes, dict) and modes:
            def _mflag(k: str) -> Optional[str]:
                v = modes.get(k)
                if v is None:
                    return None
                return 'Available' if bool(v) else 'Unavailable'
            for key, label in [("classic sr 5v5", "SR 5v5"), ("aram", "ARAM"), ("nb", "Nexus Blitz"), ("arena", "Arena")]:
                flag = _mflag(key)
                if flag is not None:
                    lines.append(f"| **{label}** | {flag} |")
        # Limits / Exclusive / Ornn
        if item.get('itemlimit'):
            lines.append(f"| **Item Limit** | {self._convert_wiki_to_markdown(str(item['itemlimit']))} |")
        if item.get('limit'):
            lines.append(f"| **Limit** | {self._convert_wiki_to_markdown(str(item['limit']))} |")
        if item.get('exclusive'):
            lines.append(f"| **Exclusive** | {self._convert_wiki_to_markdown(str(item['exclusive']))} |")
        if item.get('ornn') is True:
            lines.append(f"| **Ornn** | Can be forged by Ornn |")
        if item.get('removed') is True:
            lines.append(f"| **Removed** | True |")
        # Gold values
        buy = item.get('buy')
        if buy is not None:
            lines.append(f"| **Cost** | {buy} gold |")
            # Combine cost if recipe exists
            comb = self._compute_item_combine_cost(name, item)
            if comb is not None:
                lines.append(f"| **Combine Cost** | {comb} gold |")
            # Sell price
            sell = item.get('sell')
            if sell is None:
                # estimate using ratio
                sellratio = item.get('sellratio')
                if sellratio is None:
                    # default ratios similar to Lua getter
                    primary_type = None
                    if isinstance(item.get('type'), list) and item['type']:
                        primary_type = item['type'][0]
                    elif isinstance(item.get('type'), dict) and item['type']:
                        primary_type = list(item['type'].values())[0]
                    ratio = 0.4 if (primary_type in ("Starter", "Potion")) else 0.7
                    sellratio = ratio
                try:
                    sell = int(float(buy) * float(sellratio))
                except Exception:
                    sell = None
            if sell is not None:
                lines.append(f"| **Sell** | {sell} gold |")
        lines.append("")

        # Statistics
        lines.append("## Statistics")
        lines.append("")
        stats = item.get('stats') if isinstance(item.get('stats'), dict) else {}
        if stats:
            lines.append("| Stat | Value |")
            lines.append("|------|------:|")
            for key, val in stats.items():
                label = self._item_stat_label(key)
                sval = self._item_stat_format(key, val)
                lines.append(f"| **{label}** | {sval} |")
            lines.append("")
        else:
            lines.append("No inherent stats.")
            lines.append("")

        # Effects
        if item.get('effects') and isinstance(item['effects'], dict):
            lines.append("## Effects")
            lines.append("")
            # iterate in stable order groups
            order = ["act", "act2", "pass", "pass2", "pass3", "pass4", "pass5", "pass6", "aura"]
            for k in order:
                eff = item['effects'].get(k)
                if not isinstance(eff, dict):
                    continue
                name_part = eff.get('name')
                unique = eff.get('unique') is True
                head = []
                if k.startswith('act'):
                    head.append("Active")
                elif k.startswith('pass'):
                    head.append("Passive")
                elif k == 'aura':
                    head.append("Aura")
                if unique:
                    head.append("Unique")
                title = ' '.join(head)
                if name_part:
                    title = f"{title} — {name_part}"
                desc = eff.get('description') or ''
                desc2 = eff.get('description2') or ''
                desc3 = eff.get('description3') or ''
                desc4 = eff.get('description4') or ''
                desc5 = eff.get('description5') or ''
                extra_bits: List[str] = []
                if eff.get('cd'):
                    extra_bits.append(self._format_with_unit(str(eff['cd']), 'seconds'))
                if eff.get('recharge'):
                    extra_bits.append(self._format_with_unit(str(eff['recharge']), 'seconds recharge'))
                if eff.get('charges'):
                    extra_bits.append(f"{eff['charges']} charges")
                if eff.get('range'):
                    extra_bits.append(self._format_with_unit(str(eff['range']), 'range'))
                if eff.get('radius'):
                    extra_bits.append(self._format_with_unit(str(eff['radius']), 'radius'))
                # Build effect body
                body_parts = [p for p in [desc, desc2, desc3, desc4, desc5] if p]
                body = '<br>'.join(body_parts)
                body = self._convert_wiki_to_markdown(body)
                if extra_bits:
                    tail = '; '.join(extra_bits)
                    if body.endswith('.'):  # mirror compile_effect punctuation
                        body = body[:-1]
                    body = f"{body} ({tail})."
                lines.append(f"- **{title}:** {body}")
            lines.append("")

        # Build Path
        if item.get('recipe'):
            lines.append("## Build Path")
            lines.append("")
            recipe = item['recipe']
            if isinstance(recipe, dict):
                # numeric keys -> list
                parts = [str(v) for _, v in sorted(recipe.items(), key=lambda kv: str(kv[0]))]
            else:
                parts = [str(x) for x in recipe] if isinstance(recipe, list) else []
            if parts:
                lines.append("- **Recipe:** " + ", ".join([f"*{p}*" for p in parts]))
            comb = self._compute_item_combine_cost(name, item)
            if comb is not None:
                lines.append(f"- **Combine Cost:** {comb} gold")
            lines.append("")

        # Notes (from Item info)
        if info.get('notes'):
            lines.append("## Notes")
            lines.append("")
            notes_md = self._format_notes(info.get('notes', ''), champion_name=None)
            for ln in notes_md.splitlines():
                if ln.strip():
                    lines.append(ln if ln.strip().startswith('- ') else f"- {ln.strip()}")
            lines.append("")

        # Similar and role-based item lists
        def _render_item_list(param_key: str, header: str, anchor_override: Optional[str] = None):
            raw = info.get(param_key)
            if not raw:
                return
            lines.append(f"## {header}")
            lines.append("")
            # Values may be separated by ';'
            items = [x.strip() for x in str(raw).split(';') if x.strip()]
            for it in items:
                # Use ii template rendering behavior: italicize name
                label = self._convert_wiki_to_markdown(f"{{{{ii|{it}}}}}")
                lines.append(f"- {label}")
            lines.append("")

        _render_item_list('similaritems', 'Similar items')
        _render_item_list('assassinitems', 'Assassin items')
        _render_item_list('fighteritems', 'Fighter items')
        _render_item_list('mageitems', 'Mage items')
        _render_item_list('marksmanitems', 'Marksman items')
        _render_item_list('supportitems', 'Support items')
        _render_item_list('tankitems', 'Tank items')
        _render_item_list('prismaticitems', 'Prismatic items')

        # Strategy / Background / Quotes / Media / SFX / Revisions
        if info.get('strategy'):
            lines.append("## Strategy")
            lines.append("")
            lines.append(self._convert_wiki_to_markdown(info['strategy']))
            lines.append("")
        if info.get('background'):
            lines.append("## Background")
            lines.append("")
            # Background in Item info is typically italicized; keep italics
            bg = self._convert_wiki_to_markdown(info['background'])
            if not (bg.startswith('*') and bg.endswith('*')):
                bg = f"*{bg}*"
            lines.append(bg)
            lines.append("")
        if info.get('quotes'):
            lines.append("## Quotes")
            lines.append("")
            lines.append(self._convert_wiki_to_markdown(info['quotes']))
            lines.append("")
        if info.get('sfx'):
            lines.append("## Sound Effects")
            lines.append("")
            lines.append(self._convert_wiki_to_markdown(info['sfx']))
            lines.append("")
        if info.get('revisions'):
            lines.append("## Revisions")
            lines.append("")
            lines.append(self._convert_wiki_to_markdown(info['revisions']))
            lines.append("")

        # Patch History from Item info block if present
        if info and info.get('patchhistory'):
            lines.append("## Patch History")
            lines.append("")
            raw_ph = info['patchhistory']
            # Try to parse into structured patches, else fallback to raw conversion
            patches = self._parse_patch_history_text(raw_ph)
            if patches:
                for patch in patches:
                    lines.append(f"### {patch['version']}")
                    for ch in patch['changes']:
                        lines.append(ch)
                    lines.append("")
            else:
                ph = self._convert_wiki_to_markdown(raw_ph)
                lines.append(ph.strip())
                lines.append("")

        lines.append("---")
        lines.append("*This page was automatically generated from League of Legends Wiki data.*")
        return '\n'.join(lines)

    def _parse_item_info_block(self, content: str) -> Optional[Dict[str, str]]:
        """Extract top-level parameters from an Item info template block on the page."""
        if not content:
            return None
        m = re.search(r'\{\{\s*Item info', content)
        if not m:
            return None
        block = self._extract_balanced_braces(content, m.start())
        if not block:
            return None
        return self._parse_template_params(block)

    def _item_stat_label(self, key: str) -> str:
        labels = {
            'ad': 'Attack Damage',
            'ah': 'Ability Haste',
            'ap': 'Ability Power',
            'apunique': 'Ability Power (Unique)',
            'armor': 'Armor',
            'armorunique': 'Armor (Unique)',
            'armpen': 'Armor Penetration',
            'rpen': 'Armor Penetration',
            'as': 'Attack Speed',
            'cdr': 'Cooldown Reduction',
            'cdrunique': 'Cooldown Reduction (Unique)',
            'crit': 'Critical Strike Chance',
            'critdamage': 'Critical Strike Damage',
            'gp10': 'Gold per 10 sec',
            'hp': 'Health',
            'hp5': 'Health Regen (per 5s)',
            'hp5flat': 'Health Regen (Flat)',
            'hsp': 'Heal and Shield Power',
            'lethality': 'Lethality',
            'lethalityunique': 'Lethality (Unique)',
            'lifesteal': 'Life Steal',
            'mana': 'Mana',
            'mp5': 'Mana Regen (per 5s)',
            'mp5flat': 'Mana Regen (Flat)',
            'mpen': 'Magic Penetration',
            'mpenflat': 'Magic Penetration (Flat)',
            'mr': 'Magic Resist',
            'ms': 'Movement Speed',
            'msflat': 'Movement Speed (Flat)',
            'msunique': 'Movement Speed (Unique)',
            'omnivamp': 'Omnivamp',
            'pvamp': 'Physical Vamp',
            'spellvamp': 'Spell Vamp',
            'tenacity': 'Tenacity',
            'spec': 'Special',
            'spec2': 'Special 2',
        }
        return labels.get(key, key)

    def _item_stat_format(self, key: str, val: Any) -> str:
        percent_keys = {"armpen","as","cdr","crit","critdamage","hsp","lifesteal","ms","hp5","mp5","mpen","omnivamp","pvamp","spellvamp","tenacity"}
        try:
            f = float(val)
            if key in percent_keys:
                return f"${f:.1f}\\%$"
            # integers prefer no trailing .0
            if abs(f - round(f)) < 1e-9:
                return f"${int(round(f))}$"
            return f"${f}$"
        except Exception:
            return str(val)

    def _compute_item_combine_cost(self, item_name: str, item_data: Dict[str, Any]) -> Optional[int]:
        """Compute combine cost as buy - sum(child buys)."""
        try:
            buy = item_data.get('buy')
            recipe = item_data.get('recipe')
            if buy is None or recipe is None:
                return None
            parts: List[str] = []
            if isinstance(recipe, dict):
                parts = [str(v) for v in recipe.values()]
            elif isinstance(recipe, list):
                parts = [str(x) for x in recipe]
            total_parts = 0
            resolved = 0
            for p in parts:
                sub = self._load_item_module_data(p)
                if sub and sub.get('buy') is not None:
                    try:
                        total_parts += int(float(sub['buy']))
                        resolved += 1
                    except Exception:
                        pass
            # If we couldn't resolve any component prices, avoid returning a bogus combine cost
            if resolved == 0:
                return None
            return int(float(buy)) - total_parts
        except Exception:
            return None

    # ------ Rune helpers ------
    def _extract_rune_data(self, rune_name: str, content: str) -> Dict[str, Any]:
        data = {"name": rune_name, "notes": [], "patch_history": []}
        # Notes section
        m = re.search(r'==\s*Notes\s*==([\s\S]*?)(?==\s*[A-Z][^=]+=|\Z)', content, flags=re.IGNORECASE)
        if m:
            raw = m.group(1)
            notes = self._format_notes(raw, champion_name=None)
            data['notes'] = [ln for ln in notes.split('\n') if ln.strip()]
        # Patch History: either explicit section or embedded Scroll box
        ph_match = re.search(r'==\s*Patch History\s*==([\s\S]*?)(?==\s*[A-Z][^=]+=|\Z)', content, flags=re.IGNORECASE)
        ph_text = ph_match.group(1) if ph_match else ''
        if not ph_text:
            # try to extract from Scroll box content parameter
            sm = re.search(r'\{\{\s*Scroll box\s*\|\s*content\s*=([\s\S]*?)\}\}', content, flags=re.IGNORECASE)
            if sm:
                ph_text = sm.group(1)
        if ph_text:
            # Reuse champion patch parsing by building a synthetic page
            sections = re.split(r';?\[\[([V\d\.]+)\]\]', ph_text)
            patches = []
            for i in range(1, len(sections), 2):
                if i + 1 < len(sections):
                    version = sections[i]
                    changes_text = sections[i + 1].strip()
                    changes: List[str] = []
                    for raw in changes_text.split('\n'):
                        if not raw.strip():
                            continue
                        m2 = re.match(r'^(\*+)\s*(.*)$', raw.rstrip())
                        if not m2:
                            continue
                        level = len(m2.group(1))
                        content2 = m2.group(2).strip()
                        if not content2:
                            continue
                        content_md = self._convert_wiki_to_markdown(content2)
                        indent = '  ' * (level - 1)
                        changes.append(f"{indent}- {content_md}")
                    if changes:
                        patches.append({"version": version, "changes": changes})
            data['patch_history'] = patches
        return data

    def _generate_rune_markdown(self, data: Dict[str, Any]) -> str:
        lines: List[str] = []
        lines.append(f"# {data['name']}")
        lines.append("")
        lines.append("## Table of Contents")
        if data.get('notes'):
            lines.append("- [Notes](#notes)")
        if data.get('patch_history'):
            lines.append("- [Patch History](#patch-history)")
        lines.append("")
        if data.get('notes'):
            lines.append("## Notes")
            lines.append("")
            for n in data['notes']:
                lines.append(n if n.strip().startswith('- ') else f"- {n}")
            lines.append("")
        if data.get('patch_history'):
            lines.append("## Patch History")
            lines.append("")
            for patch in data['patch_history']:
                lines.append(f"### {patch['version']}")
                for ch in patch['changes']:
                    lines.append(ch)
                lines.append("")
        lines.append("---")
        lines.append("*This page was automatically generated from League of Legends Wiki data.*")
        return '\n'.join(lines)
    
    def _generate_markdown(self, data: Dict[str, Any]) -> str:
        """Generate markdown content."""
        lines = []
        
        # Title
        lines.append(f"# {data['name']}")
        lines.append("")
        
        # Table of contents
        lines.append("## Table of Contents")
        if data['basic_info']:
            lines.append("- [Basic Information](#basic-information)")
        if data['stats']:
            lines.append("- [Statistics](#statistics)")
            if data.get('map_stats'):
                lines.append("  - [Map-specific Stats](#map-specific-stats)")
        if data.get('pets'):
            lines.append("- [Pets](#pets)")
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
            lines.append("|-----------|------:|")
            # Friendly labels for certain keys
            label_map = {
                'be': 'Blue Essence',
                'rp': 'Riot Points',
            }
            for key, value in data['basic_info'].items():
                if value:
                    label = label_map.get(str(key).lower(), key.title().replace('_', ' '))
                    lines.append(f"| **{label}** | {value} |")
            lines.append("")
        
        # Statistics
        if data['stats']:
            lines.append("## Statistics")
            lines.append("")
            lines.append("### Base Stats")
            lines.append("")
            lines.append("| Stat | Base | Growth |")
            lines.append("|------|-----:|-------:|")
            
            # Resource-aware labeling for mana/energy (do not hide any rows; all stats are important)
            resource = (data.get('basic_info', {}) or {}).get('resource')
            def display_label(name: str) -> Optional[str]:
                if name == 'mana':
                    if resource and str(resource).lower() == 'energy':
                        return 'energy'
                if name == 'mana_regen':
                    if resource and str(resource).lower() == 'energy':
                        return 'energy_regen'
                return name

            for stat_name, stat_data in data['stats'].items():
                if isinstance(stat_data, dict) and 'base' in stat_data and 'growth' in stat_data:
                    disp = display_label(stat_name)
                    base = stat_data['base']
                    growth = stat_data['growth']
                    # Format the stat name nicely
                    display_name = disp.replace('_', ' ').title()
                    if stat_name == 'attack_speed':
                        # For attack speed, show only the base here; growth is presented separately below
                        lines.append(f"| **{display_name}** | ${float(base):.3f}$ |  |")
                    else:
                        lines.append(f"| **{display_name}** | ${base}$ | $+{growth}$ |")
            # Merge Advanced Stats into the same table as base stats
            if data.get('advanced_stats'):
                adv = data['advanced_stats']
                # helpers to format numbers similar to base table
                def _fmt_num(val: Any, decimals: int = 3) -> str:
                    try:
                        f = float(val)
                        s = f"{f:.{decimals}f}"
                        s = s.rstrip('0').rstrip('.') if '.' in s else s
                        return f"${s}$"
                    except Exception:
                        return str(val)
                def _format_adv_value(value: Optional[Any], suffix: Optional[str] = None, percent: bool = False) -> Optional[str]:
                    if value is None:
                        return None
                    if percent:
                        try:
                            v = float(value)
                            val_s = f"${v:.1f}\\%$"
                        except Exception:
                            val_s = str(value)
                    else:
                        val_s = _fmt_num(value)
                    if suffix:
                        # Always keep unit outside of math for proper spacing
                        val_s = f"{val_s} {suffix}"
                    return val_s
                adv_rows: List[Tuple[str, Optional[Any], Optional[str], bool]] = [
                    ("Base Attack Speed", adv.get("base_attack_speed"), None, False),
                    ("Attack Speed Ratio", adv.get("attack_speed_ratio"), None, False),
                    ("Bonus AS per Level", adv.get("bonus_attack_speed_per_level_percent"), None, True),
                    ("Attack Windup", adv.get("attack_windup_percent"), None, True),
                    ("Windup Modifier", adv.get("windup_modifier"), None, False),
                    ("Missile Speed", adv.get("missile_speed"), "units/second", False),
                    ("Acquisition Radius", adv.get("acquisition_radius"), "units", False),
                    ("Gameplay Radius", adv.get("gameplay_radius"), "units", False),
                    ("Pathing Radius", adv.get("pathing_radius"), "units", False),
                    ("Selection Radius", adv.get("selection_radius"), "units", False),
                    ("Selection Height", adv.get("selection_height"), "units", False),
                    ("Critical Damage", adv.get("critical_damage_percent"), None, True),
                ]
                for label, value, suffix, is_percent in adv_rows:
                    val_s = _format_adv_value(value, suffix=suffix, percent=is_percent)
                    if val_s is not None:
                        lines.append(f"| **{label}** | {val_s} |  |")
            # end of combined stats table
            lines.append("")
            # Map-specific stats tables
            if data.get('map_stats'):
                lines.append("### Map-specific Stats")
                lines.append("")
                map_names = {
                    'aram': 'ARAM',
                    'urf': 'URF',
                    'usb': 'Ultimate Spellbook',
                }
                def _fmt_percent(mult: Any) -> str:
                    try:
                        v = float(mult) * 100.0
                        return f"${v:.1f}\\%$"
                    except Exception:
                        return str(mult)
                for key in ['aram', 'urf', 'usb']:
                    if key in data['map_stats'] and data['map_stats'][key]:
                        lines.append(f"#### {map_names.get(key, key.upper())}")
                        lines.append("")
                        lines.append("| Metric | Value |")
                        lines.append("|--------|------:|")
                        entry = data['map_stats'][key]
                        if 'dmg_dealt' in entry:
                            lines.append(f"| **Damage Dealt** | {_fmt_percent(entry['dmg_dealt'])} |")
                        if 'dmg_taken' in entry:
                            lines.append(f"| **Damage Taken** | {_fmt_percent(entry['dmg_taken'])} |")
                        if 'healing' in entry:
                            lines.append(f"| **Healing** | {_fmt_percent(entry['healing'])} |")
                        lines.append("")
        
        # Pets
        if data.get('pets'):
            lines.append("## Pets")
            lines.append("")
            for pet in data['pets']:
                name = pet.get('name') or 'Pet'
                lines.append(f"### {name}")
                lines.append("")
                # Attributes table: pick common infobox fields
                attrs_map = [
                    ('range', 'Range'),
                    ('sight', 'Sight'),
                    ('gold', 'Gold'),
                    ('exp', 'Experience'),
                    ('hp', 'Health'),
                    ('armor', 'Armor'),
                    ('magicresist', 'Magic Resist'),
                    ('ccresist', 'CC Resist'),
                    ('damage', 'Damage'),
                    ('damagetype', 'Damage Type'),
                    ('attackspeed', 'Attack Speed'),
                    ('movespeed', 'Move Speed'),
                    ('control', 'Control'),
                    ('targeting', 'Targeting'),
                    ('spelleffects', 'Spell Effects'),
                    ('onhit', 'On-Hit'),
                ]
                rows: List[Tuple[str, str]] = []
                for key, label in attrs_map:
                    val = pet.get(key)
                    if val:
                        # Convert wiki to markdown and apply units where sensible
                        if key in ('range',):
                            val_fmt = self._format_with_unit(self._clean_attribute_value(self._convert_wiki_to_markdown(val)), 'units')
                        elif key in ('attackspeed',):
                            val_fmt = self._format_with_unit(self._clean_attribute_value(self._convert_wiki_to_markdown(val)), 'attack speed')
                        else:
                            val_fmt = self._clean_attribute_value(self._convert_wiki_to_markdown(val))
                        # Skip values that look like misparsed field starts (e.g., 'control = ...')
                        if re.match(r'^[a-z][a-z_ ]*\s*=\s*', val_fmt, flags=re.IGNORECASE):
                            continue
                        rows.append((label, val_fmt))
                if rows:
                    lines.append("| Attribute | Value |")
                    lines.append("|-----------|------:|")
                    for label, val in rows:
                        lines.append(f"| **{label}** | {val} |")
                    lines.append("")

                # Pet abilities
                if pet.get('abilities'):
                    lines.append("**Abilities:**")
                    lines.append("")
                    for ab in pet['abilities']:
                        title = ab.get('title', '').strip()
                        desc = ab.get('description', '').strip()
                        if title:
                            lines.append(f"- **{title}:** {desc}")
                        elif desc:
                            lines.append(f"- {desc}")
                    lines.append("")
                lines.append("---")
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
                
                # (Blurb lines will be rendered after descriptions to match template ordering)
                
                # (Descriptions will be rendered after the attributes table to match template ordering)
                
                # Stats table (ordered to match Ability template)
                stats_items = []
                # Ranges and timings
                if ability.get('range'):
                    stats_items.append(('Range', self._format_with_unit(self._clean_attribute_value(ability['range']), 'units')))
                if ability.get('cast_time'):
                    stats_items.append(('Cast Time', self._format_with_unit(self._clean_attribute_value(ability['cast_time']), 'seconds')))
                if ability.get('target_range'):
                    stats_items.append(('Target Range', self._format_with_unit(self._clean_attribute_value(ability['target_range']), 'units')))
                if ability.get('attack_range'):
                    stats_items.append(('Attack Range', self._format_with_unit(self._clean_attribute_value(ability['attack_range']), 'units')))
                if ability.get('ai_range'):
                    stats_items.append(('AI Range', self._format_with_unit(self._clean_attribute_value(ability['ai_range']), 'units')))
                if ability.get('windup'):
                    stats_items.append(('Windup', self._format_with_unit(self._clean_attribute_value(ability['windup']), 'seconds')))
                if ability.get('collision_radius'):
                    stats_items.append(('Collision Radius', self._format_with_unit(self._clean_attribute_value(ability['collision_radius']), 'units')))
                if ability.get('effect_radius'):
                    stats_items.append(('Effect Radius', self._format_with_unit(self._clean_attribute_value(ability['effect_radius']), 'units')))
                if ability.get('angle'):
                    stats_items.append(('Angle', self._clean_attribute_value(ability['angle'])))
                if ability.get('inner_radius'):
                    stats_items.append(('Inner Radius', self._format_with_unit(self._clean_attribute_value(ability['inner_radius']), 'units')))
                if ability.get('tether_radius'):
                    stats_items.append(('Tether Radius', self._format_with_unit(self._clean_attribute_value(ability['tether_radius']), 'units')))
                if ability.get('width'):
                    stats_items.append(('Width', self._format_with_unit(self._clean_attribute_value(ability['width']), 'units')))
                if ability.get('speed'):
                    stats_items.append(('Speed', self._format_with_unit(self._clean_attribute_value(ability['speed']), 'units/second')))
                # Costs
                if ability.get('cost') and ability.get('costtype'):
                    cost_val = self._clean_attribute_value(ability['cost'])
                    if ability.get('cost2'):
                        cost_val2 = self._clean_attribute_value(ability['cost2'])
                        cost_val = f"{cost_val} / {cost_val2}"
                    stats_items.append(('Cost', f"{cost_val} {self._clean_attribute_value(ability['costtype'])}"))
                # Cooldowns and recharge
                if ability.get('cooldown') or ability.get('cooldown2'):
                    cd_parts: List[str] = []
                    if ability.get('cooldown'):
                        cd_parts.append(self._format_with_unit(self._clean_attribute_value(ability['cooldown']), 'seconds'))
                    if ability.get('cooldown2'):
                        cd_parts.append(self._format_with_unit(self._clean_attribute_value(ability['cooldown2']), 'seconds'))
                    stats_items.append(('Cooldown', ' / '.join([p for p in cd_parts if p])))
                if ability.get('recharge'):
                    stats_items.append(('Recharge', self._format_with_unit(self._clean_attribute_value(ability['recharge']), 'seconds')))
                if ability.get('cdstart'):
                    stats_items.append(('Cooldown Start', self._clean_attribute_value(self._convert_wiki_to_markdown(ability['cdstart']))))
                if ability.get('static'):
                    stats_items.append(('Static Cooldown', self._clean_attribute_value(self._convert_wiki_to_markdown(ability['static']))))
                if ability.get('ontargetcd'):
                    stats_items.append(('On-target Cooldown', self._format_with_unit(self._clean_attribute_value(ability['ontargetcd']), 'seconds')))
                if ability.get('ontargetcdstatic'):
                    stats_items.append(('On-target CD Static', self._clean_attribute_value(self._convert_wiki_to_markdown(ability['ontargetcdstatic']))))
                if ability.get('queue_time'):
                    stats_items.append(('Queue Time', self._format_with_unit(self._clean_attribute_value(ability['queue_time']), 'seconds')))
                # Targeting and effects
                if ability.get('targeting'):
                    stats_items.append(('Targeting', self._clean_attribute_value(self._convert_wiki_to_markdown(ability['targeting']))))
                if ability.get('affects'):
                    stats_items.append(('Affects', self._clean_attribute_value(self._convert_wiki_to_markdown(ability['affects']))))
                if ability.get('damage_type'):
                    stats_items.append(('Damage Type', self._clean_attribute_value(self._convert_wiki_to_markdown(ability['damage_type']))))
                if ability.get('spell_shield'):
                    stats_items.append(('Spell Shield', self._clean_attribute_value(self._convert_wiki_to_markdown(ability['spell_shield']))))
                if ability.get('spell_effects'):
                    stats_items.append(('Spell Effects', self._clean_attribute_value(self._convert_wiki_to_markdown(ability['spell_effects']))))
                if ability.get('projectile'):
                    stats_items.append(('Projectile', self._clean_attribute_value(self._convert_wiki_to_markdown(ability['projectile']))))
                # Extra boolean/status fields
                for key, label in [
                    ('outofrange', 'Out of Range Behavior'),
                    ('targetwarning', 'Target Warning'),
                    ('terraingrace', 'Terrain Grace'),
                    ('parry', 'Parry'),
                    ('callforhelp', 'Call For Help'),
                    ('grounded', 'Grounded'),
                    ('knockdown', 'Knockdown'),
                    ('silence', 'Silence'),
                ]:
                    if ability.get(key):
                        stats_items.append((label, self._clean_attribute_value(self._convert_wiki_to_markdown(ability[key]))))
                # Custom info fields
                if ability.get('customlabel') and ability.get('custominfo'):
                    stats_items.append((self._clean_attribute_value(ability['customlabel']), self._clean_attribute_value(self._convert_wiki_to_markdown(ability['custominfo']))))
                if ability.get('customlabel2') and ability.get('custominfo2'):
                    stats_items.append((self._clean_attribute_value(ability['customlabel2']), self._clean_attribute_value(self._convert_wiki_to_markdown(ability['custominfo2']))))
                
                if stats_items:
                    lines.append("| Attribute | Value |")
                    lines.append("|-----------|------:|")
                    for attr, value in stats_items:
                        lines.append(f"| **{attr}** | {value} |")
                    lines.append("")
                
                # Render descriptions in order, each followed by its own scaling (leveling) block if present
                rendered_tables: set[str] = set()
                printed_descs: set[str] = set()
                for idx in range(1, 7):
                    dkey = 'description' if idx == 1 else f'description{idx}'
                    lkey = 'leveling' if idx == 1 else f'leveling{idx}'
                    desc = ability.get(dkey)
                    lev = ability.get(lkey)
                    if desc:
                        lines.append(desc)
                        lines.append("")
                        # Track normalized text for deduplication with blurbs
                        norm = re.sub(r'\s+', ' ', re.sub(r'[\*`_~]', '', desc)).strip().lower()
                        if norm:
                            printed_descs.add(norm)
                    if lev:
                        # Handle possible mix of prose + one or more st-rendered tables in a single leveling value
                        lev_lines = lev.splitlines()
                        has_table = any(re.match(r'^\s*\|', ln) for ln in lev_lines)
                        if lev.lstrip().startswith('|') and has_table and not any(ln.strip() and not ln.lstrip().startswith('|') for ln in lev_lines):
                            # Pure table block(s): may contain one or more concatenated tables
                            blocks: List[str] = []
                            current: List[str] = []
                            for ln in lev_lines:
                                if re.match(r'^\s*\|\s*Attribute\s*\|\s*Value\s*\|\s*$', ln):
                                    if current:
                                        blocks.append('\n'.join(current).strip())
                                        current = []
                                current.append(ln)
                            if current:
                                blocks.append('\n'.join(current).strip())
                            if not blocks:
                                blocks = [lev]
                            for tbl in blocks:
                                tkey = tbl.strip()
                                if tkey:
                                    rendered_tables.add(tkey)
                                lines.append(tbl)
                                lines.append("")
                        else:
                            # Mixed or pure prose: emit prose first, then any tables with a blank line separating
                            prose_parts: List[str] = []
                            table_blocks: List[str] = []
                            current: List[str] = []
                            collecting_table = False
                            for ln in lev_lines:
                                if re.match(r'^\s*\|\s*Attribute\s*\|\s*Value\s*\|\s*$', ln):
                                    # starting a new table block
                                    if collecting_table and current:
                                        table_blocks.append('\n'.join(current).strip())
                                        current = []
                                    collecting_table = True
                                    current.append(ln)
                                elif collecting_table:
                                    current.append(ln)
                                else:
                                    if ln.strip():
                                        prose_parts.append(ln.strip())
                            if collecting_table and current:
                                table_blocks.append('\n'.join(current).strip())

                            if prose_parts:
                                # Render leveling prose as a single bullet line
                                lines.append(f"- {' '.join(prose_parts)}")
                                lines.append("")
                            for tbl in table_blocks:
                                tkey = tbl.strip()
                                if tkey:
                                    rendered_tables.add(tkey)
                                # Ensure a blank line already exists after prose; just append tables
                                lines.append(tbl)
                                lines.append("")
                
                # Any extra scaling tables not tied to a specific description
                if ability.get('extra_scaling'):
                    prev_was_table = False
                    seen_tables: set[str] = set()
                    emitted_any = False
                    def _emit_table_block(tbl: str):
                        nonlocal prev_was_table
                        nonlocal emitted_any
                        tkey = tbl.strip()
                        if not tkey:
                            return
                        if tkey in seen_tables or tkey in rendered_tables:
                            return
                        seen_tables.add(tkey)
                        if not emitted_any:
                            lines.append("**Scaling:**")
                            emitted_any = True
                        if prev_was_table:
                            lines.append("")
                        lines.append(tbl)
                        prev_was_table = True
                    for scaling in ability['extra_scaling']:
                        if scaling.lstrip().startswith('|'):
                            blocks: List[str] = []
                            current: List[str] = []
                            for ln in scaling.splitlines():
                                if re.match(r'^\s*\|\s*Attribute\s*\|\s*Value\s*\|\s*$', ln):
                                    if current:
                                        blocks.append('\n'.join(current).strip())
                                        current = []
                                current.append(ln)
                            if current:
                                blocks.append('\n'.join(current).strip())
                            if not blocks:
                                blocks = [scaling]
                            for tbl in blocks:
                                _emit_table_block(tbl)
                        else:
                            if not emitted_any:
                                lines.append("**Scaling:**")
                                emitted_any = True
                            lines.append(f"- {scaling}")
                            prev_was_table = False
                    if emitted_any:
                        lines.append("")
                
                # Optional flavor text (from Ability template)
                if ability.get('flavortext'):
                    ft = ability['flavortext']
                    if ft:
                        lines.append(f"*{ft}*")
                        lines.append("")

                # Blurbs (rendered after descriptions per template order)
                # Only output blurbs if no descriptions were printed to avoid duplication.
                if not printed_descs:
                    for key, style in [('blurb', 'bold'), ('blurb2', 'italic'), ('blurb3', 'plain'), ('blurb4', 'plain')]:
                        val = ability.get(key)
                        if not val:
                            continue
                        if style == 'bold' and not val.startswith('**'):
                            lines.append(f"**{val}**")
                        elif style == 'italic' and not val.startswith('*'):
                            lines.append(f"*{val}*")
                        else:
                            lines.append(val)
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
                        # 'change' may already include indentation and '- ' prefix
                        if re.match(r'^\s*-\s', change):
                            lines.append(change)
                        else:
                            lines.append(f"- {change}")
                lines.append("")
        
        # Trivia
        if data['trivia']:
            lines.append("## Trivia")
            lines.append("")
            for item in data['trivia']:
                if item.strip():
                    # 'item' may already include indentation and a list marker ('- ' or '1. ')
                    if re.match(r'^\s*(?:-\s|\d+\.\s)', item):
                        lines.append(item)
                    else:
                        lines.append(f"- {item}")
            lines.append("")
        
        # Footer
        lines.append("---")
        lines.append("*This page was automatically generated from League of Legends Wiki data.*")
        
        return '\n'.join(lines)

    def _postprocess_markdown(self, md: str) -> str:
        """Final cleanup of markdown artifacts: remove specific stray tokens and normalize spaces."""
        # Generic cleanup (avoid champion-specific hacks)
        # Remove stray asterisks after normalized labels like '**Bug Fix:**' followed by an extra '*'
        md = re.sub(r'(\*\*\s*(?:Bug Fix|Undocumented|New Effect):\s*\*\*)\*', r'\1', md)
    # (Removed cast-time bullet safety net; ordering fix in _format_notes makes it unnecessary.)
        # Remove stray single quotes flanking emphasis markers or emphasized spans
        # Cases like: ' **Name**' -> **Name** and **Name**' -> **Name**
        md = re.sub(r"(?m)^\s*'\s*(\*{1,3}[^\n]+\*{1,3})", r"\1", md)
        md = re.sub(r"(\*{1,3}[^\n]+\*{1,3})\s*'\s*$", r"\1", md)
        # Paired quotes around emphasis: '**Name**' or '*Title*' -> **Name** / *Title*
        md = re.sub(r"'\s*(\*{1,3}[^\n]+?\*{1,3})\s*'", r"\1", md)
        # Normalize italic possessive placement: *Word's* -> *Word*’s (prefer typographic apostrophe)
        md = re.sub(r"\*([A-Za-z][^*\n]*?)'s\*", r"*\1*’s", md)
        # Also handle plural possessive inside italics: *Words'* -> *Words*’
        md = re.sub(r"\*([A-Za-z][^*\n]*?)'\*", r"*\1*’", md)
        # Normalize bold possessive placement: **Word’s** -> **Word**’s and **Word's** -> **Word**’s
        md = re.sub(r"\*\*([^*\n]+?)’s\*\*", r"**\1**’s", md)
        md = re.sub(r"\*\*([^*\n]+?)'s\*\*", r"**\1**’s", md)
        # Also handle s’-only case inside bold: **Words’** -> **Words**’
        md = re.sub(r"\*\*([^*\n]+?)’\*\*", r"**\1**’", md)
        # Fix mixed bold+italic around possessive from apostrophe parser: **Word*’s* -> **Word**’s
        md = re.sub(r"\*\*([^*\n]+?)\*’s\*", r"**\1**’s", md)
        # Fix repeated word artifacts
        md = re.sub(r'\b(critical|magic|physical) damage damage\b', r'\1 damage', md, flags=re.IGNORECASE)
        # Additional repeated-word guards for common gameplay terms
        repeated_terms = [
            'cooldown', 'range', 'shield', 'duration', 'cost', 'speed', 'movement speed', 'attack speed',
            'health', 'mana', 'energy', 'armor', 'magic resist', 'ability haste', 'healing', 'damage'
        ]
        for term in repeated_terms:
            pattern = re.compile(rf"\b({re.escape(term)})\s+\1\b", flags=re.IGNORECASE)
            md = pattern.sub(r"\1", md)
        # Repair mismatched emphasis around 'bonus' near common tokens without stripping valid bold
        # Examples fixed: '*bonus** AD' -> '**bonus** AD', '**bonus* AP' -> '**bonus** AP'
        md = re.sub(r'\*bonus\*\*\s*(?=AD\b)', r'**bonus** ', md)
        md = re.sub(r'\*bonus\*\*\s*(?=AP\b)', r'**bonus** ', md)
        md = re.sub(r'\*\*bonus\*\s*(?=AD\b)', r'**bonus** ', md)
        md = re.sub(r'\*\*bonus\*\s*(?=AP\b)', r'**bonus** ', md)
        # Also handle triple-then-double case: '***bonus** X' -> '**bonus** X'
        md = re.sub(r'\*\*\*bonus\*\*\s*(?=AD\b)', r'**bonus** ', md)
        md = re.sub(r'\*\*\*bonus\*\*\s*(?=AP\b)', r'**bonus** ', md)
        # Replace asterisk multiplication between numbers with × to avoid markdown italic parsing
        md = re.sub(r'(?<=\d)\*(?=\d)', '×', md)
        # Similarly for armor and attack speed
        md = re.sub(r'\*bonus\*\*\s*(?=armor\b)', r'**bonus** ', md, flags=re.IGNORECASE)
        md = re.sub(r'\*\*bonus\*\s*(?=armor\b)', r'**bonus** ', md, flags=re.IGNORECASE)
        md = re.sub(r'\*bonus\*\*\s*(?=attack speed\b)', r'**bonus** ', md, flags=re.IGNORECASE)
        md = re.sub(r'\*\*bonus\*\s*(?=attack speed\b)', r'**bonus** ', md, flags=re.IGNORECASE)
        md = re.sub(r'\*\*\*bonus\*\*\s*(?=armor\b)', r'**bonus** ', md, flags=re.IGNORECASE)
        md = re.sub(r'\*\*\*bonus\*\*\s*(?=attack speed\b)', r'**bonus** ', md, flags=re.IGNORECASE)
        # Fix 'modified' mismatched emphasis without removing valid bold if present
        md = re.sub(r'\*modified\*\*\s*', r'**modified** ', md)
        # Generic tidy for any Width line: strip math markers and add a comma between two numeric ranges if missing
        def _clean_width_line(m: re.Match) -> str:
            prefix = m.group('prefix')
            raw = m.group(0)
            line = raw.replace('$', '')
            range_pattern = r'(\d+(?:\.\d+)?\s*-\s*\d+(?:\.\d+)?)'
            ranges = re.findall(range_pattern, line)
            soldiers = 'soldier' in line.lower()
            units = 'unit' in line.lower()
            if ranges:
                parts = []
                if len(ranges) >= 1:
                    part = ranges[0].strip()
                    if soldiers:
                        part += ' soldiers'
                    parts.append(part)
                if len(ranges) >= 2:
                    part2 = ranges[1].strip()
                    # Append 'units' for the distance range unless it's already present
                    if not units:
                        part2 += ' units'
                    parts.append(part2)
                body = ', '.join(parts)
                return f"{prefix} {body}"
            # If no ranges found, still return remaining line content sans math markers
            content = re.sub(r'^.*?\*\*Width:\*\*\s*', '', line).strip()
            return f"{prefix} {content}"
        md = re.sub(r'(?m)^(?P<prefix>\s*-\s+\*\*Width:\*\*)[^\n]*$', _clean_width_line, md)
        # Targeted fix inside Patch History: unbalanced emphasis '*Word**' -> '**Word**'
        def _fix_patch_history(section: str) -> str:
            s = section
            # Unbalanced emphasis '*Word**' or '**Word*' -> '**Word**'
            s = re.sub(r'(?<!\*)\*([A-Za-z][A-Za-z\s\'!,-]{0,60})\*\*', r'**\1**', s)
            s = re.sub(r'\*\*([A-Za-z][A-Za-z\s\'!,-]{0,60}):\*', r'**\1:**', s)
            s = re.sub(r'\*([A-Za-z][A-Za-z\s\'!,-]{0,60}):\*\*', r'**\1:**', s)
            # Normalize a couple of frequent labels variations
            # Convert triple-asterisk labels to double first
            s = re.sub(r'\*\*\*\s*([A-Za-z][A-Za-z\s]+):\*\*\*', r'**\1:**', s)
            s = re.sub(r'\*\*\s*Bug\s*fix(?:es)?\s*:\*{1,3}', '**Bug Fixes:**', s, flags=re.IGNORECASE)
            s = re.sub(r'\*\*\s*New\s*Effect\s*:\*{1,3}', '**New Effect:**', s, flags=re.IGNORECASE)
            s = re.sub(r'\*\*\s*Undocumented\s*:\*{1,3}', '**Undocumented:**', s, flags=re.IGNORECASE)
            return s
        m = re.search(r'(## Patch History)([\s\S]*?)(\n##\s|\Z)', md)
        if m:
            before, section, after = md[:m.start(2)], m.group(2), md[m.end(2):]
            fixed_section = _fix_patch_history(section)
            md = before + fixed_section + after
        # Repair common damage scaling formatting with pass-through factors (e.g., ×0.6)
        def _fix_damage_scaling_lines(text: str) -> str:
            lines = text.split('\n')
            out = []
            i = 0
            while i < len(lines):
                line = lines[i]
                # Merge orphan bullet like '- *bonus AD)' into previous line, if applicable (rare artifact)
                m_orphan = re.match(r'^\s*-\s*\*bonus\s+(AD|AP)\)\s*$', line, flags=re.IGNORECASE)
                if m_orphan:
                    if out:
                        token = m_orphan.group(1).upper()
                        suffix = f" bonus {token})"
                        prev = out[-1].rstrip()
                        # Avoid duplicating the suffix if it's already present
                        if not prev.endswith(suffix):
                            # Ensure a separating space
                            sep = '' if prev.endswith(' ') else ' '
                            out[-1] = prev + sep + suffix
                    i += 1
                    continue
                mline = re.match(r'^(?P<prefix>\s*-\s+\*\*(?:Physical|Magic) Damage:\*\*\s*)(?P<body>.+)$', line)
                if mline:
                    prefix = mline.group('prefix')
                    body = mline.group('body')
                    # Capture first math range as base
                    mbase = re.search(r'(\$[^$]+\$)', body)
                    base = mbase.group(1) if mbase else None
                    # Capture presence of bonus AD/AP percent in the first segment
                    bonus_label = None
                    mbonus = re.search(r'\(\+\s*([0-9]+\%[^)]*?)\)', body)
                    if mbonus:
                        bonus_label = mbonus.group(1)
                    # Detect a pass-through multiplication factor (e.g., ×0.6) after the first segment
                    passthrough = None
                    mp = re.search(r'×\s*(0\.?\d+)', body)
                    if mp and base:
                        factor = mp.group(1)
                        passthrough = f"{base} × {factor}"
                    # Detect bonus ratio pass-through (e.g., $100×0.6$%)
                    bonus_pt = None
                    mbpt = re.search(r'(\$\s*\d+\s*×\s*0\.?\d+\s*\$\s*\%?)', body)
                    if mbpt:
                        # Ensure trailing '%' after math if present
                        val = mbpt.group(1)
                        if not val.endswith('%') and '%'+val not in body:
                            # Leave as-is; a following % may exist
                            pass
                        bonus_pt = val
                    if base:
                        rebuilt = base
                        if bonus_label:
                            rebuilt += f" (+ {bonus_label})"
                        if passthrough:
                            rebuilt += f"; pass-through: {passthrough}"
                            if bonus_pt:
                                rebuilt += f" (+ {bonus_pt} bonus AD)"
                        out.append(prefix + rebuilt)
                        i += 1
                        continue
                out.append(line)
                i += 1
            return '\n'.join(out)
        md = _fix_damage_scaling_lines(md)
        # Unwrap links that are not same-page anchors: keep [text](#anchor), strip others to plain text
        if self._current_page_basename:
            current = re.escape(self._current_page_basename)
            md = re.sub(rf'(?<!\!)\[(?P<text>[^\]]+)\]\(\./{current}\.md#(?P<anchor>[^)]+)\)', r'[\g<text>](#\g<anchor>)', md)
        def _unwrap_nonlocal_links(mo: re.Match) -> str:
            text = mo.group('text')
            href = mo.group('href')
            # Keep same-page anchors only
            if href.strip().startswith('#'):
                return f"[{text}]({href})"
            return text
        md = re.sub(r'(?<!\!)\[(?P<text>[^\]]+)\]\((?P<href>[^)]+)\)', _unwrap_nonlocal_links, md)
        # Drop stray wiki template param lines that leaked and are not part of a proper md table region
        def _is_table_header(i: int, lines: List[str]) -> bool:
            if i + 1 < len(lines):
                # header like: '| a | b |' then '| --- | --- |'
                return bool(re.match(r"^\|(?:[^|]+\|)+$", lines[i].strip())) and bool(re.match(r"^\|\s*-+\s*(\|\s*-+\s*)+\|?\s*$", lines[i+1].strip()))
            return False
        md_lines = md.split('\n')
        cleaned: List[str] = []
        i = 0
        while i < len(md_lines):
            ln = md_lines[i]
            # Sanitize malformed header lines like "|leveling2 = | Attribute | Value |" -> "| Attribute | Value |"
            if ln.strip().startswith('|'):
                ln = re.sub(r"^\|\s*[a-z][^=]{0,40}=\s*(\|.*)$", r"\1", ln, flags=re.IGNORECASE)
            # Keep intact if part of a markdown table (header or separator or data row)
            if ln.strip().startswith('|'):
                if _is_table_header(i, md_lines):
                    cleaned.append(ln)
                    i += 1
                    cleaned.append(md_lines[i])  # separator line
                    i += 1
                    # Consume following data rows as-is
                    while i < len(md_lines) and md_lines[i].strip().startswith('|'):
                        cleaned.append(md_lines[i])
                        i += 1
                    continue
                # Non-table pipe lines: drop if they look like '|key = value'
                if re.match(r"^\s*\|\s*[a-z][^=]{0,40}=", ln, flags=re.IGNORECASE):
                    i += 1
                    continue
                # Drop lone pipe-name lines like '|notes' or '|leveling6' (no additional pipes)
                if ln.count('|') == 1 and re.match(r"^\s*\|\s*[A-Za-z0-9_ ]+\s*$", ln):
                    i += 1
                    continue
            # Also drop escaped pipe param lines like '\\|key = value'
            if re.match(r"^\s*\\\|\s*[a-z][^=]{0,40}=", ln, flags=re.IGNORECASE):
                i += 1
                continue
            # And escaped lone pipe-name lines like '\\|notes'
            if re.match(r"^\s*\\\|\s*[A-Za-z0-9_ ]+\s*$", ln):
                i += 1
                continue
            cleaned.append(ln)
            i += 1
        md = '\n'.join(cleaned)
        # Last-resort: rescue any ct param runs that might still be present in prose
        try:
            md = self._rescue_ct_param_runs(md)
        except Exception:
            pass
        # Note: Avoid global asterisk normalization; prefer source-specific fixes above.
        # Normalize excessive spaces but keep newlines and preserve leading indentation
        # Collapse runs of spaces only after non-space characters to avoid destroying list indents
        md = re.sub(r'(?m)(?<=\S)[ \t]{2,}', ' ', md)
        # Trim stray duplicate quotes produced by template stripping
        md = md.replace("''", "'").replace('""', '"')
        # Fix possessive quote glitches like Soldiers' ' attacks -> Soldiers' attacks and Sun Disc's' -> Sun Disc's
        md = md.replace("' '", "'")
        md = md.replace("'s'", "'s")
        return md

    def _clean_attribute_value(self, value: str) -> str:
        """Clean leaked template fragments from attribute values and normalize common tokens.
        - Removes substrings like 'control = ...' or 'grounded = True'
        - Normalizes booleans and common enums (true/false/special)
        - Strips extraneous whitespace
        """
        if not value:
            return value
        v = value.strip()
        # Remove any leading/trailing field-like fragments 'key ='
        v = re.sub(r'\b[a-z][a-z_ ]*\s*=\s*', '', v, flags=re.IGNORECASE)
        v = v.strip()
        low = v.lower()
        if low in ('true', 'yes'):
            return 'True'
        if low in ('false', 'no'):
            return 'False'
        if low in ('special', 'projectile', 'auto'):
            # Normalize a few enum-like tokens
            return low.title()
        return v

    def _format_with_unit(self, value: str, unit: str) -> str:
        """Append a unit only when the value looks numeric-like and doesn't already include the unit."""
        if not value:
            return value
        v = value.strip()
        low = v.lower()
        if any(tok in low for tok in ['none', 'instant', 'no cast', 'no cost', 'n/a', '—', 'na', 'global']):
            return v
        # If already has the unit string, return as is
        if unit.lower() in low:
            return v
        # If percentage present, don't append time/distance units
        if '%' in v:
            return v
        # If contains any digit or a math formula marker, treat as numeric-like
        if re.search(r"\d", v) or ('$' in v):
            # Strip accidental leading letters like 'cr 400' -> '400'
            v_clean = re.sub(r'^[A-Za-z]+\s+', '', v)
            return f"{v_clean} {unit}"
        return v


def main():
    """Main entry point."""
    parser = argparse.ArgumentParser(description="Simple LoL Wiki to Markdown Converter")
    group = parser.add_mutually_exclusive_group(required=True)
    group.add_argument("--champion", help="Champion name to convert")
    group.add_argument("--item", help="Item name to convert")
    group.add_argument("--rune", help="Rune name to convert")
    parser.add_argument("--wiki-root", default="./out", help="Path to extracted wiki files")
    parser.add_argument("--output", default="./markdown", help="Output directory")
    
    args = parser.parse_args()
    
    converter = SimpleLoLConverter(args.wiki_root, args.output)
    if args.champion:
        result = converter.convert_champion(args.champion)
    elif args.item:
        result = converter.convert_item(args.item)
    elif args.rune:
        result = converter.convert_rune(args.rune)
    else:
        result = None
    
    if result:
        print(f"Conversion completed successfully!")
        print(f"Output: {result}")
    else:
        print("Conversion failed!")
        sys.exit(1)


if __name__ == "__main__":
    main()