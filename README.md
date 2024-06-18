# Asynchronous Deferred Calls

This crate adds a `listen` method to `Arc<RwLock<T>>` and other locking primitives.

The method takes a callback parameter and returns a [`Caller`].

The `listen` method creates a background task waiting for you to use the [`Caller`].
Whenever a call is dispatched, the locking primitive gets locked and the callback
gets called with the lock's content as well as what you sent.

The callback must:
- be asynchronous
- return `Result<(), String>`
- take either `&T` or `&mut T` as first parameter

if the callback's last parameter is an [`async_channel::Sender`], you can turn your [`Caller`] into
a [`Summoner`], which makes it easy to wait for a reply when you dispatch a call.

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

let deferred_print = world.clone().listen_mut_1(Subject::print);
let deferred_ping_pong = world.clone().listen_ref_2(Subject::ping_pong);

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
