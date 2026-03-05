use super::{EnvDropGuard, TEST_TIMEOUT};
use assert_matches::assert_matches;
use rtiddsconnector::{Connector, ConnectorResult, GlobalsDropGuard, Input, Output};
use std::path::PathBuf;

static TEST_CONFIG_FILE: &str =
    concat!(env!("CARGO_MANIFEST_DIR"), "/tests/resources", "/Test.xml");

/// Builder for constructing a `TestContext`.
#[derive(Debug)]
pub struct TestContextBuilder {
    config_file: PathBuf,
    config_name: String,
    input_name: Option<String>,
    output_name: Option<String>,
}

impl TestContextBuilder {
    pub fn new(config_file: impl Into<PathBuf>, config_name: impl Into<String>) -> Self {
        Self {
            config_file: config_file.into(),
            config_name: config_name.into(),
            input_name: None,
            output_name: None,
        }
    }

    /// Profile: simple participant with both input and output.
    pub fn simple() -> Self {
        Self::new(
            TEST_CONFIG_FILE,
            "TestDomainParticipantLibrary::SimpleParticipant",
        )
        .with_input(Some("TestSubscriber::TestReader"))
        .with_output(Some("TestPublisher::TestWriter"))
    }

    /// Profile: simple participant with input only.
    pub fn simple_input_only() -> Self {
        Self::new(
            TEST_CONFIG_FILE,
            "TestDomainParticipantLibrary::SimpleParticipant",
        )
        .with_input(Some("TestSubscriber::TestReader"))
    }

    /// Profile: simple participant with output only.
    pub fn simple_output_only() -> Self {
        Self::new(
            TEST_CONFIG_FILE,
            "TestDomainParticipantLibrary::SimpleParticipant",
        )
        .with_output(Some("TestPublisher::TestWriter"))
    }

    /// Profile: complex participant with both input and output.
    pub fn complex() -> Self {
        Self::new(
            TEST_CONFIG_FILE,
            "TestDomainParticipantLibrary::ComplexParticipant",
        )
        .with_input(Some("TestSubscriber::TestReader"))
        .with_output(Some("TestPublisher::TestWriter"))
    }

    /// Sets the config file path.
    pub fn with_config_file(mut self, config_file: impl Into<PathBuf>) -> Self {
        self.config_file = config_file.into();
        self
    }

    /// Sets the config name (participant profile).
    pub fn with_config_name(mut self, config_name: impl Into<String>) -> Self {
        self.config_name = config_name.into();
        self
    }

    /// Sets the input name.
    pub fn with_input(mut self, input_name: Option<impl Into<String>>) -> Self {
        self.input_name = input_name.map(|s| s.into());
        self
    }

    /// Sets the output name.
    pub fn with_output(mut self, output_name: Option<impl Into<String>>) -> Self {
        self.output_name = output_name.map(|s| s.into());
        self
    }

    /// Builds the `TestContext`.
    pub fn build(self) -> ConnectorResult<TestContext> {
        let partition_id: String = {
            use std::time::{SystemTime, UNIX_EPOCH};

            let timestamp = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis();
            let thread_id = format!("{:?}", std::thread::current().id());

            format!("test_partition_{:?}_ts{}", thread_id, timestamp)
        };

        let config_file_str = self
            .config_file
            .to_str()
            .expect("Config file path must be valid UTF-8");

        let connector = EnvDropGuard::with_env("PARTITION_ID", &partition_id, || {
            Connector::new(&self.config_name, config_file_str)
        })?;

        Ok(TestContext {
            connector,
            test_input_name: self.input_name,
            test_output_name: self.output_name,
            _globals: GlobalsDropGuard,
        })
    }
}

/// This is a helper struct to manage test context.
///
/// In general, tests consist of using both an Input and an Output, but some tests may only need
/// one of them. This struct allows configuring which entities to create and provides helper
/// methods to access and work with them.
///
/// The default configuration implies using `/tests/resources/Test.xml` for definitions.
#[derive(Debug)]
pub struct TestContext {
    pub connector: Connector,
    test_input_name: Option<String>,
    test_output_name: Option<String>,
    _globals: GlobalsDropGuard,
}

impl TestContext {
    pub fn test_entities(&mut self) -> ConnectorResult<TestEntities> {
        let input = match &self.test_input_name {
            Some(name) => Some(self.connector.get_input(name)?),
            None => None,
        };

        let output = match &self.test_output_name {
            Some(name) => Some(self.connector.get_output(name)?),
            None => None,
        };

        debug_assert!(
            input.is_some() || output.is_some(),
            "At least one of input or output must be present in TestContext"
        );

        Ok(TestEntities { input, output })
    }
}

#[derive(Debug)]
pub struct TestEntities {
    pub input: Option<Input>,
    pub output: Option<Output>,
}

impl TestEntities {
    pub fn ensure_discovery(self) -> Self {
        self.ensure_discovery_of_amount_with_timeout(1, TEST_TIMEOUT)
    }

    pub fn ensure_discovery_of_amount_with_timeout(
        self,
        amount: usize,
        timeout: std::time::Duration,
    ) -> Self {
        if let (Some(input), Some(output)) = (&self.input, &self.output) {
            assert_matches!(
                input.wait_for_publications_with_timeout(timeout),
                Ok(count) if count as usize >= amount,
                "Input should have discovered at least {} publications",
                amount
            );

            assert_matches!(
                output.wait_for_subscriptions_with_timeout(timeout),
                Ok(count) if count as usize >= amount,
                "Output should have discovered at least {} subscriptions",
                amount
            );
        }

        self
    }
}
