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

