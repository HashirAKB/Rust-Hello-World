# Rust Programming Language Quick Reference

## Basic Syntax and Data Types

```rust
fn main() {
    println!("Hello, world!");

    // Variables and data types
    let x = -5;              // Signed integer
    let y: u32 = 1000;       // Unsigned 32-bit integer
    let z: f32 = 1000.00;    // 32-bit floating point
    
    // Booleans
    let is_male: bool = true;
    let is_above_18: bool = true;
}
```

- Variables are immutable by default
- Use `let mut` for mutable variables
- Type inference is available, but you can explicitly specify types

## Control Flow

```rust
// If-else
if is_male {
    println!("You're a male");
} else {
    println!("You're not a male");
}

// Loops
for i in 0..11 {
    println!("{}", i);
}
```

## Functions

```rust
fn get_first_word(sentence: String) -> String {
    let mut ans = String::from("");
    for char in sentence.chars() {
        ans.push_str(char.to_string().as_str());
        if char == ' ' {
            break;
        }
    }
    return ans;
}
```

## Memory Management

### Stack vs Heap

- Stack: For fixed-size, known at compile-time data
- Heap: For dynamic-size data or unknown size at compile-time

```rust
fn stack_fn() {
    let a = 10;
    let b = 20;
    let c = a + b;
}

fn heap_fn() {
    let s1 = String::from("Hello ");
    let s2 = String::from("World");
    let combined = format!("{} {}", s1, s2);
}
```

### Ownership

- Each value has a single owner
- When the owner goes out of scope, the value is dropped

```rust
fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

let sample_string = String::from("This is the sample string.");
takes_ownership(sample_string);
// sample_string is no longer valid here
```

### Borrowing and References

```rust
fn takes_ownership_by_borrowing(some_string: &String) {
    println!("This string is borrowed: {}", some_string);
}

let mut sample_string = String::from("Hanky Panky");
takes_ownership_by_borrowing(&sample_string);
println!("But I'm the owner MF! - {}", sample_string);
```

- Use `&` for immutable references
- Use `&mut` for mutable references
- Rules:
  1. One mutable reference OR any number of immutable references
  2. References must always be valid

## Structs

```rust
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

let user1 = User {
    active: true,
    username: String::from("HashirAKB"),
    email: String::from("hashirakb@gmail.com"),
    sign_in_count: 1,
};
```

### Implementing Methods for Structs

```rust
struct Rect {
    width: u32,
    height: u32,
}

impl Rect {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    
    fn perimeter(&self) -> u32 {
        2 * (self.width + self.height)
    }
}
```

## Enums

```rust
enum Direction {
    North,
    East,
    West,
    South,
}

enum Shape {
    Circle(f64),
    Square(f64),
    Rectangle(f64, f64),
}
```

## Pattern Matching

```rust
fn calculate_area(shape: Shape) -> f64 {
    match shape {
        Shape::Circle(radius) => 3.14 * radius * radius,
        Shape::Rectangle(width, height) => width * height,
        Shape::Square(side) => side * side,
    }
}
```

## Error Handling

```rust
let response = fs::read_to_string("/notAnExistingFile");
match response {
    Ok(content) => {
        println!("File Content: {}", content);
    }
    Err(err) => {
        println!("Error: {}", err);
    }
}
```

## Option Enum

```rust
pub enum Option<T> {
    None,
    Some(T),
}

fn find_first_a(s: String) -> Option<i32> {
    for (index, character) in s.chars().enumerate() {
        if character == 'a' {
            return Some(index as i32);
        }
    }
    return None;
}
```

## Cargo

- Package manager for Rust
- Used to manage dependencies (crates)
- Popular crates:
  - actix: Web framework
  - serde: Serialization/Deserialization
  - tokio: Asynchronous runtime
  - reqwest: HTTP client
  - sqlx: SQL database