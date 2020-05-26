mod default_structures;

use crate::default_structures::pokemon;

fn main() {
    let b = pokemon::bulbasur();
    println!("{}", b.name);
    println!();
}
