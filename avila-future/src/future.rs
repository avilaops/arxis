//! Core Future trait and utilities

use crate::task::{Context, Pin};

/// The result of polling a future
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Poll<T> {
    /// The future is ready with a value
    Ready(T),
    /// The future is not yet ready
    Pending,
}

impl<T> Poll<T> {
    /// Maps a `Poll<T>` to `Poll<U>` by applying a function to a contained value
    pub fn map<U, F>(self, f: F) -> Poll<U>
    where
        F: FnOnce(T) -> U,
    {
        match self {
            Poll::Ready(t) => Poll::Ready(f(t)),
            Poll::Pending => Poll::Pending,
        }
    }

    /// Returns `true` if the poll is `Ready`
    pub fn is_ready(&self) -> bool {
        matches!(self, Poll::Ready(_))
    }

    /// Returns `true` if the poll is `Pending`
    pub fn is_pending(&self) -> bool {
        matches!(self, Poll::Pending)
    }
}

/// A future represents an asynchronous computation
pub trait Future {
    /// The type of value produced on completion
    type Output;

    /// Attempt to resolve the future to a final value
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
}

// Implement Future for &mut F
impl<F: ?Sized + Future + Unpin> Future for &mut F {
    type Output = F::Output;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        F::poll(Pin::new(&mut **self), cx)
    }
}

// Implement Future for Box<F>
#[cfg(feature = "std")]
impl<F: ?Sized + Future + Unpin> Future for std::boxed::Box<F> {
    type Output = F::Output;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        F::poll(Pin::new(&mut **self), cx)
    }
}

/// A future that is immediately ready with a value
pub struct Ready<T>(Option<T>);

impl<T> Future for Ready<T> {
    type Output = T;

    fn poll(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<T> {
        Poll::Ready(self.0.take().expect("Ready polled after completion"))
    }
}

/// Creates a future that is immediately ready with a value
pub fn ready<T>(value: T) -> Ready<T> {
    Ready(Some(value))
}

/// A future that is always pending
pub struct Pending<T> {
    _marker: core::marker::PhantomData<T>,
}

impl<T> Future for Pending<T> {
    type Output = T;

    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<T> {
        Poll::Pending
    }
}

/// Creates a future that is always pending
pub fn pending<T>() -> Pending<T> {
    Pending {
        _marker: core::marker::PhantomData,
    }
}

/// Extension trait for Future
pub trait FutureExt: Future {
    /// Map the output of this future to a different type
    fn map<U, F>(self, f: F) -> Map<Self, F>
    where
        Self: Sized,
        F: FnOnce(Self::Output) -> U,
    {
        Map {
            future: self,
            f: Some(f),
        }
    }

    /// Chain another future after this one
    fn then<Fut, F>(self, f: F) -> Then<Self, Fut, F>
    where
        Self: Sized,
        F: FnOnce(Self::Output) -> Fut,
        Fut: Future,
    {
        Then {
            state: ThenState::First {
                future: self,
                f: Some(f),
            },
        }
    }
}

impl<T: ?Sized + Future> FutureExt for T {}

/// Future for the `map` method
pub struct Map<Fut, F> {
    future: Fut,
    f: Option<F>,
}

impl<Fut, F, U> Future for Map<Fut, F>
where
    Fut: Future,
    F: FnOnce(Fut::Output) -> U,
{
    type Output = U;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<U> {
        unsafe {
            let this = self.get_unchecked_mut();
            let future = Pin::new_unchecked(&mut this.future);
            match future.poll(cx) {
                Poll::Ready(output) => {
                    let f = this.f.take().expect("Map polled after completion");
                    Poll::Ready(f(output))
                }
                Poll::Pending => Poll::Pending,
            }
        }
    }
}

enum ThenState<Fut1, Fut2, F> {
    First { future: Fut1, f: Option<F> },
    Second { future: Fut2 },
    Done,
}

/// Future for the `then` method
pub struct Then<Fut1, Fut2, F> {
    state: ThenState<Fut1, Fut2, F>,
}

impl<Fut1, Fut2, F> Future for Then<Fut1, Fut2, F>
where
    Fut1: Future,
    Fut2: Future,
    F: FnOnce(Fut1::Output) -> Fut2,
{
    type Output = Fut2::Output;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Fut2::Output> {
        unsafe {
            let this = self.get_unchecked_mut();
            loop {
                match &mut this.state {
                    ThenState::First { future, f } => {
                        let future = Pin::new_unchecked(future);
                        match future.poll(cx) {
                            Poll::Ready(output) => {
                                let f = f.take().expect("Then polled after completion");
                                let second_future = f(output);
                                this.state = ThenState::Second {
                                    future: second_future,
                                };
                            }
                            Poll::Pending => return Poll::Pending,
                        }
                    }
                    ThenState::Second { future } => {
                        let future = Pin::new_unchecked(future);
                        let result = future.poll(cx);
                        if result.is_ready() {
                            this.state = ThenState::Done;
                        }
                        return result;
                    }
                    ThenState::Done => panic!("Then polled after completion"),
                }
            }
        }
    }
}
