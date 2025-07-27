#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {{
        use colored::Colorize;
        eprintln!("{}: {}", "[Info]".green(), format_args!($($arg)*));
    }};
}

#[macro_export]
macro_rules! warning {
    (context = $ctx:expr, $($arg:tt)*) => {{
        use colored::Colorize;
        eprintln!("{}: {}", format!("[Warning -- {}]", $ctx).yellow(), format_args!($($arg)*));
    }};
}

#[macro_export]
macro_rules! error {
    (context = $ctx:expr, $($arg:tt)*) => {{
        use colored::Colorize;
        eprintln!("{}: {}", format!("[Error -- {}]", $ctx).red(), format_args!($($arg)*));
    }};
}
