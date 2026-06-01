# LoL Wiki Monorepo

This repository is a small polyglot monorepo that can:

1. export raw League Wiki wikitext,
2. convert the export into Markdown,
3. build an Rspress static site from the generated Markdown.

The repo root is only the orchestration layer now; the three primary deliverables live side by side under `apps/`.

## Layout

- `apps/converter` — Rust Markdown converter crate (`lol_wiki_md`)
- `apps/exporter` — Rust wiki exporter crate (`lol-wiki-export`)
- `apps/docs` — Rspress docs site
- `scripts/` — repo-level orchestration (`pipeline.sh`, `pipeline.ps1`, content sync)
- `Cargo.toml` — Rust workspace manifest for converter/exporter
- `package.json` — Node workspace manifest for the docs app and top-level convenience scripts
- `generated/` — default runtime output location for exports and Markdown artifacts

Useful top-level commands:

- `npm run export:wiki -- --help`
- `npm run convert -- --help`
- `npm run audit:similarity -- --help`
- `npm run docs:build`
- `npm run docs:dev`
- `npm run docs:publish`

The docs UI includes the official Rspress `llms` and `sitemap` plugins, a GitHub social link, and site version metadata sourced from the docs package version.

## One-shot pipeline

Run the full pipeline from the repository root:

- `bash ./scripts/pipeline.sh`
- `pwsh ./scripts/pipeline.ps1`

Useful flags:

- `--exporter-source workspace` — build and use the workspace exporter crate from `apps/exporter` (default)
- `--exporter-source release` — download the latest prebuilt exporter release from GitLab
- `--skip-export` — reuse an existing wiki export directory
- `--wiki-root PATH` — explicit raw export directory to convert
- `--generated-dir PATH` — override the default output root (`./generated`)
- `--skip-docs-install` — skip `npm install`/`npm ci`

Example local validation using an existing export:

- `bash ./scripts/pipeline.sh --skip-export --wiki-root ./export_out`
- `pwsh ./scripts/pipeline.ps1 -SkipExport -WikiRoot ./export_out`

### Windows note

`scripts/pipeline.sh` expects `cargo`, `node`, `npm`, and `python` to be available from the bash environment that launches it.

- On Linux and macOS, that usually just works.
- On Windows, either use `pwsh ./scripts/pipeline.ps1` or run `scripts/pipeline.sh` from Git Bash / a WSL environment with the Linux Rust/Node/Python toolchain installed inside WSL.
- A WSL-launched `bash.exe` session cannot always reuse Windows-installed toolchains via interop, so if the script reports that it cannot resolve `cargo`/`npm`/`node`, run it from a shell that has those tools directly on its own `PATH`.

## Generated outputs

By default the pipeline writes:

- `generated/wiki-export/out` — raw exported wiki pages
- `generated/wiki-export/meta` — exporter metadata
- `generated/markdown/champions` — champion Markdown output
- `generated/markdown/items` — item Markdown output
- `generated/markdown/runes` — rune Markdown output
- `apps/docs/out` — static Rspress site build

The static docs build also includes `sitemap.xml`, `llms.txt`, and `llms-full.txt`.

The generated Rspress docs tree under `apps/docs/docs` is rebuilt from the Markdown folders and is intentionally ignored by Git.

## Testing folders and workspace hygiene

Use these folders consistently when validating changes so temporary artifacts do not leak into tracked source areas:

- `generated/` — canonical pipeline output root. Use this when you want the normal export → convert → docs flow that downstream scripts expect.
- `export_out/` — reusable local wiki export cache for quick converter tests with `--skip-export`. It is ignored by Git.
- `test_output/` — scratch output for one-off conversion probes, focused comparisons, and temporary entity samples.
- `validation_reports/` — disposable analysis reports, scans, and audit summaries.

Avoid using legacy or ambiguous root folders for new work:

- `markdown/`
- `markdown_rust/`
- `meta/`

Those paths are kept only for old experiments or compatibility checks and should be treated as scratch/legacy locations, not canonical deliverables.

When in doubt:

- read from `export_out/` or `generated/wiki-export/out`
- write ad-hoc test artifacts to `test_output/<run-name>/`
- write disposable reports to `validation_reports/<run-name>/`
- reserve `generated/` for outputs that the main pipeline or docs sync step will consume

## Publishing with Wrangler

The docs site can be published to Cloudflare Pages with Wrangler from the repository root.

1. Authenticate Wrangler once for your machine, for example with `npx wrangler login`.
2. Set `CLOUDFLARE_PAGES_PROJECT_NAME` in the root `.env` file.
3. Run `npm run docs:publish`.

If the deployed site uses a custom domain, set `DOCS_SITE_URL` in `.env` so the generated sitemap uses the final public URL instead of the default `*.pages.dev` hostname.

`npm run docs:publish` will:

- sync generated Markdown into `apps/docs/docs`
- build the static site into `apps/docs/out`
- run `wrangler pages deploy apps/docs/out`

Optional overrides:

- pass `--project-name <name>` after `npm run docs:publish -- ...`
- set `CLOUDFLARE_PAGES_BRANCH` in `.env`, or pass `--branch <name>` directly