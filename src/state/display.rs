use colored::{Colorize, ColoredString};

fn display(message: ColoredString) {
    clearscreen::clear().unwrap();
    println!("{}", message);    
}

pub fn focus(current_pomodoro: u16, number_of_pomodoros: u16) {
    let text = format!("FOCUS, YOU FOOL! ({}/{})", current_pomodoro, number_of_pomodoros);
    display(text.bold().magenta());
}

pub fn rest(current_pomodoro: u16, number_of_pomodoros: u16) {
    let text = format!("TAKE A BREAK ({}/{})", current_pomodoro, number_of_pomodoros);
    display(text.bold().yellow());
}

pub fn long_rest() {
    let text = "LONG BREAK TIME, enjoy your hot beverage!";
    display(text.bold().green());
}