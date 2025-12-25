/// ANSI color codes for terminal output (True Color support)

pub const RESET: &str = "\x1b[0m";

/// Dim gray for borders and separators
pub fn dim(s: &str) -> String {
    format!("\x1b[38;2;68;68;68m{}{}", s, RESET)
}

/// Subtle text (muted gray)
pub fn muted(s: &str) -> String {
    format!("\x1b[38;2;120;120;120m{}{}", s, RESET)
}

/// Cyan accent for highlights
pub fn cyan(s: &str) -> String {
    format!("\x1b[38;2;86;182;194m{}{}", s, RESET)
}

/// Green for success/current status
pub fn green(s: &str) -> String {
    format!("\x1b[38;2;106;153;85m{}{}", s, RESET)
}
