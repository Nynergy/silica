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

    /// Color of the digits in the timer
    #[arg(long, default_value_t = 2)]
    #[arg(value_parser = clap::value_parser!(u8).range(0..9))]
    pub digit_color: u8,

    /// Color of the text alongside the timer
    #[arg(long, default_value_t = 7)]
    #[arg(value_parser = clap::value_parser!(u8).range(0..9))]
    pub text_color: u8,

    /// Color of the ascii art alongside the timer
    #[arg(long, default_value_t = 7)]
    #[arg(value_parser = clap::value_parser!(u8).range(0..9))]
    pub ascii_color: u8,

    /// Color of the blinking digits in the timer
    #[arg(long, default_value_t = 1)]
    #[arg(value_parser = clap::value_parser!(u8).range(0..9))]
    pub blink_color: u8,

    /// Turn off blinking
    #[arg(long)]
    pub noblink: bool,

    /// Second text that displays after timer has elapsed
    #[arg(long)]
    pub post_text: Option<String>,

    /// Don't show ascii art
    #[arg(long)]
    pub noascii: bool,
}

#[derive(Copy, Clone, ValueEnum)]
pub enum DigitSize {
    Small,
    Medium,
    Large,
}
