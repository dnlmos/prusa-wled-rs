#![allow(dead_code)]
use std::fmt;

/// Simple RGB colour type (0‑255 per channel)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rgb(pub u8, pub u8, pub u8);

impl fmt::Display for Rgb {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // prints as [R,G,B]
        write!(f, "[{},{},{}]", self.0, self.1, self.2)
    }
}

/// State colours for a printer / device UI.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    Operational, // normal / ready‑to‑work
    Printing,    // actively printing
    Pausing,     // transition to pause (busy‑ish)
    Paused,      // paused, awaiting resume
    Cancelling,  // being cancelled / stopped
    Error,       // serious problem
    Offline,     // no connection / idle
    Busy,        // generic busy indicator (attention‑grabber)

    Finished, // custom, finished
    Heating,  // custom
}

impl Color {
    /// Returns a pleasant RGB colour for the variant.
    pub const fn rgb(self) -> Rgb {
        match self {
            Color::Operational => Rgb(34, 139, 34), // forest‑green – ready / success
            Color::Printing => Rgb(30, 144, 255),   // dodger‑blue – active work
            Color::Pausing => Rgb(255, 165, 0),     // orange – transition to pause
            Color::Paused => Rgb(255, 215, 0),      // gold – paused but ready
            Color::Cancelling => Rgb(105, 105, 105), // dim‑gray – being stopped
            Color::Error => Rgb(220, 20, 60),       // crimson – error state
            Color::Offline => Rgb(128, 128, 128),   // slate‑gray – offline / idle
            Color::Busy => Rgb(255, 20, 147),       // deep‑pink – needs attention
            Color::Finished => Rgb(0, 200, 0),
            Color::Heating => Rgb(255, 165, 0),
        }
    }
}
