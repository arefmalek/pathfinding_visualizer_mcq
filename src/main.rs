// main.rs

use rand::Rng;

mod sorting;

fn main() {
    let capacity = 5;
    let mut vec: Vec<i64> = Vec::with_capacity(capacity);
    for idx in 0..vec.capacity() {
        vec.push(rand::thread_rng().gen_range(1..101));
        println!("{}", vec[idx]);
    }

    sorting::insertion_sort(&mut vec);

    println!("Array after sorting: ");

    for idx in 0..vec.capacity() {
        vec.push(rand::thread_rng().gen_range(1..101));
        println!("{}", vec[idx]);
    }

    println!("It works!");

}
