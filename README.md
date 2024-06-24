# Asynchronous Deferred Calls

This crate implements the [Active Object](https://en.wikipedia.org/wiki/Active_object) design pattern.

The entry point of this crate is [`Dispatcher`], which wraps an `Arc<Lock<T>>`.

*Note: Lock can be `RwLock` or `Mutex` from the `async-lock` crate.*

### [`Dispatcher`]

The [`Dispatcher`] allows you to create [`Caller`]s for some methods on `T`.

You can then schedule deferred calls to these methods via these [`Caller`]s.

[`Dispatcher`] implements `Future`; You must await this future for calls to
actually be dispatched.

### [`Caller`]

When you create a [`Caller`] for a method of `T` using [`Dispatcher`], a background
task is created, waiting for you to schedule deferred calls via the [`Caller`]. When
a call is scheduled, the background task will lock the `RwLock` or `Mutex` and call
the method on the locked `T` instance.

### Compatible `T` methods

The methods must:
- be asynchronous
- return an implementer of [`ReturnType`]

They can take `self` mutably or immutably.

*Note: you can use a freestanding function instead of a method if the first parameter
of that function is `&T` or `&mut T`.*

### [`Summoner`]

if the method's last parameter is an [`async_channel::Sender`], you can turn your [`Caller`] into
a [`Summoner`], which makes it easy to wait for a reply when you schedule a call.

### Example

```rust
use async_defer::{*, async_channel::Sender, async_lock::RwLock};
use std::time::{Instant, Duration};
use std::sync::Arc;

// An example object which will be modified in deferred calls
struct Subject;

impl Subject {
    async fn print(&mut self, msg: String) -> Result<(), String> {
        println!("msg: {}", msg);
        Ok(())
    }

    async fn ping_pong(&self, payload: u8, reply: Sender<u8>) -> Result<(), String> {
        let _ = reply.send(payload).await;
        Ok(())
    }
}

let world = Arc::new(RwLock::new(Subject));
let mut dispatcher = Dispatcher::new(world);

let deferred_print = dispatcher.listen_mut_1(Subject::print);
let deferred_ping_pong = dispatcher.listen_ref_2(Subject::ping_pong);

async {
    deferred_print.call("Hello World".to_string()).await;

    let later = Instant::now() + Duration::from_secs(5);
    deferred_print.call_later(later, "Hello World".to_string()).await;

    // `ping_pong` has a reply parameter,
    // allowing us to create a `Summoner`.
    let summoner = deferred_ping_pong.summoner();
    let rep = summoner.summon(5).await;
    println!("reply: {}", rep);
};

```

In this example, two background tasks are created, one for each method of `Subject`.
The `ping_pong` method's last parameter is a Sender, allowing us to "summon" the method.
