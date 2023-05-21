use std::thread;

fn main() {
    let x: &'static [i32; 5] = Box::leak(Box::new([1, 2, 3, 4, 5]));

    thread::spawn(move || dbg!(x)).join().unwrap();
    thread::spawn(move || dbg!(x)).join().unwrap();
}