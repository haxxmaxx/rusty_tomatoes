use std::process;
use clap::Parser;
use crate::SECS_PER_MIN;

const MAX_SECONDS: u16 = 65_535;
#[derive(Parser)]
#[command(version, about, long_about = None)]

/// Pomodoro timer with sound effects to help your lazy ass focus
pub struct Cli {
    /// Time in minutes for the focus period
    #[arg(short, long, default_value_t = 25)]
    pub focus_time: u16,
    /// Time in minutes for the break period
    #[arg(short, long, default_value_t = 5)]
    pub break_time: u16,
    /// Time in minutes for the long break period
    #[arg(short, long, default_value_t = 20)]
    pub long_break_time: u16,
    /// Number of pomodoros to complete before a long break
    #[arg(short, long, default_value_t = 4)]
    pub number_of_pomodoros: u16,
    /// Path to the sound file which plays at the start of a focus period
    #[arg(short, long, default_value_t = String::from("assets/start.wav"))]
    pub start_sound: String,
    /// Path to the sound file which plays at the end of a focus period
    #[arg(short, long, default_value_t = String::from("assets/end.mp3"))]
    pub end_sound: String,
    /// Path to the sound file which plays at the end of a focus period, before a long break
    #[arg(short, long, default_value_t = String::from("assets/trumpet.mp3"))]
    pub done_sound: String,
    /// Terminate the program when all pomodoros are done, skipping the long break
    #[arg(short, long, default_value_t = false)]
    pub terminate_when_done: bool,
}

pub fn setup_ctrl_c() {
    ctrlc::set_handler(|| {
        println!("\n\nGreat job, see you soon!");
        process::exit(0);
    }).unwrap();
}

pub fn parse() -> Cli {
    let cli = Cli::parse();
    let max_minutes = MAX_SECONDS / SECS_PER_MIN;
    // TODO: do this properly
    if cli.focus_time > max_minutes {
        panic!("Focus time cannot be greater than {} minutes", max_minutes)
    }
    if cli.focus_time > max_minutes {
        panic!("Rest time cannot be greater than {} minutes", max_minutes)
    }
    if cli.focus_time > max_minutes {
        panic!("Long rest time cannot be greater than {} minutes", max_minutes)
    }

    cli
}