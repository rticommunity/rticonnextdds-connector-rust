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

# User Guide

Start here for a tour of the crate:

* [Getting started](crate::guide::getting_started)
* [Configuration](crate::guide::configuration)
* [Connector lifecycle](crate::guide::connector)
* [Publishing data](crate::guide::output)
* [Reading data](crate::guide::input)
* [Data access and Serde](crate::guide::data)
* [Error handling](crate::guide::errors)
* [Threading and ownership](crate::guide::threading)
* [Advanced operations](crate::guide::advanced)

