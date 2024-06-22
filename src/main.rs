#![allow(dead_code)] // this line prevents compiler warnings

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
mod structs;
mod text;
mod types;
mod vectors;

fn main() {
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
    structs::run();
    text::run();
    types::run();
    vectors::run();
}
