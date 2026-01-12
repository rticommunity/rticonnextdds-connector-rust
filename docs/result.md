# Result types for error handling

The `rtiddsconnector` API uses a small set of result types:

* [`ConnectorResult<T>`]: the standard result type for fallible operations.
* [`ConnectorFallible`]: a convenience alias for `Result<(), ConnectorError>`.
* [`ConnectorError`]: the error type used throughout the crate.

## Inspecting errors

`ConnectorError` exposes helper methods to classify errors:

* [`ConnectorError::is_timeout`]
* [`ConnectorError::is_entity_not_found`]
* [`ConnectorError::is_field_not_found`]
* [`ConnectorError::is_native_error`]
* [`ConnectorError::last_error_message`]

These helpers allow you to handle common cases without depending on internal
error kinds.
