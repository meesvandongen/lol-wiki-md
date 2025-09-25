# LoL Wiki Table-Oriented Templates: Conversion Strategy to Markdown

This document surveys the requested MediaWiki templates and Lua-driven components concerned with tabular or structured data presentation, and proposes deterministic Markdown (MD) equivalents compatible with the existing `simple_converter.py` transformation pipeline. For each template we list:

- Purpose / Source Behavior
- Key Parameters / Data Semantics
- Observed Implementation Pattern (redirect, #invoke, wikitext table, etc.)
- Recommended Markdown Representation
- Suggested Parser Hook (where in code to integrate)
- Edge Cases / Fallbacks

Where the template body depends on a `#invoke:` Lua module (whose Lua source is not included in the extracted dataset), we infer intent from name and usage context and provide a _degraded but useful_ representation plan.

---

## Legend for Strategy Tags

- INLINE: Replace template call inline with computed / simplified text or symbol.
- BLOCK: Produce a fenced/table block in MD.
- COLLAPSE: Provide an optional collapsible section (GitHub flavor substitute: heading + details summary or HTML `<details>` if allowed; otherwise a heading).
- DATA LOOKUP: Requires accessing previously parsed module data (e.g., ChampionData / ItemData).
- POSTPROCESS: Safe to do late in `_postprocess_markdown` (after generic wiki→MD conversions).

---

## 1. Template:Background quote (aka `Quote`)

**File located:** `Template/Quote/page.txt`
**Behavior:** Renders a stylized pull quote table with optional attribution and source.
**Parameters:** `{{Quote|text|author|source}}`
**Current Markup:** Intricate table with stylized quotation mark and separator image.
**Markdown Strategy:**

```md
> “Quote text here.”
> — Author, _Source_
```

If only text: single blockquote line. If author but no source: omit comma/source. Curly smart quotes can be preserved or normalized.
**Hook:** During wiki→MD, pattern match `{{Quote|...}}` before generic table stripping. Add a small regex in `_convert_wiki_to_markdown` to capture up to 3 pipe-separated params (respect nested templates minimally by not splitting inside `{{...}}`).
**Edge Cases:** Nested italic/bold markup should pass through. Remove trailing separator image.

## 2. Template:Champion info / Champion info/stats / Champion info/stats/WR

**Files:** Only `Champion_info/page.txt` extracted; stats subtemplates likely composed by `Infobox champion` + `Infobox stats` + WR variant (not present separately).
**Behavior:** Introductory line + infobox combos (stats + champion infobox). Calls `Infobox stats` and `Infobox champion`.
**Parameters:** `{{Champion info|Name|[optional second param?]}}` plus forwarded named params like `disp_name`, `title`, `role`.
**Markdown Strategy:** Convert to a header + summary block referencing base stats already extracted by `simple_converter.py`.

```md
## {Champion Name}

_Role:_ {role}
_Title:_ {title}

(Stats table already produced below.)
```

We already extract stats from Module:ChampionData; reuse that instead of emulating nested tabber/mode variants.
**Hook:** When detecting champion page conversion, if a line matches `{{Champion info` remove raw template and populate structured fields inside `_generate_markdown` where champion metadata is assembled.
**Edge Cases:** Multi-form champions (Gnar, Kled) – show base form plus alt form notes: create a bullet list under stats if alt stats exist (the simple converter currently aggregates advanced stats—extend to multi-form if needed later).

## 3. Template:Collapse cell

**Status:** Not found in extracted templates (probably absent or pruned). Treat as NO-OP markup removal if encountered: just emit its inner content.
**Strategy:** Regex remove wrapper: `{{Collapse cell|...}} → content`.

## 4. Template:Early Sale row / Sale row / Retired skins / Skin row / Mythic content table / Mythic content table row

**Status:** Specific template files not found in given extraction list (directory listing truncated?). Assume these generate skin economics tables.
**Strategy (Degraded):** Replace each row template call with a Markdown table row placeholder retaining key parameters (e.g., skin name, release date, RP, status). Generic fallback pattern:
`{{Sale row|Param1|Param2|...}} → | Param1 | Param2 | ... |`
Allow the outer table syntax (if present) to be converted by existing table converter.
Add a guard so if outer wikitext table markup missing, wrap collected rows into a Markdown table with inferred headers `[Name | Type | Date | Notes]`.

## 5. Template:Ep-list / Ep-list header (Episodes or Eternals progression lists)

**Status:** Not found; fallback to enumerated lists.
**Strategy:** Convert to ordered list lines: `1. Episode/Entry ...`
If header template appears: translate to `### Episodes` heading.

## 6. Template:Highest lowest stats

**File:** `Template/Highest_lowest_stats/page.txt` (#invoke base stats get)
**Behavior:** Lua invocation returning computed champion stat extremes.
**Strategy:** Inline placeholder pulling the requested stat dimension not currently reproduced. Provide a stub:
`{{Highest lowest stats|ChampionName|rangetype=...|show=...}}` → `*Stat Range (placeholder)*`
Optionally later: compute from aggregated champion module file read once (we already parse Module:ChampionData). Add enhancement flag.
**Implementation:** Add pattern; parse key params; if `show` refers to known stat (hp, ad, etc.), compute min/max across loaded champions (cache one pass) and inject `HP: min 480 (ChampionA) – max 700 (ChampionB)`.

## 7. Template:Item stat table

**File:** `Template/Item_stat_table/page.txt` (#invoke:item stat table)
**Behavior:** Likely enumerates item stats in a table (scales, gold efficiency). Not accessible w/out Lua.
**Strategy:** Provide a summary table for a single item from parsed ItemData when converting an item page.
Columns: `Stat | Value` derived from existing `_generate_item_markdown` logic (already enumerates stats). Therefore treat template call as redundant: remove it (NO-OP) or ensure not duplicated. If encountered separately, ignore.

## 8. Template:Item haste table

**File:** `Template/Item_haste_table/page.txt`
**Behavior:** Tabber with three tables: Item Haste, Equivalent CDR, Item Cooldowns with/without Cosmic Insight.
**Strategy:** Collapse into a single Markdown section with subsections.

```md
### Item Haste

| Item           | Item Haste |
| -------------- | ---------- |
| Cosmic Insight | 10         |

### Equivalent CDR

| Item Haste          | CDR Equivalent |
| ------------------- | -------------- |
| 0                   | 0%             |
| 10 (Cosmic Insight) | 9.1% (rounded) |

### Item Cooldowns (with 10 Item Haste)

| Items                   | Base CD | Adjusted CD |
| ----------------------- | ------- | ----------- |
| Sheen / Bloodsong / ... | 1.5     | 1.36        |

...
```

We can parse the existing wikitext directly:

- Capture each table between `{|` and `|}` within the tabber order.
- Extract rows starting with `!` or `|` converting header cells first row into MD header.
- Evaluate `{{#expr:...}}` with the existing numeric expression evaluator (`_safe_eval_num_expr`) after stripping wrappers.
  **Implementation:** Add a specialized transformer invoked when pattern `{{Item haste table` (if such template invocation occurs) or when encountering raw embedded markup recognized by a leading `<tabber>` containing Item Haste strings.

## 9. Template:Matchups

**Status:** Not found; degrade to heading `### Matchups` followed by raw bullet list inside template parameters.

## 10. Template:Mythic content table / row

(See #4).

## 11. Template:PoC Adventure row (Proof of Concept?), Position row

**Status:** Not present. Fallback similar to sale row—represent as pipe-separated columns.

## 12. Template:Rune combinations table / Rune table row / Rune table row/WR

**Status:** Not present in extracted dataset (likely omitted). We have `Rune_header`, `Rune_footer`.
**Strategy:** Use existing rune conversion: compile a rune's metadata then ignore absent combination tables; if a call is found, produce placeholder:
`> Rune Combinations Table (omitted)`
Optionally later generate from rune path synergy by scanning other rune pages.

## 13. Template:Sale row (see #4)

## 14. Template:Skin row (see #4) – Add note to group sequential skin rows inside a single Markdown table, performing one pass to detect cluster.

## 15. Template:TFT champion row stats / TFT champion row / TFT item (not explicitly listed but similar pattern) – can share a generic row fallback.

## 16. Template:Upgrades (+ subtemplates Ironback, Ocklepod, Plundercrab, Razorfin)

**Status:** Not found. Provide placeholder grid:
`| Upgrade | Cost | Effect |` etc. If parameters parsed, fill columns; else show raw.

## 17. Template:Ward table

**Status:** Present as directory but not yet opened (content not retrieved). Expect a wikitext table of ward stats.
**Strategy:** Let existing generic wiki table→MD logic handle it; we only need to ensure any nested template icons (`{{ii|Item}}`) convert to plain text. Already partly handled by link/template conversions. If needed add mapping `{{ii|Name|icononly=yes}}` → `Name`.

## 18. Template:Channel type / Channel type table (and alias Ct, Ctable)

**Files:** `Channel_type/page.txt`, `Channel_type_table/page.txt`, redirects `Ct`, `Ctable`.
**Behavior:** Parameterized table enumerating interaction rules for channel, cast, charge. Rammus example shows usage via `{{ct|channel|attack=false|move=true|cast={{ai|Defensive Ball Curl|Rammus}} interrupts...}}`.
**Strategy:** Create a concise Markdown mini-table:

```md
Channel Behavior (channel)
| Aspect | State / Notes |
|-------------|---------------|
| Attacking | Interrupts |
| Abilities | Interrupts |
| Movement | Allowed |
| Items Usable| All items usable (unless...) |
| Items Disabled | Zhonya's Hourglass ... |
| Items Interrupt | Hextech Rocketbelt ... |
| Summoner Spells Usable | Flash, Teleport ... |
| Summoner Spells Disabled | (list) |
| Summoner Spells Interrupt | (list) |
| Consumables | Usable |
| Interrupted by | Death; Silence; etc. |
```

**Implementation:** Add a parser for `{{ct|...}}` that:

- First param is type (channel/cast/charge)
- Named params: `attack, move, cast, items, spells, consume, interrupts, damage`
- Values may be comma lists mixing tokens: true, false, interrupts, recasts, PFT.
  Map tokens to human text.
  **Edge Cases:** Embedded templates inside param values (like `{{ai|Defensive Ball Curl|Rammus}} interrupts.`) – we can leave them as raw wiki text for now; they will pass through subsequent conversion to ability/italic formatting.

## 19. Template:Patch box

**File:** `Patch_box/page.txt`
**Behavior:** Link to patch history and scrollable inclusion of patch changes via `#invoke:PatchBox`. We already extract patch history separately in `_extract_patch_history`.
**Strategy:** Replace with inline reference if patch history list already generated:
`See Patch History section below.` If version param present, annotate: `(Filtered to version X)`. Otherwise ignore to avoid duplicate content.
**Hook:** Detect `{{Patch box` early and remove, store optional version filter.

---

## Implementation Guidance in Code

Below are recommended insertion points and function sketches.

### A. Add Template Dispatch Layer

File: `simple_converter.py`
Inside `_convert_wiki_to_markdown_ctx` before calling `_convert_wiki_to_markdown`, insert a pre-pass:

1. `text = self._expand_custom_templates(text, champion_name)`
2. Then proceed as existing.

Add new method `_expand_custom_templates` handling regex substitutions for:

- Quote
- ct (channel type)
- Patch box
- Highest lowest stats (optional compute)
- Item haste tabber (detect `<tabber>` containing `Item Haste` phrase)
- Generic row templates (sale row, skin row) clustering logic (scan entire text for repeated `{{Skin row` occurrences; wrap results).

### B. Parsing Helpers

- `_parse_pipe_template(name, text)` to safely split top-level pipes (you already have `_split_top_level_pipes`; reuse it).
- `_token_list(value)` splitting on commas trimming whitespace.
- Category suppression: remove `[[Category:Pending for test]]` when generating MD tables for channel types; keep token as note.

### C. Channel Type Conversion Pseudocode

```
pat = re.compile(r"\{\{ct\|([^|}]+)(.*?)\}\}", re.I|re.S)
for each match:
  ctype = group1.strip()
  params_text = group2
  params = parse_named_params(params_text)
  table_rows = [] -> map aspects
  md_table = render_markdown_table(title=f"Channel Behavior ({ctype})", rows)
  replace
```

### D. Quote Conversion Regex

```
re.sub(r"\{\{Quote\|([^|}]+)(?:\|([^|}]*))?(?:\|([^|}]*))?\}\}", lambda m: blockquote(m), text)
```

Handle nesting by discouraging greedy matches (stop at first `}}` – acceptable approximation).

### E. Item Haste Table

Detect `<tabber>` then split on `|-|` boundaries capturing the 3 labeled sections. Parse wikitext tables with simple state machine: header row starts with `!`; cells: `!` or `|`. Pipe separators inside templates should not create splits (limit to lines starting with `!` or `|` at line start).

### F. Highest Lowest Stats (Optional Enhancement)

When first needed, load Module:ChampionData once (already have loader). Build summary dictionaries for each stat base value. Provide min/max champion names; cache.

---

## Markdown Output Examples

### Quote

Input: `{{Quote|The rivers will run red.|Darius|League Judgment: Darius}}`
Output:

```
> “The rivers will run red.”
> — Darius, *League Judgment: Darius*
```

### Channel Type (Rammus Excerpt)

Input excerpt:

```
{{ct|channel|attack=false|move=true|cast={{ai|Defensive Ball Curl|Rammus}} interrupts. {{ai|Frenzying Taunt|Rammus}} is disabled. {{ai|Soaring Slam|Rammus}} modifies this ability. This ability recasts to end channel.|items=interrupts,true,,false|spells=true,true,interrupts,interrupts,interrupts}}
```

Output (abbreviated):

```
#### Channel Behavior (channel)
| Aspect | State |
|--------|-------|
| Attacking | Disabled |
| Abilities | Interrupts |
| Movement | Allowed |
| Items Usable | (some usable) |
| Items Disabled | (list false) |
| Items Interrupt | (list interrupts) |
| Summoner Spells Usable | Barrier, Clarity, Cleanse, ... |
| Summoner Spells Interrupt | Flash, Teleport, ... |
| Consumables | Usable |
| Interrupted by | Silence; Death |
| Notes | Defensive Ball Curl interrupts. Frenzying Taunt disabled. Soaring Slam modifies ability. Recast ends channel. |
```

### Patch Box

Input: `{{Patch box|Rammus|||LOL}}`
Output:
`_See Patch History section below._`

### Item Haste Table (Section)

Simplified MD after parsing base + modified cooldown columns.

---

## Edge Case Handling Summary

| Template             | If Missing Data                | Fallback                                  |
| -------------------- | ------------------------------ | ----------------------------------------- |
| Highest_lowest_stats | Module parse fails             | Emit italic placeholder                   |
| Channel type         | Unknown token                  | Echo raw token text                       |
| Item_haste_table     | Expr eval fails                | Keep original numeric expression raw      |
| Quote                | Nested templates inside params | Convert outer, leave inner for later pass |
| Sale / Skin rows     | Incomplete param count         | Show only provided cells                  |

---

## Incremental Implementation Order

1. Add `_expand_custom_templates` scaffold with Quote + Patch box + ct (most beneficial to readability quickly).
2. Add Item Haste table parser.
3. Add Highest_lowest_stats computation if desired (optional performance optimization: single champion data scan).
4. Generic row cluster handling (skin/sale rows) – low priority until such pages exhibit them.

---

## Future Enhancements (Not Implemented Yet)

- Collapsible sections using `<details>` if target renderer supports raw HTML.
- Rich icon substitution with alt text (mapping item/rune icons to plain names already partially done by existing conversions).
- Unified stat expression evaluation inside parsed tables (reuse `_evaluate_expr_templates`).

---

## Brief Code Sketch (Non-invasive)

```python
def _expand_custom_templates(self, text: str, champion_name: Optional[str]) -> str:
    if not text:
        return text
    text = self._expand_quote(text)
    text = self._expand_patch_box(text)
    text = self._expand_channel_type(text)
    # Potential: text = self._expand_item_haste(text)
    return text
```

Each `_expand_*` returns modified text; keep regexes compiled at module load for performance.

---

## Conclusion

This plan maps each requested table-centric or structural template to a deterministic Markdown representation relying on: (a) existing module data loaders, (b) simple regex and pipe-aware parsing, and (c) modest extension points that do not require full MediaWiki or Lua execution. The approach prioritizes readability and reproducibility while leaving room for future fidelity improvements.

If you want, next step I can implement the `_expand_custom_templates` scaffolding plus one or two concrete converters (Quote + Channel Type) as a starting point.
