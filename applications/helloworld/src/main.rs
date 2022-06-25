extern crate greeter_lib;
extern crate log;

use greeter_lib::greeter;

fn main() {
    env_logger::init();
    log::info!("About to say hello");
    let hello = greeter::Greeter::new("Hello");
    hello.greet("world");
}
