/*******************************************************************************
 * (c) 2025 Copyright, Real-Time Innovations.  All rights reserved.            *
 * No duplications, whole or partial, manual or electronic, may be made        *
 * without express written permission.  Any such copies, or revisions thereof, *
 * must display this notice unaltered.                                         *
 * This code contains trade secrets of Real-Time Innovations, Inc.             *
 *******************************************************************************/

//! Everything related to the `rtiddsconnector` C API, including types, functions and helpers.

use std::{ffi, ptr::NonNull};

// C representation of the ReturnCode enum.
pub type NativeReturnCode = ffi::c_int;

/// Rust representation of the ReturnCode enum.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReturnCode {
    Ok,
    Error,
    AlreadyDeleted,
    Timeout,
    NoData,
    IllegalOperation,
    Unknown(NativeReturnCode),
}

impl From<ReturnCode> for NativeReturnCode {
    fn from(kind: ReturnCode) -> Self {
        kind.as_native()
    }
}

impl From<NativeReturnCode> for ReturnCode {
    fn from(value: NativeReturnCode) -> Self {
        ReturnCode::from_native(value)
    }
}

impl std::fmt::Display for ReturnCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ReturnCode::Ok => write!(f, "OK"),
            ReturnCode::Error => write!(f, "Error"),
            ReturnCode::AlreadyDeleted => write!(f, "Already Deleted"),
            ReturnCode::Timeout => write!(f, "Timeout"),
            ReturnCode::NoData => write!(f, "No Data"),
            ReturnCode::IllegalOperation => write!(f, "Illegal Operation"),
            ReturnCode::Unknown(code) => write!(f, "Unknown error code: {}", code),
        }
    }
}

impl ReturnCode {
    fn as_native(&self) -> NativeReturnCode {
        match self {
            ReturnCode::Ok => 0,
            ReturnCode::Error => 1,
            ReturnCode::AlreadyDeleted => 9,
            ReturnCode::Timeout => 10,
            ReturnCode::NoData => 11,
            ReturnCode::IllegalOperation => 12,
            ReturnCode::Unknown(code) => *code,
        }
    }

    fn from_native(value: NativeReturnCode) -> Self {
        match value {
            0 => ReturnCode::Ok,
            1 => ReturnCode::Error,
            9 => ReturnCode::AlreadyDeleted,
            10 => ReturnCode::Timeout,
            11 => ReturnCode::NoData,
            12 => ReturnCode::IllegalOperation,
            2 | 3 | 4 | 5 | 6 | 7 | 8 | 1000 => ReturnCode::Error, // Map ignored codes to Error
            _ => ReturnCode::Unknown(value), // Map unrecognized codes to Unknown variant
        }
    }
}

#[repr(transparent)]
pub struct OpaqueConnector(ffi::c_void);

#[repr(transparent)]
pub struct OpaqueDataWriter(ffi::c_void);

#[repr(transparent)]
pub struct OpaqueDataReader(ffi::c_void);

#[repr(transparent)]
pub struct OpaqueSample(ffi::c_void);

pub trait NativeStringTrait {
    fn as_raw_ptr(&self) -> *const ffi::c_char;
    fn as_str(&self) -> Option<&str> {
        if self.as_raw_ptr().is_null() {
            None
        } else {
            unsafe { ffi::CStr::from_ptr(self.as_raw_ptr()) }
                .to_str()
                .ok()
        }
    }
}

#[repr(transparent)]
#[derive(Default)]
pub struct NativeAllocatedString(*mut ffi::c_char);

impl Drop for NativeAllocatedString {
    fn drop(&mut self) {
        if !self.0.is_null() {
            unsafe {
                RTI_Connector_free_string(self.0);
            }
        }
    }
}

impl NativeStringTrait for NativeAllocatedString {
    fn as_raw_ptr(&self) -> *const ffi::c_char {
        self.0 as *const ffi::c_char
    }
}

#[repr(transparent)]
#[derive(Default)]
pub struct NativeStaticString(*const ffi::c_char);

impl NativeStringTrait for NativeStaticString {
    fn as_raw_ptr(&self) -> *const ffi::c_char {
        self.0
    }
}

#[repr(transparent)]
pub struct ConnectorIndex(pub(crate) ffi::c_int);

#[repr(C)]
pub struct ConnectorOptions {
    enable_on_data_event: ffi::c_int,        // boolean
    one_based_sequence_indexing: ffi::c_int, // boolean
}

impl Default for ConnectorOptions {
    fn default() -> Self {
        Self {
            enable_on_data_event: 1,
            one_based_sequence_indexing: 0,
        }
    }
}

/// C representation of the AnyValueKind enum.
pub type NativeAnyValue = ffi::c_int;

impl From<AnyValue> for NativeAnyValue {
    fn from(kind: AnyValue) -> Self {
        kind.as_native()
    }
}

/// Rust representation of the AnyValueKind enum.
pub enum AnyValue {
    None,
    Number,
    Boolean,
    String,
    Unknown(NativeAnyValue),
}

impl From<NativeAnyValue> for AnyValue {
    fn from(value: NativeAnyValue) -> Self {
        AnyValue::from_native(value)
    }
}

impl AnyValue {
    fn as_native(&self) -> NativeAnyValue {
        match self {
            AnyValue::None => 0,
            AnyValue::Number => 1,
            AnyValue::Boolean => 2,
            AnyValue::String => 3,
            AnyValue::Unknown(code) => *code,
        }
    }

    fn from_native(value: NativeAnyValue) -> Self {
        match value {
            0 => AnyValue::None,
            1 => AnyValue::Number,
            2 => AnyValue::Boolean,
            3 => AnyValue::String,
            other => AnyValue::Unknown(other),
        }
    }
}

#[link(name = "rtiddsconnector")]
unsafe extern "C" {
    pub unsafe fn RTI_Connector_new(
        config_name: *const std::ffi::c_char,
        config_file: *const std::ffi::c_char,
        options: *const ConnectorOptions,
    ) -> *mut OpaqueConnector;

    pub unsafe fn RTI_Connector_delete(connector: *mut OpaqueConnector);

    pub unsafe fn RTI_Connector_get_datawriter(
        connector: NonNull<OpaqueConnector>,
        entity_name: *const std::ffi::c_char,
    ) -> *mut OpaqueDataWriter;

    pub unsafe fn RTI_Connector_get_datareader(
        connector: NonNull<OpaqueConnector>,
        entity_name: *const std::ffi::c_char,
    ) -> *mut OpaqueDataReader;

    pub unsafe fn RTI_Connector_get_native_sample(
        connector: NonNull<OpaqueConnector>,
        entity_name: *const std::ffi::c_char,
        index: ConnectorIndex,
    ) -> *mut OpaqueSample;

    pub unsafe fn RTI_Connector_set_number_into_samples(
        connector: NonNull<OpaqueConnector>,
        entity_name: *const std::ffi::c_char,
        name: *const std::ffi::c_char,
        value: ffi::c_double,
    ) -> NativeReturnCode;

    pub unsafe fn RTI_Connector_set_boolean_into_samples(
        connector: NonNull<OpaqueConnector>,
        entity_name: *const std::ffi::c_char,
        name: *const std::ffi::c_char,
        value: ffi::c_int, // boolean
    ) -> NativeReturnCode;

    pub unsafe fn RTI_Connector_set_string_into_samples(
        connector: NonNull<OpaqueConnector>,
        entity_name: *const std::ffi::c_char,
        name: *const std::ffi::c_char,
        value: *const std::ffi::c_char,
    ) -> NativeReturnCode;

    pub unsafe fn RTI_Connector_clear_member(
        connector: NonNull<OpaqueConnector>,
        entity_name: *const std::ffi::c_char,
        name: *const std::ffi::c_char,
    ) -> NativeReturnCode;

    pub unsafe fn RTI_Connector_write(
        connector: NonNull<OpaqueConnector>,
        entity_name: *const std::ffi::c_char,
        params_json: *const std::ffi::c_char,
    ) -> NativeReturnCode;

    pub unsafe fn RTI_Connector_wait_for_matched_subscription(
        writer: NonNull<OpaqueDataWriter>,
        timeout: ffi::c_int,
        current_count_change: *mut ffi::c_int,
    ) -> NativeReturnCode;

    pub unsafe fn RTI_Connector_get_matched_subscriptions(
        writer: NonNull<OpaqueDataWriter>,
        json_str: *mut NativeAllocatedString,
    ) -> NativeReturnCode;

    pub unsafe fn RTI_Connector_wait_for_acknowledgments(
        writer: NonNull<OpaqueDataWriter>,
        timeout: ffi::c_int,
    ) -> NativeReturnCode;

    pub unsafe fn RTI_Connector_read(
        connector: NonNull<OpaqueConnector>,
        entity_name: *const std::ffi::c_char,
    ) -> NativeReturnCode;

    pub unsafe fn RTI_Connector_take(
        connector: NonNull<OpaqueConnector>,
        entity_name: *const std::ffi::c_char,
    ) -> NativeReturnCode;

    pub unsafe fn RTI_Connector_return_loan(
        connector: NonNull<OpaqueConnector>,
        entity_name: *const std::ffi::c_char,
    ) -> NativeReturnCode;

    pub unsafe fn RTI_Connector_wait_for_data(
        connector: NonNull<OpaqueConnector>,
        timeout: ffi::c_int,
    ) -> NativeReturnCode;

    pub unsafe fn RTI_Connector_wait_for_data_on_reader(
        reader: NonNull<OpaqueDataReader>,
        timeout: ffi::c_int,
    ) -> NativeReturnCode;

    pub unsafe fn RTI_Connector_wait_for_matched_publication(
        reader: NonNull<OpaqueDataReader>,
        timeout: ffi::c_int,
        current_count_change: *mut ffi::c_int,
    ) -> NativeReturnCode;

    pub unsafe fn RTI_Connector_get_matched_publications(
        reader: NonNull<OpaqueDataReader>,
        json_str: *mut NativeAllocatedString,
    ) -> NativeReturnCode;

    pub unsafe fn RTI_Connector_clear(
        connector: NonNull<OpaqueConnector>,
        entity_name: *const std::ffi::c_char,
    ) -> NativeReturnCode;

    pub unsafe fn RTI_Connector_get_boolean_from_infos(
        connector: NonNull<OpaqueConnector>,
        return_value: *mut ffi::c_int, // boolean
        entity_name: *const std::ffi::c_char,
        index: ConnectorIndex,
        name: *const std::ffi::c_char,
    ) -> NativeReturnCode;

    pub unsafe fn RTI_Connector_get_json_from_infos(
        connector: NonNull<OpaqueConnector>,
        entity_name: *const std::ffi::c_char,
        index: ConnectorIndex,
        name: *const std::ffi::c_char,
        value: *mut NativeAllocatedString,
    ) -> NativeReturnCode;

    pub unsafe fn RTI_Connector_get_sample_count(
        connector: NonNull<OpaqueConnector>,
        entity_name: *const std::ffi::c_char,
        out_value: *mut ffi::c_double,
    ) -> NativeReturnCode;

    pub unsafe fn RTI_Connector_get_number_from_sample(
        connector: NonNull<OpaqueConnector>,
        return_value: *mut ffi::c_double,
        entity_name: *const std::ffi::c_char,
        index: ConnectorIndex,
        name: *const std::ffi::c_char,
    ) -> NativeReturnCode;

    pub unsafe fn RTI_Connector_get_boolean_from_sample(
        connector: NonNull<OpaqueConnector>,
        return_value: *mut ffi::c_int, // boolean
        entity_name: *const std::ffi::c_char,
        index: ConnectorIndex,
        name: *const std::ffi::c_char,
    ) -> NativeReturnCode;

    pub unsafe fn RTI_Connector_get_string_from_sample(
        connector: NonNull<OpaqueConnector>,
        return_value: *mut NativeAllocatedString,
        entity_name: *const std::ffi::c_char,
        index: ConnectorIndex,
        name: *const std::ffi::c_char,
    ) -> NativeReturnCode;

    pub unsafe fn RTI_Connector_get_any_from_sample(
        connector: NonNull<OpaqueConnector>,
        double_value_out: *mut ffi::c_double,
        bool_value_out: *mut ffi::c_int, // boolean
        string_value_out: *mut NativeAllocatedString,
        selected_out: *mut NativeAnyValue,
        entity_name: *const std::ffi::c_char,
        index: ConnectorIndex,
        name: *const std::ffi::c_char,
    ) -> NativeReturnCode;

    pub unsafe fn RTI_Connector_get_any_from_info(
        connector: NonNull<OpaqueConnector>,
        double_value_out: *mut ffi::c_double,
        bool_value_out: *mut ffi::c_int, // boolean
        string_value_out: *mut NativeAllocatedString,
        selected_out: *mut NativeAnyValue,
        entity_name: *const std::ffi::c_char,
        index: ConnectorIndex,
        name: *const std::ffi::c_char,
    ) -> NativeReturnCode;

    pub unsafe fn RTI_Connector_get_json_sample(
        connector: NonNull<OpaqueConnector>,
        entity_name: *const std::ffi::c_char,
        index: ConnectorIndex,
        json_str: *mut NativeAllocatedString,
    ) -> NativeReturnCode;

    pub unsafe fn RTI_Connector_get_json_member(
        connector: NonNull<OpaqueConnector>,
        entity_name: *const std::ffi::c_char,
        index: ConnectorIndex,
        member_name: *const std::ffi::c_char,
        json_str: *mut NativeAllocatedString,
    ) -> NativeReturnCode;

    pub unsafe fn RTI_Connector_set_json_instance(
        connector: NonNull<OpaqueConnector>,
        entity_name: *const std::ffi::c_char,
        json: *const std::ffi::c_char,
    ) -> NativeReturnCode;

    pub unsafe fn RTI_Connector_get_last_error_message() -> NativeAllocatedString;

    #[allow(unused)]
    pub unsafe fn RTI_Connector_get_native_instance(
        connector: NonNull<OpaqueConnector>,
        entity_name: *const std::ffi::c_char,
        native_pointer: *mut *const OpaqueSample,
    ) -> NativeReturnCode;

    pub unsafe fn RTIDDSConnector_getJSONInstance(
        connector: NonNull<OpaqueConnector>,
        entity_name: *const std::ffi::c_char,
    ) -> NativeAllocatedString;

    pub unsafe fn RTI_Connector_free_string(s: *mut ffi::c_char);

    pub unsafe fn RTI_Connector_get_build_versions(
        client_version: *const NativeStaticString,
        connector_version: *const NativeStaticString,
    ) -> NativeReturnCode;
}

#[link(name = "nddsc")]
unsafe extern "C" {
    pub unsafe fn DDS_DomainParticipantFactory_finalize_instance() -> NativeReturnCode;
}
