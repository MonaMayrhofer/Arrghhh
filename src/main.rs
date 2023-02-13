#![feature(unboxed_closures)]
#![feature(fn_traits)]
#![feature(tuple_trait)]

pub mod aaa;

use aaa::{Arghh, HelloWorldNTimes};

fn main() {
    HelloWorldNTimes::O();
}
