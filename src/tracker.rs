use crate::detector::{Detector, Location};
use std::{
    process::Command,
    time::{Duration, Instant},
};

pub struct Tracker {
    current: Option<Location>,
    entered: Option<Instant>,
    triggered: bool,
}

impl Tracker {
    pub fn new() -> Self {
        Self {
            current: None,
            entered: None,
            triggered: false,
        }
    }

    pub fn update(&mut self, location: Option<Location>, detector: &Detector) {
        if self.current != location {
            if let Some(ref old) = self.current
                && let Some(hs) = detector.get_hotspot(&old.screen, &old.position)
                && let Some(ref cmd) = hs.on_leave {
                    exec(cmd);
                }

            self.current = location;
            self.entered = self.current.as_ref().map(|_| Instant::now());
            self.triggered = false;
            return;
        }

        if let (Some(loc), Some(entered), false) = (&self.current, self.entered, self.triggered)
            && let Some(hs) = detector.get_hotspot(&loc.screen, &loc.position)
        {
            let delay = hs.delay.unwrap_or(0);
            if entered.elapsed() >= Duration::from_millis(delay) {
                if let Some(ref cmd) = hs.on_enter {
                    exec(cmd);
                }
                self.triggered = true;
            }
        }
    }
}

fn exec(cmd: &str) {
    let _ = Command::new("sh").arg("-c").arg(cmd).spawn();
}
