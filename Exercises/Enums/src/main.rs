fn main() {
    execute_coins_example();
    execute_option_match_example();
}

/********************************
 * Coin defs
 *******************************/

#[derive(Debug)]
enum UsState {
    Alaska,
    Florida,
    // etc
}

impl UsState {
    fn existed_in(&self, year: u16) -> bool {
        match self {
            UsState::Alaska => year >= 1959,
            UsState::Florida => year >= 1845,
            // -- Continue with states
        }
    }
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

struct CoinPouch {
    pennies: u32,
    nickels: u32,
    dimes: u32,
    quarters: u32,
}

/**
* Execute the coin example
*/
fn execute_coins_example() {
    let pouch = CoinPouch {
        pennies: 50,
        nickels: 10,
        dimes: 5,
        quarters: 2,
    };

    let total_val: u32 = value_in_cents(Coin::Penny) as u32 * pouch.pennies
        + value_in_cents(Coin::Nickel) as u32 * pouch.nickels
        + value_in_cents(Coin::Dime) as u32 * pouch.dimes
        + value_in_cents(Coin::Quarter(UsState::Florida)) as u32 * pouch.quarters;

    println!("Total value in coin pouch: {}", total_val);

    let mut display_res = describe_state_quarters(Coin::Quarter(UsState::Alaska));
    if let Some(str) = display_res {
        println!("{str}");
    }
    display_res = describe_state_quarters2(Coin::Quarter(UsState::Florida));
    if let Some(str) = display_res {
        println!("{str}");
    }
    display_res = describe_state_quarters3(Coin::Quarter(UsState::Florida));
    if let Some(str) = display_res {
        println!("{str}");
    }
}

/**
* Calculates the value in cents of the coin & displays the quarter state
*/
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("The state of the quarter is {:?}", state);
            25
        }
    }
}

/**
* Just using the let if here instead of the match
*
* NOTE: notice that the happy day case and rainy day cases are mixed together here
*/
fn describe_state_quarters(coin: Coin) -> Option<String> {
    if let Coin::Quarter(state) = coin {
        if state.existed_in(1900) {
            Some(format!("{state:?} is pretty old, for America!"))
        } else {
            Some(format!("{state:?} is relatively new."))
        }
    } else {
        None
    }
}

/**
* Just using the let if here
*
* NOTE: notice that the happy day case and rainy day cases are separate, this is better than before
* NOTE2: This is still annoying regardless though, lots of boilerplate here
*/
fn describe_state_quarters2(coin: Coin) -> Option<String> {
    let state = if let Coin::Quarter(state) = coin {
        state
    } else {
        return None;
    };

    if state.existed_in(1900) {
        Some(format!("{state:?} is pretty old, for America!"))
    } else {
        Some(format!("{state:?} is relatively new."))
    }
}

/**
* This is very clean, we keep the happy day and rainy day cases separate while also keeping the boilerplate to a minimum for max readability
*/
fn describe_state_quarters3(coin: Coin) -> Option<String> {
    let Coin::Quarter(state) = coin else {
        return None;
    };

    if state.existed_in(1900) {
        Some(format!("{state:?} is pretty old, for America!"))
    } else {
        Some(format!("{state:?} is relatively new."))
    }
}

fn execute_option_match_example() {
    let x = Some(5);
    let y = None;

    let mut result = add_one_optional(x);
    println!("result of add one: {}", result.unwrap());

    result = add_one_optional(y);
    println!("result y is none: {}", result.is_none());
}

fn add_one_optional(option: Option<i32>) -> Option<i32> {
    // Apparently this has an Optional::map standard implementation
    match option {
        None => None,
        Some(i) => Some(i + 1),
    }
}
