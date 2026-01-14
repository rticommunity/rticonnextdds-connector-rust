# Result types for Error Handling

The [`ConnectorError`] enum represents possible errors that can occur
when using the Connector API.

The [`ConnectorResult`] type alias is used throughout the crate
to represent operations that can succeed or fail with a
[`ConnectorError`].

The [`ConnectorFallible`] type alias is used for operations
that can can fail with [`ConnectorError`] but that doesn't have a
meaningful return value (it is `()`).
