View Documentaion at https://docs.rs/loop_unwrap/  
Github Repository: https://github.com/Mrp1Dev/loop_unwrap

# loop_unwrap
Provides utility macros for unwrapping during loops.

Works like `.unwrap`, if it's an Err or None, it calls `continue` on the loop.
Prints an error message with `println!()` if provided.
Loop Label can be provided in any order besides the Result/Option being the first argument.
If loop label is proivded, the specified loop will be continued.
# Examples
```
loop {
    let input = "Not a number";
    let parsed_input: i32 = unwrap_continue!(input.parse()); //parse returns Err for this input
    break; //<-- never reached, since `continue` is called.
}
```
```
loop {
    let input = "Not a number";
    let parsed_input: i32 = unwrap_continue!(input.parse(), "Please Enter a Number!");
    // "Please Enter a Number!" is printed in console with a `println!()`
    break;
}
```
```
loop {
    let some_value: i32 = unwrap_continue!(Some(32), "Please Enter a Number!");
    assert_eq!(some_value, 32_i32)
}
```
```
'main: loop {
    loop {
        let n = unwrap_continue!("t".parse::<i32>(), "Couldn't parse, retrying main loop", 'main);
        break 'main; //<-- this line will never be reached, and main will go into an infinite loop
    }
    break; //<-- this line won't be reached, since 'main will be continued infinitely
}
```
