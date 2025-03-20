use std::time::{Instant, Duration};
use std::thread;
use std::io;
use std::io::Write;
use crate::SECS_PER_MIN;

fn format_double_digit(num: u16) -> String {
    if num < 10 {
        format!("0{}", num)
    } else {
        format!("{}", num)
    }
}

fn display_time_left(time_left: u16) {
    let minutes_float = (time_left / SECS_PER_MIN) as f32;
    let minutes = minutes_float.floor() as u16;
    let seconds = time_left % SECS_PER_MIN;
    print!("\rTime left - {}:{}", format_double_digit(minutes), format_double_digit(seconds));
    io::stdout().flush().unwrap();
}

pub fn run(time: u16) {
    let one_second = Duration::from_secs(1);
    let mut next_time = Instant::now() + one_second;
    let time_range = (1..=time*60).rev();
    
    for time_left in time_range {
        display_time_left(time_left);
        thread::sleep(next_time - Instant::now());
        next_time += one_second;
    }
}