# Template review – plan and guide (single document)

Purpose

- Review each Template doc entry and make a clear decision for the Rust converter: include, exclude.

What to capture from a doc page (only what you need) into notes.

- What it does: One sentence. Example: “Shows an ability icon and label with a link.”
- Parameters: Only those that affect text (positional or key=value). Skip visual-only options.

Category (pick one)

- content — shows text/data we need in Markdown
- helper — numeric/text helper we normalize (e.g., fd, pp, ap, tt)
- structural — layout/navigation/infobox frames; not rendered as text
- data-retrieval — fetches module data (we read Lua directly)
- lua — generic module calls (#invoke)
- other — anything else

Decision (pick one)

- include — we will render text/meaning; implement or use an existing handler
- exclude — layout-only; render nothing

Priority

- 1 — champion-critical or common helpers
- 2 — important extras (e.g., module/data readers)
- 3 — structural/long-tail

Where to record your work

- Edit the checklist file directly, update the status, decision, and notes.

Quality rules

- No silent data loss. Exclude only if it’s layout-only or duplicated elsewhere.
- Deterministic text. Normalize numbers/series via helpers.
- Keep it short. One sentence for behavior and a short note is enough.
