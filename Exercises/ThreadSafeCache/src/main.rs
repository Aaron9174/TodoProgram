use std::collections::HashMap;

const DEFAULT_VAL: &str = "DEFAULT";

/**
* Problem: The Thread-Safe Cache
*
* Objective: Implement a function get_or_insert_default that takes a mutable reference to a string map (HashMap<String, String>) and a key (&str).
* If the key exists, return a shared reference (&str) to the value. If it doesn't, insert a default value and return a reference to that newly inserted value.
*/
fn main() {
    let mut cache = HashMap::new();

    let mut key = "test";
    let mut insert_val = get_or_insert_default(&mut cache, key);
    println!(
        "Newly inserted (key,value) pair (solution): {}, {}",
        key, insert_val
    );
    insert_val = get_or_insert_default_best(&mut cache, key);
    println!(
        "Newly inserted (key,value) pair (best): {}, {}",
        key, insert_val
    );
    let mut get_val = get_or_insert_default(&mut cache, key);
    println!(
        "Retrieved value (key,value) pair (solution): {}, {}",
        key, get_val
    );
    get_val = get_or_insert_default_best(&mut cache, key);
    println!(
        "Retrieved value (key,value) pair (best): {}, {}",
        key, get_val
    );

    key = "key2";
    insert_val = get_or_insert_default(&mut cache, key);
    println!(
        "Newly inserted (key,value) pair (solution): {}, {}",
        key, insert_val
    );
    insert_val = get_or_insert_default_best(&mut cache, key);
    println!(
        "Newly inserted (key,value) pair (best): {}, {}",
        key, insert_val
    );
    get_val = get_or_insert_default(&mut cache, key);
    println!(
        "Retrieved value (key,value) pair (solution): {}, {}",
        key, get_val
    );
    get_val = get_or_insert_default_best(&mut cache, key);
    println!(
        "Retrieved value (key,value) pair (best): {}, {}",
        key, get_val
    );
}

/**
 * This solves the borrower problem! The possible_val is a shared borrow (regular reference) from
* the cache, this falls out of scope after the is_none() is used. Then we get a mutable reference
* for the insert itself. Finally, we get another shared borrow reference from the cache and return
* it as a string slice.
*
* This is however, very ineffecient. There is three hits to the map, is there a way to make this
* better?
 */
fn get_or_insert_default<'a>(cache: &'a mut HashMap<String, String>, key: &str) -> &'a str {
    let possible_val: Option<&String> = cache.get(key);
    if possible_val.is_none() {
        cache.insert(String::from(key), String::from(DEFAULT_VAL));
    }

    cache.get(key).unwrap().as_str()
}

/**
* The cleanest solution uses the maps API to help with checking a key and inserting a value if
* doesn't exist
*/
fn get_or_insert_default_best<'a>(cache: &'a mut HashMap<String, String>, key: &str) -> &'a str {
    cache
        .entry(key.to_string())
        .or_insert_with(|| DEFAULT_VAL.to_string())
        .as_str()
}
