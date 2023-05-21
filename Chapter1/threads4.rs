use std::{iter::FromIterator, thread};

fn main() {
    let numbers = Vec::from_iter(0..=1000);

    let average = thread::spawn(move || {
        let n = numbers.len();
        let s = numbers.into_iter().sum::<usize>();

        s / n
    })
    .join()
    .unwrap();

    println!("Average = {average}");
}