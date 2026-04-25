use crate::garden::vegetables::{Asparagus, Mint};

pub mod garden;

fn main() {
    let plant = Mint {};
    println!("I'm growing {plant:?}!");
}
