**Information integrity (the most important category)**

- **Fail fast on unhandled templates.** The template registry returns a hard error (`E_UNKNOWN_TEMPLATE` / `E_MALFORMED_TEMPLATE`) for any template it cannot expand; there is no "unhandled template" placeholder marker. A conversion that hits an unknown or unexpandable template fails loudly and is reported in the batch conversion report. Add a real handler (an explicit `TemplateExpander`) rather than papering over it — decorative/navigation templates get an explicit expander that renders empty, not a silent skip.
- **Resolve data-lookup templates; never invent their values.** If a template fetches from a Lua module (`Module:ChampionData`, etc.), fetch the module via API. Guessing scaling numbers from training data is forbidden.
- **When in doubt, verify against dumped data.** The wiki API is the source of truth, not your prior knowledge. This applies to template behavior, page structure, and numerical values.
- **Tooltip and `data-*` content is content.** It must end up in the output somewhere. Decide where in `STYLE.md`, then be consistent.
- **Numbers round-trip exactly.** Don't reformat `0.625` to `0.63`, don't convert percentages, don't "clean up" ranges.

**Generalization (avoid overfitting)**

- **No hardcoded entity names in handlers.** If a handler contains "Ahri", "Infinity Edge", or any specific champion/item string, it's wrong. Handlers operate on template *shape*, not instance.
- **Look at ≥3 wild instances before writing a handler.** Search the corpus for other uses of the template. The first example you see is rarely representative.
- **Don't expand the test corpus and add handlers in the same commit.** Separate "I added a new test case (and watched it fail)" from "I made it pass." Otherwise you can't tell which handler change broke what.

**Process discipline**

- **Run the full test suite after every change.** Not just the page you were working on. Regressions in this kind of project are constant.
- **Parse, don't regex.** Use the wikitext AST (`mwparserfromhell`) or the parsed HTML DOM. String matching on `{{...}}` will eventually bite.
- **Prefer fixing an existing handler over adding a new one.** Check whether a near-match handler already covers the case before introducing parallel logic.
- **Document formatting decisions in `STYLE.md` as you make them.** "Scaling renders as `base (+ratio stat)`" — if it's not written down, the next handler will choose differently.

**Workspace hygiene (generated files and testing)**

- **Use the canonical pipeline folders by default.** The normal output root is `generated/`, with wiki export data under `generated/wiki-export/{out,meta}` and converted markdown under `generated/markdown/{champions,items,runes}`.
- **Use `export_out/` only as a reusable local export cache.** It is fine for quick `--wiki-root ./export_out` validation runs, but it is not a tracked deliverable.
- **Write ad-hoc test artifacts to `test_output/`.** If you need temporary conversion samples, diff targets, or inspection outputs, put them under a named subfolder there.
- **Write disposable scans and audits to `validation_reports/`.** Do not drop report files into the repo root.
- **Do not revive legacy output roots unless explicitly asked.** Avoid creating or updating `markdown/`, `markdown_rust/`, or root-level `meta/` for normal work.
- **If you create a new scratch-output folder, ignore it in the same change.** Future agents should not leave surprise untracked artifacts behind.
