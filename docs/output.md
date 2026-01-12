# Output, Instance, and related utilities

The [`Output`] interface is used to write data out to a DDS domain. It wraps a
`DataWriter` configured in your XML file and exposes a single writable
[`Instance`] that represents the next sample to be published.

## Writing data

Modify the instance with `set_*` methods or JSON, then publish it using
[`Output::write`]:

```text
let mut instance = output.instance();
instance.set_string("color", "BLUE")?;
instance.set_number("x", 100.0)?;
instance.set_number("y", 150.0)?;

output.write()?;
```

For typed data, use [`Instance::serialize`] to serialize a struct via Serde.

## Write parameters

Use [`Output::write_with_params`] and [`WriteParams`] to control actions like
`dispose` or `unregister`, or to attach timestamps and identities.

## Waiting and matching

You can wait for acknowledgments with [`Output::wait`] or
[`Output::wait_with_timeout`], and you can wait for subscriptions with
[`Output::wait_for_subscriptions`] or [`Output::wait_for_subscriptions_with_timeout`].
Use [`Output::display_matched_subscriptions`] to retrieve matched subscriptions
as JSON.
