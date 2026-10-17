# Build pivot: an Excel-style pivot table in Rust, step by step.
# Run `just` to see every recipe.

set quiet

trips_url := "https://d37ci6vzurychx.cloudfront.net/trip-data/yellow_tripdata_2024-01.parquet"
zones_url := "https://d37ci6vzurychx.cloudfront.net/misc/taxi_zone_lookup.csv"

# The coding agent the LLM recipes ask. Override: LLM="claude -p" just review
llm := env("LLM", "codex exec")

# List all recipes
default:
    just --list

# ── Data ───────────────────────────────────────────────

# Download the public TLC files into data/ (never committed)
[group('data')]
data:
    mkdir -p data
    just _download {{trips_url}} data/yellow_tripdata_2024-01.parquet
    just _download {{zones_url}} data/taxi_zone_lookup.csv

# Download to a .part file and rename it, so a broken download is retried next time
_download url file:
    test -f {{file}} || (curl -fL --progress-bar -o {{file}}.part {{url}} && mv {{file}}.part {{file}})

# Build data/taxi.db (SQLite) from the downloaded files
[group('data')]
db: data
    cargo run --release --bin taxi-db

# Count rows in data/taxi.db
[group('data')]
db-stats:
    sqlite3 data/taxi.db "SELECT 'trips', COUNT(*) FROM trips UNION ALL SELECT 'zones', COUNT(*) FROM zones"
    ls -lh data/taxi.db

# ── Daily ──────────────────────────────────────────────

# Unit tests + BDD scenarios (features/*.feature)
[group('daily')]
test:
    cargo test

# Only the BDD scenarios
[group('daily')]
bdd:
    cargo test --test acceptance

# Everything a change must pass before a commit
[group('daily')]
check: fmt-check lint shape test

# ── Code quality ───────────────────────────────────────

# Format the code
[group('code')]
fmt:
    cargo fmt

[group('code')]
fmt-check:
    cargo fmt --check

# Clippy with the limits from clippy.toml as errors
[group('code')]
lint:
    cargo clippy --all-targets -- -D warnings -D clippy::too_many_arguments -D clippy::too_many_lines -D clippy::cognitive_complexity -D clippy::excessive_nesting

# File length and items per file (AGENTS.md "Code shape")
[group('code')]
shape:
    scripts/shape-check.sh

# Functions that are too long, too deep or take too many arguments
[group('code')]
dev-complexity:
    cargo clippy --all-targets -- -W clippy::too_many_lines -W clippy::cognitive_complexity -W clippy::too_many_arguments -W clippy::excessive_nesting

# Pedantic clippy: mutation, needless clones, style
[group('code')]
fp-lint:
    cargo clippy --all-targets -- -W clippy::pedantic

# Code nobody calls
[group('code')]
code-dead:
    cargo clippy --all-targets -- -W dead_code -W unused

# Direct dependencies of the crate
[group('code')]
deps:
    cargo tree --depth 1 --edges normal

# ── Robot (LLM tools) ──────────────────────────────────

# Ask the agent to review uncommitted changes (new files too) against AGENTS.md
[group('robot')]
review:
    (git diff HEAD; git ls-files --others --exclude-standard | xargs -I{} git diff --no-index /dev/null {}) | {{llm}} "Review this diff against AGENTS.md. List real problems only, with file:line."

# Explain a file, top to bottom
[group('robot')]
explain file:
    {{llm}} "Explain {{file}} top to bottom for a viewer who knows a little Rust."

# Ask a question about a file
[group('robot')]
ask file question:
    {{llm}} "In {{file}}: {{question}}"

# Run the tests and ask the agent what failed and why
[group('robot')]
triage:
    cargo test 2>&1 | tail -80 | {{llm}} "These are test results. Name each failure, its likely cause, and the file:line to look at."
