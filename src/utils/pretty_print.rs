use chrono;
use std::process::Command;

trait PrettyPrint {
    fn bold(&self) -> String;
    fn italic(&self) -> String;
    fn underline(&self) -> String;
    fn strikethrough(&self) -> String;
    fn color(&self, hex: u32) -> String;
    fn dimmed(&self) -> String;
}

impl PrettyPrint for &str {
    fn bold(&self) -> String {
        return format!("\x1b[1m{}\x1b[22m", self);
    }

    fn italic(&self) -> String {
        return format!("\x1b[3m{}\x1b[23m", self);
    }

    fn underline(&self) -> String {
        return format!("\x1b[4m{}\x1b[24m", self);
    }

    fn strikethrough(&self) -> String {
        return format!("\x1b[9m{}\x1b[29m", self);
    }

    fn color(&self, hex: u32) -> String {
        return format!(
            "\x1b[38;2;{};{};{}m{}\x1b[39m",
            (hex >> 16) & 0xFF,
            (hex >> 8) & 0xFF,
            hex & 0xFF,
            self
        );
    }

    fn dimmed(&self) -> String {
        return format!("\x1b[2m{}\x1b[22m", self);
    }
}

impl PrettyPrint for String {
    fn bold(&self) -> String {
        return self.as_str().bold();
    }

    fn italic(&self) -> String {
        return self.as_str().italic();
    }

    fn underline(&self) -> String {
        return self.as_str().underline();
    }

    fn strikethrough(&self) -> String {
        return self.as_str().strikethrough();
    }

    fn color(&self, hex: u32) -> String {
        return self.as_str().color(hex);
    }

    fn dimmed(&self) -> String {
        return self.as_str().dimmed();
    }
}

pub fn local_datetime() -> String {
    return chrono::Local::now().format("%H:%M:%S").to_string();
}

pub fn ok(tag: &str, text: &str) {
    println!(
        "{} {} {}",
        local_datetime().bold().color(0x0C0E12),
        tag.bold().color(0x00FFA6),
        text.color(0x00A170)
    );
}

pub fn error(tag: &str, text: &str) {
    println!(
        "{} {} {}",
        local_datetime().bold().color(0x0C0E12),
        tag.bold().color(0xE41B17),
        text.italic().color(0xE41B17)
    );
}

pub fn log(tag: &str, text: &str) {
    println!(
        "{} {} {}",
        local_datetime().bold().color(0x0C0E12),
        tag.bold().color(0x4A2C4C),
        text.color(0x0c0e12)
    );
}

pub fn high_prio_log(tag: &str, text: &str) {
    println!(
        "{} {} {}",
        local_datetime().bold().color(0x0C0E12),
        tag.bold().color(0xFF1F7A),
        text.color(0xD65076)
    );
}

pub fn wtf(tag: &str, text: &str) {
    println!(
        "{} {} {} :?",
        local_datetime().bold().color(0x0C0E12),
        tag.bold().color(0xF9BEAF),
        text.color(0xE9ADAE)
    );
}

pub fn ask(tag: &str, text: &str) -> String {
    println!(
        "{} {} {}",
        local_datetime().bold().color(0x0C0E12),
        tag.bold().color(0x00FFA6),
        text.color(0x00A170)
    );

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    return input.trim().to_string();
}

/// The console clear is required
/// on windows to have the console
/// ACTUALLY use the text formats.
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
