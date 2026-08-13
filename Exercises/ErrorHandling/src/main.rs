use std::{
    error::Error as StdError,
    fs::{self, File},
    io::{Error as IoError, ErrorKind, Read},
};

/**
* Main can also return a Result<(), E> as well! The main function is valid as long as it implements the
* 'std::process::Termination' trait. We can look more into this trait and implement in our own
* functions as well.
*
* Box<dyn Error> is a trait object that means any kind of error
*/
fn main() -> Result<(), Box<dyn StdError>> {
    match_error_handling();
    closure_error_handling();
    std_unwrap_example();
    let username_result: Result<String, IoError> = read_username_from_file_verbose();
    match username_result {
        Ok(username) => println!("username result: {}", username),
        Err(err) => panic!("Failed to obtain username: {err:?}"),
    }

    let mut username: String = read_username_from_file_concise().unwrap();
    println!("username result: {}", username);
    // These are allowed since return any error from the new main signature is allowed
    username = read_username_from_file_shortest()?;
    username = read_username_from_file_final()?;

    Ok(())
}

/**
* This uses match to show how to deal with error handling
*/
fn match_error_handling() {
    let greeting_file_result: Result<File, IoError> = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {e:>}"),
            },
            _ => {
                panic!("Problem opening the file: {error:?}");
            }
        },
    };
}

fn closure_error_handling() {
    let greeting_file_result: File = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {error:?}");
            })
        } else {
            panic!("Problem opening the file: {error:?}");
        }
    });
}

fn std_unwrap_example() {
    let greeting_file = File::open("hello.txt").unwrap();
}

fn read_username_from_file_verbose() -> Result<String, IoError> {
    let file_result: Result<File, IoError> = File::open("hello.txt");

    let mut file: File = match file_result {
        Ok(file) => file,
        Err(err) => return Err(err),
    };

    let mut username = String::new();

    match file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

/**
* Same implementation as the verbose one but using the '?' operator
*
* '?' operator
* OK(res) => The res is returned from the expression
* Err(e) => The err is returned from the whole function (as if the return word was used)
*
* There is difference between the two implementations, the match implementation does not go through
* the 'from function' defined in the From trait in the standard library. The '?' operator uses this 'from function'
* to convert values from one type into another, in this case to convert the recieved error type to
* the functions return value error type. So the latter produces a single error type, while the
* former preserves different error types.
*
* NOTE: this also means that io::Error implements this From trait
* NOTE2: The operator can also be used on Option<T> type
*/
fn read_username_from_file_concise() -> Result<String, IoError> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

/**
* Utilize the '?' operator twice here to be able to unwrap result
*/
fn read_username_from_file_shortest() -> Result<String, IoError> {
    let mut username = String::new();

    File::open("hello.txt")?.read_to_string(&mut username)?;

    Ok(username)
}

/*
* Standard library gives us this fairly common functionality
*/
fn read_username_from_file_final() -> Result<String, IoError> {
    fs::read_to_string("hello.txt")
}

/**
* Use the '?' operator with Option<T> type
*
* What this does:
* - Takes a string slice to some text
* - lines() returns an iterator over the lines of the string
* - next() returns an Option<String>, since the first could be empty, if it is None returns out,
*   otherwise the Some(String) value is returned
* - chars() gets the characters from that string
* - last() returns the final character of the string
*/
fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}
