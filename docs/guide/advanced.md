# Advanced operations

This section collects less common operations that are still useful in
production applications.

## Write parameters

[`crate::Output::write_with_params`] accepts a [`crate::WriteParams`] struct
that can change the write action (`write`, `dispose`, `unregister`) and attach
identity or timestamp metadata.

The available constructors are:

* [`crate::WriteParams::write`]
* [`crate::WriteParams::dispose`]
* [`crate::WriteParams::unregister`]

## Waiting and matching

* [`crate::Connector::wait_for_data`]: wait for any input to have data.
* [`crate::Input::wait_for_publications`]: wait for matched writers.
* [`crate::Output::wait_for_subscriptions`]: wait for matched readers.
* [`crate::Output::wait`]: wait for acknowledgments.

Both `Input` and `Output` provide JSON helpers to inspect matches:

* [`crate::Input::display_matched_publications`]
* [`crate::Output::display_matched_subscriptions`]

## Loan management

If you call [`crate::Input::take`], you can return the loan with
[`crate::Input::return_loan`] after processing to release native resources.
