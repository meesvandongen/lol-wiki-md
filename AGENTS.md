**Information integrity (the most important category)**

- **Never silently drop content.** If you can't handle a template, emit a visible marker like `<!-- UNHANDLED: {{templatename|...}} -->` so it shows up in diffs. Silent omission is worse than ugly output.
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
