#![allow(dead_code)] // this line prevents compiler warnings

mod enums;
mod error_handling;
mod flow;
mod functions;
mod generics;
mod methods;
mod ownership;
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
    methods::run();
    ownership::run();
    structs::run();
    text::run();
    types::run();
    vectors::run();
}
