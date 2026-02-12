# RTI Connector for Rust (Experimental)

_RTI Connext_ is a connectivity software framework for integrating data sources
of all types. At its core is the world's leading ultra-high performance,
distributed networking databus.

_RTI Connector_ provides a quick and easy way to write applications that publish
and subscribe to the _RTI Connext_ DDS databus in Rust and other languages.

_RTI Connector for Rust_ is an experimental crate that provides Rust bindings
for the _RTI Connector_ API. It does so by exposing a safe and idiomatic Rust
interface over the underlying C API, in a crate called `rtiddsconnector`.

## Experimental Status

_RTI Connector for Rust_ is **currently experimental** and intended for evaluation, prototyping,
feedback and _Not recommended_ for production use.
Use this crate to explore Rust + DDS integration and help shape the future of Rust
support in RTI products.

**Do not use Connector for Rust in production**.

## Documentation

See the [Connector for Rust API][api-reference] for complete API documentation.

You can find documentation for _RTI Connext_, _RTI Connector_, and all other
RTI products on the following sites:

* [RTI Website](https://www.rti.com/)
* [RTI Community][rti-community]

[rti-community]: https://community.rti.com/ "RTI Community Forum"
[api-reference]: https://rticommunity.github.io/rticonnextdds-connector-rust/ "RTI Connector for Rust API Reference"

## Quick Start

This crate is intended to be managed through Cargo as a Git repository. To add
`rtiddsconnector` to your project dependencies, enter:

```console
cargo add \
    --git https://www.github.com/rticommunity/rticonnextdds-connector-rust
```

Once the crate is available, you can start using it in your Rust code. See the
quickstart file located at [`snippets/quickstart.rs`](snippets/quickstart.rs)
for a simple example of how to use the _RTI Connector_ API.

## Dependencies, Platform Support, and Versioning

Connector for Rust depends on native RTI Connector C libraries and is compatible with the latest
release of [rticonnextdds-connector][rti-github-connector]. The platform availability and _Connext_
_Professional_ release compatibility is linked to the _Connector_ version and can be found in the
release notes.

At build time, the scripts in `build.rs` will download the required libraries from the
[_RTI Connector_ releases in Github][rti-github-connector]. For more information, see the
[Connector for Rust API reference][api-reference].

At runtime, ensure that your system's linker can find the required libraries. This usually involves
placing the native libraries next to the executable or using your system's dynamic library path
environment variable (`LD_LIBRARY_PATH` on Linux, `DYLD_LIBRARY_PATH` on macOS, or `PATH` on Windows).

[rti-github-connector]: https://github.com/rticommunity/rticonnextdds-connector/releases "RTI Connector Github repository"

## Examples

The crate includes a `shapes` example compatible with _[RTI Shapes Demo][shapes-demo]_.
This example can be run as a publisher or as a subscriber. You can read more
about it in the example's [README](examples/shapes/README.md).

[shapes-demo]: https://community.rti.com/static/documentation/connext-dds/current/doc/manuals/connext_dds_professional/tools/shapes_demo/shapes_demo/ShapesTitle.htm "RTI Shapes Demo"

The example contains read-only snippets in the `snippets` module
that demonstrate various features of the _Connector_ API, and are used in the
documentation.

## Feedback and Support

* Technical Issues: For bugs, build problems, or API-related issues, open an [issue in GitHub][gh-issues].
  Provide clear reproduction steps and environment details to help us respond more effectively.
* Product Feedback and General Questions: For non-technical questions, usability feedback, or input
  on future direction, email [product-feedback@rti.com]. Your feedback helps shape the evolution
  of Rust support in RTI products.

[gh-issues]: https://www.github.com/rticommunity/rticonnextdds-connector-rust/issues "RTI Connector for Rust Github Issues"

## License

_RTI Connector_ for Rust is part of the RTI Connext Professional Package.
If you have a valid license for the RTI Connext Professional Package,
such license shall govern your use of _RTI Connector_ for Rust.
All other use of this software shall be governed solely by the terms of
RTI’s Software License for Non-Commercial Use \#4040, included at the
[top level of this repository](LICENSE.pdf).

With the sole exception of the contents of the "examples" subdirectory, all use
of this product is subject to the RTI Software License Agreement included at the
top level of this repository. Files within the "examples" subdirectory are
licensed as marked within the file.

This software is an experimental (aka "pre-production") product. The Software is
provided "as is", with no warranty of any type, including any warranty for
fitness for any purpose. RTI is under no obligation to maintain or support the
Software. RTI shall not be liable for any incidental or consequential damages
arising out of the use or inability to use the software.
