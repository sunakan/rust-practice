extern crate communicator;

pub mod a {
    pub mod series {
        pub mod of {
            pub fn nested_modules() {}
        }
    }
}

use a::series::of::nested_modules;

enum TrafficLight {
    Red,
    Yellow,
    Green,
}

//use TrafficLight::{Red, Yellow};
use TrafficLight::*;

fn main() {
    //===============================
    //nested_modules();
    //===============================
    let red = Red;
    let yellow = Yellow;
    let green = Green;
    //===============================
}
