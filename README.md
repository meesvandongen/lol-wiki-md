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

The generated Rspress docs tree under `apps/docs/docs` is rebuilt from the Markdown folders and is intentionally ignored by Git.