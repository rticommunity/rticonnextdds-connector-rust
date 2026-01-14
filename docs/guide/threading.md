# Threading and ownership

The underlying RTI Connector C API is not thread-safe. The Rust bindings provide
synchronization around the native connector and enforce single-threaded
ownership of `Input` and `Output` handles.

## Ownership rules

Use the following ownership rules:

* `Connector::get_input` and `Connector::get_output` return exclusive handles.
* If an entity is already owned by another thread, you will receive an error.
* Use `Connector::take_input` and `Connector::take_output` to block until the
  entity is free.

## Practical guidance about threading

Keep each `Input` or `Output` on a single thread at a time. If you need to share
work, consider a worker thread that owns the handle and communicates via
channels with the rest of your application.

While the connector uses internal locks for native access, this is not a
guarantee of safe concurrent access to the same `Input` or `Output`. Treat the
API as single-threaded unless you control synchronization at the application
level.
