fn main() {
    example1();
    example2();
    example3();
}

fn example1() {
    let s1 = String::from("abcd");
    let s2 = "xyz";

    let result = longest(s1.as_str(), s2);
    println!("The longest string is {result}");
}

fn example2() {
    // longer lifetime
    let s1 = String::from("long string is long");

    {
        // Shorter lifetime
        let s2 = String::from("xyz");
        // 'a is is the lifetime of s2 here, the return value is valid until the end of this scope as well
        let result = longest(s1.as_str(), s2.as_str());
        println!("The longest string is {result}");
    }
}

/// This example shows that the structs lifetime is based on the lifetime of the reference passed into it's part portion
struct ImportantExcerpt<'a> {
    part: &'a str,
}
fn example3() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split(".").next().unwrap();
    let i = ImportantExcerpt {
        part: first_sentence,
    };

    println!("First sentence: {}", i.part);
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }

    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {announcement}");
        self.part
    }
}

//////////////////////////////////////////////////////////////////////
//
// This doesn't compile the return result has the lifetime of 'a which
// has the lifetime of the shortest shared scope of the parameters passed
// in. In this, case that would be s2's lifetime
//
// fn badExample() {
//  let s1 = String::from("long string is long");
//  let result;
//   {
//      let s2 = String::from("xyz");
//      result = longest(s1.as_str(), s2.as_str());
//  }
//  println!("The longest string is {result}");
// }
//////////////////////////////////////////////////////////////////////

//////////////////////////////////////////////////////////////////////
// This function fails because it is missing a lifetime specifier   //
//                                                                  //
// fn longest(str1: &str, str2: &str) -> &str {                     //
//     if str1.len() > str2.len() { str1 } else { str2 }            //
// }                                                                //
//////////////////////////////////////////////////////////////////////

/// 'a is the shortest lifetime of the passed in parameters
fn longest<'a>(str1: &'a str, str2: &'a str) -> &'a str {
    if str1.len() > str2.len() { str1 } else { str2 }
}

/// This function shows that the lifetime of the first parameter and the return
/// reference are the same, the second parameter lifetime is independent of the
/// returned reference's lifetime
fn relationshipFn<'a>(str1: &'a str, str2: &str) -> &'a str {
    str1
}
