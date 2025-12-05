# RTI Connector for Rust

RTI® Connext® is a connectivity software framework for integrating data sources
of all types. At its core is the world's leading ultra-high performance,
distributed networking databus.

_RTI Connector_ provides a quick and easy way to write applications that publish
and subscribe to the _RTI Connext_ DDS databus in Rust and other languages.

## Documentation

This crate provides Rust bindings for the _RTI Connector_ API, which allows
easy integration of _RTI Connext_ DDS functionality into Rust applications.

* [RTI Website](https://www.rti.com/)
* [RTI Community](https://community.rti.com/)
* [API Reference][api-reference]

[api-reference]: https://rticommunity.github.io/rticonnextdds-connector-rs/ "RTI Connector for Rust API Reference"

## Quick Start

This crate is intended to be managed through Cargo as a Git repository. To add
it to your project dependencies, enter:

```console
cargo add \
    --git https://www.github.com/rticommunity/rticonnextdds-connector-rs \
    --branch <branch-name>
```

`<branch-name>` is the branch you want to use; for example, `master` or a
specific in-development branch. If `--branch` is not defined, `master` is used by default.

Once the crate is available, you can start using it in your Rust code. See the
quickstart file located at [`snippets/quickstart.rs`](snippets/quickstart.rs)
for a simple example of how to use the _RTI Connector_ API.

### Note on Native Libraries

This crate requires the _RTI Connector_ C libraries to be available both during
build and at runtime.

At build time, the scripts in `build.rs` will attempt to download the required
libraries from the [_RTI Connector_ releases in Github][rti-github-connector].

[rti-github-connector]: https://github.com/rticommunity/rticonnextdds-connector/releases "RTI Connector Github repository"

To tune this behavior, set the environment variable
`RTI_CONNECTOR_VERSION` to the desired version (e.g. `1.4.0`) before building
your application with Cargo. For more information, see
[the HTML documentation][api-reference].

At runtime, ensure that your system's linker can find the
required libraries. This usually involves placing the native libraries
next to the executable or using your system's dynamic library path
environment variable (`LD_LIBRARY_PATH` on Linux, `DYLD_LIBRARY_PATH` on
macOS, or `PATH` on Windows).

## Examples

The crate includes a `shapes` example compatible with *RTI Shapes Demo*.
This example can be run as a publisher or as a subscriber. You can read more
about it [in its README](examples/shapes/README.md).

Additionally, it contains read-only snippets in the `snippets` module that
demonstrate various features of the Connector API, and are used in the
documentation.

## License

With the sole exception of the contents of the "examples" subdirectory, all use
of this product is subject to the RTI Software License Agreement included at the
top level of this repository. Files within the "examples" subdirectory are
licensed as marked within the file.

This software is an experimental (aka "pre-production") product. The Software is
provided "as is", with no warranty of any type, including any warranty for
fitness for any purpose. RTI is under no obligation to maintain or support the
Software. RTI shall not be liable for any incidental or consequential damages
arising out of the use or inability to use the software.

## Contributing

See the [CONTRIBUTING.md](CONTRIBUTING.md) file for contribution and
development guidelines.
