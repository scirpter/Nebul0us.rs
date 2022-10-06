use colored::Colorize;
use std::process::Command;

pub fn ok(tag: &str, text: &str) {
    print!("{} {}", tag.bold().green(), text);
}

pub fn error(tag: &str, text: &str) {
    print!("{} {}", tag.bold().red(), text);
}

pub fn log(tag: &str, text: &str) {
    println!("{} {}", tag.bold().purple(), text);
}

pub fn low_prio_log(tag: &str, text: &str) {
    println!("{} {}", tag.bold().blue(), text);
}

pub fn high_prio_log(tag: &str, text: &str) {
    println!("{} {}", tag.bold().bright_magenta(), text.clear());
}

pub fn clear_console() {
    if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(&["/C", "cls"])
            .status()
            .expect("failed to execute process");
    } else {
        // run "clear" command via bash if on linux
        Command::new("bash")
            .args(&["-c", "clear"])
            .status()
            .expect("failed to execute process");
    }
}
