#![allow(dead_code)] // this line prevents compiler warnings

mod boxing;
mod dispatch;
mod enums;
mod error_handling;
mod flow;
mod functions;
mod generics;
mod lifetimes;
mod math;
mod methods;
mod ownership;
mod pointers;
mod smart_pointers;
mod structs;
mod text;
mod traits;
mod types;
mod vectors;

fn main() {
    boxing::run();
    dispatch::run();
    enums::run();
    error_handling::run().unwrap();
    flow::run();
    functions::run();
    generics::run().unwrap();
    lifetimes::run();
    math::run();
    methods::run();
    ownership::run();
    pointers::run();
    smart_pointers::run();
    structs::run();
    text::run();
    traits::run();
    types::run();
    vectors::run();
}
