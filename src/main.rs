use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
    let mut hash0 = vec![1,2,3];

    println!("{:?}", hash0);

    let mut hash1 = hash0.clone();
    hash1.push(4);
    println!("{:?}", hash0);
    println!("{:?}", hash1);
}