# Rust 🦀

According to WikiPedia: Rust is a multi-paradigm, general-purpose programming language that emphasizes performance, type safety, and concurrency. It enforces memory safety—meaning that all references point to valid memory—without a garbage collector. To simultaneously enforce memory safety and prevent data races, its "borrow checker" tracks the object lifetime of all references in a program during compilation.

Rust was influenced by ideas from functional programming, including immutability, higher-order functions, and algebraic data types. It is popular for systems programming.

This repository is created as a learning tutorials by following various youtube videos and rust docs.


# Navigate to:
1. [Basics and Variable Folder](https://github.com/ChiragChauhan4579/Rust-Basics/blob/main/README.md#basics_and_variables-folder)
2. [Functions Folder](https://github.com/ChiragChauhan4579/Rust-Basics/blob/main/README.md#functions-folder)
3. [Ownership Folder](https://github.com/ChiragChauhan4579/Rust-Basics/blob/main/README.md#ownership-folder)
4. [Reference and Borrowing Folder](https://github.com/ChiragChauhan4579/Rust-Basics/blob/main/README.md#reference_and_borrowing-folder)

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

There are char and string types in rust. Char holds a single character or part of a character while String holds a sequence of characters. To define charater we use '' and for string we use "".

Like python we have tuple and array (list). tuple elements can be accessed by using . and array elements can be accessed by using [].

To print all values of tuple and array one must use :? to print. Example: println!("{arr:?}") or println!("{:?}",arr)

## Variable mutability
Variables are immutable by default. Prefix the variable name with mut keyword to make it mutable.

## Notes
- Variable declared without keyword mut will raise error if its value is tried to change
- Variable if not used anywhere will raise compilation warnings. The code will still work.

## Unused variables

When there are are unused variables, warnings are shown. In case we have two solutions.
1. add _ before the variable
2. add #[allow(unused_variables)] at the top of code

## Shadowing and Rebinding

You can declare a new variable with the same name as a previous variable, here we can say the first one is shadowed by the second one.

When we shadow a variable it is necessary to add mut keyword again so that it is still mutable.

## Checking variable type

To check variable type we use std::any::type_name crate and write a function for it.

## Check memory size 

TO check size use size_of_val from std::mem crate.

# functions folder

One must have understood the use of fn keyword and main function which is the entry point of any rust program. Now we will look at how to call another function from our main function.

In rust when we pass argument value we can define its specific type as well as the returning type to avoid any errors in the system.

## Statement and expression

Statement and expression are two important things in function. Statement are used when you do not have produce a value and only run a piece of code. They end with ;. All the code lines are statements. Expression is used to produce a resultant value. Can be used to return value from one function to another. Does not end with ; or anything.

So when you want to print something out of a statement you can give return type as () by using `fn func() -> ()` and also put a semicolon after the print statement.

But if you want to return a value don't put semicolon after that particular value.

In the code I have shown a code with function sum. It takes x and y as i32 and returns a i32 data type. If you pass some other value there then it will throw an error.

## Note:
1. To return a specific data type from function use -> datatype.
2. Don't end the value with semicolon if it to be returned.
3. Ensure return type and returned value to be of same type. 
4. An evaluating value cannot be returned. Example if you try to write x = x + 2 as an return value it would return (). So you have to write that as an statement ending with semicolon and give x as an expression in the end. 

# Ownership Folder

Ownership in Rust is a system that manages memory without a garbage collector or manual allocation. Rust’s compiler enforces rules that ensure memory safety, and if these rules are broken, the program won't compile.

Memory in Rust is divided into the stack and the heap. The stack stores values in a last-in, first-out manner with fixed sizes, while the heap stores data dynamically, using pointers.

Ownership tracks which parts of the code use heap data, minimizes duplicate data, and cleans up unused data. Understanding ownership simplifies memory management in Rust, reducing the need to worry about the stack and heap directly.

Ownership Rules:

1. Each value in Rust has an owner.
2. There can only be one owner at a time.
3. When the owner goes out of scope, the value will be dropped.

In earlier programs we saw the integer, float and boolean data types, they all are of fixed size and therefore stored on stack and does not require to implement ownership rules but with String we might not what is the length of string where user input might be needed etc. So here ownership rules need to be taken care of.

When creating a string type variable it would be now essential to use String::from. This String type would have 2 values: a pointer and a memory which holds its values. Now if we create another variable and use variable_b = variable_a, then the pointer of variable_a is moved to variable_b and the vairbale_a will no longer be valid. To avoid this we can use clone fucntion.

For stack based data clonining does not need to happen as the data is of fixed size and can be copied to another variale easily and this is what we call as copy.

Taking ownership and then returning ownership with every function is very complicated. Actually we can even do these things without transferring ownerships by using reference. We will see that in next project.

# Reference and Borrowing Folder

A reference is like a pointer in that it’s an address we can follow to access the data stored at that address; that data is owned by some other variable. Unlike a pointer, a reference is guaranteed to point to a valid value of a particular type for the life of that reference. The opposite of referencing by using & is dereferencing, which is accomplished with the dereference operator, *.

Reference can be used to pass the value of something without transferring the ownership. In the code example we pass &s1 to the calculate_length function (not we need to define the type as &String here), this will pass the pointer reference to the fucntion instead of transfering the ownership to it. When functions have references as parameters instead of the actual values, we won’t need to return the values in order to give back ownership, because we never had ownership. This is what we call as borrow.

When we need to pass mutable reference we can give it as &mut data_type. Here if you want to return some value make sure to return the same type &mut data_type.

Referencing rules in terms of mutability:
1. At a given time, there can only be a single mutable reference or any number of immutable references.
2. Multiple mutable references are not allowed.
3. Mutable variable can be borrowed as immutable but immutable cannot be borrowed as mutable.
4. References must always be valid.

The benefit of having this restriction is that Rust can prevent data races at compile time. A data race is similar to a race condition and happens when these three behaviors occur:

* Two or more pointers access the same data at the same time.
* At least one of the pointers is being used to write to the data.
* There’s no mechanism being used to synchronize access to the data.

Data races cause undefined behavior and can be difficult to diagnose and fix when you’re trying to track them down at runtime; Rust prevents this problem by refusing to compile code with data races!

A reference’s scope starts from where it is introduced and continues through the last time that reference is used. Therefore in the example when we create mutable reference after immutable reference it is still getting accepted.

Dangling pointer - A pointer that references a location in memory that may have been given to someone else—by freeing some memory while preserving a pointer to that memory. A example of this is when you return a reference of some value from one function to main function, when the other function ends the scope of the variables also end and their memory are freed. This will cause the returned reference to hold an invalid value. Rust does not allow this and will cause a direct error at time of compiling. Simply pass the value that needs to be returned and not the reference to it.