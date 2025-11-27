# RTI Connector for Rust

This crate provides Rust bindings for the RTI Connector API, which allows
easy integration of RTI Connext DDS functionality into Rust applications.

* [RTI Website](https://www.rti.com/)
* [RTI Community](https://community.rti.com/)
* [API Reference][api-reference]

[api-reference]: https://rticommunity.github.io/rticonnextdds-connector-rs/ "RTI Connector for Rust API Reference"

## Quick Start

This crate is intended to be managed through Cargo as a Git repository. That is,
in order to add it to your project dependencies, you would do:

```console
cargo add \
    --git https://www.github.com/rticommunity/rticonnextdds-connector-rs \
    --branch <branch-name>
```

Where `<branch-name>` is the branch you want to use, e.g. `master` or a specific
in-development branch. Omitting `--branch` will default to `master`.

Once available, you can start using the crate in your rust code. Check the
quickstart located at [`snippets/quickstart.rs`](snippets/quickstart.rs) for a
simple example of how to use the Connector API.

### Note on Native Libraries

This crate requires the RTI Connector C libraries to be available both during
build and at runtime.

At build time, the scripts in `build.rs` will attempt to download the required
libraries from the [RTI Connector releases in Github][rti-github-connector].

[rti-github-connector]: https://github.com/rticommunity/rticonnextdds-connector/releases "RTI Connector Github repository"

You can tune this behavior by setting the environment variable
`RTI_CONNECTOR_VERSION` to the desired version (e.g. `1.4.0`) before building
your application with Cargo. For more information, see
[the HTML documentation][api-reference].

At runtime, you'll need to ensure that your system's linker can find the
required libraries. This usually involves placing the native libraries
side-to-side with the executable or using your system's dynamic library path
environment variable (e.g. `LD_LIBRARY_PATH` on Linux, `DYLD_LIBRARY_PATH` on
macOS, or `PATH` on Windows).

## Examples

The crate includes a `shapes` example compatible with the RTI Shapes Demo.
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

Please, check the [CONTRIBUTING.md](CONTRIBUTING.md) file for contribution and
development guidelines.
