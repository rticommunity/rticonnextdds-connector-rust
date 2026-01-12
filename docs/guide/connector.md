# Connector lifecycle

[`crate::Connector`] represents a DDS `DomainParticipant` configured from XML.
It owns native resources and creates `Input` and `Output` handles.

## Importing the crate

```rust
use rtiddsconnector::Connector;
```

## Creating a connector

Create a connector with [`crate::Connector::new`], passing the namespaced
participant name and the XML file path:

```rust
use rtiddsconnector::Connector;

fn create_connector() -> rtiddsconnector::ConnectorFallible {
    let _connector = Connector::new("MyLibrary::MyParticipant", "App.xml")?;
    Ok(())
}
```

The XML file defines your types, QoS profiles, and DDS entities. The call above
loads a `<domain_participant>` from a `<domain_participant_library>`, for example:

```xml
<domain_participant_library name="MyParticipantLibrary">
  <domain_participant name="MyParticipant" domain_ref="MyDomainLibrary::MyDomain">
    ...
  </domain_participant>
</domain_participant_library>
```

See a complete example in `examples/MyApplication.xml`.

## Closing a connector

There is no explicit `close()` method in Rust. Instead, `Connector`, `Input`, and
`Output` are released when they go out of scope. The crate uses RAII to free
native resources.

If you want to force cleanup of Connext global resources at the end of a scope
(for example in tests), use [`crate::GlobalsDropGuard`].

## Getting inputs and outputs

[`crate::Connector::get_input`] and [`crate::Connector::get_output`] return
owned handles to the named reader/writer. The crate enforces single-threaded
ownership: if an entity is already in use, these calls return an error. To wait
until the entity becomes available, use [`crate::Connector::take_input`] or
[`crate::Connector::take_output`].

See [Publishing data](crate::guide::output) and [Reading data](crate::guide::input)
for the workflow that follows.

## Threading note

Operations on the same `Connector` or its contained entities are not protected
for multi-threaded access. See [Threading and ownership](crate::guide::threading)
for guidance.

## Auto-enable considerations

Connext DDS controls entity enablement through XML QoS settings (for example,
`<entity_factory>/<autoenable_created_entities>`). If readers are enabled before
you acquire an `Input`, data may already be available when you call
`get_input`. This behavior comes from Connext configuration rather than the
Rust API itself.
