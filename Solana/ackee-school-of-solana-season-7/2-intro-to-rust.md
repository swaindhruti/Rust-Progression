# Session-2: Introduction to Rust

## What is Rust?
ust is a modern systems programming language focusing on safety, speed, and concurrency. It accomplishes these goals by being memory safe without using garbage collection.Rust is a statically and strongly typed systems programming language. Statically means that all types are known at compile-time, strongly means that these types are designed to make it harder to write incorrect programs. The big difference from C and C++ is that Rust is safe by default. All memory accesses are checked. It is not possible to corrupt memory by accident.

## Data Types in Rust
Rust has several built-in data types, including:
- **Scalar Types:** Represent a single value. Includes integers, floating-point numbers, booleans, and characters.
- **Compound Types:** Combine multiple values into one. Includes tuples and arrays.
- **String Types:** Represent a sequence of characters. Includes `String` (heap-allocated) and `&str` (string slice).
- **Option Type:** Represents an optional value that can be either `Some(value)` or `None`. This is used for error handling and to avoid null references.
- **Result Type:** Represents a value that can be either `Ok(value)` or `Err(error)`. This is used for error handling in functions that can fail.
- **Structs:** Custom data types that allow you to create complex data structures with named fields.
- **Enums:** Define a type that can be one of several variants. Useful for representing different states or types of data.
- **Traits:** Define shared behavior that can be implemented by different types. Similar to interfaces in other languages.

- Use of `let` keyword to declare variables.
- Variables are immutable by default, but can be made mutable using the `mut` keyword.
- Used of `const` keyword to declare constants.

## Shadowing
Shadowing allows you to declare a new variable with the same name as a previous variable, effectively replacing it. This can be useful for changing the type of a variable or for reusing variable names in a limited scope. Shadowing is different from mutability; it creates a new variable rather than modifying the existing one. For example:
```rust
let x = 5;
let x = x + 1; // This shadows the previous x, creating a new variable
println!("{}", x); // Prints 6
```

## Structs and Enums
Structs and enums are used to create custom data types in Rust.
- **Structs:** Used to create complex data structures with named fields. For example:
```rust
struct Person {
    name: String,
    age: u32,
}
let person = Person {
    name: String::from("Alice"),
    age: 30,
};
```
- **Enums:** Define a type that can be one of several variants. Useful for representing different states or types of data. For example:
```rust
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
let move_direction = Direction::Up;
```

## Functions
Functions in Rust are defined using the `fn` keyword. They can take parameters and return values. Rust supports both named and anonymous functions (closures). Here's an example of a simple function:
```rust
fn add(x: i32, y: i32) -> i32 {
    x + y
}
let result = add(5, 3);
println!("The sum is: {}", result); // Prints "The sum is: 8"

// anonymous function (closure)
let add_closure = |x: i32, y: i32| x + y
let closure_result = add_closure(5, 3);
println!("The closure sum is: {}", closure_result); // Prints "The closure sum is: 8"
```

## Expressions & Statements
In Rust, expressions are fundamental building blocks of the language. Expression is a piece of code that evaluates to a resultant value. Calling a function is an expression. Calling a macro is an expression. A new scope block created with curly brackets is an expression, for example:
```rust
let x = {
    let y = 5;
    y + 1 // This is an expression that evaluates to 6
};
println!("The value of x is: {}", x); // Prints "The value of x is
: 6"
```
This block evaluates to 4. Expressions do not include ending semicolons. If you add a semicolon to the end of an expression, you turn it into a statement, and it will then not return a value.
```rust
let x = 5; // This is an expression, it evaluates to 5
let y = {
    let z = 10;
    z + 2 // This is an expression, it evaluates to 12
}; // This is a statement, it does not return a value
println!("The value of y is: {}", y); // Prints "The value of y is: 12"
```
Statements, on the other hand, do not return a value. They are used to perform actions, such as variable declarations or control flow statements. For example:
```rust
let x = 5; // This is a statement, it does not return a value
if x > 0 {
    println!("x is positive"); // This is a statement, it does not return a value
}
```

## Ownership and Borrowing
Ownership is a key feature of Rust that ensures memory safety without needing a garbage collector. The rules of ownership are:
1. Each value in Rust has an owner.
2. There can only be one owner at a time.
3. When the owner goes out of scope, the value will be dropped.

Ownership allows Rust to manage memory automatically, preventing issues like dangling pointers and memory leaks. When a variable goes out of scope, Rust automatically deallocates the memory associated with it.

Borrowing allows you to use a value without taking ownership of it. There are two types of borrowing:
- **Immutable Borrowing:** Allows multiple references to a value without modifying it. You can have any number of immutable references to a value at the same time.
- **Mutable Borrowing:** Allows a single mutable reference to a value, enabling modification. You can only have one mutable reference to a value at a time, and no immutable references can coexist with a mutable reference.
```rust
let s = String::from("Hello");
let r1 = &s; // Immutable borrow
let r2 = &s; // Another immutable borrow
println!("{} and {}", r1, r2); // Both can be used simultaneously
let mut s2 = String::from("World");
let r3 = &mut s2; // Mutable borrow
r3.push_str("!"); // Modify the value through the mutable reference
println!("{}", r3); // Prints "World!"
// println!("{}", s2); // This would cause a compile-time error if uncommented, as you cannot use s2 while r3 is still in scope
```

## The Slice Type
A slice is a pointer to a block of memory. Slices can be used to access portions of data stored in contiguous memory blocks. It can be used with data structures like arrays, vectors and strings. Slices use index numbers to access portions of data. The size of a slice is determined at runtime.

Slices let you reference a contiguous sequence of elements in a collection rather than the whole collection. A slice is a kind of reference, so it does not have ownership.

The concepts of ownership, borrowing, and slices ensure memory safety in Rust programs at compile time. The Rust language gives you control over your memory usage in the same way as other systems programming languages, but having the owner of data automatically clean up that data when the owner goes out of scope means you don’t have to write and debug extra code to get this control.
```rust
let arr = [1, 2, 3, 4, 5];
let slice = &arr[1..4]; // This creates a slice of the array from index 1 to index 3
println!("{:?}", slice); // Prints "[2, 3, 4]"
let str_slice = &"Hello, world!"[0..5]; // This creates a slice of the string from index 0 to index 4
println!("{}", str_slice); // Prints "Hello"
```

## Traits
