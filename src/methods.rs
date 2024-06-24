struct Animal {
    pub name: String,
    sound: String,
}

impl Animal {
    pub fn speak(&self) {
        println!("animal {} says {}", self.name, self.sound);
    }
}

pub fn run() {
    let animal = Animal {
        name: String::from("cat"),
        sound: String::from("meow"),
    };

    animal.speak();
}
