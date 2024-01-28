use clap::{
    Parser,
    ValueEnum,
};

#[derive(Parser)]
#[command(name = "silica")]
#[command(author = "Ben Buchanan")]
#[command(version = "0.1.0")]
#[command(about = "A sand timer for the terminal")]
pub struct SilicaArgs {
    /// Starting countdown time, in seconds
    pub time: u64,

    /// Text to appear above the timer
    #[arg(short, long)]
    pub text: Option<String>,

    /// Size of the digits in the timer
    #[arg(short, long, value_enum, default_value_t = DigitSize::Medium)]
    pub digit_size: DigitSize,

    // TODO: Specify colors for digits, text, ascii, blink
    // TODO: Turn off blinking
    // TODO: Allow user to give second text that displays in AppState::Elapsed
}

#[derive(Copy, Clone, ValueEnum)]
pub enum DigitSize {
    Small,
    Medium,
    Large,
}
