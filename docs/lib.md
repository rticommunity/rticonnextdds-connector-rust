# A simple DDS API for Rust (Experimental)

The `rtiddsconnector` crate provides a lightweight interface with which to
access DDS domains from Rust using [RTI Connext][rti-pro] C libraries and the
simplified _RTI Connector_ API.

The API offered by `rtiddsconnector` does **NOT** match the
[standard DDS API][omg-dds]. Rather, it offers a simplified interface  developed
for faster and easier integration in any programming language with access to
native C code.

The [_RTI Connector_ Github project page][gh-connector] contains more information about
`rtiddsconnector`'s C interface and bindings to other languages (such as
Javascript). Following are the most important characteristics of
the _RTI Connector_ API:

* It is based on an external [XML configuration][omg-dds-xml] file that
  fully describes the DDS _DomainParticipant_ and its contained entities
  (_Types_, _Topics_, _DataReaders_, _DataWriters_, etc).
* It is based on Dynamic Data, removing the need for code generation steps
  typically required by DDS applications.
* It features implicit memory management for data samples, simplifying
  the application code.
* It features implicit data conversions between DDS primitive types and
  native types supported by the target programming language.

> **IMPORTANT:**
> _Connector for Rust_ is an experimental product; do not use it in production
> systems. This release is an evaluation distribution; use it to explore using
> _RTI Connext_ functionality into Rust applications. As an experimental RTI
> product, we only offer support through the [RTI Community Forum][rti-community],
> backed by RTI engineers who will answer your questions.

[rti-pro]: https://www.rti.com/products/dds "RTI Connext Professional"
[rti-community]: https://community.rti.com/ "RTI Community Forum"
[omg-dds]: https://www.omg.org/spec/DDS/ "OMG DDS Specification"
[omg-dds-xml]: https://www.omg.org/spec/DDS-XML/ "OMG DDS XML Specification"
[gh-connector]: https://github.com/rticommunity/rticonnextdds-connector "RTI Connector on Github"

## Overview

The `rtiddsconnector` crate exposes the _RTI Connector_ API through the following main abstractions:

* [`Connector`]: Represents a DDS _DomainParticipant_, and is used to create
  `Input` and `Output` objects for reading and writing data.
* [`Input`]: Represents a DDS _DataReader_, and is used to read data samples
  from DDS _Topics_.
* [`Output`]: Represents a DDS _DataWriter_, and is used to write data samples
  to DDS _Topics_.

In addition to these main abstractions, the crate also exposes, among others, the following types:

* [`Sample`]: A trait representing a data sample read from an `Input`.
* [`SampleIterator`]: An iterator over valid samples read from an `Input`.
* [`Instance`]: A struct representing a data sample to be written to an `Output`.
* [`ConnectorError`]: A struct representing errors that can occur when using the _Connector_ API.
* [`ConnectorResult`]: A type alias for `Result<T, ConnectorError>`, used for error handling.
* [`ConnectorFallible`]: A type alias for `Result<T, Box<dyn std::error::Error>>`, used for fallible operations.
* [`GlobalsDropGuard`]: A struct that ensures proper cleanup of global resources used by the _Connector_ API.

### Typed Data Support

This crate provides Serde-based serialization and deserialization support
for working with strongly-typed data structures instead of raw JSON.
It extends the `Instance` and `Sample` types with methods to serialize
Rust structs to JSON and deserialize JSON to Rust structs, respectively.

See the documentation for [`Instance::serialize`] and [`Sample::deserialize`].

### Error Handling

Because many operations in the _RTI Connector_ API can fail due to various reasons, most of them handled
externally in the underlying C implementation, the `rtiddsconnector` crate provides
the [`ConnectorError`] struct as an opaque representation of errors that can occur when using the API.

The [`ConnectorResult`] type alias is used throughout the crate to represent operations that can
succeed or fail with a [`ConnectorError`]. Most public methods in the crate return a [`ConnectorResult<T>`],
where `T` is the expected return type of the operation.

### Thread safety

The `rtiddsconnector` crate attempts to provide safe abstractions over the underlying
C implementation, which is not thread-safe. This means that while the crate attempts to use
Rust's safety guarantees, developers must be careful when using multiple threads.

### Build-time linking

Because `rtiddsconnector` is built on top of [RTI Connext][rti-pro] C libraries,
applications using this crate must ensure that the required C libraries are
available at runtime.

The `build.rs` script included in the crate can automatically configure
the linker to find the required libraries, namely:

1. If the `RTI_CONNECTOR_VERSION` environment variable is defined, it will attempt to download
   the target-specific C libraries from the [corresponding Github release][gh-connector-releases].
2. If the `RTI_CONNECTOR_DIR` environment variable is defined, it will use the C libraries
   found in the specified directory.
3. If neither of the above environment variables is defined, it will attempt to find the C libraries
   in a default location based on common installation paths.

If neither of these methods is successful, the build will fail with an error message
indicating that the C libraries could not be found.

Once your application has been built, `cargo` commands will automatically pick up the
linker instructions generated by `build.rs`, so no additional configuration is needed
to link the [RTI Connext][rti-pro] C libraries when using `cargo test` or similar.

However, when deploying your application, make sure that the required C libraries
are available in the system's library path, or in a location specified by
the appropriate environment variable (e.g., `LD_LIBRARY_PATH` on Linux,
`DYLD_LIBRARY_PATH` on macOS, or `PATH` on Windows).

[gh-connector-releases]: https://github.com/rticommunity/rticonnextdds-connector/releases "RTI Connector Releases on Github"
