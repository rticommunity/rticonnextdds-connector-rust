# Connector lifecycle

This chapter covers creating a connector, acquiring inputs and outputs, and
cleaning up native resources.

[`crate::Connector`] represents a DDS `DomainParticipant` configured from XML.
It owns native resources and creates `Input` and `Output` handles.

## Importing the crate

To import the crate:

```rust
use rtiddsconnector::{self, Connector};
```

## Creating a connector

To create a connector, pass an XML file and a configuration name to
[`crate::Connector::new`]:

```rust
use rtiddsconnector::Connector;

fn create_connector() -> rtiddsconnector::ConnectorFallible {
    let _connector = Connector::new("MyLibrary::MyParticipant", "App.xml")?;
    Ok(())
}
```

The XML file defines your types, QoS profiles, and DDS entities. The call above
loads a `<domain_participant>` from a `<domain_participant_library>`. For example:

```xml
<domain_participant_library name="MyParticipantLibrary">
  <domain_participant name="MyParticipant" domain_ref="MyDomainLibrary::MyDomain">
    ...
  </domain_participant>
</domain_participant_library>
```

See a complete example in `examples/MyApplication.xml`.

> **Note:** Operations on the same `Connector` or its contained entities are not
> protected for multi-threaded access. See
> [Threading and ownership](crate::guide::threading) for guidance.

## Closing a connector

There is no explicit `close()` method in Rust. Instead, `Connector`, `Input`, and
`Output` are released when they go out of scope. The crate uses RAII to free
native resources.

To force cleanup of Connext global resources at the end of a scope (for example,
in tests), use [`crate::GlobalsDropGuard`].

## Getting inputs and outputs

Once you have created a connector, use [`crate::Connector::get_input`] and
[`crate::Connector::get_output`] to retrieve inputs and outputs.

> **Note:** If the `<domain_participant>` you load contains both `<data_writer>`
> and `<data_reader>` tags for the same topic and they have matching QoS, inputs
> may receive data before you call `get_input`. To avoid this, configure the
> `<subscriber>` that contains the `<data_reader>` with
> `<subscriber_qos>/<entity_factory>/<autoenable_created_entities>` set to
> `false`. Then inputs will only receive data after you call `get_input`.

See [Publishing data](crate::guide::output) and [Reading data](crate::guide::input)
for the workflow that follows.
