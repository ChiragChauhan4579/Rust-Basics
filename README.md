# Rust 🦀

# basics_and_variables folder

## Starting a rust project

It is necessary to run `cargo init` so that all the necessary cargo files and one main.rs inside src folder will be created in the directory.

```
mkdir rust-project
cd rust-project
cargo init
```

## Variables 

You can define variables using the let keyword similar to JS. You can assign the type of the variable, or it can be inferred as well.

Also we can use variables directly while printing by usin them inside {}. Similar to f-string in python just that it does not require f.

## Data types

The data types in rust are similar to every other language. Most common integer, float, boolean.

There are char and string types in rust. Char holds a single character or part of a character while String holds a sequence of characters.

Like python we have tuple and array (list). tuple elements can be accessed by using . and array elements can be accessed by using [].

To print all values of tuple and array one must use :? to print. Example: println!("{arr:?}") or println!("{:?}",arr)

## Variable mutability
Variables are immutable by default. Prefix the variable name with mut keyword to make it mutable.

## Notes
- Variable declared without keyword mut will raise error if its value is tried to change
- Variable if not used anywhere will raise compilation warnings. The code will still work.
