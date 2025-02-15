// errors3.rs
//
// This is a program that is trying to use a completed version of the
// `total_cost` function from the previous exercise. It's not working though!
// Why not? What should we do to fix it?
//
// Execute `rustlings hint errors3` or use the `hint` watch subcommand for a
// hint.

use std::num::ParseIntError;

fn main() -> Result<(), ParseIntError> {                            // The main function then checks if the cost is greater than the available tokens. If it is, 
    let mut tokens = 100;                                      // it prints a message indicating that the user can't afford that many items. If it isn't,
    let pretend_user_input = "8";                             // it subtracts the cost from the tokens and prints the remaining tokens.

    let cost = total_cost(pretend_user_input)?; // The main function is defined to return a Result<(), ParseIntError> type. This means tha if
                                                                   // everything goes well and no erros are ecnountered, it will return an Ok(()). If a ParseIntError 
    if cost > tokens {                                             // irt will be returned from the main function. PADRAO COMUN ENCONTRADO EM RUST     
        println!("You can't afford that many!");
    } else {
        tokens -= cost;
        println!("You now have {} tokens.", tokens);
    }

    Ok(())
}

pub fn total_cost(item_quantity: &str) -> Result<i32, ParseIntError> {
    let processing_fee = 1;
    let cost_per_item = 5;
    let qty = item_quantity.parse::<i32>()?;

    Ok(qty * cost_per_item + processing_fee)
}
