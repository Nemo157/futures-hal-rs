#![no_std]
#![feature(never_type)]
#![feature(arbitrary_self_types)]

extern crate anchor_experiment;
extern crate embedded_hal as hal;
extern crate futures_core as futures;
extern crate futures_stable as stable;
extern crate nb;

use core::time::Duration;

use anchor_experiment::{MovePinned, Pin};
use futures::{Async, Poll, task::Context};
use stable::StableFuture;

pub use hal::digital::OutputPin;

pub trait CountDown: Sized {
    type Future: StableFuture<Item = Self, Error = !>;

    fn start(self, count: Duration) -> Self::Future;
}

pub struct CountDownRunning<C>
where
    C: hal::timer::CountDown<Time = Duration> + MovePinned,
{
    countdown: Option<C>,
}

impl<C> CountDownRunning<C>
where
    C: hal::timer::CountDown<Time = Duration> + MovePinned,
{
    fn new(mut countdown: C, count: Duration) -> Self {
        hal::timer::CountDown::start(&mut countdown, count);
        CountDownRunning {
            countdown: Some(countdown),
        }
    }
}

impl<C> CountDown for C
where
    C: hal::timer::CountDown<Time = Duration> + MovePinned,
{
    type Future = CountDownRunning<C>;

    fn start(self, count: Duration) -> Self::Future {
        CountDownRunning::new(self, count)
    }
}

impl<C> StableFuture for CountDownRunning<C>
where
    C: hal::timer::CountDown<Time = Duration> + MovePinned,
{
    type Item = C;
    type Error = !;

    fn poll(mut self: Pin<Self>, _: &mut Context) -> Poll<Self::Item, Self::Error> {
        let countdown = self.countdown
            .as_mut()
            .expect("Cannot poll after completion");
        match countdown.wait() {
            Ok(()) => Ok(Async::Ready(self.countdown.take().unwrap())),
            Err(nb::Error::WouldBlock) => Ok(Async::Pending),
        }
    }
}
