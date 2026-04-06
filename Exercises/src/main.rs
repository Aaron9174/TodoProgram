/**
 * Memoization of the fibonacci problem
 */
// TODO: do this


/**
 * Regular iterative fibonacci sequence (tabulation)
 *
 * NOTE; this uses compile-time generic constant
 * NOTE2: Downside to doing it this way is the hardware dependency on usize
 * NOTE3: Also, this is stupid since the N is compile time, the +1 has to happen a tthe function
 * call rather than inside the function
 */
fn fibonacci_tabulation<const N: usize>() -> i32
{
    let mut fib_array: [i32; N] = [0; N];

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
    const start_num: u32 = 20;
    const start_num_special: usize = 20;
    let tab_result = fibonacci_tabulation::<{start_num_special+1}>();
    println!("tab result: {tab_result}");
    let recursive_result = fibonacci_recursive(start_num);
    println!("iterative result: {recursive_result}");
}
