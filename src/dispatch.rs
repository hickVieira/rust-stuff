struct Person {
    name: String,
}

impl Person {
    fn new(name: String) -> Self {
        Self { name }
    }
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

// -----------------------------------

fn generic_function<T>(trait_object: T)
where
    T: MyTrait,
{
    println!("This is a generic function {}", trait_object.MyMethod());
}

fn generic_function_shorthand1<T: MyTrait>(trait_object: T) {
    println!(
        "This is a generic function shorthand {}",
        trait_object.MyMethod()
    );
}

fn generic_function_shorthand2(trait_object: impl MyTrait) {
    println!(
        "This is a generic function shorthand {}",
        trait_object.MyMethod()
    );
}

// -----------------------------------

pub fn run() {
    let person = Person::new(String::from("John"));
    print_person(&person);
    print_named(&person);
}
