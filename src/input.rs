/*******************************************************************************
 * (c) 2025 Copyright, Real-Time Innovations.  All rights reserved.            *
 * No duplications, whole or partial, manual or electronic, may be made        *
 * without express written permission.  Any such copies, or revisions thereof, *
 * must display this notice unaltered.                                         *
 * This code contains trade secrets of Real-Time Innovations, Inc.             *
 *******************************************************************************/

#![doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/docs/input.md"))]

use crate::{
    Connector, ConnectorFallible, ConnectorResult, SelectedValue,
    result::{ErrorKind, InvalidErrorKind},
};

/// A wrapper which provides access to a single sample owned by an [`Input`].
///
/// Instances of this type are returned by the [`SampleIterator`] that can be
/// derived from an [`Input`] object:
///
/// ```rust
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/snippets/input/using_sample.rs"))]
/// ```
///
/// In addition to the data itself, each instance contains metadata which can be
/// accessed with [`Sample::get_info`] and related methods.
/// The list of available info fields include, but is not limited to:
///
/// - `valid_data`: A boolean indicating whether the sample contains valid data.
/// - `source_timestamp`: A string representing the source timestamp of the sample.
/// - `reception_timestamp`: A string representing the reception timestamp of the sample.
/// - `instance_state`: A string representing the instance state of the sample.
/// - `view_state`: A string representing the view state of the sample.
/// - `sample_state`: A string representing the sample state of the sample.
/// - `identity`: A string representing the identity of the sample publisher.
#[derive(Debug)]
pub struct Sample<'a> {
    /// The index of the sample within the [`Input`]'s samples cache.
    index: usize,

    /// A reference to the parent [`Input`] object.
    input: &'a Input<'a>,
}

/// Display the [`Sample`] as a JSON string.
impl std::fmt::Display for Sample<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.get_as_json() {
            Ok(json) => write!(f, "{}", json),
            Err(_) => write!(f, "<error retrieving sample as JSON>"),
        }
    }
}

impl Sample<'_> {
    /// Returns whether the sample contains valid data.
    pub fn is_valid(&self) -> ConnectorResult<bool> {
        self.input.is_valid(self.index)
    }

    /// Access a variant-type field in the sample's info.
    pub fn get_info(&self, field_name: &str) -> ConnectorResult<SelectedValue> {
        self.input.get_info(self.index, field_name)
    }

    /// Access a sample's info field as JSON.
    pub fn get_info_json(&self, field_name: &str) -> ConnectorResult<String> {
        self.input.get_info_json(self.index, field_name)
    }

    /// Access a boolean field in the sample.
    pub fn get_boolean(&self, field_name: &str) -> ConnectorResult<bool> {
        self.input.get_boolean(self.index, field_name)
    }

    /// Access a string field in the sample.
    pub fn get_string(&self, field_name: &str) -> ConnectorResult<String> {
        self.input.get_string(self.index, field_name)
    }

    /// Access a numeric field in the sample.
    pub fn get_number(&self, field_name: &str) -> ConnectorResult<f64> {
        self.input.get_number(self.index, field_name)
    }

    /// Access a variant-type field in the sample.
    pub fn get_value(&self, field_name: &str) -> ConnectorResult<SelectedValue> {
        self.input.get_field(self.index, field_name)
    }

    /// Access a field (as JSON) in the sample.
    pub fn get_value_json(&self, field_name: &str) -> ConnectorResult<String> {
        self.input.get_field_json(self.index, field_name)
    }

    /// Deserialize the sample into a concrete type using Serde.
    ///
    /// This method converts the sample's JSON representation into a strongly-typed
    /// Rust struct, providing type safety for received data.
    ///
    /// # Example
    /// ```rust
    #[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/snippets/input/using_deserialization.rs"))]
    /// ```
    pub fn deserialize<T>(&self) -> ConnectorResult<T>
    where
        T: for<'de> serde::Deserialize<'de>,
    {
        let json = self.get_as_json().map_err(|e| ErrorKind::Invalid {
            what: InvalidErrorKind::Deserialization,
            context: std::format!(
                "Failed getting JSON for deserialization of type '{}': {}",
                std::any::type_name::<T>(),
                e
            ),
        })?;

        let json = serde_json::from_str::<T>(&json).map_err(|e| ErrorKind::Invalid {
            what: InvalidErrorKind::Deserialization,
            context: std::format!(
                "Failed deserializing JSON ({}) into type '{}': {}",
                json,
                std::any::type_name::<T>(),
                e
            ),
        })?;

        Ok(json)
    }

    /// Turn the sample into a JSON string.
    pub(crate) fn get_as_json(&self) -> ConnectorResult<String> {
        self.input.get_json(self.index)
    }
}

/// An [`Iterator`] which returns individual [`Sample`] elements.
///
/// ```rust
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/snippets/input/using_sample_iterator.rs"))]
/// ```
pub struct SampleIterator<'a> {
    /// The current index in the iteration.
    index: usize,

    /// The total number of samples available.
    samples_len: usize,

    /// A reference to the parent [`Input`] object.
    input: &'a Input<'a>,
}

/// Implements the core iteration logic for [`SampleIterator`].
impl<'a> Iterator for SampleIterator<'a> {
    type Item = Sample<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.samples_len {
            let result = Some(Self::Item {
                index: self.index,
                input: self.input,
            });
            self.index += 1;

            result
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.len();
        (remaining, Some(remaining))
    }
}

/// Allows [`SampleIterator`] to implement `len()`.
impl ExactSizeIterator for SampleIterator<'_> {
    fn len(&self) -> usize {
        self.samples_len - self.index
    }
}

/// Allows transforming a [`SampleIterator`] into a [`ValidSampleIterator`].
impl<'a> SampleIterator<'a> {
    /// Create a [`ValidSampleIterator`] which yields only valid samples,
    /// out of this [`SampleIterator`].
    pub fn valid_only(self) -> ValidSampleIterator<'a> {
        ValidSampleIterator(self)
    }
}

/// A specialized [`SampleIterator`] which returns only valid [`Sample`] elements.
///
/// ```rust
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/snippets/input/using_valid_sample_iterator.rs"))]
/// ```
pub struct ValidSampleIterator<'a>(SampleIterator<'a>);

impl<'a> Iterator for ValidSampleIterator<'a> {
    type Item = Sample<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        for sample in &mut self.0.by_ref() {
            match sample.is_valid() {
                Ok(true) => return Some(sample),
                // Skip invalid samples and try the next one
                other => {
                    if let Err(e) = other {
                        eprintln!(
                            "Error checking sample validity, skipping sample: {}",
                            e
                        );
                    }
                    continue;
                }
            }
        }

        None // No more samples or error occurred
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        // We can't know how many valid samples are left without iterating
        (0, Some(self.0.len()))
    }
}

/// An interface to read data from a DDS `Topic`.
///
/// Created with [`Connector::get_input`], an [`Input`] represents a DDS
/// `DataReader` associated with a specific `Topic` within a `Participant`.
///
/// The main operations are [`Input::read`] and [`Input::take`], which move data
/// from the underlying `DataReader` into the [`Input`], at which point you can
/// access [`Sample`] by means of a [`SampleIterator`].
///
/// ```rust
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/snippets/input/using_input.rs"))]
/// ```
#[derive(Debug)]
pub struct Input<'a> {
    /// The name of the [`Input`] as known to the parent [`Connector`].
    name: String,

    /// A reference to the parent [`Connector`] object.
    parent: &'a Connector,
}

/// Allows obtaining a [`SampleIterator`] from an [`Input`].
impl<'a> IntoIterator for &'a Input<'a> {
    type Item = Sample<'a>;
    type IntoIter = SampleIterator<'a>;

    /// Create an `Iterator` over an `Input`'s samples cache.
    /// Note that the iterator will not consume the `Input`, but it
    /// will take an immutable borrow on it, preventing the
    /// sample cache from being modified by calls to [`Input::take()`]
    /// or [`Input::read()`].
    fn into_iter(self) -> Self::IntoIter {
        SampleIterator {
            index: 0,
            samples_len: self.get_count().unwrap_or(0), // On error, assume 0 samples
            input: self,
        }
    }
}

/// Ensures that the [`Input`] is freed to the parent [`Connector`]
impl<'a> Drop for Input<'a> {
    fn drop(&mut self) {
        if let Err(e) = self.parent.release_input(&self.name) {
            eprintln!(
                "Warning: Failed to release Input '{}' on drop: {}",
                self.name, e
            );
        }
    }
}

/// Kinds of data acquisition for the [`Input`].
enum ReadOrTake {
    /// Read samples without removing them from the underlying `DataReader`.
    Read,

    /// Take samples and remove them from the underlying `DataReader`.
    Take,
}

impl<'a> Input<'a> {
    pub(crate) fn new(name: &str, connector: &'a Connector) -> Input<'a> {
        Input {
            name: name.to_string(),
            parent: connector,
        }
    }

    /// Fill the [`Input`]'s received sample cache without
    /// emptying the underlying `DataReader`'s cache.
    /// This samples will be discard by the [`Input`] next time either
    /// [`Input::take()`] or [`Input::read()`] are called, but they will
    /// still be available for accesse until they are pushed out of
    /// the `DataReader`'s cache for other reasons (i.e. Quality of
    /// Service parameters, such as History or Resource Limits).
    pub fn read(&mut self) -> ConnectorFallible {
        self.impl_read_or_take(ReadOrTake::Read)
    }

    /// Fill the [`Input`]'s received sample cache by
    /// extracting samples from the underlying `DataReader`'s cache.
    /// This samples will be discard by the [`Input`] next time either
    /// [`Input::take()`] or [`Input::read()`] are called, and they
    /// will never be available for access again.
    pub fn take(&mut self) -> ConnectorFallible {
        self.impl_read_or_take(ReadOrTake::Take)
    }

    fn impl_read_or_take(&mut self, operation: ReadOrTake) -> ConnectorFallible {
        let result = {
            let native_mut = self.parent.native_mut()?;
            match operation {
                ReadOrTake::Read => native_mut.read(&self.name),
                ReadOrTake::Take => native_mut.take(&self.name),
            }
        };

        if let Err(e) = result
            && !e.is_native_error_code(crate::ffi::ReturnCode::NoData)
        {
            Err(e)
        } else {
            Ok(())
        }
    }

    /// Return the loan on the samples previously taken
    /// from the underlying `DataReader`'s cache.
    pub fn return_loan(&mut self) -> ConnectorFallible {
        self.parent.native_mut()?.return_loan(&self.name)
    }

    /// Wait indefinitely for data to be available on an `Input`.
    pub fn wait(&self) -> ConnectorFallible {
        self.impl_wait_for_data(None)
    }

    /// Wait for data to be available on an `Input`, or
    /// for a specified timeout to expire.
    pub fn wait_with_timeout(&self, timeout: std::time::Duration) -> ConnectorFallible {
        self.impl_wait_for_data(Some(
            // Durations cannot be negative
            timeout.as_millis().try_into().unwrap_or(i32::MAX),
        ))
    }

    fn impl_wait_for_data(&self, timeout_ms: Option<i32>) -> ConnectorFallible {
        self.parent
            .native_ref()?
            .get_input(&self.name)?
            .wait_for_data(timeout_ms)
    }

    /// Wait indefinitely for a publication to be matched
    pub fn wait_for_publications(&self) -> ConnectorResult<i32> {
        self.impl_wait_for_publications(None)
    }

    /// Wait for a publication to be matched, or
    /// for a specified timeout to expire.
    pub fn wait_for_publications_with_timeout(
        &self,
        timeout: std::time::Duration,
    ) -> ConnectorResult<i32> {
        self.impl_wait_for_publications(Some(
            // Durations cannot be negative
            timeout.as_millis().try_into().unwrap_or(i32::MAX),
        ))
    }

    fn impl_wait_for_publications(
        &self,
        timeout_ms: Option<i32>,
    ) -> ConnectorResult<i32> {
        self.parent
            .native_ref()?
            .get_input(&self.name)?
            .wait_for_matched_publication(timeout_ms)
    }

    /// Access the size of the `Input`'s received sample cache.
    fn get_count(&self) -> ConnectorResult<usize> {
        self.parent
            .native_ref()?
            .get_sample_count(&self.name)
            .map(|res| res as usize)
    }

    /// Access a numeric field in a received sample.
    fn get_number(&self, index: usize, field_name: &str) -> ConnectorResult<f64> {
        self.parent
            .native_ref()?
            .get_number_from_sample(&self.name, index, field_name)
    }

    /// Access a boolean field in a received sample.
    fn get_boolean(&self, index: usize, field_name: &str) -> ConnectorResult<bool> {
        self.parent
            .native_ref()?
            .get_boolean_from_sample(&self.name, index, field_name)
    }

    /// Access a string field in a received sample.
    fn get_string(&self, index: usize, field_name: &str) -> ConnectorResult<String> {
        self.parent
            .native_ref()?
            .get_string_from_sample(&self.name, index, field_name)
    }

    /// Access a variant-type field in a received sample.
    fn get_field(
        &self,
        index: usize,
        field_name: &str,
    ) -> ConnectorResult<SelectedValue> {
        self.parent
            .native_ref()?
            .get_from_sample(&self.name, index, field_name)
    }

    /// Access a field (as JSON) in a received sample.
    fn get_field_json(&self, index: usize, field_name: &str) -> ConnectorResult<String> {
        self.parent
            .native_ref()?
            .get_json_member(&self.name, index, field_name)
    }

    /// Access a variant-type field in a received sample's info.
    fn get_info(&self, index: usize, field_name: &str) -> ConnectorResult<SelectedValue> {
        self.parent
            .native_ref()?
            .get_from_info(&self.name, index, field_name)
    }

    /// Access a received sample's info field as JSON.
    fn get_info_json(&self, index: usize, field_name: &str) -> ConnectorResult<String> {
        self.parent
            .native_ref()?
            .get_json_from_infos(&self.name, index, field_name)
    }

    /// Access a received sample as JSON string.
    fn get_json(&self, index: usize) -> ConnectorResult<String> {
        self.parent.native_ref()?.get_json_sample(&self.name, index)
    }

    /// Check whether a received sample contains valid data.
    fn is_valid(&self, index: usize) -> ConnectorResult<bool> {
        self.parent
            .native_ref()?
            .get_boolean_from_infos(&self.name, index, "valid_data")
    }

    /// Display the list of publications currently matched.
    pub fn display_matched_publications(&self) -> ConnectorResult<String> {
        self.parent
            .native_ref()?
            .get_input(&self.name)?
            .get_matched_publications()
    }
}
