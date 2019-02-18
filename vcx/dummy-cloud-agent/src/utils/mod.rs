#[macro_use]
pub mod futures;
pub mod rand;

#[allow(unused)] // FIXME:
#[cfg(test)]
pub mod tests;

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