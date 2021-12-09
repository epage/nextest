// Copyright (c) The diem-devtools Contributors
// SPDX-License-Identifier: MIT OR Apache-2.0

use crossbeam_channel::Receiver;

/// A receiver that generates signals if ctrl-c is pressed.
///
/// A `SignalReceiver` can be passed into [`TestRunnerOpts::build`].
#[derive(Debug)]
pub struct SignalHandler {
    pub(crate) receiver: Receiver<SignalEvent>,
}

impl SignalHandler {
    /// Creates a new `SignalReceiver` that handles Ctrl-C errors.
    ///
    /// Errors if a signal handler has already been registered in this process. Only one signal
    /// handler can be registered for a process at any given time.
    pub fn new() -> Result<Self, ctrlc::Error> {
        let (sender, receiver) = crossbeam_channel::unbounded();
        ctrlc::set_handler(move || {
            let _ = sender.send(SignalEvent::Interrupted);
        })?;

        Ok(Self { receiver })
    }

    /// Creates a new `SignalReceiver` that does nothing.
    pub fn noop() -> Self {
        let (_sender, receiver) = crossbeam_channel::bounded(1);
        Self { receiver }
    }
}

// Just a single-valued enum for now, might have more information in the future.
#[derive(Debug)]
pub(crate) enum SignalEvent {
    Interrupted,
}
