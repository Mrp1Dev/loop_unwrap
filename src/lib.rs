//! # loop_unwrap
//! Provides utility macros for unwrapping during loops.

/// Works like `.unwrap`, if it's an Err or None, it calls `continue` on the loop.
/// Prints an error message with `println!()` if provided.
/// Loop Label can be provided in any order besides the Result/Option being the first argument.
/// If loop label is proivded, the specified loop will be continued.
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
/// ```
/// 'main: loop {
///         loop {
///             let n =
///                 unwrap_continue!("t".parse::<i32>(), "Couldn't parse, retrying main loop", 'main);
///             break 'main; //<-- this line will never be reached, and main will go into an infinite loop
///         }
///         break; //<-- this line won't be reached, since 'main will be continued infinitely
///     }
/// ```
#[macro_export]
macro_rules! unwrap_continue {
    ($x:expr) => {
        match $x.to_option() {
            Some(v) => v,
            None => {
                continue;
            }
        }
    };
    ($x:expr, $label:lifetime) => {
        match $x.to_option() {
            Some(v) => v,
            None => {
                continue $label;
            }
        }
    };
    ($x:expr, $label:lifetime, $err_msg:expr) => {
        match $x.to_option() {
            Some(v) => v,
            None => {
                println!("{}", $err_msg);
                continue $label;
            }
        }
    };
    ($x:expr, $err_msg:expr) => {
        match $x.to_option() {
            Some(v) => v,
            None => {
                println!("{}", $err_msg);
                continue;
            }
        }
    };
    ($x:expr, $err_msg:expr, $label:lifetime) => {
        match $x.to_option() {
            Some(v) => v,
            None => {
                println!("{}", $err_msg);
                continue $label;
            }
        }
    };



}

/// Works like `.unwrap`, if it's an Err or None, it calls `break` on the loop.
/// Prints an error message with `println!()` if provided.
/// Loop Label can be provided in any order besides the Result/Option being the first argument.
/// If loop label is proivded, the specified loop will be break;-ed.
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
/// ```
/// 'main: loop {
///        loop {
///            let n = unwrap_break!("t".parse::<i32>(), "Couldn't parse, exiting main loop", 'main);
///            break; //<-- this line won't be reached.
///        }
///        println!("This line will never be reached, because 'main breaks.");
///    }
/// ```   
#[macro_export]
macro_rules! unwrap_break {
    ($x:expr) => {
        match $x.to_option() {
            Some(v) => v,
            None => {
                break;
            }
        }
    };
    ($x:expr, $label:lifetime) => {
        match $x.to_option() {
            Some(v) => v,
            None => {
                break $label;
            }
        }
    };
    ($x:expr, $label:lifetime, $err_msg:expr) => {
        match $x.to_option() {
            Some(v) => v,
            None => {
                println!("{}", $err_msg);
                break $label;
            }
        }
    };
    ($x:expr, $err_msg:expr) => {
        match $x.to_option() {
            Some(v) => v,
            None => {
                println!("{}", $err_msg);
                break;
            }
        }
    };
    ($x:expr, $err_msg:expr, $label:lifetime) => {
        match $x.to_option() {
            Some(v) => v,
            None => {
                println!("{}", $err_msg);
                break $label;
            }
        }
    };
}

/// Works only on Result enum. If the value is Err(e), breaks the loop returning Err(e).
/// Otherwise, it unwraps and the code continues.
/// Supports loop labels.
/// # Examples
/// ```
/// let value = loop {
///        let s = "not a number";
///        let n = unwrap_break_err!(s.parse::<i32>(), "Couldn't parse number."); // breaks with error value
///        break Ok(n + 1); //<-- this line will never be reached since the macro breaks
///    };
///    assert_eq!(true, value.is_err());
/// ```
/// ```
/// let result = 'main: loop {
///         loop {
///             let n = unwrap_break_err!("t".parse::<i32>(), 'main);
///             break 'main Ok(100);
///         }
///     };
///     assert_eq!(result.is_err(), true);
/// ```    
#[macro_export]
macro_rules! unwrap_break_err {
    ($x:expr) => {
        match $x {
            Ok(v) => v,
            Err(e) => {
                break Err(e);
            }
        }
    };
    ($x:expr, $label:lifetime) => {
        match $x {
            Ok(v) => v,
            Err(e) => {
                break $label Err(e);
            }
        }
    };
    ($x:expr, $label:lifetime, $err_msg:expr) => {
        match $x {
            Ok(v) => v,
            Err(e) => {
                println!("{}", $err_msg);
                break $label Err(e);
            }
        }
    };
    ($x:expr, $err_msg:expr) => {
        match $x {
            Ok(v) => v,
            Err(e) => {
                println!("{}", $err_msg);
                break Err(e);
            }
        }
    };
    ($x:expr, $err_msg:expr, $label:lifetime) => {
        match $x {
            Ok(v) => v,
            Err(e) => {
                println!("{}", $err_msg);
                break $label Err(e);
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
