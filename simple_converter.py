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
        if not m:
            return pets
        tab_body = m.group(1)
        # Split by tab delimiters `|-|`
        chunks = re.split(r'\|\-\|', tab_body)
        for chunk in chunks:
            # Extract tab title like `Sand Soldier=`
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
            # Abilities: prefer from infobox 'abilities' field if present; else parse after the infobox
            abilities: List[Dict[str, str]] = []
            infobox_abilities = info.get('abilities')
            if isinstance(infobox_abilities, str) and infobox_abilities.strip():
                text = infobox_abilities
                # Trim any trailing 'General' or other headers starting with '=' that sometimes follow abilities
                hdr = re.search(r'(?m)^\s*={2,}[^\n]*', text)
                if hdr:
                    text = text[:hdr.start()].rstrip()
                for am in re.finditer(r'(?m)^;\s*(?P<title>[^\n]+)\n(?P<body>[\s\S]*?)(?=^\s*;|\Z)', text):
                    title = am.group('title').strip()
                    body = am.group('body').strip()
                    # Skip header-like titles
                    if title.startswith('='):
                        continue
                    if title or body:
                        abilities.append({
                            'title': self._convert_wiki_to_markdown(title),
                            'description': self._convert_wiki_to_markdown(body)
                        })
            if not abilities:
                # Fallback: parse abilities following the infobox
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
            if not key_m:
                # Can't parse further
                break
            key = key_m.group(1)
            i += key_m.end()
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
                try:
                    out[key] = self._parse_lua_simple_kv(sub)
                except Exception:
                    out[key] = sub
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
            val = raw_val.strip()
            if dst_key == 'notes':
                ability_data[dst_key] = self._format_notes(val)
            else:
                ability_data[dst_key] = self._convert_wiki_to_markdown(val)

        # Collect any free-floating {{st|...}} blocks not tied to a parameter line
        extra_st = self._collect_free_st_blocks(content)
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
        """Extract trivia items, preserving nested bullets as indentation."""
        trivia: List[str] = []
        for raw in trivia_content.split('\n'):
            if not raw.strip():
                continue
            m = re.match(r'^(\*+)\s*(.*)$', raw.rstrip())
            if not m:
                continue
            level = len(m.group(1))
            content = m.group(2).strip()
            if not content:
                continue
            item_md = self._convert_wiki_to_markdown(content)
            indent = '  ' * (level - 1)
            trivia.append(f"{indent}- {item_md}")
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
        # Tooltips: {{tt|value|tooltip}} -> value (tooltip)
        text = re.sub(r'\{\{tt\|([^}]+)\}\}', self._convert_tt, text)

        # Convert styled text
        text = re.sub(r'\{\{as\|([^}|]+)(?:\|[^}]*)?\}\}', r'\1', text)
        # Skill tabs: {{st|Label1|Value1|Label2|Value2|...}} -> table (brace-aware)
        text = self._convert_st_blocks(text)
        text = re.sub(r'\{\{sbc\|([^}]+)\}\}', r'**\1**', text)  # Small bold caps
        text = re.sub(r'\{\{sti\|([^}]+)\}\}', r'*\1*', text)  # Styled italic

        # Fix double bold markers
        text = re.sub(r'\*\*\*\*([^*]+):\*\*', r'**\1:**', text)  # ****text:** -> **text:**

        # Convert section links BEFORE general links so they don't get captured by the general rule
        # [[Page#Anchor|Display]] -> [Display](./Page.md#Anchor)
        text = re.sub(r'\[\[([^|\]#]+)#([^|\]]+)(?:\|([^\]]*))?\]\]', self._convert_section_link, text)
        # Convert links with better handling (general case)
        text = re.sub(r'\[\[([^|\]]+)(?:\|([^]]*))?\]\]', self._convert_link, text)

        # Convert champion/ability references
        text = re.sub(r'\{\{ci\|([^}|]+)(?:\|[^}]*)?\}\}', r'**\1**', text)
        # cis: champion info possessive (e.g., {{cis|Lulu}} -> **Lulu**’s)
        text = re.sub(r'\{\{cis\|([^}|]+)(?:\|[^}]*)?\}\}', r'**\1**’s', text)
        # ai template: {{ai|Ability|Champion}} or {{ai|Ability|Champion|Display}}
        # Prefer 3rd arg (Display) when present, else 1st arg (Ability). Keep italics, drop linking.
        def _ai_repl(m: re.Match) -> str:
            full = m.group(0)
            # Capture up to 3 args loosely to allow pipes
            inner = full[2:-2]  # strip '{{' '}}'
            parts = inner.split('|')
            # parts[0] == 'ai'
            args = parts[1:]
            ability = args[0].strip() if len(args) >= 1 else ''
            display = args[2].strip() if len(args) >= 3 and args[2].strip() else ability
            return f"*{display}*"
        text = re.sub(r'\{\{ai\|[^}]+\}\}', _ai_repl, text)
        # ais template: like ai but possessive; render italic with trailing ’s
        def _ais_repl(m: re.Match) -> str:
            full = m.group(0)
            inner = full[2:-2]
            parts = inner.split('|')
            args = parts[1:]
            ability = args[0].strip() if len(args) >= 1 else ''
            display = args[2].strip() if len(args) >= 3 and args[2].strip() else ability
            # Use typographic apostrophe for possessive
            return f"*{display}*’s"
        text = re.sub(r'\{\{ais\|[^}]+\}\}', _ais_repl, text)
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
        text = re.sub(r'\{\{ri\|([^}|]+)(?:\|[^}]*)?\}\}', r'*\1*', text)  # Rune references
        text = re.sub(r'\{\{ii\|([^}|]+)(?:\|[^}]*)?\}\}', r'*\1*', text)  # Item references
        # TFT item template: {{TFT Item|Name}} -> *Name*
        # The surrounding text already mentions Teamfight Tactics item, so postfix is redundant
        text = re.sub(r'\{\{TFT Item\|([^}|]+)(?:\|[^}]*)?\}\}', r'*\1*', text)

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
        text = re.sub(r'\{\{cai\|([^}|]+)(?:\|[^}]*)?\}\}', r'\1', text, flags=re.IGNORECASE)
        text = re.sub(r'\{\{cid\|([^}|]+)(?:\|[^}]*)?\}\}', r'\1', text, flags=re.IGNORECASE)
        # Items (plural) -> italicize like ii
        text = re.sub(r'\{\{iis\|([^}|]+)(?:\|[^}]*)?\}\}', r'*\1*', text, flags=re.IGNORECASE)
        # Styled italic (linked variant) -> italicize content
        text = re.sub(r'\{\{stil\|([^}|]+)(?:\|[^}]*)?\}\}', r'*\1*', text, flags=re.IGNORECASE)
        # Gold-related templates -> append unit
        text = re.sub(r'\{\{g\|([^}|]+)(?:\|[^}]*)?\}\}', r'\1 gold', text, flags=re.IGNORECASE)
        text = re.sub(r'\{\{\s*g\s*\}\}', 'gold', text, flags=re.IGNORECASE)
        text = re.sub(r'\{\{gold value\|([^}|]+)(?:\|[^}]*)?\}\}', r'\1 gold', text, flags=re.IGNORECASE)
        # Symbol helpers
        text = re.sub(r'\{\{times(?:\|[^}]*)?\}\}', '×', text, flags=re.IGNORECASE)
        text = re.sub(r'\{\{degree(?:\|[^}]*)?\}\}', '°', text, flags=re.IGNORECASE)
        text = re.sub(r'\{\{plus(?:\|[^}]*)?\}\}', '+', text, flags=re.IGNORECASE)
        text = re.sub(r'\{\{tftt\}\}', 'Teamfight Tactics', text, flags=re.IGNORECASE)
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
        # Wild Rift wrappers
        text = re.sub(r'\{\{wr\|([^}|]+)(?:\|[^}]*)?\}\}', r'\1', text, flags=re.IGNORECASE)
        text = re.sub(r'\{\{wri\|([^}|]+)(?:\|[^}]*)?\}\}', r'*\1*', text, flags=re.IGNORECASE)
        # Unit/item plural wrappers
        text = re.sub(r'\{\{uis\|([^}|]+)(?:\|[^}]*)?\}\}', r'*\1*', text, flags=re.IGNORECASE)
        text = re.sub(r'\{\{items\|([^}|]+)(?:\|[^}]*)?\}\}', r'*\1*', text, flags=re.IGNORECASE)
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

        # Remove references
        text = re.sub(r'<ref[^>]*>.*?</ref>', '', text, flags=re.DOTALL)
        text = re.sub(r'<ref[^>]*\s*/>', '', text)

        # Clean up HTML tags
        text = re.sub(r'<br\s*/?>', '\n', text)
        text = re.sub(r'<[^>]+>', '', text)

        # Convert bold/italic with better handling
        text = re.sub(r"'''([^']+)'''", r'**\1**', text)
        text = re.sub(r"''([^']+)''", r'*\1*', text)

        # Fix malformed bold/italic combinations and extra asterisks
        text = re.sub(r'\*\*\*\*([^*]+)\*\*', r'**\1**', text)  # ****text** -> **text**
        text = re.sub(r'\*\*\*([^*]+)\*\*([^*]+)\*', r'**\1\2**', text)  # ***text**other* -> **textother**

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

        # Clean up extra whitespace and keep newlines to preserve tables and lists
        text = re.sub(r'[ \t]+', ' ', text)
        text = text.strip()

        return text

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
            value_md = self._convert_wiki_to_markdown(value) if value else ''
            rows.append((label, value_md))
        if not rows:
            return ''
        lines = ["| Attribute | Value |", "|-----------|------:|"]
        for label, value in rows:
            lines.append(f"| **{label}** | {value} |")
        return "\n".join(lines)
    
    def _convert_ap_formula(self, match) -> str:
        """Convert {{ap|...}} formulas to markdown math."""
        content = match.group(1)
        if ' to ' in content:
            parts = content.split(' to ')
            if len(parts) == 2:
                return f"${parts[0].strip()}-{parts[1].strip()}$"
        return f"${content}$"
    
    def _convert_pp_formula(self, match) -> str:
        """Convert {{pp|...}} templates to a compact list of values aligned to level breakpoints.
        Expected common form: {{pp|<start> to <end> for <n>|<levels>|...}}
        - Values: generate n evenly spaced values from start..end inclusive
        - Levels: tokens separated by ';' with optional 'a to b' inclusive expansion
        Output (compact, level-annotated): e.g. '30@1; 35–90@7–18'. Falls back to endpoints when needed.
        """
        content = match.group(1)
        parts = [p.strip() for p in content.split('|') if p.strip()]
        if not parts:
            return f"${content}$"
        values_spec = parts[0]
        levels_spec = parts[1] if len(parts) > 1 and '=' not in parts[1] else ''
        # Optional key label via named params; prioritize 'type=' (as seen in Sun Disc hp), then others
        # Default behavior: if no label provided, omit legend (level is implied)
        key_label: Optional[str] = None
        if len(parts) > 2:
            # First pass: look for 'type='
            for extra in parts[2:]:
                if '=' in extra:
                    k, v = extra.split('=', 1)
                    if k.strip().lower() == 'type' and v.strip():
                        key_label = v.strip()
                        break
            # Second pass: fallbacks if type not set
            if key_label is None:
                for extra in parts[2:]:
                    if '=' in extra:
                        k, v = extra.split('=', 1)
                        k = k.strip().lower()
                        v = v.strip()
                        if k in ('key', 'label', 'per', 'by', 'scale') and v:
                            key_label = v
                            break

        # Try to parse canonical 'start to end for n' form
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
            values_num = seq
            values = [self._pp_num_fmt(x) for x in seq]
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
                        f = float(s)
                        values.append(self._pp_num_fmt(f))
                        values_num.append(f)
                    else:
                        values.append(s)
                        values_num.append(None)
                if levels_spec:
                    levels = self._pp_parse_levels(levels_spec)
            else:
                # Plain value or unknown form: return as-is
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

        # Without usable levels, prefer endpoints when possible
        if values:
            if len(values) >= 2:
                return f"{values[0]}–{values[-1]}"
            return values[0]
        return ''

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
                        # If it's a table block (from st), it may contain one or more concatenated tables
                        if lev.lstrip().startswith('|'):
                            blocks: List[str] = []
                            current: List[str] = []
                            for ln in lev.splitlines():
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
                            # Non-table: render as a bullet list line
                            lines.append(f"- {lev}")
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
                    # 'item' may already include indentation and '- ' prefix
                    if re.match(r'^\s*-\s', item):
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
        # Normalize malformed emphasis around common tokens
        md = re.sub(r'\*bonus\*\*\s*AD\b', 'bonus AD', md)
        md = re.sub(r'\*bonus\*\*\s*AP\b', 'bonus AP', md)
        # Additional robust cleanup for emphasis around bonus labels
        md = re.sub(r'\*{1,3}bonus\*{1,3}\s*AD\b', 'bonus AD', md, flags=re.IGNORECASE)
        md = re.sub(r'\*{1,3}bonus\*{1,3}\s*AP\b', 'bonus AP', md, flags=re.IGNORECASE)
        # Replace asterisk multiplication between numbers with × to avoid markdown italic parsing
        md = re.sub(r'(?<=\d)\*(?=\d)', '×', md)
        md = re.sub(r'\*bonus\*\*\s*armor\b', 'bonus armor', md)
        md = re.sub(r'\*bonus\*\*\s*attack speed\b', 'bonus attack speed', md)
        md = re.sub(r'\*modified\*\*\s*', 'modified ', md)
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