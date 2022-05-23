// main.rs


use rand::Rng;

mod sorting;

fn main() {
    let mut vec: Vec<i64> = Vec::with_capacity(39);
    for idx in 0..vec.capacity() {
        vec.push(rand::random());
        println!("{}", vec[idx]);
    }

    println!("{}", arr);

    sorting::print_sorting();
    println!("Hello, world!");
}
