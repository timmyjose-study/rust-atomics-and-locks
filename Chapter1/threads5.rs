use std::thread;

fn main() {
    // outlives the scope `s`, so we can borrow from it
    let numbers = vec![1, 2, 3, 4, 5];

    thread::scope(|s| {
        s.spawn(|| {
            println!("length = {}", numbers.len());
        });

        s.spawn(|| {
            let s = numbers.iter().sum::<usize>();
            println!("sum = {s}");
        });
    });

    assert_eq!([1, 2, 3, 4, 5], &numbers[..]);
}