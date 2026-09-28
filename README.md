# model

The Sootmark record and event model, and the adapter contract.

Records carry full fidelity and provenance: evidence hash, physical locator, parser name and version, typed fields, shared facets and timestamps with their meaning. Record ids are derived from physical location, never parse order, so marks and findings survive re-parsing. The `adapter` module defines the contract every parser adapter keeps: deterministic, never silent, never panics, honest provenance, physical locators.

## Quality

`#![forbid(unsafe_code)]`, `clippy::pedantic` clean, `cargo-deny` (permissive licences, no network crates).

## License

Licensed under either of [Apache License 2.0](LICENSE-APACHE) or [MIT](LICENSE-MIT), at your option.
