#![no_std]
#![feature(never_type)]
#![feature(arbitrary_self_types)]

extern crate pin_api;
extern crate embedded_hal as hal;
extern crate futures_core as futures;
extern crate futures_stable as stable;
extern crate nb;

use core::time::Duration;

use stable::StableFuture;

pub use hal::digital::OutputPin;
pub use hal::digital::InputPin;

pub mod bridge;

pub trait CountDown: Sized {
    type Future: StableFuture<Item = Self, Error = !>;

    fn start(self, count: Duration) -> Self::Future;
}
