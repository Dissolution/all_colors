#![allow(dead_code)]

use std::fmt::{Display, Formatter, Result};
use std::time::{Duration, Instant};

#[derive(Clone, Copy, Debug)]
pub struct Stopwatch {
    /// The `Instant` that this `Stopwatch` was started
    start_time: Instant,
}

impl Stopwatch {
    /// Starts a new, running `Stopwatch`
    pub fn start_new() -> Stopwatch {
        Stopwatch {
            start_time: Instant::now(),
        }
    }

    /// Restarts this `Stopwatch` _now_
    pub fn restart(&mut self) {
        self.start_time = Instant::now();
    }

    /// Gets a split `Duration` measured from the starting time to _now_
    pub fn split_elapsed(&self) -> Duration {
        let end_time = Instant::now();
        end_time - self.start_time
    }

    /// Restarts this `Stopwatch` and returns the `Duration` it measured up until it restarted
    pub fn restart_elapsed(&mut self) -> Duration {
        let end_time = Instant::now();
        let elapsed = end_time - self.start_time;
        self.start_time = Instant::now();
        elapsed
    }
}

impl Display for Stopwatch {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        // Show what has elapsed until now
        write!(
            f,
            "Started at {:?}, {:?} ago",
            self.start_time,
            self.split_elapsed()
        )
    }
}
