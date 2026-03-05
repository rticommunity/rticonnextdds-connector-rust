# Reading data (Input)

[`crate::Input`] wraps a DDS `DataReader` and provides an iterator-based
interface over samples returned by the RTI Connector API.

## Getting the input

To read or take samples, first get a reference to the input:

```rust
use rtiddsconnector::{Connector, Input};

fn get_input(connector: &Connector) -> rtiddsconnector::ConnectorResult<Input> {
    connector.get_input("MySubscriber::MyReader")
}
```

## Reading or taking the data

Call [`crate::Input::take`] to access and remove samples:

```rust,compile_fail
input.take()?;
```

Or call [`crate::Input::read`] to access samples but leave them available for a
future `read` or `take`:

```rust,compile_fail
input.read()?;
```

Use [`crate::Input::wait`] or [`crate::Input::wait_with_timeout`] to block until
new data is available on a specific input. These methods do not read data; call
`read` or `take` afterward.

If you want to wait for data on any input owned by a connector, use
[`crate::Connector::wait_for_data`] or
[`crate::Connector::wait_for_data_with_timeout`]. These methods do not read
samples; call `read` or `take` afterward.

## Accessing the data samples

After calling [`crate::Input::read`] or [`crate::Input::take`], iterate over the
samples:

```rust,compile_fail
for sample in input.into_iter() {
    if sample.is_valid()? {
        println!("{}", sample);
    }
}
```

To skip invalid samples, use `valid_only()`:

```rust,compile_fail
for sample in input.into_iter().valid_only() {
    println!("{}", sample);
}
```

`Sample` provides typed accessors and JSON access:

* [`crate::Sample::get_number`]
* [`crate::Sample::get_boolean`]
* [`crate::Sample::get_string`]
* [`crate::Sample::get_value_json`]

`Sample` implements `Display` to print the full JSON representation of the
sample.

If you need to access meta-data fields (SampleInfo), see [Accessing sample meta-data](#accessing-sample-meta-data).

## Returning the loan

If you used [`crate::Input::take`], return the loan when you are done using
[`crate::Input::return_loan`]. This allows the underlying reader to reuse
resources sooner.

## Accessing sample meta-data

Every sample contains an associated SampleInfo with meta-data about the
sample:

```rust,compile_fail
for sample in input.into_iter() {
    let source_timestamp = sample.get_info("source_timestamp")?;
    println!("source_timestamp: {:?}", source_timestamp);
}
```

See [`crate::Sample::get_info`] for the list of available meta-data fields.

*Connext DDS* can produce samples with invalid data, which contain meta-data
only. For more information about this, see the Valid Data flag in the RTI
Connext DDS Core Libraries User's Manual:
<https://community.rti.com/static/documentation/connext-dds/7.3.0/doc/manuals/connext_dds_professional/users_manual/index.htm#users_manual/AccessingManagingInstances.htm#Valid>.
These samples indicate a change in the instance state. Samples with invalid data
still provide the following information:

* The SampleInfo
* When an instance is disposed (`sample.get_info("instance_state")` is
  `NOT_ALIVE_DISPOSED`), the sample data contains the value of the key that has
  been disposed. You can access the key fields only.

## Matching with a publication

Use [`crate::Input::wait_for_publications`] or
[`crate::Input::wait_for_publications_with_timeout`] to detect when a
compatible publication is matched or unmatched. These methods return the change
in the number of matched publications since the last call.

You can inspect the current list with
[`crate::Input::display_matched_publications`], which returns JSON.
