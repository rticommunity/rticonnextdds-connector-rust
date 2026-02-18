/*******************************************************************************
 * (c) 2025 Copyright, Real-Time Innovations.  All rights reserved.            *
 * No duplications, whole or partial, manual or electronic, may be made        *
 * without express written permission.  Any such copies, or revisions thereof, *
 * must display this notice unaltered.                                         *
 * This code contains trade secrets of Real-Time Innovations, Inc.             *
 *******************************************************************************/

#![doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/docs/connector.md"))]

use crate::{
    ConnectorFallible, ConnectorResult, Input, Output, ffi::FfiConnector,
    result::ErrorKind,
};
use std::{
    collections::HashMap,
    sync::{Condvar, Mutex, RwLock},
};

/// A variant type that can hold a [number][selected_number],
/// a [boolean][selected_boolean], or a [string][selected_string] value.
///
/// This type is used for both [setting][set_value] and [retrieving][get_value]
/// values from DDS samples in a type-safe manner, respectively with
/// [`Instance::set_value`][set_value] and [`Sample::get_value`][get_value].
///
/// Note that complex types (such as nested structures) are
/// internally represented as JSON strings, and should be set and retrieved
/// using [`SelectedValue::String`].
///
/// # Examples
///
/// ```rust
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/snippets/connector/using_selected_value.rs"))]
/// ```
///
/// [selected_number]: SelectedValue::Number
/// [selected_boolean]: SelectedValue::Boolean
/// [selected_string]: SelectedValue::String
/// [set_value]: crate::Instance::set_value
/// [get_value]: crate::Sample::get_value
#[derive(Debug, Clone, PartialEq)]
pub enum SelectedValue {
    /// A numeric value
    Number(f64),

    /// A boolean value
    Boolean(bool),

    /// A string value
    String(String),
}

/// Allows quick conversion from [f64] to [SelectedValue::Number].
impl From<f64> for SelectedValue {
    fn from(v: f64) -> Self {
        SelectedValue::Number(v)
    }
}

/// Allows quick conversion from [bool] to [SelectedValue::Boolean].
impl From<bool> for SelectedValue {
    fn from(v: bool) -> Self {
        SelectedValue::Boolean(v)
    }
}

/// Allows quick conversion from [String] to [SelectedValue::String].
impl From<String> for SelectedValue {
    fn from(v: String) -> Self {
        SelectedValue::String(v)
    }
}

/// Allows quick conversion from [str] to [SelectedValue::String].
impl From<&str> for SelectedValue {
    fn from(v: &str) -> Self {
        v.to_string().into()
    }
}

/// The main interface to the RTI Connector for Rust API.
///
/// Representing a DDS `DomainParticipant` and its contained
/// `DataReader`s and `DataWriter`s, a `Connector` object is
/// used to create [`Input`] and [`Output`] objects for reading
/// and writing DDS data, respectively.
///
/// [`Connector::get_input`] and [`Connector::get_output`] are the main
/// methods of this struct, allowing to acquire owned references to
/// [`Input`] and [`Output`] objects for reading and writing DDS data.
///
/// # Examples
/// ```rust
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/snippets/connector/using_connector.rs"))]
/// ```
pub struct Connector {
    /// The name of the configuration used to create this Connector.
    name: String,

    /// The native connector instance, protected by a RwLock for thread-safe access.
    native: RwLock<FfiConnector>,

    /// Thread-safe holders for Input entities.
    inputs: ThreadSafeEntityHolder<InputRecord>,

    /// Thread-safe holders for Output entities.
    outputs: ThreadSafeEntityHolder<OutputRecord>,
}

/// Unsafe marker traits for Connector; disables sharing between threads.
#[allow(unsafe_code)]
unsafe impl Sync for Connector {
    /* Marker trait */
}

/// Unsafe marker traits for Connector; disables sharing between threads.
#[allow(unsafe_code)]
unsafe impl Send for Connector {
    /* Marker trait */
}

/// Display implementation for Connector; displaying only the name.
impl std::fmt::Debug for Connector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, r#"Connector {{ name: "{}" }}"#, self.name)
    }
}

impl Connector {
    /// Retrieve a string describing the version of the RTI Connector for Rust
    /// and the underlying [RTI Connext] installation.
    ///
    /// [RTI Connext]: https://www.rti.com/products/dds "RTI Connext Professional"
    pub fn get_versions_string() -> String {
        static VERSION_STRING: &str = env!("CARGO_PKG_VERSION");

        let (ndds_build_id_string, rtiddsconnector_build_id_string) =
            FfiConnector::get_build_versions().unwrap_or((
                "<Unknown RTI Connext version>".to_string(),
                "<Unknown RTI Connector for Rust version>".to_string(),
            ));

        format!(
            "RTI Connector for Rust, version {}\n{}\n{}",
            VERSION_STRING, ndds_build_id_string, rtiddsconnector_build_id_string
        )
    }

    /// Get the last error message from the underlying RTI Connector C API.
    pub(crate) fn get_last_error_message() -> Option<String> {
        FfiConnector::get_last_error_message()
    }

    /// Create a new [`Connector`] from a named configuration contained
    /// in an external XML file.
    pub fn new(config_name: &str, config_file: &str) -> ConnectorResult<Connector> {
        static NATIVE_CONNECTOR_CREATION_LOCK: Mutex<()> = Mutex::new(());

        let native: FfiConnector = {
            let _guard = NATIVE_CONNECTOR_CREATION_LOCK
                .lock()
                .inspect_err(|_| {
                    eprintln!("An error occurred while trying to lock the global native connector creation lock, continuing anyway...");
                })
                .unwrap_or_else(|poisoned| poisoned.into_inner());
            FfiConnector::new(config_name, config_file)?
        };

        Ok(Connector {
            name: config_name.to_string(),
            native: RwLock::new(native),
            inputs: ThreadSafeEntityHolder::new(),
            outputs: ThreadSafeEntityHolder::new(),
        })
    }

    /// Wait until data is available to read from any of its [`Input`], indefinitely.
    pub fn wait_for_data(&self) -> ConnectorFallible {
        self.impl_wait_for_data(None)
    }

    /// Wait until data is available to read from any of its [`Input`], with a timeout.
    pub fn wait_for_data_with_timeout(
        &self,
        timeout: std::time::Duration,
    ) -> ConnectorFallible {
        self.impl_wait_for_data(Some(
            // Durations cannot be negative
            timeout.as_millis().try_into().unwrap_or(i32::MAX),
        ))
    }

    /// Implementation of wait for data functionality.
    fn impl_wait_for_data(&self, timeout: Option<i32>) -> ConnectorFallible {
        self.native_ref()?.wait_for_data(timeout)
    }

    /// Get an [`Input`] instance contained in this [`Connector`].
    ///
    /// This is a thread-aware operation that enforces single-threaded ownership
    /// of the underlying [`Input`]'s resources, the `DataReader`.
    /// Thread-aware ownership of the [`Input`] is implemented by means of
    /// [`Drop::drop`] on the returned [`Input`].
    ///
    /// An error will be returned if another thread already owns the named [`Input`],
    /// or if named [`Input`] is not contained in the Connector.
    pub fn get_input(&self, name: &str) -> ConnectorResult<Input<'_>> {
        self.inputs
            .acquire_entity(name, &self, BlockingBehavior::NonBlocking)
    }

    /// Get an [`Input`] instance contained in this [`Connector`], potentially
    /// blocking until it becomes available.
    ///
    /// This is a thread-aware operation that enforces single-threaded ownership,
    /// and the blocking counterpart of [`Connector::get_input`].
    pub fn take_input(&self, name: &str) -> ConnectorResult<Input<'_>> {
        self.inputs
            .acquire_entity(name, &self, BlockingBehavior::BlockForever)
    }

    /// Mark an [`Input`] as released, making it available to other threads.
    pub(crate) fn release_input(&self, name: &str) -> ConnectorFallible {
        self.inputs.release_entity(name)
    }

    /// Get an [`Output`] instance contained in this [`Connector`].
    ///
    /// This is a thread-aware operation that enforces single-threaded ownership
    /// of the underlying [`Output`]'s resources, the `DataWriter`.
    /// Thread-aware ownership of the [`Output`] is implemented by means of
    /// [`Drop::drop`] on the returned [`Output`].
    ///
    /// An error will be returned if another thread already owns the named [`Output`],
    /// or if named [`Output`] is not contained in the Connector.
    pub fn get_output(&self, name: &str) -> ConnectorResult<Output<'_>> {
        self.outputs
            .acquire_entity(name, &self, BlockingBehavior::NonBlocking)
    }

    /// Get an [`Output`] instance contained in this [`Connector`], potentially
    /// blocking until it becomes available.
    ///
    /// This is a thread-aware operation that enforces single-threaded ownership,
    /// and the blocking counterpart of [`Connector::get_output`].
    pub fn take_output(&self, name: &str) -> ConnectorResult<Output<'_>> {
        self.outputs
            .acquire_entity(name, &self, BlockingBehavior::BlockForever)
    }

    /// Mark an [`Output`] as released, making it available to other threads.
    pub(crate) fn release_output(&self, name: &str) -> ConnectorFallible {
        self.outputs.release_entity(name)
    }

    /// Get immutable access to the [`FfiConnector`] (for read operations)
    pub(crate) fn native_ref(
        &self,
    ) -> ConnectorResult<std::sync::RwLockReadGuard<'_, FfiConnector>> {
        self.native.read().map_err(|_| {
            ErrorKind::lock_poisoned_error(
                "Another thread panicked while holding the native connector lock",
            )
            .into()
        })
    }

    /// Get mutable access to the [`FfiConnector`] (for write operations)
    pub(crate) fn native_mut(
        &self,
    ) -> ConnectorResult<std::sync::RwLockWriteGuard<'_, FfiConnector>> {
        self.native.write().map_err(|_| {
            ErrorKind::lock_poisoned_error(
                "Another thread panicked while holding the native connector lock",
            )
            .into()
        })
    }
}

// Trait specializations for Input entities
impl<'a> EntityHandler<Input<'a>, InputRecord> for &'a Connector {
    fn validate_name(&self, name: &str) -> ConnectorFallible {
        self.native_ref()?.get_input(name).map(drop)
    }

    fn create_entity(&self, name: &str) -> Input<'a> {
        Input::new(name, self)
    }

    fn create_record() -> InputRecord {
        InputRecord
    }
}

// Trait specializations for Output entities
impl<'a> EntityHandler<Output<'a>, OutputRecord> for &'a Connector {
    fn validate_name(&self, name: &str) -> ConnectorFallible {
        self.native_ref()?.get_output(name).map(drop)
    }

    fn create_entity(&self, name: &str) -> Output<'a> {
        Output::new(name, self)
    }

    fn create_record() -> OutputRecord {
        OutputRecord
    }
}

/// Marker struct for Input ownership records
#[derive(Debug)]
struct InputRecord;

/// Unsafe marker traits for InputRecord; disables sharing between threads.
#[allow(unsafe_code)]
unsafe impl Sync for InputRecord {}

/// Unsafe marker traits for InputRecord; disables sharing between threads.
#[allow(unsafe_code)]
unsafe impl Send for InputRecord {}

/// Marker struct for Output ownership records
#[derive(Debug)]
struct OutputRecord;

/// Unsafe marker traits for OutputRecord; disables sharing between threads.
#[allow(unsafe_code)]
unsafe impl Sync for OutputRecord {}

/// Unsafe marker traits for OutputRecord; disables sharing between threads.
#[allow(unsafe_code)]
unsafe impl Send for OutputRecord {}

/// Trait for handling entity operations (validation, creation, and record management)
trait EntityHandler<T, R> {
    /// Validate that the given name corresponds to a valid entity
    fn validate_name(&self, name: &str) -> ConnectorFallible;

    /// Create a new entity with the given name
    fn create_entity(&self, name: &str) -> T;

    /// Create a record for tracking entity ownership
    fn create_record() -> R;
}

/// Thread-safe holder for entities with blocking acquisition behavior
#[derive(Debug)]
struct ThreadSafeEntityHolder<R> {
    /// Map of entity names to their ownership records
    entities: Mutex<HashMap<String, R>>,

    /// Condition variable for managing blocking behavior
    queue: Condvar,
}

/// Blocking behavior configuration for entity acquisition
#[derive(Debug, Clone)]
enum BlockingBehavior {
    /// Return immediately if entity is not available
    NonBlocking,

    /// Block indefinitely until entity becomes available
    BlockForever,
}

impl<R> ThreadSafeEntityHolder<R> {
    /// Create a new ThreadSafeEntityHolder
    fn new() -> Self {
        ThreadSafeEntityHolder {
            entities: Mutex::new(HashMap::new()),
            queue: Condvar::new(),
        }
    }

    /// Helper function to create and register
    fn get_entity_from_guard<T, H>(
        &self,
        name: &str,
        entities: &mut HashMap<String, R>,
        handler: &H,
    ) -> ConnectorResult<T>
    where
        H: EntityHandler<T, R>,
    {
        if entities.contains_key(name) {
            ErrorKind::entity_busy_error(format!(
                "{} named '{}' already in use",
                std::any::type_name::<T>(),
                name,
            ))
            .into_err()
        } else {
            let entity = handler.create_entity(name);
            let record = H::create_record();
            entities.insert(name.to_string(), record);

            Ok(entity)
        }
    }

    /// Release an entity, making it available to other threads
    fn release_entity(&self, name: &str) -> ConnectorFallible {
        let mut entities = self.entities.lock().map_err(|_| {
            ErrorKind::lock_poisoned_error(
                "Another thread panicked while holding the entities lock",
            )
        })?;

        match entities.remove(name) {
            None => ErrorKind::entity_busy_error(format!(
                "{} named '{}' not found or already released",
                std::any::type_name::<R>(),
                name,
            ))
            .into_err(),
            Some(_) => {
                self.queue.notify_all();
                Ok(())
            }
        }
    }

    /// Retrieve an entity with configurable blocking behavior
    fn acquire_entity<T, H>(
        &self,
        name: &str,
        handler: &H,
        behavior: BlockingBehavior,
    ) -> ConnectorResult<T>
    where
        H: EntityHandler<T, R>,
    {
        let mut entities = self.entities.lock().map_err(|_| {
            ErrorKind::lock_poisoned_error(
                "Another thread panicked while holding the entities lock",
            )
        })?;

        // Validate the name first
        handler.validate_name(name)?;

        loop {
            // Try to acquire the entity
            if !entities.contains_key(name) {
                return self.get_entity_from_guard(name, &mut entities, handler);
            }

            // Entity is already taken, decide what to do based on blocking behavior
            match &behavior {
                BlockingBehavior::NonBlocking => {
                    return ErrorKind::entity_busy_error(format!(
                        "{} '{}' already in use",
                        std::any::type_name::<T>(),
                        name,
                    ))
                    .into_err();
                }

                BlockingBehavior::BlockForever => {
                    entities = self.queue.wait(entities).map_err(|_| {
                        ErrorKind::lock_poisoned_error(
                            "Another thread panicked while holding the entities lock",
                        )
                    })?;
                }
            }
        }
    }
}
