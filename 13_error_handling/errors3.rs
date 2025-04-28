// This is a program that is trying to use a completed version of the
// `total_cost` function from the previous exercise. It's not working though!
// Why not? What should we do to fix it?

use std::num::ParseIntError;

// Don't change this function.
fn total_cost(item_quantity: &str) -> Result<i32, ParseIntError> {
    let processing_fee = 1;
    let cost_per_item = 5;
    let qty = item_quantity.parse::<i32>()?;

    Ok(qty * cost_per_item + processing_fee)
}

// TODO: Fix the compiler error by changing the signature and body of the
// `main` function.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut tokens = 100;
    let pretend_user_input = "8";

    // Don't change this line.
    let cost = total_cost(pretend_user_input)?;

    if cost > tokens {
        println!("You can't afford that many!");
    } else {
        tokens -= cost;
        println!("You now have {tokens} tokens.");
    }
    Ok(())
}

/*
// Alternate solution

use std::num::ParseIntError;

fn total_cost(item_quantity: &str) -> Result<i32, ParseIntError> {
    let processing_fee = 1;
    let cost_per_item = 5;
    
    // Явная обработка Result с match вместо оператора ?
    let parse_result = item_quantity.parse::<i32>();
    let qty = match parse_result {
        Ok(n) => n,
        Err(e) => return Err(e),
    };
    
    Ok(qty * cost_per_item + processing_fee)
}

fn main() {
    let mut tokens = 100;
    let pretend_user_input = "8";
    
    // Обработка результата через match
    match total_cost(pretend_user_input) {
        Ok(cost) => {
            if cost > tokens {
                println!("You can't afford that many!");
            } else {
                tokens -= cost;
                println!("You now have {tokens} tokens.");
            }
        },
        Err(e) => {
            eprintln!("Error calculating cost: {}", e);
        }
    }
}
*/
