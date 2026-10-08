# model

The Sootmark record and event model, and the adapter contract.

```toml
[dependencies]
sootmark-model = "0.2"
```

Records carry full fidelity and provenance: evidence hash, physical locator, parser name and version, typed fields, shared facets and timestamps with their meaning. Record ids are derived from physical location, never parse order, so marks and findings survive re-parsing. The `adapter` module defines the contract every parser adapter keeps: deterministic, never silent, never panics, honest provenance, physical locators. An adapter can also read files too large to hold in memory as a stream (`Adapter::parse_stream`), an SQLite database with the write-ahead log beside it (`Adapter::parse_with_log`), a file with the companion files beside it in its folder (`Adapter::parse_with_companions`), and a file with any other file of its collection, looked up by path (`Adapter::parse_with_collection` and the `Collection` trait: a macOS unified log's tracev3 file and the `uuidtext` files its format strings are in).

## Quality

`#![forbid(unsafe_code)]`, `clippy::pedantic` clean, `cargo-deny` (permissive licences, no network crates).

## License

Licensed under either of [Apache License 2.0](LICENSE-APACHE) or [MIT](LICENSE-MIT), at your option.
