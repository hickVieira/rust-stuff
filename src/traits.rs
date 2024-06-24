// data
struct Person {
    name: String,
    age: u8,
}

// traits
trait Named {
    fn name(&self) -> &String;
}

trait Aged {
    fn age(&self) -> u8;
}

trait Human: Named + Aged {
    fn species(&self) -> &'static str {
        "Homo sapiens"
    }

    fn description(&self) -> String {
        format!("{} ({} years old)", self.name(), self.age())
    }
}

// implementations
impl Named for Person {
    fn name(&self) -> &String {
        &self.name
    }
}

impl Aged for Person {
    fn age(&self) -> u8 {
        self.age
    }
}

impl Human for Person {}

// types of dispatchs
fn static_dispatch<T: Human>(item: T) {}

fn dynamic_dispatch(item: &dyn Human) {}

// usage
pub fn run() {
    let person = Person {
        name: String::from("John"),
        age: 30,
    };

    println!("Name: {}", person.name());
    println!("Age: {}", person.age());
    println!("Species: {}", person.species());
    println!("Description: {}", person.description());
}
