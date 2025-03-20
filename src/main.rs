mod state;
mod cli_tools;

use state::sound;

pub const SECS_PER_MIN: u16 = 60;

fn main() {
    let cli = cli_tools::parse();
    cli_tools::setup_ctrl_c();

    loop {
        for current_pomodoro in 1..=cli.number_of_pomodoros  {
            state::focus(&cli, current_pomodoro);
            
            if current_pomodoro < cli.number_of_pomodoros {
                state::rest(&cli, current_pomodoro);
            }     
        }

        if cli.terminate_when_done {
            sound::play(cli.done_sound.to_string());
            break;
        }

        state::long_rest(&cli);
    }
}