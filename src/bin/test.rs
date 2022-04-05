use std::time::{Duration, Instant};

fn main() {
    let now = Instant::now();
    std::thread::sleep(Duration::from_secs(2));
    dbg!(now.elapsed().as_millis());
}