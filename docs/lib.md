# RTI Connector for Rust

The `rtiddsconnector` crate provides a lightweight interface with which to
access DDS domains from Rust using [RTI Connext][rti-pro] C libraries and the
simplified _RTI Connector_ API.

The API offered by `rtiddsconnector` does **NOT** match the
[standard DDS API][omg-dds]. Rather, it offers a simplified interface developed
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

## User Guide

Start here for a walkthrough-first tour of the crate:

* [API Overview](#api-overview)
* [Getting started](crate::guide::getting_started)
* [Configuration](crate::guide::configuration)
* [Connector lifecycle](crate::guide::connector)
* [Publishing data](crate::guide::output)
* [Reading data](crate::guide::input)
* [Data access and Serde](crate::guide::data)
* [Error handling](crate::guide::errors)
* [Threading and ownership](crate::guide::threading)
* [Advanced operations](crate::guide::advanced)

## API Overview

The `rtiddsconnector` crate exposes the _RTI Connector_ API through the following main abstractions:

* [`Connector`]: Represents a DDS _DomainParticipant_, and is used to create
  `Input` and `Output` objects for reading and writing data.
* [`Input`]: Represents a DDS _DataReader_, and is used to read data samples
  from DDS _Topics_.
* [`Output`]: Represents a DDS _DataWriter_, and is used to write data samples
  to DDS _Topics_.

In addition to these main abstractions, the crate also exposes, among others, the following types:

* [`Sample`]: A struct representing a data sample read from an `Input`.
* [`SampleIterator`]: An iterator over samples read from an `Input`.
* [`ValidSampleIterator`]: An iterator that filters only valid samples.
* [`Instance`]: A struct representing a data sample to be written to an `Output`.
* [`SelectedValue`]: A variant type used to set or read fields without JSON.
* [`ConnectorError`]: A struct representing errors that can occur when using the _Connector_ API.
* [`ConnectorResult`]: A type alias for `Result<T, ConnectorError>`, used for error handling.
* [`ConnectorFallible`]: A type alias for `Result<(), ConnectorError>`, used for fallible operations.
* [`GlobalsDropGuard`]: A struct that ensures proper cleanup of global resources used by the _Connector_ API.
