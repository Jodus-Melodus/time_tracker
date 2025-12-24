use std::time::Instant;

use chrono::TimeDelta;

pub struct StopWatch {
    start: Option<Instant>,
    elapsed: TimeDelta,
}

impl StopWatch {
    pub fn new() -> Self {
        Self {
            start: None,
            elapsed: TimeDelta::zero(),
        }
    }

    pub fn start(&mut self) {
        self.start = Some(Instant::now());
    }

    pub fn stop(&mut self) {
        if let Some(s) = self.start.take() {
            if let Ok(delta) = TimeDelta::from_std(s.elapsed()) {
                self.elapsed += delta;
            }
        }
    }

    pub fn reset(&mut self) {
        self.start = None;
        self.elapsed = TimeDelta::zero();
    }

    pub fn elapsed(&mut self) -> TimeDelta {
        match self.start {
            Some(s) => {
                self.elapsed
                    + TimeDelta::from_std(s.elapsed()).unwrap_or_else(|_| TimeDelta::zero())
            }
            None => self.elapsed,
        }
    }

    pub fn is_running(&self) -> bool {
        self.start.is_some()
    }
}
