futures-hal
===========

An **experimental** bridge between the worlds of [`embedded-hal`][] and
[`futures`][].

One big forward-compatibility note: The traits specified in here will most
likely be greatly changed once [generic associated types][GAT] are implemented.
These + self-borrowing generators will hopefully allow much nicer APIs where you
can (for example) cancel a timer by simply dropping the returned future and then
re-use the `Timer` object to start another timer, instead of having to do the
current `let delay = timer.start(...); timer = delay.cancel();` dance.


[`embedded-hal`]: https://github.com/japaric/embedded-hal
[`futures`]: https://github.com/rust-lang-nursery/futures-rs
[GAT]: https://github.com/rust-lang/rust/issues/44265
