// Copyright 2021-2022 Aaron Erhardt <aaron.erhardt@t-online.de>
// Copyright 2022 System76 <info@system76.com>
// SPDX-License-Identifier: MIT or Apache-2.0

use crate::*;

#[derive(Debug)]
/// Controls the component from afar.
pub struct Controller<W, I> {
    /// The widget that this component manages.
    pub widget: W,

    /// Used for emitting events to the component.
    pub sender: Sender<I>,
}

impl<W, I> Controller<W, I> {
    /// Emits an input to the component.
    pub fn emit(&self, event: I) {
        let _ = self.sender.send(event);
    }
}