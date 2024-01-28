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
    pub post_text: Option<String>,
    pub blink: bool,
    pub noblink: bool,
    pub noascii: bool,
    pub digit_size: DigitSize,
    pub digit_color: u8,
    pub text_color: u8,
    pub ascii_color: u8,
    pub blink_color: u8,
}

impl App {
    pub fn new(args: SilicaArgs) -> Self {
        Self {
            state: AppState::Counting,
            quit: false,
            time: Duration::new(args.time, 0),
            text: args.text.unwrap_or(String::new()),
            post_text: args.post_text,
            blink: false,
            noblink: args.noblink,
            noascii: args.noascii,
            digit_size: args.digit_size,
            digit_color: args.digit_color,
            text_color: args.text_color,
            ascii_color: args.ascii_color,
            blink_color: args.blink_color,
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
                self.blink = !self.blink && !self.noblink;
            },
        }
    }
}
