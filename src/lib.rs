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
pub mod guide;
mod input;
mod output;
mod result;
