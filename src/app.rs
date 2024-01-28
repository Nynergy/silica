use std::time::Duration;

use crate::args::{
    DigitSize,
    SilicaArgs
};

pub enum AppState {
    Counting,
    Elapsed
}

pub struct App {
    pub state: AppState,
    pub quit: bool,
    pub time: Duration,
    pub text: String,
    pub blink: bool,
    pub digit_size: DigitSize,
}

impl App {
    pub fn new(args: SilicaArgs) -> Self {
        Self {
            state: AppState::Counting,
            quit: false,
            time: Duration::new(args.time, 0),
            text: args.text.unwrap_or(String::new()),
            blink: false,
            digit_size: args.digit_size,
        }
    }

    pub fn on_tick(&mut self) {
        match self.state {
            AppState::Counting => {
                self.time = self.time - Duration::new(1, 0);
                if self.time.as_secs() <= 0 {
                    self.state = AppState::Elapsed;
                }
            },
            AppState::Elapsed => {
                self.blink = !self.blink;
            },
        }
    }
}
