# Build pivot

An Excel-style pivot table in Rust, built step by step on public New York City taxi trip data.

This is the code for the **Build pivot** playlist on [Ship by Step](https://www.youtube.com/@shipbystep), a channel about turning ideas into working software, one step at a time.

- Channel: [youtube.com/@shipbystep](https://www.youtube.com/@shipbystep)
- Playlist: [Build pivot](https://www.youtube.com/playlist?list=PLXOLqiYzf0f8)
- Other projects: [github.com/shipbystep](https://github.com/shipbystep)

## What we build

A pivot table turns a long list of rows into a small summary: you pick what goes down the rows, what goes across the columns, and what number goes in the cells. For example, taxi trips per borough and time of day:

| Borough | Night (0–6) | Morning (6–12) | Afternoon (12–18) | Evening (18–24) |
|---|---:|---:|---:|---:|
| Manhattan | 203,958 | 590,550 | 991,446 | 860,994 |
| Queens | 20,070 | 54,432 | 98,151 | 100,475 |

(January 2024, 2,964,624 trips, from the public NYC TLC data.)

## Aim

Pick rows, columns and a value, like in Excel, and see a pivot of three million taxi trips **in under a second**: first in the terminal, then in a browser, then as a file for Excel. Today the same pivot takes about 4.5 seconds in plain SQLite. That is the number to beat.

## Plan

| Phase | Episodes | What we build |
|---|---|---|
| 0. Prepare | 0 | Rules for the AI coding agent, high-level design, justfile, `taxi.db` in SQLite |
| 1. Learn the engine | 1–4 | Arrow in memory, Parquet on disk, DataFusion SQL, the first pivot in the terminal, honest measurement |
| 2. Real data flow | 5–7 | `taxi.db` to a Parquet snapshot, safe refresh, exact money |
| 3. Click it | 8–10 | A web page with the pivot, filters and drill-down, export to Excel |
| 4. Many users | 11–12 | Many users at once, loading only what changed |

After episode 4 we check the result and the audience before we go on. Details: [docs/design.md](docs/design.md).

## Status

Preparation. There is no pivot code yet: this step builds the starting database. Each published episode gets a git tag (`ep01`, `ep02`, …), so you can check out the code exactly as it was at the end of that episode.

## Quick start

You need Rust (stable), [just](https://github.com/casey/just) and `sqlite3`.

```sh
just db        # download the TLC files to data/ and build data/taxi.db
just db-stats  # row counts and file size
just check     # format, lint, code-shape limits, unit tests, BDD scenarios
```

The design is in [docs/design.md](docs/design.md).

## Stack

Rust (stable), Apache Arrow, Parquet, DataFusion, SQLite. Tests: cucumber (BDD) outside, unit tests (TDD) inside.

## Data

[NYC TLC trip records](https://www.nyc.gov/site/tlc/about/tlc-trip-record-data.page) and the taxi zone lookup, both public. Data is downloaded to `data/` on your machine and never committed.

## How it is built

Every change starts from a test. Every number said on camera (file size, query time, memory) is measured in this repo, with the machine and the command written down.

## License

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or [MIT license](LICENSE-MIT), at your option.
