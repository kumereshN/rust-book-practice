use crate::garden::vegetables::Asparagus;

pub mod vegetables;

fn main() {
    let plant = Asparagus {};
    println!("I'm growing {:?}!", plant);
}
