#![no_std]

#![feature(never_type)]

extern crate nb;
extern crate embedded_hal as hal;
extern crate futures_core as futures;

use core::time::Duration;

use futures::{Async, Future, Poll, task::Context};

pub use hal::digital::OutputPin;

pub trait CountDown: Sized {
    type Future: Future<Item = Self, Error = !>;

    fn start(self, count: Duration) -> Self::Future;
}

pub struct CountDownRunning<C: hal::timer::CountDown> {
    countdown: Option<C>,
}

impl<C: hal::timer::CountDown<Time=Duration>> CountDownRunning<C> {
    fn new(mut countdown: C, count: Duration) -> Self {
        hal::timer::CountDown::start(&mut countdown, count);
        CountDownRunning { countdown: Some(countdown) }
    }
}

impl<C: hal::timer::CountDown<Time=Duration>> CountDown for C {
    type Future = CountDownRunning<C>;

    fn start(self, count: Duration) -> Self::Future {
        CountDownRunning::new(self, count)
    }
}

impl<C: hal::timer::CountDown<Time=Duration>> Future for CountDownRunning<C> {
    type Item = C;
    type Error = !;

    fn poll(&mut self, _: &mut Context) -> Poll<Self::Item, Self::Error> {
        match self.countdown.as_mut().expect("Cannot poll after completion").wait() {
            Ok(()) => Ok(Async::Ready(self.countdown.take().unwrap())),
            Err(nb::Error::WouldBlock) => Ok(Async::Pending),
        }
    }
}
