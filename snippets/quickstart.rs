use rtiddsconnector::{self, Connector, Input, Output};

/// The main entry point for this simple application demonstrating how to use
/// the RTI Connector for Rust API.
///
/// It starts by creating a [`rtiddsconnector::GlobalsDropGuard`] to ensure
/// proper cleanup of native after the "Connector scope" is exited, so that
/// your application does not show any memory leaks related to native
/// resources.
///
/// Then, it calls the [`start_using_connector`] function which contains the
/// main logic of this example application.
///
/// Finally, based on the [`rtiddsconnector::ConnectorError`] returned by
/// [`start_using_connector`], it returns a [`Result<(), String>`] to
/// simulate a mapping to your own Application Error type.
fn main() -> std::result::Result<(), String> {
    let _globals = rtiddsconnector::GlobalsDropGuard;

    if let Err(e) = start_using_connector() {
        Err(format!("Application failed: {}", e))
    } else {
        println!("Application completed successfully.");
        Ok(())
    }
}

/// A simple function demonstrating how to create a [`Connector`], publish data
/// to an [`Output`] and read data from an [`Input`].
///
/// See the associated constants, [`XML_PATH`], [`XML_PARTICIPANT`],
/// [`XML_OUTPUT`] and [`XML_INPUT`] for details on how a [`Connector`] is
/// configured.
fn start_using_connector() -> rtiddsconnector::ConnectorFallible {
    // Create a Connector instance and its contained entities
    let connector = Connector::new(XML_PARTICIPANT, XML_PATH)?;

    // Fetch the Output associated with the Connector instance
    let mut output: Output<'_> = connector.get_output(XML_OUTPUT)?;

    // Modifies the data contained by the Output instance
    let mut instance = output.instance();
    instance.set_number("x", 100_f64)?;
    instance.set_number("y", 150_f64)?;
    instance.set_number("shapesize", 15_f64)?;
    instance.set_string("color", "BLUE")?;

    // Once the data has been written, it can now be published
    output.write()?;

    // Fetch the Input associated with the Connector instance
    let mut input: Input<'_> = connector.get_input(XML_INPUT)?;

    // Wait for the data to be available before actually retrieving the samples
    input.wait_with_timeout(std::time::Duration::from_secs(5))?;

    // Retrieve the available samples from the Input instance
    input.take()?; // Take available samples

    // Iterate through the valid samples and print their content
    for sample in input.into_iter().valid_only() {
        println!(
            "Position: x={}, y={}, shapesize={}, color={}",
            sample.get_number("x")?,
            sample.get_number("y")?,
            sample.get_number("shapesize")?,
            sample.get_string("color")?
        );

        // Alternatively, you can use the std::fmt::Display implementation
        // for Sample instances to print their content
        println!("Sample content: {}", sample);
    }

    Ok(())
}

/// The path to your XML configuration file containing the relevant entities.
const XML_PATH: &str = "/path/to/your_xml_configuration_file.xml";

/// The namespaced name to the DomainParticipant which will be used for
/// the [`Connector`] instance.
const XML_PARTICIPANT: &str = "MyLibrary::MyParticipant";

/// The namespaced name to the DataWriter which will be associated with an
/// [`Output`] instance.
const XML_OUTPUT: &str = "ShapePublisher::ShapeSquareWriter";

/// The namespaced name to the DataReader which will be associated with an
/// [`Input`] instance.
const XML_INPUT: &str = "ShapeSubscriber::ShapeSquareReader";
