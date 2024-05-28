use fps_clock::*;
use std::io::Write;

fn main() {
    let mut fps_clock = FpsClock::new(60);
    loop {
        let i = fps_clock.tick();
        print!("\rTime since last tick (in secs): {:?}", i / 1000000000.);
        std::io::stdout().flush().unwrap();
    }
}
