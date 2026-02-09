// Publisher functionality

use super::{
    OUTPUT_NAME, PUB_PARTICIPANT_NAME as PARTICIPANT_NAME, TypedMode, config_path,
};

use std::{thread, time::Duration};

use rtiddsconnector::Connector;

macro_rules! tlog {
        ($fmt:expr) => {
            println!("[Pub] {}", $fmt)
        };
        ($fmt:expr, $($arg:tt)*) => {
            println!("[Pub] {}", format!($fmt, $($arg)*))
        };
    }

pub fn main(
    typed_mode: TypedMode,
    samples: usize,
    wait_ms: u64,
    wait_for_subscriptions_ms: u64,
) -> super::Result<()> {
    let config_path = config_path()?;

    tlog!(
        "Loading publisher configuration: file={}, participant={}, output={}",
        config_path.display(),
        PARTICIPANT_NAME,
        OUTPUT_NAME
    );

    let connector = Connector::new(PARTICIPANT_NAME, &config_path.to_string_lossy())?;
    let sleep_duration = Duration::from_millis(wait_ms);
    let discovery_duration = super::optional_duration_from_ms(wait_for_subscriptions_ms);

    tlog!("Started publisher...");

    let mut output = connector
        .take_output(OUTPUT_NAME)
        .map_err(|e| format!("Failed to take output: {}", e))?;

    loop {
        let wait_result = match discovery_duration {
            Some(timeout) => output.wait_for_subscriptions_with_timeout(timeout),
            None => output.wait_for_subscriptions(),
        };

        match wait_result {
            Ok(count) => {
                tlog!(
                    "Discovered {} subscriptions, proceeding to publish...",
                    count
                );
                break;
            }
            Err(e) if e.is_timeout() => {
                tlog!("No subscriptions discovered yet, retrying...");
            }
            Err(e) => {
                return Err(format!("Wait for subscriptions failed: {}", e).into());
            }
        }
    }

    for sample_id in 1..=samples {
        tlog!("Writing sample #{}...", sample_id);

        output
            .clear_members()
            .map_err(|e| format!("Failed to clear members: {}", e))?;

        // Compute the value of the fields based on the sample ID
        let shape: super::ShapeType = compute_sample_for_id(sample_id);

        // Set the value of the fields
        let mut instance = output.instance();
        match typed_mode {
            TypedMode::Enabled => {
                // Use typed serialization when the feature is enabled
                instance
                    .serialize(&shape)
                    .expect("Failed to serialize typed shape");
            }
            TypedMode::Disabled => {
                // Manual field setting when typed feature is disabled
                instance
                    .set_number("x", shape.x as f64)
                    .expect("Failed to set x coordinate");
                instance
                    .set_number("y", shape.y as f64)
                    .expect("Failed to set y coordinate");
                instance
                    .set_number("shapesize", shape.shapesize as f64)
                    .expect("Failed to set shapesize");
                instance
                    .set_string("color", &shape.color)
                    .expect("Failed to set color");
            }
        }

        output
            .write()
            .map_err(|e| format!("Failed to write sample: {}", e))?;

        if sample_id < samples {
            thread::sleep(sleep_duration);
        }
    }

    tlog!("Completed {} samples, exiting...", samples);
    tlog!("Publisher completed successfully!");
    Ok(())
}

/// Computes the field values for a given sample ID
fn compute_sample_for_id(sample_id: usize) -> super::ShapeType {
    const COLOR: &str = "BLUE"; // Fixed color for simplicity
    const CANVAS: (f64, f64) = (250.0, 270.0);
    const CENTER: (f64, f64) = (CANVAS.0 / 2.0, CANVAS.1 / 2.0);
    const INCREMENT: (f64, f64) = (CANVAS.0 / 5.0, CANVAS.1 / 5.0);

    let x = (CENTER.0 + f64::sin(sample_id as f64) * INCREMENT.0) as i64;
    let y = (CENTER.1 + f64::cos(sample_id as f64) * INCREMENT.1) as i64;
    let shapesize =
        (CANVAS.0 / 10.0 + f64::cos(sample_id as f64) * CANVAS.0 / 20.0) as i64;

    super::ShapeType {
        color: COLOR.to_string(),
        x,
        y,
        shapesize,
    }
}
