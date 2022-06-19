pub struct Greeter {
    greeting: String,
}

/// Implementation of Greeter.
impl Greeter {
    pub fn new(greeting: &str) -> Greeter {
        Greeter {
            greeting: greeting.to_string(),
        }
    }

    pub fn greet(&self, thing: &str) {
        println!("{} {}", &self.greeting, thing);
    }
}
