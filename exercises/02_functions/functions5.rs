// TODO: Fix the function body without changing the signature.
fn square(num: i32) -> i64 {
    num as i64 * num as i64
}

fn main() {
    let answer = square(3);
    println!("The square of 3 is {answer}");
}
