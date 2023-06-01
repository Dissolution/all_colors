use std::fmt::*;
use std::time::{Duration, Instant};

#[derive(Clone, Copy, Debug)]
pub struct Stopwatch {
    /// the time the stopwatch was started
    start_time: Instant,
}

impl Stopwatch {
    /// starts a new Stopwatch
    pub fn start_new() -> Stopwatch {
        Stopwatch {
            start_time: Instant::now(),
        }
    }

    #[allow(dead_code)]
    pub fn restart(&mut self) {
        self.start_time = Instant::now();
    }

    pub fn split_elapsed(&self) -> Duration {
        let end_time = Instant::now();
        end_time - self.start_time
    }

    pub fn restart_elapsed(&mut self) -> Duration {
        let end_time = Instant::now();
        let elapsed = end_time - self.start_time;
        self.start_time = Instant::now();
        elapsed
    }
}

impl Display for Stopwatch {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{:.2}ms", self.split_elapsed().as_millis())
    }
}
