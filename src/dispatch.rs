struct Person {
    name: String,
}

trait MyTrait {
    fn MyMethod(&self) -> &String;
}

impl MyTrait for Person {
    fn MyMethod(&self) -> &String {
        &self.name
    }
}

// -----------------------------------

fn print_person(person: &Person) {
    println!("This is a raw object {}", person.name);
}

fn print_named(trait_object: &dyn MyTrait) {
    println!("This is a trait object {}", trait_object.MyMethod());
}

pub fn run() {
    let person = Person {
        name: String::from("John"),
    };
    print_person(&person);
    print_named(&person);
}
