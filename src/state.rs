mod display;
mod timer;
pub mod sound;

use crate::cli_tools::Cli;

pub fn focus(cli: &Cli, current_pomodoro: u16) {
    if current_pomodoro > 1 {
        sound::play_threaded(cli.start_sound.to_string());
    }
    display::focus(current_pomodoro, cli.number_of_pomodoros);
    timer::run(cli.focus_time);
}

pub fn rest(cli: &Cli, current_pomodoro: u16) {
    sound::play_threaded(cli.end_sound.to_string());
    display::rest(current_pomodoro, cli.number_of_pomodoros);
    timer::run(cli.break_time);
}

pub fn long_rest(cli: &Cli) {
    sound::play_threaded(cli.done_sound.to_string());
    display::long_rest();
    timer::run(cli.long_break_time);
}