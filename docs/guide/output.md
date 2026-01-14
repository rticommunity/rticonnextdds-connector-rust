# Publishing data (Output)

[`crate::Output`] wraps a DDS DataWriter and exposes a single mutable
[`crate::Instance`] representing the next sample to write.

## Getting the output

To write a data sample, first look up an output:

```rust
use rtiddsconnector::{Connector, Output};

fn get_output(connector: &Connector) -> rtiddsconnector::ConnectorResult<Output<'_>> {
    connector.get_output("MyPublisher::MyWriter")
}
```

## Populating the data sample

The next step is to set the `Instance` fields. You can set them member-by-member:

```rust
use rtiddsconnector::Output;

fn set_fields(output: &mut Output) -> rtiddsconnector::ConnectorFallible {
    let mut instance = output.instance();
    instance.set_number("x", 1.0)?;
    instance.set_number("y", 2.0)?;
    instance.set_number("shapesize", 30.0)?;
    instance.set_string("color", "BLUE")?;
    Ok(())
}
```

Or using JSON:

```rust
use rtiddsconnector::Output;

fn set_json(output: &mut Output) -> rtiddsconnector::ConnectorFallible {
    let mut instance = output.instance();
    instance.set_as_json(r#"{"x":1,"y":2,"shapesize":30,"color":"BLUE"}"#)?;
    Ok(())
}
```

For strongly-typed models, see [Data access and Serde](crate::guide::data) and
[`crate::Instance::serialize`].

Field names correspond to the type assigned to the output in XML. For example:

```xml
<struct name="ShapeType">
  <member name="color" type="string" stringMaxLength="128" key="true" default="RED"/>
  <member name="x" type="long" />
  <member name="y" type="long" />
  <member name="shapesize" type="long" default="30"/>
</struct>
```

## Writing the data sample

To write the values that have been set in the instance, call
[`crate::Output::write`]:

```rust
use rtiddsconnector::Output;

fn write_once(output: &mut Output) -> rtiddsconnector::ConnectorFallible {
    output.write()
}
```

If the DataWriter QoS is reliable, you can use [`crate::Output::wait`] or
[`crate::Output::wait_with_timeout`] to wait for acknowledgments:

```rust
use rtiddsconnector::Output;

fn write_and_wait(output: &mut Output) -> rtiddsconnector::ConnectorFallible {
    output.write()?;
    output.wait()
}
```

To write with parameters such as a source timestamp, use [`crate::WriteParams`]
with [`crate::Output::write_with_params`]:

```rust
use rtiddsconnector::{Output, WriteParams};

fn write_with_timestamp(output: &mut Output, ts: i64) -> rtiddsconnector::ConnectorFallible {
    let params = WriteParams::write().with_source_timestamp(ts);
    output.write_with_params(&params)
}
```

It is also possible to dispose or unregister an instance:

```rust
use rtiddsconnector::{Output, WriteParams};

fn dispose_instance(output: &mut Output) -> rtiddsconnector::ConnectorFallible {
    let params = WriteParams::dispose();
    output.write_with_params(&params)
}
```

In these two cases, only the key fields are relevant.

## Matching with a subscription

Use [`crate::Output::wait_for_subscriptions`] or
[`crate::Output::wait_for_subscriptions_with_timeout`] to detect when a
compatible subscription is matched or unmatched. These methods return the
change in the number of matches since the last call.

You can inspect the current list of matched subscriptions as JSON with
[`crate::Output::display_matched_subscriptions`].
