use std::process;
use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[arg(short, long, default_value_t = 25)]
    pub focus_time: u16,
    #[arg(short, long, default_value_t = 5)]
    pub rest_time: u16,
    #[arg(short, long, default_value_t = 20)]
    pub long_rest_time: u16,
    #[arg(short, long, default_value_t = 4)]
    pub number_of_pomodoros: u16,
    #[arg(short, long, default_value_t = String::from("resources/start.wav"))]
    pub start_sound: String,
    #[arg(short, long, default_value_t = String::from("resources/end.mp3"))]
    pub end_sound: String,
    #[arg(short, long, default_value_t = String::from("resources/trumpet.mp3"))]
    pub done_sound: String,
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
    Cli::parse()
}