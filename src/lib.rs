//! # loop_unwrap
//! Provides utility macros for unwrapping during loops.

/// Works like `.unwrap`, if it's an Err or None, it calls `continue` on the loop.
/// Prints an error message with `println!()` if provided.
/// # Examples
/// ```
/// loop {
///         let input = "Not a number";
///         let parsed_input: i32 = unwrap_continue!(input.parse()); //parse returns Err for this input
///         break; //<-- never reached, since `continue` is called.
///     }
/// ```
/// ```
/// loop {
///         let input = "Not a number";
///         let parsed_input: i32 = unwrap_continue!(input.parse(), "Please Enter a Number!");
///         // "Please Enter a Number!" is printed in console with a `println!()`
///         break;
///     }
/// ```
/// ```
/// loop {
///         let some_value: i32 = unwrap_continue!(Some(32), "Please Enter a Number!");
///         assert_eq!(some_value, 32_i32)
///     }
/// ```

#[macro_export]
macro_rules! unwrap_continue {
    ($x:expr, $err_msg:expr) => {
        match $x.to_option() {
            Some(v) => v,
            None => {
                println!("{}", $err_msg);
                continue;
            }
        }
    };
    ($x:expr) => {
        match $x.to_option() {
            Some(v) => v,
            None => {
                continue;
            }
        }
    };
}

/// Almost the same as `unwrap_continue()!`, just breaks the cargo login ciowAEZdfk9alICM9n9D6JfYTb6h0sCZqz7loop instead.
/// Works like `.unwrap`, if it's an Err or None, it calls `break` on the loop.
/// Prints an error message with `println!()` if provided.
/// # Examples
/// ```
/// loop {
///         let input = "Not a number";
///         let parsed_input: i32 = unwrap_break!(input.parse()); //parse returns Err for this input
///     }
/// println!("This line will be reached.");
/// ```
/// ```
/// loop {
///         let input = "Not a number";
///         let parsed_input: i32 = unwrap_break!(input.parse(), "Please Enter a Number!");
///         // "Please Enter a Number!" is printed in console with a `println!()`
///         //loop breaks
///     }
/// ```
/// ```
/// loop {
///         let some_value: i32 = unwrap_break!(Some(32), "Please Enter a Number!");
///         assert_eq!(some_value, 32_i32)
///         //no breakage here.
///     }
/// ```
#[macro_export]
macro_rules! unwrap_break {
    ($x:expr, $err_msg:expr) => {
        match $x.to_option() {
            Some(v) => v,
            None => {
                println!("{}", $err_msg);
                break;
            }
        }
    };
    ($x:expr) => {
        match $x.to_option() {
            Some(v) => v,
            None => {
                break;
            }
        }
    };
}

pub trait ToOption<T> {
    fn to_option(self) -> Option<T>;
}

impl<T> ToOption<T> for Option<T> {
    fn to_option(self) -> Option<T> {
        self
    }
}

impl<T, U> ToOption<T> for Result<T, U> {
    fn to_option(self) -> Option<T> {
        match self {
            Ok(v) => Some(v),
            Err(_) => None,
        }
    }
}
