#[macro_export]
macro_rules! println_err {
    ($($arg:tt)*) => ( println!("{}", $crate::ansi_term::Color::Red.bold().paint(format!($($arg)*))) )
}


#[macro_export]
macro_rules! println_succ {
    ($($arg:tt)*) => ( println!("{}", $crate::ansi_term::Color::Green.bold().paint(format!($($arg)*))) )
}
