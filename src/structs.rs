struct Character {
    name: String,
    arms: u32,
    legs: u32,
    weapon: String,
    health: u32,
    stamina: u32,
    mana: u32,
}

struct float2(f32, f32);
struct float3(f32, f32, f32);
struct NoDataStruct;

// -----------------------

trait MyTrait {
    fn MyMethod(&self) -> &String;
}

struct MyStruct<T>
where
    T: MyTrait,
{
    name: String,
    trait_type: T,
}

impl<T> MyStruct<T>
where
    T: MyTrait,
{
    fn MyMethod(&self) -> &String {
        self.trait_type.MyMethod()
    }
}

// -----------------------

pub fn run() {
    let s = String::from("Hello world"); // static methods use ::
    let s_len = s.len(); // instance methods use .

    let loc1 = float2(1.0, 2.0);
    let loc2 = float3(1.0, 2.0, 3.0);
}
