#![no_std]
#![feature(never_type)]
#![feature(arbitrary_self_types)]

extern crate pin_api;
extern crate embedded_hal as hal;
extern crate futures_core as futures;
extern crate nb;

use core::time::Duration;

use futures::{Future, Stream};

pub use hal::digital::{InputPin, OutputPin, Event};

pub mod bridge;

pub trait CancellableFuture: Future {
    fn cancel(self) -> Self::Item;
}

pub trait CountDown: Sized {
    type Future: CancellableFuture<Item = Self, Error = !>;

    fn start(self, count: Duration) -> Self::Future;
}

pub trait DetectingInputPin {
    type Stream: Stream<Item = (), Error = !>;

    fn detect(self, event: Event) -> Self::Stream;
}
