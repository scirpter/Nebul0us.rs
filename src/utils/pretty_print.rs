use colored::Colorize;
use std::process::Command;

pub fn ok(tag: &str, text: &str) {
    print!("{} {}", tag.bold().bright_green(), text);
}

pub fn error(tag: &str, text: &str) {
    print!("{} {}", tag.bold().bright_red(), text);
}

pub fn log(tag: &str, text: &str) {
    println!("{} {}", tag.bold().bright_purple(), text);
}

pub fn high_prio_log(tag: &str, text: &str) {
    println!("{} {}", tag.bold().bright_magenta(), text.clear());
}

pub fn wtf(tag: &str, text: &str) {
    println!("{} {} :?", tag.bold().bright_yellow().dimmed(), text);
}

pub fn clear_console() {
    if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(&["/C", "cls"])
            .status()
            .expect("failed to execute process");
    } else {
        Command::new("bash")
            .args(&["-c", "clear"])
            .status()
            .expect("failed to execute process");
    }
}
