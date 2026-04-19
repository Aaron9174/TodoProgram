fn main() {

    // While loop example
    let mut x = 200;
    while x >= 10 {
        x = x / 2;
    }
    dbg!("while output {}:", x);

    // Example of for loops iterating of a 
    for x in 1..5 {
        dbg!(x);
    }

    // Example of looping with 
    let mut i = 0;
    loop {
        i += 1;
        if i > 5 {
            break;
        }
        if i % 2 == 0 {
            continue;
        }
        dbg!(i);
    }
}
