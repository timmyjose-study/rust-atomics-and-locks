use std::thread;

fn main() {
    let numbers = vec![1, 2, 3, 4, 5];

    thread::spawn(move || {
        for n in numbers {
            println!("{n}");
        }
    })
    .join()
    .unwrap();
}