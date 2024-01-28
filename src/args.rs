use clap::Parser;

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
}
