#![no_std]

use core::future::Future;
use core::pin::Pin;
use core::task::Context;
use core::task::Poll;
use nb::Error;
use nb::Result;

#[macro_export]
macro_rules! nb_async {
    ($e: expr) => {
        nb_async_adapter::NbFuture::new(|| $e);
    }
}

pub struct NbFuture<T, E, F> 
    where F: FnMut() -> Result<T, E>
{
    inner: F,
}

impl<T, E, F> NbFuture<T, E, F> 
    where F: FnMut() -> Result<T, E>
{
    pub fn new(inner: F) -> Self {
        NbFuture {
            inner
        }
    }

    pub fn call(&mut self) -> Result<T, E> {
        (self.inner)()
    }
}


impl<T, E, F> Future for NbFuture<T, E, F>
    where F: FnMut() -> Result<T, E> + core::marker::Unpin
{
    type Output = core::result::Result<T, E>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>)
        -> Poll<Self::Output>
    {
        // let r = (self.inner)();
        let r = self.get_mut().call();
        match r {
            Ok(t) => Poll::Ready(Ok(t)),
            Err(Error::WouldBlock) => {
                cx.waker().wake_by_ref();
                Poll::Pending
            },
            Err(Error::Other(e)) => Poll::Ready(Err(e)),
        }
    }
} 
