/*
    Appellation: resolver <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

use futures::channel::oneshot;
use futures::executor::block_on;
use futures::future::{select, Either, FusedFuture, FutureExt, Shared};

/// Future that resolves when the exit signal has fired.
#[derive(Clone, Debug)]
pub struct Exit(pub(crate) Shared<oneshot::Receiver<()>>);

impl Exit {
    pub fn new(receiver: oneshot::Receiver<()>) -> Self {
        Self(receiver.shared())
    }
}

impl Future for Exit {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        let receiver = &mut Pin::into_inner(self).0;

        if receiver.is_terminated() {
            Poll::Ready(())
        } else {
            Pin::new(receiver).poll(cx).map(drop)
        }
    }
}

impl Exit {
    /// Perform given work until complete.
    pub fn until<F: Future + Unpin>(self, future: F) -> impl Future<Output = Option<F::Output>> {
        select(self, future).map(|either| match either {
            Either::Left(_) => None,
            Either::Right((output, _)) => Some(output),
        })
    }

    /// Block the current thread until complete.
    pub fn wait(self) {
        block_on(self)
    }
}
