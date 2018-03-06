#![no_std]

#![feature(never_type)]

extern crate nb;
extern crate embedded_hal as hal;
extern crate futures_core as futures;

use futures::{Async, Future, Poll, task::Context};

pub use hal::digital::OutputPin;

pub trait CountDown: Sized {
    type Time;
    type Future: Future<Item = Self, Error = !>;

    fn start<T: Into<Self::Time>>(self, count: T) -> Self::Future;
}

pub struct CountDownRunning<C: hal::timer::CountDown> {
    countdown: Option<C>,
}

impl<C: hal::timer::CountDown> CountDownRunning<C> {
    fn new<T: Into<C::Time>>(mut countdown: C, count: T) -> Self {
        hal::timer::CountDown::start(&mut countdown, count);
        CountDownRunning { countdown: Some(countdown) }
    }
}

impl<C: hal::timer::CountDown> CountDown for C {
    type Time = <C as hal::timer::CountDown>::Time;
    type Future = CountDownRunning<C>;

    fn start<T: Into<Self::Time>>(self, count: T) -> Self::Future {
        CountDownRunning::new(self, count.into())
    }
}

impl<C: hal::timer::CountDown> Future for CountDownRunning<C> {
    type Item = C;
    type Error = !;

    fn poll(&mut self, _: &mut Context) -> Poll<Self::Item, Self::Error> {
        match self.countdown.as_mut().expect("Cannot poll after completion").wait() {
            Ok(()) => Ok(Async::Ready(self.countdown.take().unwrap())),
            Err(nb::Error::WouldBlock) => Ok(Async::Pending),
        }
    }
}
