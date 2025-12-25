#[inline(always)]
pub fn link(text: &str, url: &str) -> String {
    if std::env::var("NO_COLOR").is_err() {
        return format!("\x1b]8;;{}\x07{}\x1b]8;;\x07", url, text);
    }

    format!("{} ({})", text, url)
}
