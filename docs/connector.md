# Connector and related utilities

The [`Connector`] type is the entry point to the RTI Connector for Rust API.
It wraps a DDS `DomainParticipant` configured from an XML file and is used
to acquire [`Input`] and [`Output`] handles for reading and writing data.

## Typical flow

1. Create a `Connector` with [`Connector::new`], passing a participant name and
   the path to an XML configuration file.
2. Acquire an [`Input`] or [`Output`] with [`Connector::get_input`] and
   [`Connector::get_output`].
3. Use the `Input`/`Output` APIs to read or write samples.

```text
let connector = Connector::new("MyLibrary::MyParticipant", "/path/to/App.xml")?;
let input = connector.get_input("MySubscriber::MyReader")?;
let output = connector.get_output("MyPublisher::MyWriter")?;
```

## Ownership and blocking acquisition

`Connector` enforces thread-aware ownership for `Input` and `Output` instances.
If a named entity is already in use, [`Connector::get_input`] and
[`Connector::get_output`] will return an error. Use [`Connector::take_input`]
or [`Connector::take_output`] to wait until the entity becomes available.

## Waiting for data

You can wait for any `Input` owned by the connector to receive data using
[`Connector::wait_for_data`] or [`Connector::wait_for_data_with_timeout`].
These calls do not read data; you still need to call [`Input::read`] or
[`Input::take`] on the specific `Input` you want to process.
