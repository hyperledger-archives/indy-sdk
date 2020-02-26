#[macro_use]
pub mod futures;
pub mod rand;
pub mod wallet;
pub mod config_env;

#[allow(unused)] // FIXME:
#[cfg(test)]
pub mod tests;
pub mod dyn_lib;

macro_rules! map (
    { $($key:expr => $value:expr),+ } => {
        {
            let mut m = ::std::collections::HashMap::new();
            $(
                m.insert($key, $value);
            )+
            m
        }
     };
);

pub fn to_i8(bytes: &Vec<u8>) -> Vec<i8> {
    let mut buf: Vec<i8> = Vec::new();
    for i in bytes {buf.push(*i as i8);}
    buf.to_owned()
}