/**
 * The Collatz Sequence is defined as follows, for an arbitrary n1 greater than zero:
 •   If ni is 1, then the sequence terminates at ni.
 •   If ni is even, then ni+1 = ni / 2.
 •   If ni is odd, then ni+1 = 3 * ni + 1.
 */
fn collatz_length(n: i32) -> u32 {
    let mut collatz_list: Vec<i32> = Vec::new();
    collatz_list.push(n);

    let mut most_recent_item: usize = collatz_list.len() - 1;
    while collatz_list[most_recent_item] != 1 {
        most_recent_item = collatz_list.len() - 1;
        let n_temp: i32 = collatz_list[most_recent_item];

        if n_temp % 2 == 0 {
            collatz_list.push(n_temp / 2);
        } else {
            collatz_list.push(3 * n_temp + 1);
        }
    }

    return (collatz_list.len() as u32) - 1;
}

// Actual solution, less overhead
fn solution(mut n: i32) -> u32 {
    let mut len = 1;
    while n > 1 {
        n = if n % 2 == 0 { n / 2 } else { n * 3 + 1 };
        len += 1;
    }
    len
}

fn main() {
    println!("Length: {}", collatz_length(11));
    println!("Answer Length: {}", solution(11));
}
