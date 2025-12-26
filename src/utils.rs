#[macro_export]
macro_rules! link {
    ($txt:expr, $url:expr) => {
        if std::env::var("NO_COLOR").is_err() {
            concat!("\x1b]8;;", $url, "\x07", $txt, "\x1b]8;;\x07")
        } else {
            concat!($txt, " (", $url, ")")
        }
    };
}

#[macro_export]
macro_rules! err {
    ($msg:ident) => {
        eprintln!("{}", $msg);
        std::process::exit(1);
    };
    ($msg:expr) => {
        eprintln!($msg);
        std::process::exit(1);
    };
    ($msg:expr, $idx:ident) => {
        eprintln!($msg, $idx);
        std::process::exit(1);
    };
}
