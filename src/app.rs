use std::time::Duration;

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
}

impl App {
    pub fn new(time: u64, text: Option<String>) -> Self {
        Self {
            state: AppState::Counting,
            quit: false,
            time: Duration::new(time, 0),
            text: text.unwrap_or(String::new()),
            blink: false,
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
