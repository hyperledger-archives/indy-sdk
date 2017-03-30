#[macro_use]
extern crate log;

mod api;
mod commands;
mod errors;
mod services;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dummy() {
        assert! (true, "Dummy check!");
    }
}