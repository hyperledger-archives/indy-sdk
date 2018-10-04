use errors::*;
pub use futures_rs::*;

pub type BoxedFuture<T> = Box<::futures::Future<Item = T, Error = Error>>;

pub trait FutureChainErr<T> {
    fn chain_err<F, E>(self, callback: F) -> BoxedFuture<T>
        where F: FnOnce() -> E + 'static,
              E: Into<ErrorKind>;
}

impl<F> FutureChainErr<F::Item> for F
    where F: ::futures::Future + 'static,
          F::Error: ::std::error::Error + Send + 'static,
{
    fn chain_err<C, E>(self, callback: C) -> BoxedFuture<F::Item>
        where C: FnOnce() -> E + 'static,
              E: Into<ErrorKind>,
    {
        Box::new(self.then(|r| r.chain_err(callback)))
    }
}

/// Like `try`, but returns an BoxedFuture instead of a Result.
/// #[warn(unused_macros)]
macro_rules! ftry {
    ($e:expr) => {
        match $e {
            Ok(v) => v,
            Err(e) => return Box::new($crate::futures::future::err(e.into())) as BoxedFuture<_>,
        }
    }
}

pub fn f_res<T, E: ::std::convert::Into<Error>>(t: ::std::result::Result<T, E>) -> BoxedFuture<T>
    where T: 'static,
{
    Box::new(::futures::future::result(t.map_err(Into::into)))
}

pub fn f_ok<T>(t: T) -> BoxedFuture<T>
    where T: 'static,
{
    Box::new(::futures::future::ok(t))
}

pub fn f_err<T, E>(e: E) -> BoxedFuture<T>
    where T: 'static,
          E: Into<Error>,
{
    Box::new(::futures::future::err(e.into()))
}

