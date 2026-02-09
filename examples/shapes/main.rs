//! # RTI Connector for Rust example for Shape types
//!
//! This example demonstrates how to use the RTI Connector for Rust
//! to publish and subscribe to Shape data types in a DDS domain.
//!
//! ## Usage
//!
//! It uses a command-line interface to allow users to choose
//! between publishing and subscribing modes, as well as configure
//! parameters such as the number of samples and timeouts.
//!
//! ```console
#![doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/examples/shapes/help_main.txt"))]
//! ```
//!
//! ### Publisher Command
//!
//! Publishes samples of [ShapeType][ShapeType] data at specified intervals
//!
//! It can be invoked from the command line as follows:
//! ```console
#![doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/examples/shapes/help_pub.txt"))]
//! ```
//!
//! ### Subscriber Command
//!
//! Subscribes to samples of [`ShapeType`][ShapeType] data and prints
//! them to the console
//!
//! It can be invoked from the command line as follows:
//! ```console
#![doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/examples/shapes/help_sub.txt"))]
//! ```
//!
//! ## XML Configuration
//!
//! The example uses an XML configuration file (`Shapes.xml`) with the following content:
//! ```xml
#![doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/examples/shapes/Shapes.xml"))]
//! ```
//!

#![deny(missing_docs)]

mod publisher;
mod subscriber;

const PUB_PARTICIPANT_NAME: &str = "ShapeParticipantLibrary::Pub";
const SUB_PARTICIPANT_NAME: &str = "ShapeParticipantLibrary::Sub";
const OUTPUT_NAME: &str = "ShapePublisher::ShapeSquareWriter";
const INPUT_NAME: &str = "ShapeSubscriber::ShapeSquareReader";

use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

fn validate_samples(s: &str) -> std::result::Result<usize, String> {
    let value: usize = s
        .parse()
        .map_err(|_| format!("`{s}` isn't a valid number"))?;
    if value == 0 {
        Err("samples must be greater than 0".to_string())
    } else {
        Ok(value)
    }
}

/// Indicates whether typed serialization is enabled or disabled
#[derive(Debug, Clone, Copy)]
pub enum TypedMode {
    /// Use JSON serialization when enabled
    Enabled,
    /// Use dynamic, by-field serialization when disabled
    Disabled,
}

/// Structure matching the ShapeType DDS data model.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ShapeType {
    /// The color of the shape (used as the key field)
    pub color: String,
    /// The X coordinate of the shape
    pub x: i64,
    /// The Y coordinate of the shape
    pub y: i64,
    /// The size of the shape
    pub shapesize: i64,
}

/// Command-line arguments for the shapes example application
#[derive(Parser)]
#[command(name = "shapes")]
#[command(about = "RTI Connector for Rust example for Shape data")]
struct Args {
    #[command(subcommand)]
    /// Command to execute (publish or subscribe)
    command: Commands,

    #[arg(long)]
    /// Enable typed mode for shapes
    typed: bool,
}

impl Args {
    fn typed_mode(&self) -> TypedMode {
        if self.typed {
            TypedMode::Enabled
        } else {
            TypedMode::Disabled
        }
    }
}

/// Specific command-line arguments for components of the shapes example
#[derive(Subcommand)]
enum Commands {
    /// Publish shape data to DDS
    Pub {
        #[arg(short = 's', long, default_value_t = usize::MAX, value_parser = validate_samples)]
        /// Total number of samples to publish
        samples: usize,

        #[arg(short = 'w', long, default_value_t = 200)]
        /// Sleep duration between samples in milliseconds (0 = no wait)
        wait_ms: u64,

        #[arg(short = 'd', long, default_value_t = 3000)]
        /// Wait for subscriptions timeout in milliseconds (0 = infinite)
        wait_for_subscriptions_ms: u64,
    },
    /// Subscribe to shape data from DDS
    Sub {
        #[arg(short = 's', long, default_value_t = usize::MAX, value_parser = validate_samples)]
        /// Total number of samples to read
        samples: usize,

        #[arg(short = 'w', long, default_value_t = 500)]
        /// Wait timeout in milliseconds (0 = infinite)
        wait_ms: u64,

        #[arg(short = 'd', long, default_value_t = 3000)]
        /// Wait for publications timeout in milliseconds (0 = infinite)
        wait_for_publications_ms: u64,
    },
}

// Shared utility functions
fn config_path() -> Result<std::path::PathBuf> {
    use std::{env, fs};

    let contents = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/examples/shapes/Shapes.xml"
    ));

    // Create a temporary file with the XML configuration
    // Create temp file in system temp directory
    let temp_dir = env::temp_dir();
    let temp_path = temp_dir.join("Shapes.xml");

    // Write contents to temp file
    fs::write(&temp_path, contents)?;

    Ok(temp_path)
}

fn optional_duration_from_ms(ms: u64) -> Option<std::time::Duration> {
    if ms == 0 {
        None
    } else {
        Some(std::time::Duration::from_millis(ms))
    }
}

fn main() -> Result<()> {
    run(Args::parse())
}

fn run(args: Args) -> Result<()> {
    let typed_mode = args.typed_mode();
    println!(
        "Running with typed support: {}",
        matches!(typed_mode, TypedMode::Enabled)
    );

    match args.command {
        Commands::Pub {
            samples,
            wait_ms,
            wait_for_subscriptions_ms,
        } => publisher::main(typed_mode, samples, wait_ms, wait_for_subscriptions_ms),
        Commands::Sub {
            samples,
            wait_ms,
            wait_for_publications_ms,
        } => subscriber::main(typed_mode, samples, wait_ms, wait_for_publications_ms),
    }
}

#[cfg(test)]
mod tests {
    use super::{Args, Parser, TypedMode, run};
    use std::thread;

    /// The purpose of this test is to actually execute the shapes example code to ensure it works as expected. It's not a unit test of individual components, but rather an integration test to validate the overall functionality.
    #[test]
    fn use_shapes_example() {
        impl_shapes_example(TypedMode::Disabled);
        impl_shapes_example(TypedMode::Enabled);
    }

    fn impl_shapes_example(typed_mode: TypedMode) {
        let typed_flag = matches!(typed_mode, TypedMode::Enabled);

        // Prepare the threads operations for publisher and subscriber
        let pub_fn = move || {
            let args = if typed_flag {
                "pub --samples 10 --typed"
            } else {
                "pub --samples 10"
            };
            let args = {
                let program_iter = std::iter::once("shapes");
                let args_iter = args.split_whitespace();
                Args::try_parse_from(program_iter.chain(args_iter))
            }
            .expect("Failed to parse publisher arguments");

            run(args).expect("Publisher run should succeed");
        };
        let sub_fn = move || {
            let args = if typed_flag {
                "sub --samples 10 --typed"
            } else {
                "sub --samples 10"
            };
            let args = {
                let program_iter = std::iter::once("shapes");
                let args_iter = args.split_whitespace();
                Args::try_parse_from(program_iter.chain(args_iter))
            }
            .expect("Failed to parse subscriber arguments");

            run(args).expect("Subscriber run should succeed");
        };

        // Prepare and run the threads
        let pub_thread_handle = thread::spawn(move || pub_fn());
        let sub_thread_handle = thread::spawn(move || sub_fn());

        // Wait for both threads to complete
        pub_thread_handle
            .join()
            .expect("Publisher thread has panicked");
        sub_thread_handle
            .join()
            .expect("Subscriber thread has panicked");
    }
}
