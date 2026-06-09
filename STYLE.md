# Style Decisions

- Every champion, item, and rune markdown artifact must preserve the full input content in a `## Source Appendix` section.
- `Source Appendix` entries are rendered as collapsible `<details>` blocks with a fenced code block payload.
- Use `wikitext` fences for raw wiki pages and template pages.
- Use `json` fences for structured module-data snapshots derived from champion/item Lua entries.
- Structured sections (`Overview`, `Stats`, `Abilities`, `Patch History`, etc.) are preferred for readability, but the appendix is the lossless fallback and must not be omitted.
- Do not append standalone `Raw excerpt` sections; preserve source losslessly in the appendix instead of using top-of-page excerpts as a coverage crutch.
- If a related page or template page is not yet modeled structurally, preserve it in the appendix instead of dropping it or warning about omission alone.
- When a champion exposes multiple stat views or mode-specific overrides, render them under `## Stats` with per-form subheadings and nested `Special Statistics` mode tables for the corresponding form.
- Item module captions (for example Ornn Masterwork upgrade text) should render in the reader-facing item output, not only in the source appendix.
- Removed items are excluded from the output by default; pass `--include-removed` to convert them. When included, removed items should render a reader-facing historical note derived from module metadata, including the explicit removed patch when available.
