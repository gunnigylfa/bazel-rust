extern crate greeter_lib;
extern crate log;
extern crate rand;

use greeter_lib::greeter;
use rand::Rng;

fn main() {
    env_logger::init();
    log::info!("About to say hello");
    let hello = greeter::Greeter::new("Hello");
    hello.greet("world");
}
