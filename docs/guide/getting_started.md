# Getting started

This crate is intended to be consumed from Cargo as a Git dependency. The
examples in `snippets/` and `examples/` are also included in the repository.

## Add the dependency

```console
cargo add --git https://github.com/rticommunity/rticonnextdds-connector-rs
```

If you need a specific branch, add `--branch <branch-name>`.

## Ensure native libraries are available

At build time, `build.rs` locates the RTI Connector C libraries in this order:

1. `RTI_CONNECTOR_VERSION`: downloads the target-specific libraries from the
   RTI Connector GitHub releases.
2. `RTI_CONNECTOR_DIR`: uses a local directory containing the libraries.
3. `CARGO_MANIFEST_DIR/rticonnextdds-connector`: uses a local directory next to
   the crate.
4. `CONNECTOR_VERSION` file: falls back to a version file in the crate root.

At runtime, ensure the native libraries are discoverable in your system's
library path (for example `DYLD_LIBRARY_PATH` on macOS).

## Minimal example

This is the typical flow: create a connector, obtain an output and input, write
one sample, then read samples back.

```rust
use rtiddsconnector::{Connector, GlobalsDropGuard, Input, Output};

fn main() -> rtiddsconnector::ConnectorFallible {
    let _globals = GlobalsDropGuard;

    let connector = Connector::new("MyLibrary::MyParticipant", "/path/to/App.xml")?;
    let mut output: Output<'_> = connector.get_output("MyPublisher::MyWriter")?;
    let mut input: Input<'_> = connector.get_input("MySubscriber::MyReader")?;

    let mut instance = output.instance();
    instance.set_number("x", 100.0)?;
    instance.set_number("y", 200.0)?;
    instance.set_string("color", "BLUE")?;
    output.write()?;

    input.wait_with_timeout(std::time::Duration::from_secs(5))?;
    input.take()?;

    for sample in input.into_iter().valid_only() {
        println!("Sample: {}", sample);
    }

    Ok(())
}
```

For a larger example, see `examples/shapes`.
