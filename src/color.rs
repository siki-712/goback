/// ANSI color codes for terminal output (True Color support)

pub const RESET: &str = "\x1b[0m";

/// #3D8361 - Main green
pub fn green(s: &str) -> String {
    format!("\x1b[38;2;61;131;97m{}{}", s, RESET)
}

/// Lighter shade for accents
pub fn green_light(s: &str) -> String {
    format!("\x1b[38;2;93;163;129m{}{}", s, RESET)
}

/// Gray for borders
pub fn gray(s: &str) -> String {
    format!("\x1b[38;2;100;100;100m{}{}", s, RESET)
}
