struct Person {
    name: String,
}

struct Car {
    name: String,
}

struct City {
    name: String,
    things: Vec<Box<dyn Thing>>,
}

trait Thing {
    fn name(&self) -> String;
}

impl Thing for Person {
    fn name(&self) -> String {
        self.name.clone()
    }
}

impl Thing for Car {
    fn name(&self) -> String {
        self.name.clone()
    }
}

pub fn run() {
    let person = Person {
        name: "John".to_string(),
    };
    
    let car = Car {
        name: "Ford".to_string(),
    };

    let city = City {
        name: "London".to_string(),
        things: vec![Box::new(person), Box::new(car)],
    };
}
