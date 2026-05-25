#!/usr/bin/env bash

set -euo pipefail
IFS=$'\n\t'

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
GENERATED_DIR="$ROOT_DIR/generated"
WIKI_ROOT=""
SKIP_EXPORT=0
SKIP_DOCS_INSTALL=0
EXPORTER_SOURCE="workspace"

ROOT_CATEGORY="${ROOT_CATEGORY:-Category:Browse}"
MAX_DEPTH="${MAX_DEPTH:-5}"
BATCH_SIZE="${BATCH_SIZE:-200}"
TRAVERSAL_DELAY_MS="${TRAVERSAL_DELAY_MS:-50}"
BATCH_DELAY_MS="${BATCH_DELAY_MS:-400}"
CONCURRENCY="${CONCURRENCY:-3}"

resolve_command() {
  local unix_name="$1"
  local windows_name="${2:-$1}"
  local candidate
  local cmd_bridge=""

  if command -v "$unix_name" >/dev/null 2>&1; then
    command -v "$unix_name"
    return 0
  fi

  if command -v "$windows_name" >/dev/null 2>&1; then
    command -v "$windows_name"
    return 0
  fi

  if command -v cmd.exe >/dev/null 2>&1; then
    cmd_bridge="$(command -v cmd.exe)"
  elif [[ -x /mnt/c/Windows/System32/cmd.exe ]]; then
    cmd_bridge="/mnt/c/Windows/System32/cmd.exe"
  elif [[ -x /c/Windows/System32/cmd.exe ]]; then
    cmd_bridge="/c/Windows/System32/cmd.exe"
  fi

  if [[ -n "$cmd_bridge" ]] && command -v wslpath >/dev/null 2>&1; then
    candidate="$("$cmd_bridge" /c "where $windows_name" 2>/dev/null | tr -d '\r' | head -n 1 || true)"
    if [[ -n "$candidate" ]]; then
      wslpath -u "$candidate"
      return 0
    fi
  fi

  echo "Could not resolve required command: $unix_name / $windows_name" >&2
  return 1
}

CARGO_BIN="$(resolve_command cargo cargo.exe)"
NPM_BIN="$(resolve_command npm npm.cmd)"
NODE_BIN="$(resolve_command node node.exe)"
PYTHON_BIN="$(resolve_command python python.exe || resolve_command python3 python3.exe)"

usage() {
  cat <<'EOF'
Usage: bash ./scripts/pipeline.sh [options]

Options:
  --exporter-source <workspace|release>  Use the vendored exporter crate or download the latest GitLab release binary.
  --skip-export                          Reuse an existing raw wiki export directory.
  --wiki-root <path>                     Raw wiki export directory to convert (required with --skip-export if not using generated default).
  --generated-dir <path>                 Override the generated output root (default: ./generated).
  --skip-docs-install                    Skip npm install/npm ci before building the docs site.
  -h, --help                             Show this help text.

Environment overrides for exporter pacing:
  ROOT_CATEGORY, MAX_DEPTH, BATCH_SIZE, TRAVERSAL_DELAY_MS, BATCH_DELAY_MS, CONCURRENCY

Examples:
  bash ./scripts/pipeline.sh
  bash ./scripts/pipeline.sh --skip-export --wiki-root ./export_out
  bash ./scripts/pipeline.sh --exporter-source release
EOF
}

while [[ $# -gt 0 ]]; do
  case "$1" in
    --exporter-source)
      EXPORTER_SOURCE="$2"
      shift 2
      ;;
    --skip-export)
      SKIP_EXPORT=1
      shift
      ;;
    --wiki-root)
      WIKI_ROOT="$2"
      shift 2
      ;;
    --generated-dir)
      GENERATED_DIR="$2"
      shift 2
      ;;
    --skip-docs-install)
      SKIP_DOCS_INSTALL=1
      shift
      ;;
    -h|--help)
      usage
      exit 0
      ;;
    *)
      echo "Unknown argument: $1" >&2
      usage >&2
      exit 1
      ;;
  esac
done

if [[ "$EXPORTER_SOURCE" != "workspace" && "$EXPORTER_SOURCE" != "release" ]]; then
  echo "--exporter-source must be 'workspace' or 'release'" >&2
  exit 1
fi

EXPORT_OUT_DIR="$GENERATED_DIR/wiki-export/out"
EXPORT_META_DIR="$GENERATED_DIR/wiki-export/meta"
MARKDOWN_ROOT="$GENERATED_DIR/markdown"
DOCS_CONTENT_DIR="$ROOT_DIR/apps/docs/docs"
DOCS_APP_DIR="$ROOT_DIR/apps/docs"
DOWNLOAD_CACHE_DIR="$ROOT_DIR/.cache/apps/exporter"

mkdir -p "$GENERATED_DIR" "$MARKDOWN_ROOT"

resolve_release_asset_name() {
  local uname_s uname_m
  uname_s="$(uname -s)"
  uname_m="$(uname -m)"

  case "$uname_s" in
    Linux)
      case "$uname_m" in
        x86_64) echo "x86_64-unknown-linux-musl" ;;
        aarch64|arm64) echo "aarch64-unknown-linux-musl" ;;
        *) echo "Unsupported Linux architecture for release download: $uname_m" >&2; return 1 ;;
      esac
      ;;
    Darwin)
      case "$uname_m" in
        x86_64) echo "x86_64-apple-darwin" ;;
        arm64|aarch64) echo "aarch64-apple-darwin" ;;
        *) echo "Unsupported macOS architecture for release download: $uname_m" >&2; return 1 ;;
      esac
      ;;
    MINGW*|MSYS*|CYGWIN*)
      echo "x86_64-pc-windows-gnu.exe"
      ;;
    *)
      echo "Unsupported platform for release download: $uname_s" >&2
      return 1
      ;;
  esac
}

download_release_exporter() {
  local release_api asset_suffix release_json tag_name asset_name asset_url bin_dir bin_path
  release_api="https://gitlab.com/api/v4/projects/lol-math%2Flol-wiki-export/releases/permalink/latest"
  asset_suffix="$(resolve_release_asset_name)"

  echo "Downloading latest lol-wiki-export release metadata..."
  release_json="$(curl -fsSL "$release_api")"
  tag_name="$(printf '%s' "$release_json" | "$PYTHON_BIN" -c 'import json,sys; print(json.load(sys.stdin)["tag_name"])')"
  asset_name="lol-wiki-export-${tag_name}-${asset_suffix}"

  asset_url="$(printf '%s' "$release_json" | "$PYTHON_BIN" -c 'import json,sys; data=json.load(sys.stdin); want=sys.argv[1]; links=data.get("assets", {}).get("links", []); 
for link in links:
    if link.get("name") == want:
        print(link["url"])
        break
else:
    raise SystemExit(f"Could not find release asset: {want}")' "$asset_name")"

  bin_dir="$DOWNLOAD_CACHE_DIR/$tag_name"
  bin_path="$bin_dir/$asset_name"
  mkdir -p "$bin_dir"

  if [[ ! -f "$bin_path" ]]; then
    echo "Downloading ${asset_name}..."
    curl -fsSL "$asset_url" -o "$bin_path"
    chmod +x "$bin_path" || true
  fi

  printf '%s\n' "$bin_path"
}

build_workspace_tools() {
  echo "Building vendored exporter and converter..."
  "$CARGO_BIN" build --release -p lol-wiki-export -p lol_wiki_md --bin convert
}

run_export() {
  local exporter_bin="$1"
  echo "Running wiki export into $EXPORT_OUT_DIR"
  "$exporter_bin" \
    --root-category "$ROOT_CATEGORY" \
    --max-depth "$MAX_DEPTH" \
    --out-dir "$EXPORT_OUT_DIR" \
    --meta-dir "$EXPORT_META_DIR" \
    --batch-size "$BATCH_SIZE" \
    --traversal-delay-ms "$TRAVERSAL_DELAY_MS" \
    --batch-delay-ms "$BATCH_DELAY_MS" \
    --concurrency "$CONCURRENCY"
}

run_convert() {
  local converter_bin="$1"
  local source_root="$2"
  echo "Converting champions into $MARKDOWN_ROOT/champions"
  "$converter_bin" --wiki-root "$source_root" --output "$MARKDOWN_ROOT/champions" --all-champions
  echo "Converting items into $MARKDOWN_ROOT/items"
  "$converter_bin" --wiki-root "$source_root" --output "$MARKDOWN_ROOT/items" --all-items
  echo "Converting runes into $MARKDOWN_ROOT/runes"
  "$converter_bin" --wiki-root "$source_root" --output "$MARKDOWN_ROOT/runes" --all-runes
}

install_docs_dependencies() {
  if [[ $SKIP_DOCS_INSTALL -eq 1 ]]; then
    echo "Skipping docs dependency install"
    return 0
  fi

  echo "Installing docs app dependencies..."
  if [[ -f "$ROOT_DIR/package-lock.json" ]]; then
    "$NPM_BIN" ci
  else
    "$NPM_BIN" install
  fi
}

build_docs() {
  echo "Syncing generated markdown into the Rspress docs tree..."
  "$NODE_BIN" "$ROOT_DIR/scripts/sync-rspress-content.mjs" "$MARKDOWN_ROOT" "$DOCS_CONTENT_DIR"
  echo "Building static Rspress site..."
  "$NPM_BIN" run docs:build
  echo "Static docs site available at $DOCS_APP_DIR/out"
}

cd "$ROOT_DIR"

if [[ $SKIP_EXPORT -eq 1 ]]; then
  if [[ -z "$WIKI_ROOT" ]]; then
    WIKI_ROOT="$ROOT_DIR/export_out"
  fi
  echo "Skipping export step; using existing wiki root: $WIKI_ROOT"
else
  WIKI_ROOT="$EXPORT_OUT_DIR"
fi

case "$EXPORTER_SOURCE" in
  workspace)
    build_workspace_tools
    EXPORTER_BIN="$ROOT_DIR/target/release/lol-wiki-export"
    ;;
  release)
    EXPORTER_BIN="$(download_release_exporter)"
    "$CARGO_BIN" build --release -p lol_wiki_md --bin convert
    ;;
esac

CONVERTER_BIN="$ROOT_DIR/target/release/convert"

if [[ $SKIP_EXPORT -eq 0 ]]; then
  run_export "$EXPORTER_BIN"
fi

run_convert "$CONVERTER_BIN" "$WIKI_ROOT"
install_docs_dependencies
build_docs

echo "Pipeline complete."