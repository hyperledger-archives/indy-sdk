use atty;

#[macro_export]
macro_rules! println_err {
    ($($arg:tt)*) => (
        if $crate::utils::term::is_term() {
            println!("{}", $crate::ansi_term::Color::Red.bold().paint(format!($($arg)*)))
        } else {
            println!($($arg)*)
        }
    )
}

#[macro_export]
macro_rules! println_succ {
    ($($arg:tt)*) => (
        if $crate::utils::term::is_term() {
            println!("{}", $crate::ansi_term::Color::Green.bold().paint(format!($($arg)*)))
        } else {
            println!($($arg)*)
        }
    )
}
#[macro_export]
macro_rules! println_warn {
    ($($arg:tt)*) => (
        if $crate::utils::term::is_term() {
            println!("{}", $crate::ansi_term::Color::Blue.bold().paint(format!($($arg)*)))
        } else {
            println!($($arg)*)
        }
    )
}

#[macro_export]
macro_rules! println_acc {
    ($($arg:tt)*) => (
       if $crate::utils::term::is_term() {
           println!("{}", $crate::ansi_term::Style::new().bold().paint(format!($($arg)*)))
       } else {
           println!($($arg)*)
       }
    )
}

// TODO: move to more relevant place
#[macro_export]
macro_rules! map_println_err {
    ($($arg:tt)*) => ( |err| println_err!("{}: {}", $($arg)*, err) )
}

pub fn is_term() -> bool {
    atty::is(atty::Stream::Stdout)
}