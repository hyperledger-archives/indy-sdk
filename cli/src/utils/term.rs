#[macro_export]
macro_rules! println_err {
    ($($arg:tt)*) => ( println!("{}", $crate::ansi_term::Color::Red.bold().paint(format!($($arg)*))) )
}

#[macro_export]
macro_rules! println_succ {
    ($($arg:tt)*) => ( println!("{}", $crate::ansi_term::Color::Green.bold().paint(format!($($arg)*))) )
}
#[macro_export]
macro_rules! println_warn {
    ($($arg:tt)*) => ( println!("{}", $crate::ansi_term::Color::Blue.bold().paint(format!($($arg)*))) )
}

#[macro_export]
macro_rules! println_acc {
    ($($arg:tt)*) => ( println!("{}", $crate::ansi_term::Style::new().bold().paint(format!($($arg)*))) )
}

// TODO: move to more relevant place
#[macro_export]
macro_rules! map_println_err {
    ($($arg:tt)*) => ( |err| println_err!("{}: {}", $($arg)*, err) )
}
