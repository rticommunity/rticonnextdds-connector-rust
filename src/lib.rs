/******************************************************************************
* (c) 2005-2018 Copyright, Real-Time Innovations.  All rights reserved.       *
* No duplications, whole or partial, manual or electronic, may be made        *
* without express written permission.  Any such copies, or revisions thereof, *
* must display this notice unaltered.                                         *
* This code contains trade secrets of Real-Time Innovations, Inc.             *
******************************************************************************/

#![doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/docs/lib.md"))]
#![deny(
    missing_docs,
    unsafe_code,
    rustdoc::all,
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic
)]

pub use connector::{Connector, SelectedValue};
pub use ffi::GlobalsDropGuard;
pub use input::{Input, Sample, SampleIterator, ValidSampleIterator};
pub use output::{Instance, Output, WriteParams, WriteParamsAction, WriteParamsIdentity};
pub use result::{ConnectorError, ConnectorFallible, ConnectorResult};

mod connector;
mod ffi;
mod input;
mod output;
mod result;

#[cfg(doc)]
/// Guide module with in-depth documentation about various aspects of the crate.
pub mod guide {
    #![doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/docs/guide/index.md"))]

    /// A quick walkthrough to get started with the Connector.
    pub mod getting_started {
        #![doc = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/docs/guide/getting_started.md"
    ))]
    }
    /// Configuration options and details for the Connector.
    pub mod configuration {
        #![doc = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/docs/guide/configuration.md"
    ))]
    }
    /// Connector lifecycle: creating, using, and destroying a Connector.
    pub mod connector {
        #![doc = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/docs/guide/connector.md"
    ))]
    }
    /// Subscribing to data and reading samples from an Input.
    pub mod input {
        #![doc = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/docs/guide/input.md"
    ))]
    }
    /// Publishing data and writing samples to an Output.
    pub mod output {
        #![doc = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/docs/guide/output.md"
    ))]
    }
    /// Data access patterns and Serde integration.
    pub mod data {
        #![doc = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/docs/guide/data.md"
    ))]
    }
    /// Error handling and Connector errors.
    pub mod errors {
        #![doc = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/docs/guide/errors.md"
    ))]
    }
    /// Threading, ownership, and safety considerations.
    pub mod threading {
        #![doc = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/docs/guide/threading.md"
    ))]
    }
    /// Advanced operations and customization.
    pub mod advanced {
        #![doc = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/docs/guide/advanced.md"
    ))]
    }
}
