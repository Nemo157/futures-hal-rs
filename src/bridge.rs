use core::time::Duration;

use nb;
use hal;
use futures::{Async, Poll, Future, Stream, task::Context};

use {CancellableFuture, CountDown, DetectingInputPin, Event};

pub struct CountDownRunning<C>
where
    C: hal::timer::CountDown<Time = Duration>,
{
    countdown: Option<C>,
}

impl<C> CountDownRunning<C>
where
    C: hal::timer::CountDown<Time = Duration>,
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
    C: hal::timer::CountDown<Time = Duration>,
{
    type Future = CountDownRunning<C>;

    fn start(self, count: Duration) -> Self::Future {
        CountDownRunning::new(self, count)
    }
}

impl<C> Future for CountDownRunning<C>
where
    C: hal::timer::CountDown<Time = Duration>,
{
    type Item = C;
    type Error = !;

    fn poll(&mut self, _: &mut Context) -> Poll<Self::Item, Self::Error> {
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

impl<C> CancellableFuture for CountDownRunning<C>
where
    C: hal::timer::CountDown<Time = Duration>,
{
    fn cancel(mut self) -> Self::Item {
        self.countdown.take().expect("Cannot cancel after completion")
    }
}

pub struct Detector<I> where I: hal::digital::DetectingInputPin {
    detector: I::Detector,
}

impl<I> Detector<I>
where
    I: hal::digital::DetectingInputPin,
{
    fn new(pin: I, event: Event) -> Self {
        Detector {
            detector: pin.detect(event),
        }
    }
}

impl<I> DetectingInputPin for I
where
    I: hal::digital::DetectingInputPin,
{
    type Stream = Detector<I>;

    fn detect(self, event: Event) -> Self::Stream {
        Detector::new(self, event)
    }
}

impl<I> Stream for Detector<I>
where
    I: hal::digital::DetectingInputPin,
{
    type Item = ();
    type Error = !;

    fn poll_next(&mut self, _: &mut Context) -> Poll<Option<Self::Item>, Self::Error> {
        match hal::digital::Detector::poll(&self.detector) {
            Ok(()) => Ok(Async::Ready(Some(()))),
            Err(nb::Error::WouldBlock) => Ok(Async::Pending),
        }
    }
}
