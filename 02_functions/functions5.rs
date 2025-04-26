// TODO: Fix the function body without changing the signature.
fn square<T>(num: T) -> T
where
    T: Copy
    + std::ops::Mul<Output = T>
    + From<i32>,
{
    num * num
}

fn main() {
    let answer = square(3);
    println!("The square of 3 is {answer}");
}
