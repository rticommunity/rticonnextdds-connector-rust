# Reading data

[`crate::Input`] wraps a DDS `DataReader` and provides an iterator-based
interface over samples returned by the RTI Connector API.

## Getting the input

```rust
use rtiddsconnector::{Connector, Input};

fn get_input(connector: &Connector) -> rtiddsconnector::ConnectorResult<Input<'_>> {
    connector.get_input("MySubscriber::MyReader")
}
```

## Reading or taking the data

* [`crate::Input::read`]: copies samples into the local cache without removing
  them from the reader.
* [`crate::Input::take`]: removes samples from the reader and places them into
  the local cache.

After calling either, iterate over the cache:

```rust
use rtiddsconnector::Input;

fn process(input: &mut Input) -> rtiddsconnector::ConnectorFallible {
    input.take()?;

    for sample in input.into_iter().valid_only() {
        println!("Sample: {}", sample);
    }

    Ok(())
}
```

`valid_only()` skips samples with invalid data.

## Waiting for data

Use [`crate::Input::wait`] or [`crate::Input::wait_with_timeout`] to block until
data is available. These methods do not read data; call `read` or `take`
afterward.

If you want to wait for data on any input owned by a connector, use
[`crate::Connector::wait_for_data`] or
[`crate::Connector::wait_for_data_with_timeout`]. These methods do not read
samples; call `read` or `take` afterward.

## Accessing data samples

`Sample` provides typed accessors and JSON access:

* [`crate::Sample::get_number`]
* [`crate::Sample::get_boolean`]
* [`crate::Sample::get_string`]
* [`crate::Sample::get_value_json`]

`Sample` implements `Display` to print the full JSON representation of the
sample.

If you need to access meta-data fields (SampleInfo), see the next section.

## Returning the loan

If you used [`crate::Input::take`], return the loan when you are done using
[`crate::Input::return_loan`]. This allows the underlying reader to reuse
resources sooner.

## Accessing sample meta-data

Every sample includes associated meta-data (SampleInfo). You can access these
fields using [`crate::Sample::get_info`] and [`crate::Sample::get_info_json`].

If a sample has invalid data, it still carries meta-data. You can detect this
with [`crate::Sample::is_valid`]. For the list of available info fields and their
meaning, refer to the RTI Connector documentation.

## Matching publications

Use [`crate::Input::wait_for_publications`] or
[`crate::Input::wait_for_publications_with_timeout`] to wait for matched
writers. These methods return the change in the number of matched publications
since the last call.

You can inspect the current list with
[`crate::Input::display_matched_publications`], which returns JSON.
