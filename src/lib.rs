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
pub mod guide {
    #![doc = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/docs/guide/index.md"
    ))]
    #![doc(alias = "user guide")]

    #[doc(alias = "getting started")]
    pub mod getting_started {
        #![doc = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/docs/guide/getting_started.md"
        ))]
    }

    pub mod configuration {
        #![doc = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/docs/guide/configuration.md"
        ))]
    }

    pub mod connector {
        #![doc = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/docs/guide/connector.md"
        ))]
    }

    pub mod input {
        #![doc = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/docs/guide/input.md"
        ))]
    }

    pub mod output {
        #![doc = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/docs/guide/output.md"
        ))]
    }

    pub mod data {
        #![doc = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/docs/guide/data.md"
        ))]
    }

    pub mod errors {
        #![doc = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/docs/guide/errors.md"
        ))]
    }

    pub mod threading {
        #![doc = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/docs/guide/threading.md"
        ))]
    }

    pub mod advanced {
        #![doc = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/docs/guide/advanced.md"
        ))]
    }
}

