# Build pivot

An Excel-style pivot table in Rust, built step by step on public New York City taxi trip data.

This is the code for the **Build pivot** playlist on [Ship by Step](https://www.youtube.com/@shipbystep), a channel about turning ideas into working software, one step at a time.

- Channel: [youtube.com/@shipbystep](https://www.youtube.com/@shipbystep)
- Playlist: [Build pivot](https://www.youtube.com/playlist?list=PLXOLqiYzf0f8)
- Other projects: [github.com/shipbystep](https://github.com/shipbystep)

## Status

Planning. There is no pivot code yet. Each published episode gets a git tag (`ep01`, `ep02`, …), so you can check out the code exactly as it was at the end of that episode.

## Stack

Rust (stable), Apache Arrow, Parquet, DataFusion.

## Data

[NYC TLC trip records](https://www.nyc.gov/site/tlc/about/tlc-trip-record-data.page) and the taxi zone lookup, both public. Data is downloaded to `data/` on your machine and never committed.

## How it is built

Every change starts from a test. Every number said on camera (file size, query time, memory) is measured in this repo, with the machine and the command written down.

## License

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or [MIT license](LICENSE-MIT), at your option.
