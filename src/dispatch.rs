struct Person {
    name: String,
}

impl Person {
    fn new(name: String) -> Self {
        Self { name }
    }
}

trait Named {
    fn name(&self) -> &String;
}

impl Named for Person {
    fn name(&self) -> &String {
        &self.name
    }
}

// -----------------------------------

fn print_person(person: &Person) {
    println!("This is {}", person.name);
}

fn print_named(named: &dyn Named) {
    println!("This is a trait object, named {} as a Named", named.name());
}

pub fn run() {
    let person = Person::new(String::from("John"));
    print_person(&person);
    print_named(&person);
}
