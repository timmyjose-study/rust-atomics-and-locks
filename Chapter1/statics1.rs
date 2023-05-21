use std::thread;

static X: [i32; 5] = [1, 2, 3, 4, 5];

fn main() {
    thread::spawn(|| {
        dbg!(X);
    })
    .join()
    .unwrap();

    thread::spawn(|| {
        dbg!(&X);
    })
    .join()
    .unwrap();
}