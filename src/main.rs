extern crate playground;

use a::series::of;
use of::nested_modules;
use TrafficLight::*;

pub mod a {
    pub mod series {
        pub mod of {
            pub fn nested_modules() {}
        }
    }
}

enum TrafficLight {
    Red,
    Yellow,
    Green,
}

fn main() {
    playground::client::connect();

    nested_modules();

    let red = Red;
    let yellow = Yellow;
    let green = Green;
}
