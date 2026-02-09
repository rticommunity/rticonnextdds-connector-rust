# RTI Connector for Rust

## Contributions

This project is not open for contributions at this time.
If you have suggestions or feedback, please share them through [Github Issues][gh-issues].

[gh-issues]: https://www.github.com/rticommunity/rticonnextdds-connector-rust/issues "RTI Connector for Rust Github Issues"

## Development

This section describes how to build and test Connector for Rust from source.

### Building

To build the crate from source, ensure you have Rust and Cargo installed.
You can build the crate using Cargo:

```console
cargo build
```

### Testing

While tests can be run with `cargo test`, we use `cargo-nextest`.
You can install it with:

```console
cargo install cargo-nextest
```

Once installed, you can run the tests with:

```console
cargo nextest run
```

### Code Coverage

We use `cargo-tarpaulin` to measure code coverage.
You can install it with:

```console
cargo install cargo-tarpaulin
```

You can then run it with:

```console
cargo tarpaulin
```
