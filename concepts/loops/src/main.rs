#[allow(unused_labels)]
fn returning_from_loop() -> u32 {
    let mut counter: u32 = 0;
    'return_loop: loop { // '<loop label>:
        counter += 1;
        if counter == 10 {
            break counter; // This is the return value
        }
    }
}

fn main() {
    println!("{}", returning_from_loop());
}
