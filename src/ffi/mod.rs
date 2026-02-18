/*******************************************************************************
 * (c) 2025 Copyright, Real-Time Innovations.  All rights reserved.            *
 * No duplications, whole or partial, manual or electronic, may be made        *
 * without express written permission.  Any such copies, or revisions thereof, *
 * must display this notice unaltered.                                         *
 * This code contains trade secrets of Real-Time Innovations, Inc.             *
 *******************************************************************************/

// TODO: Consider extracting to local, separate crate

#![doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/docs/ffi.md"))]
// Allow unsafe code in this module since it wraps FFI calls
#![allow(unsafe_code)]

mod rtiddsconnector;

pub const INFINITE_TIMEOUT_IN_MS: i32 = -1;

pub use rtiddsconnector::ReturnCode;

use crate::result::ErrorKind;
use rtiddsconnector::{ConnectorIndex, NativeAllocatedString, NativeStringTrait};
use std::{ffi::CString, ptr::NonNull};

/// Helper for converting a [`std::ffi::NulError`] into a [`ConnectorError`][crate::ConnectorError]
impl From<std::ffi::NulError> for crate::ConnectorError {
    fn from(_: std::ffi::NulError) -> Self {
        ErrorKind::invalid_string_conversion_error().into()
    }
}

/// A guard that finalizes [RTI Connext] globals when dropped.
///
/// When an instance of this struct goes out of scope, it will call the
/// [`Drop::drop`] implementation which finalizes the Connext globals.
///
/// This is useful when you want to assert that all Connext-allocated resources
/// are properly released at the end of a test or a specific scope.
///
/// These globals can only be deleted if there are no outstanding
/// [`Connector`][crate::Connector] instances. On failure, it will log an error.
///
/// ```rust
#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/snippets/connector/using_globals_drop_guard.rs"))]
/// ```
///
/// [RTI Connext]: https://www.rti.com/products/dds "RTI Connext Professional"
#[derive(Debug)]
pub struct GlobalsDropGuard;

impl Drop for GlobalsDropGuard {
    fn drop(&mut self) {
        if let Err(e) = Self::finalize_connext_globals() {
            eprintln!("ERROR: failed to finalize Connext globals: {}", e);
        }
    }
}

impl GlobalsDropGuard {
    fn finalize_connext_globals() -> crate::ConnectorFallible {
        InvokeResult::no_output(|| unsafe {
            rtiddsconnector::DDS_DomainParticipantFactory_finalize_instance()
        })
        .into_result()
    }
}

/// Newtype wrappers for native Sample pointers
#[allow(unused)]
pub struct FfiSample(NonNull<rtiddsconnector::OpaqueSample>);

/// Newtype wrappers for native DataReader pointers
pub struct FfiInput(NonNull<rtiddsconnector::OpaqueDataReader>);

impl FfiInput {
    pub fn wait_for_matched_publication(
        &self,
        timeout: Option<i32>,
    ) -> crate::ConnectorResult<i32> {
        InvokeResult::with_output(|current_count_change: &mut i32| unsafe {
            rtiddsconnector::RTI_Connector_wait_for_matched_publication(
                self.0,
                timeout.unwrap_or(INFINITE_TIMEOUT_IN_MS),
                current_count_change,
            )
        })
        .into_result()
    }

    pub fn wait_for_data(&self, timeout: Option<i32>) -> crate::ConnectorFallible {
        InvokeResult::no_output(|| unsafe {
            rtiddsconnector::RTI_Connector_wait_for_data_on_reader(
                self.0,
                timeout.unwrap_or(INFINITE_TIMEOUT_IN_MS),
            )
        })
        .into()
    }

    pub fn get_matched_publications(&self) -> crate::ConnectorResult<String> {
        InvokeResult::with_output(|out_value: &mut NativeAllocatedString| unsafe {
            rtiddsconnector::RTI_Connector_get_matched_publications(self.0, out_value)
        })
        .into_string()
    }
}

/// Newtype wrappers for native DataWriter pointers
pub struct FfiOutput(NonNull<rtiddsconnector::OpaqueDataWriter>);

impl FfiOutput {
    pub fn wait_for_matched_subscription(
        &self,
        timeout: Option<i32>,
    ) -> crate::ConnectorResult<i32> {
        InvokeResult::with_output(|current_count_change: &mut i32| unsafe {
            rtiddsconnector::RTI_Connector_wait_for_matched_subscription(
                self.0,
                timeout.unwrap_or(INFINITE_TIMEOUT_IN_MS),
                current_count_change,
            )
        })
        .into_result()
    }

    pub fn wait_for_acknowledgments(
        &self,
        timeout: Option<i32>,
    ) -> crate::ConnectorFallible {
        InvokeResult::no_output(|| unsafe {
            rtiddsconnector::RTI_Connector_wait_for_acknowledgments(
                self.0,
                timeout.unwrap_or(INFINITE_TIMEOUT_IN_MS),
            )
        })
        .into()
    }

    pub fn get_matched_subscriptions(&self) -> crate::ConnectorResult<String> {
        InvokeResult::with_output(|out_value: &mut NativeAllocatedString| unsafe {
            rtiddsconnector::RTI_Connector_get_matched_subscriptions(self.0, out_value)
        })
        .into_string()
    }
}

/// Newtype wrappers for native Connector pointers
pub struct FfiConnector(NonNull<rtiddsconnector::OpaqueConnector>);

impl Drop for FfiConnector {
    fn drop(&mut self) {
        if let Err(e) = self.delete() {
            eprintln!("ERROR: failed to delete native participant: {}", e);
        }
    }
}

impl FfiConnector {
    pub fn new(
        connector_name: &str,
        config_file: &str,
    ) -> crate::ConnectorResult<FfiConnector> {
        let config_name = CString::new(connector_name)?;
        let config_file = CString::new(config_file)?;

        NonNull::new(unsafe {
            rtiddsconnector::RTI_Connector_new(
                config_name.as_ptr(),
                config_file.as_ptr(),
                &rtiddsconnector::ConnectorOptions::default(),
            )
        })
        .map(FfiConnector)
        .ok_or_else(|| ErrorKind::entity_not_found_error(connector_name).into())
    }

    pub fn get_output(&self, output_name: &str) -> crate::ConnectorResult<FfiOutput> {
        let entity_name = CString::new(output_name)?;

        NonNull::new(unsafe {
            rtiddsconnector::RTI_Connector_get_datawriter(self.0, entity_name.as_ptr())
        })
        .map(FfiOutput)
        .ok_or_else(|| ErrorKind::entity_not_found_error(output_name).into())
    }

    pub fn get_input(&self, input_name: &str) -> crate::ConnectorResult<FfiInput> {
        let entity_name = CString::new(input_name)?;

        NonNull::new(unsafe {
            rtiddsconnector::RTI_Connector_get_datareader(self.0, entity_name.as_ptr())
        })
        .map(FfiInput)
        .ok_or_else(|| ErrorKind::entity_not_found_error(input_name).into())
    }

    #[allow(unused)]
    pub fn get_native_sample(
        &self,
        output_name: &str,
        index: usize,
    ) -> crate::ConnectorResult<FfiSample> {
        let entity_name: CString = CString::new(output_name)?;
        let index: ConnectorIndex = index.try_into()?;

        NonNull::new(unsafe {
            rtiddsconnector::RTI_Connector_get_native_sample(
                self.0,
                entity_name.as_ptr(),
                index,
            )
        })
        .map(FfiSample)
        .ok_or_else(|| ErrorKind::entity_not_found_error(output_name).into())
    }

    fn delete(&mut self) -> crate::ConnectorFallible {
        InvokeResult::never_fails(|| unsafe {
            rtiddsconnector::RTI_Connector_delete(self.0.as_ptr())
        })
        .into()
    }

    pub fn set_number_into_samples(
        &self,
        entity_name: &str,
        field_name: &str,
        value: f64,
    ) -> crate::ConnectorFallible {
        let entity_name = CString::new(entity_name)?;
        let field_name = CString::new(field_name)?;

        InvokeResult::no_output(|| unsafe {
            rtiddsconnector::RTI_Connector_set_number_into_samples(
                self.0,
                entity_name.as_ptr(),
                field_name.as_ptr(),
                value,
            )
        })
        .into()
    }

    pub fn set_boolean_into_samples(
        &self,
        entity_name: &str,
        name: &str,
        value: bool,
    ) -> crate::ConnectorFallible {
        let entity_name = CString::new(entity_name)?;
        let name = CString::new(name)?;

        InvokeResult::no_output(|| unsafe {
            rtiddsconnector::RTI_Connector_set_boolean_into_samples(
                self.0,
                entity_name.as_ptr(),
                name.as_ptr(),
                value as i32,
            )
        })
        .into()
    }

    pub fn set_string_into_samples(
        &self,
        entity_name: &str,
        name: &str,
        value: &str,
    ) -> crate::ConnectorFallible {
        let entity_name = CString::new(entity_name)?;
        let name = CString::new(name)?;
        let c_value = CString::new(value)?;

        InvokeResult::no_output(|| unsafe {
            rtiddsconnector::RTI_Connector_set_string_into_samples(
                self.0,
                entity_name.as_ptr(),
                name.as_ptr(),
                c_value.as_ptr(),
            )
        })
        .into()
    }

    pub fn set_into_samples(
        &self,
        entity_name: &str,
        name: &str,
        value: crate::SelectedValue,
    ) -> crate::ConnectorFallible {
        match value {
            crate::SelectedValue::Number(v) => {
                self.set_number_into_samples(entity_name, name, v)
            }
            crate::SelectedValue::Boolean(v) => {
                self.set_boolean_into_samples(entity_name, name, v)
            }
            crate::SelectedValue::String(v) => {
                self.set_string_into_samples(entity_name, name, &v)
            }
        }
    }

    pub fn clear_member(
        &self,
        entity_name: &str,
        name: &str,
    ) -> crate::ConnectorFallible {
        let entity_name = CString::new(entity_name)?;
        let name = CString::new(name)?;

        InvokeResult::no_output(|| unsafe {
            rtiddsconnector::RTI_Connector_clear_member(
                self.0,
                entity_name.as_ptr(),
                name.as_ptr(),
            )
        })
        .into()
    }

    pub fn write(&self, entity_name: &str) -> crate::ConnectorFallible {
        self.write_impl(entity_name, None)
    }

    #[allow(unused)]
    pub fn write_with_params(
        &self,
        entity_name: &str,
        params_json: &str,
    ) -> crate::ConnectorFallible {
        self.write_impl(entity_name, Some(params_json))
    }

    fn write_impl(
        &self,
        entity_name: &str,
        params_json: Option<&str>,
    ) -> crate::ConnectorFallible {
        let entity_name = CString::new(entity_name)?;
        let params_json = match params_json.map(CString::new) {
            Some(r) => Some(r?),
            None => None,
        };

        InvokeResult::no_output(|| unsafe {
            rtiddsconnector::RTI_Connector_write(
                self.0,
                entity_name.as_ptr(),
                match params_json {
                    Some(ref s) => s.as_ptr(),
                    None => std::ptr::null(),
                },
            )
        })
        .into()
    }

    pub fn read(&self, entity_name: &str) -> crate::ConnectorFallible {
        let entity_name = CString::new(entity_name)?;

        InvokeResult::no_output(|| unsafe {
            rtiddsconnector::RTI_Connector_read(self.0, entity_name.as_ptr())
        })
        .into()
    }

    pub fn take(&self, entity_name: &str) -> crate::ConnectorFallible {
        let entity_name = CString::new(entity_name)?;

        InvokeResult::no_output(|| unsafe {
            rtiddsconnector::RTI_Connector_take(self.0, entity_name.as_ptr())
        })
        .into()
    }

    pub fn return_loan(&self, entity_name: &str) -> crate::ConnectorFallible {
        let entity_name = CString::new(entity_name)?;

        InvokeResult::no_output(|| unsafe {
            rtiddsconnector::RTI_Connector_return_loan(self.0, entity_name.as_ptr())
        })
        .into()
    }

    pub fn wait_for_data(&self, timeout: Option<i32>) -> crate::ConnectorFallible {
        InvokeResult::no_output(|| unsafe {
            rtiddsconnector::RTI_Connector_wait_for_data(
                self.0,
                timeout.unwrap_or(INFINITE_TIMEOUT_IN_MS),
            )
        })
        .into()
    }

    pub fn clear(&self, entity_name: &str) -> crate::ConnectorFallible {
        let entity_name = CString::new(entity_name)?;

        InvokeResult::no_output(|| unsafe {
            rtiddsconnector::RTI_Connector_clear(self.0, entity_name.as_ptr())
        })
        .into()
    }

    pub fn get_boolean_from_infos(
        &self,
        entity_name: &str,
        index: usize,
        name: &str,
    ) -> crate::ConnectorResult<bool> {
        let entity_name = CString::new(entity_name)?;
        let name = CString::new(name)?;
        let index: ConnectorIndex = index.try_into()?;

        InvokeResult::with_output(|out_value: &mut bool| unsafe {
            rtiddsconnector::RTI_Connector_get_boolean_from_infos(
                self.0,
                out_value as *mut bool as *mut i32,
                entity_name.as_ptr(),
                index,
                name.as_ptr(),
            )
        })
        .into()
    }

    pub fn get_json_from_infos(
        &self,
        entity_name: &str,
        index: usize,
        name: &str,
    ) -> crate::ConnectorResult<String> {
        let entity_name = CString::new(entity_name)?;
        let index: ConnectorIndex = index.try_into()?;
        let name = CString::new(name)?;

        InvokeResult::with_output(|out_value: &mut NativeAllocatedString| unsafe {
            rtiddsconnector::RTI_Connector_get_json_from_infos(
                self.0,
                entity_name.as_ptr(),
                index,
                name.as_ptr(),
                out_value,
            )
        })
        .into_string()
    }

    pub fn get_sample_count(&self, entity_name: &str) -> crate::ConnectorResult<f64> {
        let entity_name = CString::new(entity_name)?;

        InvokeResult::with_output(|out_value: &mut f64| unsafe {
            rtiddsconnector::RTI_Connector_get_sample_count(
                self.0,
                entity_name.as_ptr(),
                out_value,
            )
        })
        .into()
    }

    pub fn get_number_from_sample(
        &self,
        entity_name: &str,
        index: usize,
        name: &str,
    ) -> crate::ConnectorResult<f64> {
        let entity_name = CString::new(entity_name)?;
        let name = CString::new(name)?;
        let index: ConnectorIndex = index.try_into()?;

        InvokeResult::with_output(|out_value: &mut f64| unsafe {
            rtiddsconnector::RTI_Connector_get_number_from_sample(
                self.0,
                out_value,
                entity_name.as_ptr(),
                index,
                name.as_ptr(),
            )
        })
        .into()
    }

    pub fn get_boolean_from_sample(
        &self,
        entity_name: &str,
        index: usize,
        name: &str,
    ) -> crate::ConnectorResult<bool> {
        let entity_name = CString::new(entity_name)?;
        let name = CString::new(name)?;
        let index: ConnectorIndex = index.try_into()?;

        InvokeResult::with_output(|out_value: &mut bool| unsafe {
            rtiddsconnector::RTI_Connector_get_boolean_from_sample(
                self.0,
                out_value as *mut bool as *mut i32,
                entity_name.as_ptr(),
                index,
                name.as_ptr(),
            )
        })
        .into()
    }

    pub fn get_string_from_sample(
        &self,
        entity_name: &str,
        index: usize,
        name: &str,
    ) -> crate::ConnectorResult<String> {
        let entity_name = CString::new(entity_name)?;
        let index: ConnectorIndex = index.try_into()?;
        let name = CString::new(name)?;

        InvokeResult::with_output(|out_value: &mut NativeAllocatedString| unsafe {
            rtiddsconnector::RTI_Connector_get_string_from_sample(
                self.0,
                out_value,
                entity_name.as_ptr(),
                index,
                name.as_ptr(),
            )
        })
        .into_string()
    }

    pub fn get_from_sample(
        &self,
        entity_name: &str,
        index: usize,
        name: &str,
    ) -> crate::ConnectorResult<crate::SelectedValue> {
        let entity_name = CString::new(entity_name)?;
        let index: ConnectorIndex = index.try_into()?;
        let name = CString::new(name)?;

        InvokeResult::with_output(|holder: &mut NativeAnyValueHolder| unsafe {
            rtiddsconnector::RTI_Connector_get_any_from_sample(
                self.0,
                &mut holder.double_value,
                &mut holder.bool_value,
                &mut holder.string_value,
                &mut holder.selected,
                entity_name.as_ptr(),
                index,
                name.as_ptr(),
            )
        })
        .into_selected_value()
    }

    pub fn get_from_info(
        &self,
        entity_name: &str,
        index: usize,
        name: &str,
    ) -> crate::ConnectorResult<crate::SelectedValue> {
        let entity_name = CString::new(entity_name)?;
        let index: ConnectorIndex = index.try_into()?;
        let name = CString::new(name)?;

        InvokeResult::with_output(|holder: &mut NativeAnyValueHolder| unsafe {
            rtiddsconnector::RTI_Connector_get_any_from_info(
                self.0,
                &mut holder.double_value,
                &mut holder.bool_value,
                &mut holder.string_value,
                &mut holder.selected,
                entity_name.as_ptr(),
                index,
                name.as_ptr(),
            )
        })
        .into_selected_value()
    }

    pub fn get_json_sample(
        &self,
        entity_name: &str,
        index: usize,
    ) -> crate::ConnectorResult<String> {
        let entity_name = CString::new(entity_name)?;
        let index: ConnectorIndex = index.try_into()?;

        InvokeResult::with_output(|out_value: &mut NativeAllocatedString| unsafe {
            rtiddsconnector::RTI_Connector_get_json_sample(
                self.0,
                entity_name.as_ptr(),
                index,
                out_value,
            )
        })
        .into_string()
    }

    pub fn get_json_member(
        &self,
        entity_name: &str,
        index: usize,
        member_name: &str,
    ) -> crate::ConnectorResult<String> {
        let entity_name = CString::new(entity_name)?;
        let index: ConnectorIndex = index.try_into()?;
        let member_name = CString::new(member_name)?;

        InvokeResult::with_output(|out_value: &mut NativeAllocatedString| unsafe {
            rtiddsconnector::RTI_Connector_get_json_member(
                self.0,
                entity_name.as_ptr(),
                index,
                member_name.as_ptr(),
                out_value,
            )
        })
        .into_string()
    }

    pub fn set_json_instance(
        &self,
        entity_name: &str,
        json: &str,
    ) -> crate::ConnectorFallible {
        let entity_name = CString::new(entity_name)?;
        let json = CString::new(json)?;

        InvokeResult::no_output(|| unsafe {
            rtiddsconnector::RTI_Connector_set_json_instance(
                self.0,
                entity_name.as_ptr(),
                json.as_ptr(),
            )
        })
        .into()
    }

    pub fn get_last_error_message() -> Option<String> {
        unsafe { rtiddsconnector::RTI_Connector_get_last_error_message() }
            .as_str()
            .filter(|&s| !s.is_empty())
            .map(str::to_string)
    }

    #[allow(unused)]
    pub fn get_native_instance(
        &self,
        entity_name: &str,
    ) -> crate::ConnectorResult<*const rtiddsconnector::OpaqueSample> {
        unimplemented!();
    }

    pub fn get_json_instance(&self, entity_name: &str) -> crate::ConnectorResult<String> {
        let entity_name = CString::new(entity_name)?;

        // We need to call a function that returns a pointer. Then, based on this pointer, we can move the value
        // into a String and free the output pointer, or fail if the pointer is null.
        InvokeResult::with_string_return(|| unsafe {
            rtiddsconnector::RTIDDSConnector_getJSONInstance(self.0, entity_name.as_ptr())
        })
        .into_string()
    }

    pub fn get_build_versions() -> crate::ConnectorResult<(String, String)> {
        let (client_version, connector_version) =
            InvokeResult::with_output(|(client_version, connector_version)| unsafe {
                rtiddsconnector::RTI_Connector_get_build_versions(
                    client_version,
                    connector_version,
                )
            })
            .into_result()?;

        Ok((
            client_version
                .as_str()
                .unwrap_or("<Unknown Client>")
                .to_string(),
            connector_version
                .as_str()
                .unwrap_or("<Unknown Connector>")
                .to_string(),
        ))
    }
}

#[derive(Default)]
pub struct NativeAnyValueHolder {
    pub selected: rtiddsconnector::NativeAnyValue,
    pub double_value: f64,
    pub bool_value: i32,
    pub string_value: rtiddsconnector::NativeAllocatedString,
}

impl InvokeResult<NativeAnyValueHolder> {
    pub fn into_selected_value(self) -> crate::ConnectorResult<crate::SelectedValue> {
        use rtiddsconnector::AnyValue;

        let holder = self.into_result()?;
        match AnyValue::from(holder.selected) {
            AnyValue::Number => Ok(crate::SelectedValue::Number(holder.double_value)),
            AnyValue::Boolean => {
                Ok(crate::SelectedValue::Boolean(holder.bool_value != 0))
            }
            AnyValue::String => match holder.string_value.as_str() {
                Some(s) => Ok(crate::SelectedValue::String(s.to_string())),
                None => ErrorKind::assertion_failed_error(
                    "Returned string value shouldn't be null",
                )
                .into_err(),
            },
            AnyValue::Unknown(code) => ErrorKind::assertion_failed_error(format!(
                "Unknown AnyValue kind: {}",
                code
            ))
            .into_err(),
            AnyValue::None => {
                ErrorKind::assertion_failed_error("Unavaiable AnyValue kind").into_err()
            }
        }
    }
}

impl TryFrom<usize> for rtiddsconnector::ConnectorIndex {
    type Error = crate::ConnectorError;

    fn try_from(value: usize) -> std::result::Result<Self, Self::Error> {
        if value > i32::MAX as usize {
            ErrorKind::invalid_argument_error("index exceeds maximum allowed value")
                .into_err()
        } else {
            Ok(Self(1 + value as i32))
        }
    }
}

impl InvokeResult<rtiddsconnector::NativeAllocatedString> {
    /// Helper to invoke an FFI function that returns a NativeAllocatedString output.
    pub fn with_string_return<F>(op: F) -> Self
    where
        F: FnOnce() -> rtiddsconnector::NativeAllocatedString,
    {
        InvokeResult(rtiddsconnector::ReturnCode::Ok, op())
    }

    /// Helper to convert a NativeAllocatedString result into a Rust String.
    pub fn into_string(self) -> crate::ConnectorResult<String> {
        self.into_result().and_then(|native| {
            native
                .as_str()
                .map(str::to_string)
                .ok_or_else(|| ErrorKind::invalid_string_conversion_error().into())
        })
    }
}

// TODO: Review if this can be turned into an Enum or into Result outright.
pub struct InvokeResult<T>(rtiddsconnector::ReturnCode, T);

impl<T> InvokeResult<T> {
    /// Helper to convert the InvokeResult into a ConnectorResult, mapping return codes to errors.
    pub fn into_result(self) -> crate::ConnectorResult<T> {
        match self.0 {
            rtiddsconnector::ReturnCode::Ok => Ok(self.1),
            rtiddsconnector::ReturnCode::Timeout => ErrorKind::timeout_error().into_err(),
            other => ErrorKind::native_error(other).into_err(),
        }
    }
}

impl<T: Default> InvokeResult<T> {
    /// Helper to invoke an FFI function that returns a return code and a single output value.
    pub fn with_output<F>(op: F) -> Self
    where
        F: FnOnce(&mut T) -> rtiddsconnector::NativeReturnCode,
    {
        let mut out_value: T = Default::default();
        InvokeResult(op(&mut out_value).into(), out_value)
    }
}

impl InvokeResult<()> {
    /// Helper to invoke an FFI function that returns only a return code.
    pub fn no_output<F>(op: F) -> Self
    where
        F: FnOnce() -> rtiddsconnector::NativeReturnCode,
    {
        InvokeResult(op().into(), ())
    }

    /// Helper to invoke an operation that cannot fail, returning a success return code.
    pub fn never_fails<F>(op: F) -> Self
    where
        F: FnOnce(),
    {
        op();
        InvokeResult(ReturnCode::Ok, ())
    }
}

impl From<()> for InvokeResult<()> {
    /// Helper to convert a `()` into an `InvokeResult<()>` with a success return code.
    fn from(_: ()) -> Self {
        InvokeResult(ReturnCode::Ok, ())
    }
}

impl<T> From<InvokeResult<T>> for crate::ConnectorResult<T> {
    fn from(ir: InvokeResult<T>) -> Self {
        ir.into_result()
    }
}
