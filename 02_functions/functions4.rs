// This store is having a sale where if the price is an even number, you get 10
// Rustbucks off, but if it's an odd number, it's 3 Rustbucks off.
// Don't worry about the function bodies themselves, we are only interested in
// the signatures for now.

fn is_even<T>(num: T) -> bool
where
    T: Copy + From<i32> + std::ops::Rem<Output = T> + std::cmp::PartialEq,
{
    let two: T = 2.into();
    num % two == 0.into()
}

// TODO: Fix the function signature.
fn sale_price<T>(price: T) -> T
where
    T: Copy
        + std::ops::Rem<Output = T>
        + std::cmp::PartialEq
        + std::ops::Sub<Output = T>
        + From<i32>,
{
    let zero = T::from(0);
    let two = T::from(2);
    let discount = if price % two == zero {
        T::from(10)
    } else {
        T::from(3)
    };
    
    price - discount
}
fn main() {
    let original_price = 51;
    println!("Your sale price is {}", sale_price(original_price));
}
