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

## 4. Template:Item stat table

**File:** `Template/Item_stat_table/page.txt` (#invoke:item stat table)
**Behavior:** Likely enumerates item stats in a table (scales, gold efficiency). Not accessible w/out Lua.
**Strategy:** Provide a summary table for a single item from parsed ItemData when converting an item page.
Columns: `Stat | Value` derived from existing `_generate_item_markdown` logic (already enumerates stats). Therefore treat template call as redundant: remove it (NO-OP) or ensure not duplicated. If encountered separately, ignore.

## 5. Template:Item haste table

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

## 6. Template:Ward table

**Status:** Present as directory but not yet opened (content not retrieved). Expect a wikitext table of ward stats.
**Strategy:** Let existing generic wiki table→MD logic handle it; we only need to ensure any nested template icons (`{{ii|Item}}`) convert to plain text. Already partly handled by link/template conversions. If needed add mapping `{{ii|Name|icononly=yes}}` → `Name`.

## 7. Template:Channel type / Channel type table (and alias Ct, Ctable)

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

### Item Haste Table (Section)

Simplified MD after parsing base + modified cooldown columns.

---

## Incremental Implementation Order

1. Add `_expand_custom_templates` scaffold with Quote + ct (most beneficial to readability quickly).
2. Add Item Haste table parser.
3. Ward table: rely on generic table conversion (no extra work unless formatting issues observed).

---

## Brief Code Sketch (Non-invasive)

```python
def _expand_custom_templates(self, text: str, champion_name: Optional[str]) -> str:
    if not text:
        return text
  text = self._expand_quote(text)
  text = self._expand_channel_type(text)
    # Potential: text = self._expand_item_haste(text)
    return text
```

Each `_expand_*` returns modified text; keep regexes compiled at module load for performance.

---

## Conclusion

This plan maps each requested (and actually present) table-centric or structural template to a deterministic Markdown representation relying on: (a) existing module data loaders, (b) simple regex and pipe-aware parsing, and (c) modest extension points that do not require full MediaWiki or Lua execution. Removed entries for templates not found in the extracted dataset to avoid speculative behavior.

If you want, next step I can implement the `_expand_custom_templates` scaffolding plus one or two concrete converters (Quote + Channel Type) as a starting point.
