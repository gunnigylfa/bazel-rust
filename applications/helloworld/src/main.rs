extern crate greeter_lib;

use greeter_lib::greeter;

fn main() {
    let hello = greeter::Greeter::new("Hello");
    hello.greet("world");
}
