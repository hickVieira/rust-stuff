struct Animal {
    name: String,
    sound: String,
}

impl Animal {
    fn speak(&self) {
        println!("animal {} says {}", self.name, self.sound);
    }
}

fn methods() {
    
}
