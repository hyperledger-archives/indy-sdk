use futures::Future;

/// This is the equivalent `try!` adapted to deal with futures.
#[macro_export]
macro_rules! ftry {
    ($res:expr) => {
        match $res {
            Ok(elt) => elt,
            Err(e) => {
                use $crate::utils::futures::FutureExt;
                return ::futures::future::err(From::from(e)).into_box();
            }
        }
    };
}

/// This is the equivalent of `Result::Ok()` adapted to deal with futures.
#[macro_export]
macro_rules! ok {
    ($elt:expr) => {{
        use $crate::utils::futures::FutureExt;
        ::futures::future::ok($elt).into_box()
    }};
}

/// This is the equivalent of `Result::Ok()` adapted to deal with futures.
#[macro_export]
macro_rules! err {
    ($elt:expr) => {{
        use $crate::utils::futures::FutureExt;
        ::futures::future::err(From::from($elt)).into_box()
    }};
}

pub type BoxedFuture<I, E> = Box<Future<Item = I, Error = E>>;

pub trait FutureExt: Future + Sized {
    /// Box this future. Similar to `boxed` combinator, but does not require
    /// the future to implement `Send`.
    fn into_box(self) -> BoxedFuture<Self::Item, Self::Error>;
}

impl<F: Future + 'static> FutureExt for F {
    fn into_box(self) -> Box<Future<Item = Self::Item, Error = Self::Error>> {
        Box::new(self)
    }
}