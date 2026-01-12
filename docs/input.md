# Input, Sample, and related utilities

The [`Input`] interface is used to read data samples from DDS topics. It wraps a
`DataReader` configured in your XML file and provides a cached view of samples
retrieved by [`Input::read`] or [`Input::take`].

## Reading data

Use [`Input::read`] to fill the local sample cache without removing samples from
DDS, or [`Input::take`] to remove them from the reader. After either call, iterate
samples using `for sample in input.into_iter()` or filter valid samples with
[`ValidSampleIterator`]:

```text
input.take()?;

for sample in input.into_iter().valid_only() {
    println!("Sample: {}", sample);
}
```

If you call `take`, you can return the loan with [`Input::return_loan`] after you
finish processing.

## Waiting for data

To block until data is available, use [`Input::wait`] or
[`Input::wait_with_timeout`]. These calls do not read data; they only wait for
availability. You still need to call `read` or `take` to populate the cache.

## Accessing fields

A [`Sample`] provides typed accessors like [`Sample::get_number`],
[`Sample::get_string`], and [`Sample::get_boolean`]. For dynamic access, use
[`Sample::get_value`] which returns a [`SelectedValue`], or get a JSON string
using [`Sample::get_value_json`]. You can also access sample info fields via
[`Sample::get_info`] and [`Sample::get_info_json`].

## Matched publications

Use [`Input::wait_for_publications`] or [`Input::wait_for_publications_with_timeout`]
to wait for writers, and [`Input::display_matched_publications`] to retrieve the
matched publications as JSON.
