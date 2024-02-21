// functions5.rs
//
// Execute `rustlings hint functions5` or use the `hint` watch subcommand for a
// hint.

// I AM DONE

fn main() {
    let answer = square(3.0);
    println!("The square of 3 is {}", answer);
}

fn square(num: f32) -> f32 {
    (num * num)/2.0
}
