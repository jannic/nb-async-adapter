use std::time::{Instant, Duration};

use nb_async_adapter::nb_async;


struct Waiter {
    until: Instant,
    count: u32,
}

impl Waiter {
    fn new(until: Instant) -> Self {
        Waiter {
            until,
            count: 0,
        }
    }

    fn wait(&mut self) -> nb::Result<(), ()> {
        self.count += 1;
        println!("Call #{}", self.count);
        if self.until > Instant::now() {
            Err(nb::Error::WouldBlock)
        } else {
            Ok(())
        }
    }
}

#[tokio::main]
async fn main() {
    println!("Hello, ");
    let until = Instant::now() + Duration::from_secs(3);
    let mut waiter = Waiter::new(until);
    nb_async!(waiter.wait()).await.unwrap();
    println!("world!");
}
