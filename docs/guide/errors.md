# Error handling

Most operations return [`crate::ConnectorResult<T>`] or
[`crate::ConnectorFallible`]. When an operation fails, you receive a
[`crate::ConnectorError`].

## Common patterns

```rust
use rtiddsconnector::{Connector, ConnectorError};

fn try_connect() -> Result<(), ConnectorError> {
    let _connector = Connector::new("MyLibrary::MyParticipant", "App.xml")?;
    Ok(())
}
```

Use helper methods to detect common cases:

* [`crate::ConnectorError::is_timeout`]
* [`crate::ConnectorError::is_entity_not_found`]
* [`crate::ConnectorError::is_field_not_found`]
* [`crate::ConnectorError::is_native_error`]

To inspect the last native error message, call
[`crate::ConnectorError::last_error_message`].

## Timeout example

This example waits for data and treats a timeout as a non-fatal outcome.

```rust
use rtiddsconnector::Input;

fn wait_with_timeout(input: &Input) -> rtiddsconnector::ConnectorFallible {
    match input.wait_with_timeout(std::time::Duration::from_secs(2)) {
        Ok(()) => Ok(()),
        Err(e) if e.is_timeout() => {
            println!("Timed out waiting for data");
            Ok(())
        }
        Err(e) => Err(e),
    }
}
```

## Native error details

If an operation fails because of a native RTI Connector error, the
[`crate::ConnectorError`] will include the last error message from the native library.
This can provide additional context when debugging configuration or data access
issues.
