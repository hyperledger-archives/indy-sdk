#[macro_use]
pub mod futures;
pub mod rand;
pub mod sequence;

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