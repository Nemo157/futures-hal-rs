use core::time::Duration;

use nb;
use hal;
use pin_api::{PinMut, Unpin};
use futures::{Async, Poll, task::Context};
use stable::{StableFuture, StableStream};

use {CountDown, DetectingInputPin, Event};

pub struct CountDownRunning<C>
where
    C: hal::timer::CountDown<Time = Duration> + Unpin,
{
    countdown: Option<C>,
}

impl<C> CountDownRunning<C>
where
    C: hal::timer::CountDown<Time = Duration> + Unpin,
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
    C: hal::timer::CountDown<Time = Duration> + Unpin,
{
    type Future = CountDownRunning<C>;

    fn start(self, count: Duration) -> Self::Future {
        CountDownRunning::new(self, count)
    }
}

impl<C> StableFuture for CountDownRunning<C>
where
    C: hal::timer::CountDown<Time = Duration> + Unpin,
{
    type Item = C;
    type Error = !;

    fn poll(mut self: PinMut<Self>, _: &mut Context) -> Poll<Self::Item, Self::Error> {
        match self.countdown
            .as_mut()
            .expect("Cannot poll after completion")
            .wait()
        {
            Ok(()) => Ok(Async::Ready(self.countdown.take().unwrap())),
            Err(nb::Error::WouldBlock) => Ok(Async::Pending),
        }
    }
}

pub struct Detector<I> where I: hal::digital::DetectingInputPin + Unpin {
    detector: I::Detector,
}

impl<I> Detector<I>
where
    I: hal::digital::DetectingInputPin + Unpin,
{
    fn new(pin: I, event: Event) -> Self {
        Detector {
            detector: pin.detect(event),
        }
    }
}

impl<I> DetectingInputPin for I
where
    I: hal::digital::DetectingInputPin + Unpin,
{
    type Stream = Detector<I>;

    fn detect(self, event: Event) -> Self::Stream {
        Detector::new(self, event)
    }
}

impl<I> StableStream for Detector<I>
where
    I: hal::digital::DetectingInputPin + Unpin,
{
    type Item = ();
    type Error = !;

    fn poll_next(self: PinMut<Self>, _: &mut Context) -> Poll<Option<Self::Item>, Self::Error> {
        match hal::digital::Detector::poll(&self.detector) {
            Ok(()) => Ok(Async::Ready(Some(()))),
            Err(nb::Error::WouldBlock) => Ok(Async::Pending),
        }
    }
}
