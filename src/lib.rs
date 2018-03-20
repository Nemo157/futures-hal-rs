#![no_std]
#![feature(never_type)]
#![feature(exhaustive_patterns)]
#![feature(arbitrary_self_types)]

extern crate embedded_hal as hal;
extern crate futures_core as futures;
extern crate nb;

use core::time::Duration;

use futures::{Future, Stream};

pub use hal::digital::{InputPin, OutputPin, Event};

pub mod bridge;

pub trait Cancellable {
    type Item;

    fn cancel(self) -> Self::Item;
}

pub trait CountDown: Sized {
    type Future: Future<Item = Self, Error = !> + Cancellable<Item = Self>;

    fn start(self, count: Duration) -> Self::Future;
}

pub trait Periodic: CountDown {
    type Stream: Stream<Item = (), Error = !> + Cancellable<Item = Self>;

    fn periodic(self, count: Duration) -> Self::Stream;
}

pub trait DetectingInputPin {
    type Stream: Stream<Item = (), Error = !>;

    fn detect(self, event: Event) -> Self::Stream;
}
