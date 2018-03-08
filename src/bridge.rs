use core::time::Duration;

use nb;
use hal;
use pin_api::{PinMut, Unpin};
use futures::{Async, Poll, task::Context};
use stable::StableFuture;

use CountDown;

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
