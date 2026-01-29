use std::time::Instant;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TimerState {
    Stopped,
    Running,
    Paused,
}

pub struct Timer {
    /// Total duration in seconds
    pub duration: u32,
    /// Remaining time in seconds
    pub remaining: f32,
    /// Current state
    pub state: TimerState,
    /// Last tick time
    last_tick: Option<Instant>,
}

impl Default for Timer {
    fn default() -> Self {
        Self {
            duration: 120, // Default 2 minutes
            remaining: 120.0,
            state: TimerState::Stopped,
            last_tick: None,
        }
    }
}

impl Timer {
    pub fn new(duration_seconds: u32) -> Self {
        Self {
            duration: duration_seconds,
            remaining: duration_seconds as f32,
            state: TimerState::Stopped,
            last_tick: None,
        }
    }

    pub fn start(&mut self) {
        if self.remaining > 0.0 {
            self.state = TimerState::Running;
            self.last_tick = Some(Instant::now());
        }
    }

    pub fn pause(&mut self) {
        if self.state == TimerState::Running {
            self.state = TimerState::Paused;
            self.last_tick = None;
        }
    }

    pub fn toggle(&mut self) {
        match self.state {
            TimerState::Stopped => self.start(),
            TimerState::Running => self.pause(),
            TimerState::Paused => self.start(),
        }
    }

    pub fn reset(&mut self) {
        self.remaining = self.duration as f32;
        self.state = TimerState::Stopped;
        self.last_tick = None;
    }

    pub fn set_duration(&mut self, seconds: u32) {
        self.duration = seconds;
        if self.state == TimerState::Stopped {
            self.remaining = seconds as f32;
        }
    }

    pub fn add_minutes(&mut self, minutes: i32) {
        let new_duration = (self.duration as i32 + minutes * 60).max(0).min(5999) as u32;
        self.duration = new_duration;
        if self.state == TimerState::Stopped {
            self.remaining = new_duration as f32;
        }
    }

    pub fn add_seconds(&mut self, seconds: i32) {
        let new_duration = (self.duration as i32 + seconds).max(0).min(5999) as u32;
        self.duration = new_duration;
        if self.state == TimerState::Stopped {
            self.remaining = new_duration as f32;
        }
    }

    /// Update the timer - call this every frame
    pub fn tick(&mut self) {
        if self.state != TimerState::Running {
            return;
        }

        if let Some(last) = self.last_tick {
            let elapsed = last.elapsed().as_secs_f32();
            self.remaining -= elapsed;
            
            if self.remaining <= 0.0 {
                self.remaining = 0.0;
                self.state = TimerState::Stopped;
                self.last_tick = None;
            } else {
                self.last_tick = Some(Instant::now());
            }
        }
    }

    pub fn minutes(&self) -> u32 {
        (self.remaining as u32) / 60
    }

    pub fn seconds(&self) -> u32 {
        (self.remaining as u32) % 60
    }

    pub fn is_running(&self) -> bool {
        self.state == TimerState::Running
    }

    pub fn is_finished(&self) -> bool {
        self.remaining <= 0.0 && self.state == TimerState::Stopped && self.duration > 0
    }
}
