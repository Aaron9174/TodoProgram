use std::time::{Instant};
use std::collections::HashMap;

/**
 * Tabularization with space efficiencies added, this has the increase
 *
 * Theoretical
 * ===========
 * Time complexity: O(n)
 * Space complexity: O(1)
 *
 * Actual
 * ======
 * Sample> N=20
 * Time: 292ns
 * Data size: usize * 2
 *  => 8 bytes on 32-bit machines
 *  => 16 bytes on 64-bit machines
 */
fn fibonacci_golden_goose(n: usize) -> u32
{
    const ARR_SIZE: usize = 2;
    let mut fib_array: [u32; ARR_SIZE] = [0, 1];

    let mut i: usize = 2;
    let mut index: usize = 0;
    while i <= n
    {
        index = i % ARR_SIZE;
        fib_array[index] = fib_array[(i-1) % ARR_SIZE] + fib_array[(i-2) % ARR_SIZE];
        i += 1;
    }

    return fib_array[index];
}

/**
 * Memoization of the fibonacci problem
 *
 * Theoretical
 * ===========
 * Time complexity: O(n)
 * Space complexity: O(n)
 *
 * Actual
 * ======
 * Sample> N=20
 * Time: 2.60ms (WOW)
 * Data size: ~496 bytes on a 64-bit machine
 *  => 48 byte stack handle
 *  => 256 byte heap data (32 buckets x 4 bytes in the map itself)
 *      => Map size allocations happen in powers of 2 so 2^5 = 32 is the smallest value to occupy
 *         21 map values
 *  => 32 bytes (1 control byte for each bucket)
 *  => 16 or 32 bytes of padding added to the end of SIMD padding
 *
 * NOTE: this is even slower since the hash map in rust is stupid slow
 */
fn fibonacci_memoization(n: u32, memo: &mut HashMap<u32, u32>) -> u32
{
    if Some(&n) == memo.get(&n) {
        return n;
    }

    let result = match n {
        0 => 0,
        1 => 1,
        _ => fibonacci_memoization(n-1, memo) + fibonacci_memoization(n-2, memo),
    };

    memo.insert(n, result);
    return result;
}


/**
 * Regular iterative fibonacci sequence (tabulation)
 * 
 * Theoretical
 * ===========
 * Time complexity: O(n)
 * Space complexity: O(n)
 *
 * Actual
 * ======
 * Sample> N=20
 * Time: 209ns
 * Space: 80 bytes (20 array entries of u32)
 *
 * NOTE; this uses compile-time generic constant
 * NOTE2: Downside to doing it this way is the hardware dependency on usize
 * NOTE3: Also, this is stupid since the N is compile time, the +1 has to happen a tthe function
 * call rather than inside the function
 */
fn fibonacci_tabulation<const N: usize>() -> u32
{
    let mut fib_array: [u32; N] = [0; N];

    fib_array[0] = 0;
    fib_array[1] = 1;

    let mut i: usize = 2;
    while i < N
    {
        fib_array[i] = fib_array[i-1] + fib_array[i-2];
        i += 1;
    }

    return fib_array[N-1];
}

/**
 * Regular recursive fibonacci sequence 
 *
 * Theoretical
 * ===========
 * Time complexity: O(2^n)
 * Space complexity: O(n)
 *
 * Actual
 * ======
 * Sample> N=20
 * Time: 71.875 microseconds
 * Space: 20 stack frames (the lowest possible value is 160 bytes, the highest can be in the
 * kilobyte range)
 *
 */
fn fibonacci_recursive(n: u32) -> u32
{
    if n < 2 {
        return n;
    } else {
        return fibonacci_recursive(n-1) + fibonacci_recursive(n-2);
    }
}

fn main() {
    const START_NUM: u32 = 20;
    // TODO: fix this, it bothers me
    const START_NUM_SPECIAL: usize = 20;

    let mut now = Instant::now();
    let tab_result = fibonacci_tabulation::<{START_NUM_SPECIAL+1}>();
    let mut later = Instant::now();
    println!("tabulation duration: {:?}", later.duration_since(now));
    println!("tab result: {tab_result}");

    now = Instant::now();
    let recursive_result = fibonacci_recursive(START_NUM);
    later = Instant::now();
    println!("recursive duration: {:?}", later.duration_since(now));
    println!("recursive result: {recursive_result}");

    now = Instant::now();
    let mut memo: HashMap<u32, u32> = HashMap::new();
    let memo_result = fibonacci_memoization(START_NUM, &mut memo);
    later = Instant::now();
    println!("memo duration: {:?}", later.duration_since(now));
    println!("memo result: {memo_result}");

    now = Instant::now();
    let golden_goose_result = fibonacci_golden_goose(START_NUM_SPECIAL);
    later = Instant::now();
    println!("golden goose duration: {:?}", later.duration_since(now));
    println!("golden goose result: {golden_goose_result}");

}
