# Rust Cheat Sheet 9 — Error Handling, Files, JSON, CLI, Testing, and Cargo

This is **cheat_sheet9.md**.

Focus: practical Rust project tasks that commonly appear in exams and real projects:

- `Option<T>` and `Result<T, E>`
- safe parsing and conversion
- `unwrap`, `expect`, `unwrap_or`, `?`
- custom errors
- reading and writing files
- JSON with `serde_json`
- command-line arguments
- stdout and stderr
- Cargo commands and `Cargo.toml`
- tests with `assert!` and `assert_eq!`

Style rules used in this sheet:

- Method tables are placed inside their related subject sections.
- Method/function return types are shown where useful.
- Every executable `println!` line includes `// output:` on the same line whenever practical.

---

# 1. Error Handling Big Picture

Rust does not usually use exceptions.

Rust commonly uses two enums:

```rust
Option<T> // value may exist or may not exist
Result<T, E> // operation may succeed or fail
```

## Core error-handling forms

| Type | Variants | Meaning |
|---|---|---|
| `Option<T>` | `Some(value)` / `None` | maybe there is a value |
| `Result<T, E>` | `Ok(value)` / `Err(error)` | operation can fail |

## Small example

```rust
fn main() {
    let maybe_number = Some(10);
    let parsed = "42".parse::<i32>();

    println!("{:?}", maybe_number); // output: Some(10)
    println!("{:?}", parsed); // output: Ok(42)
}
```

---

# 2. `Option<T>`

Use `Option<T>` when a value may be missing.

```rust
Some(value)
None
```

## Common methods returning `Option`

| Method / function | Common return type | Why it returns `Option` |
|---|---:|---|
| `vec.get(index)` | `Option<&T>` | index may not exist |
| `vec.first()` | `Option<&T>` | collection may be empty |
| `vec.last()` | `Option<&T>` | collection may be empty |
| `vec.pop()` | `Option<T>` | collection may be empty |
| `iter.find(...)` | `Option<T>` or `Option<&T>` | item may not be found |
| `iter.position(...)` | `Option<usize>` | matching index may not exist |
| `str.find(pattern)` | `Option<usize>` | pattern may not exist |
| `str.strip_prefix(prefix)` | `Option<&str>` | prefix may not exist |
| `str.strip_suffix(suffix)` | `Option<&str>` | suffix may not exist |

## Handle `Option` with `match`

```rust
fn main() {
    let numbers = vec![10, 20, 30];

    match numbers.get(1) {
        Some(n) => println!("found = {}", n), // output: found = 20
        None => println!("not found"), // output: not found if index is missing
    }
}
```

## Handle `Option` with `if let`

Use `if let` when you only care about `Some`.

```rust
fn main() {
    let numbers = vec![10, 20, 30];

    if let Some(n) = numbers.first() {
        println!("first = {}", n); // output: first = 10
    }
}
```

## `.unwrap_or(default) -> T`

Returns the value inside `Some`, or a default value if `None`.

```rust
fn main() {
    let numbers = vec![10, 20, 30];

    let a = numbers.get(0).copied().unwrap_or(0);
    let b = numbers.get(99).copied().unwrap_or(0);

    println!("{}", a); // output: 10
    println!("{}", b); // output: 0
}
```

## `.expect(message) -> T or panic`

Use only when the missing value should be impossible or when learning.

```rust
fn main() {
    let numbers = vec![10, 20, 30];
    let first = numbers.first().expect("list should not be empty");

    println!("{}", first); // output: 10
}
```

## `Option` method index

| Method | Return type | Meaning |
|---|---:|---|
| `.is_some()` | `bool` | true if `Some` |
| `.is_none()` | `bool` | true if `None` |
| `.unwrap_or(default)` | `T` | value or default |
| `.expect(message)` | `T` or panic | value or stop with message |
| `.map(func)` | `Option<U>` | transform value if `Some` |
| `.and_then(func)` | `Option<U>` | chain another optional operation |

## `.map()` with `Option`

```rust
fn main() {
    let value = Some(10);
    let doubled = value.map(|n| n * 2);

    println!("{:?}", doubled); // output: Some(20)
}
```

## `.and_then()` with `Option`

Use when the closure also returns `Option`.

```rust
fn half_even(n: i32) -> Option<i32> {
    if n % 2 == 0 {
        Some(n / 2)
    } else {
        None
    }
}

fn main() {
    let result = Some(20).and_then(half_even);

    println!("{:?}", result); // output: Some(10)
}
```

---

# 3. `Result<T, E>`

Use `Result<T, E>` when an operation can fail.

```rust
Ok(value)
Err(error)
```

## Common methods returning `Result`

| Method / function | Common return type | Why it returns `Result` |
|---|---:|---|
| `text.parse::<T>()` | `Result<T, ParseIntError>` etc. | text may not parse |
| `value.try_into()` | `Result<T, E>` | conversion may fail |
| `std::fs::read_to_string(path)` | `Result<String, io::Error>` | file may not exist |
| `std::fs::write(path, data)` | `Result<(), io::Error>` | write may fail |
| `File::open(path)` | `Result<File, io::Error>` | file may not exist |
| `serde_json::from_str::<T>(text)` | `Result<T, serde_json::Error>` | invalid JSON |

## Handle `Result` with `match`

```rust
fn main() {
    let result = "42".parse::<i32>();

    match result {
        Ok(n) => println!("number = {}", n), // output: number = 42
        Err(_) => println!("not a number"), // output: not a number if parsing fails
    }
}
```

## Handle `Result` with `if let`

```rust
fn main() {
    let result = "42".parse::<i32>();

    if let Ok(n) = result {
        println!("number = {}", n); // output: number = 42
    }
}
```

## `.unwrap_or(default) -> T`

```rust
fn main() {
    let a = "42".parse::<i32>().unwrap_or(0);
    let b = "abc".parse::<i32>().unwrap_or(0);

    println!("{}", a); // output: 42
    println!("{}", b); // output: 0
}
```

## `.expect(message) -> T or panic`

```rust
fn main() {
    let n = "42".parse::<i32>().expect("expected a number");

    println!("{}", n); // output: 42
}
```

## `Result` method index

| Method | Return type | Meaning |
|---|---:|---|
| `.is_ok()` | `bool` | true if `Ok` |
| `.is_err()` | `bool` | true if `Err` |
| `.unwrap_or(default)` | `T` | value or default |
| `.expect(message)` | `T` or panic | value or stop with message |
| `.map(func)` | `Result<U, E>` | transform `Ok` value |
| `.map_err(func)` | `Result<T, F>` | transform error |
| `.and_then(func)` | `Result<U, E>` | chain another fallible operation |

## `.map()` with `Result`

```rust
fn main() {
    let result = "21".parse::<i32>().map(|n| n * 2);

    println!("{:?}", result); // output: Ok(42)
}
```

## `.map_err()` with `Result`

```rust
fn main() {
    let result = "abc"
        .parse::<i32>()
        .map_err(|_| String::from("invalid number"));

    println!("{:?}", result); // output: Err("invalid number")
}
```

---

# 4. `unwrap`, `expect`, `unwrap_or`, and `unwrap_or_else`

These methods exist on `Option` and `Result`.

## Method comparison

| Method | Safe? | When to use |
|---|---:|---|
| `.unwrap()` | no | quick tests only |
| `.expect("message")` | no | quick tests with clear panic message |
| `.unwrap_or(default)` | yes | use default on failure/missing |
| `.unwrap_or_else(|| ...)` | yes | compute default only when needed |

## `.unwrap_or_else()`

```rust
fn main() {
    let result = "abc".parse::<i32>().unwrap_or_else(|_| 0);

    println!("{}", result); // output: 0
}
```

## Avoid `.unwrap()` in final exam answers when safe handling is expected

Prefer:

```rust
match value {
    Ok(v) => v,
    Err(_) => default_value,
}
```

or:

```rust
value.unwrap_or(default_value)
```

---

# 5. The `?` Operator

The `?` operator is used inside a function that returns `Result` or `Option`.

It means:

```text
If Ok/Some: take the inside value.
If Err/None: return early from the function.
```

## `?` with `Result`

```rust
fn parse_and_double(text: &str) -> Result<i32, std::num::ParseIntError> {
    let n = text.parse::<i32>()?;
    Ok(n * 2)
}

fn main() {
    let result = parse_and_double("21");

    println!("{:?}", result); // output: Ok(42)
}
```

Without `?`, you would write:

```rust
fn parse_and_double(text: &str) -> Result<i32, std::num::ParseIntError> {
    match text.parse::<i32>() {
        Ok(n) => Ok(n * 2),
        Err(e) => Err(e),
    }
}
```

## `?` with `Option`

```rust
fn first_plus_one(numbers: &[i32]) -> Option<i32> {
    let first = numbers.first()?;
    Some(*first + 1)
}

fn main() {
    let result = first_plus_one(&[10, 20]);

    println!("{:?}", result); // output: Some(11)
}
```

---

# 6. Parsing and Safe Conversion

## Parsing method index

| Method | Return type | Example |
|---|---:|---|
| `.parse::<i32>()` | `Result<i32, _>` | text to signed integer |
| `.parse::<u32>()` | `Result<u32, _>` | text to unsigned integer |
| `.parse::<f64>()` | `Result<f64, _>` | text to decimal |
| `.try_into()` | `Result<T, E>` | checked numeric conversion |
| `as` | target type | unchecked numeric cast |

## Parse string to number

```rust
fn main() {
    let text = "42";
    let n = text.parse::<i32>().unwrap_or(0);

    println!("{}", n + 1); // output: 43
}
```

## Parse list of numbers from text

```rust
fn parse_numbers(text: &str) -> Vec<i32> {
    text.split_whitespace()
        .filter_map(|part| part.parse::<i32>().ok())
        .collect()
}

fn main() {
    let nums = parse_numbers("10 abc 20 30");

    println!("{:?}", nums); // output: [10, 20, 30]
}
```

## Safe numeric conversion with `try_into()`

```rust
use std::convert::TryInto;

fn main() {
    let n: i32 = 10;
    let converted: u32 = n.try_into().expect("must be positive");

    println!("{}", converted); // output: 10
}
```

---

# 7. Panic

`panic!` stops the program immediately.

Use it when the program reaches an impossible or unrecoverable state.

## Panic-related syntax

| Syntax | Return behavior | Meaning |
|---|---:|---|
| `panic!("message")` | never returns | stop program |
| `.unwrap()` | value or panic | panic on `None` / `Err` |
| `.expect("message")` | value or panic | panic with clearer message |

## Example without panic

```rust
fn safe_divide(a: i32, b: i32) -> Option<i32> {
    if b == 0 {
        None
    } else {
        Some(a / b)
    }
}

fn main() {
    println!("{:?}", safe_divide(10, 2)); // output: Some(5)
    println!("{:?}", safe_divide(10, 0)); // output: None
}
```

---

# 8. Custom Error Types

For beginner and exam code, a simple `String` error is often enough.

## Simple custom error using `Result<T, String>`

```rust
fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err(String::from("division by zero"))
    } else {
        Ok(a / b)
    }
}

fn main() {
    println!("{:?}", divide(10, 2)); // output: Ok(5)
    println!("{:?}", divide(10, 0)); // output: Err("division by zero")
}
```

## Custom enum error

```rust
#[derive(Debug)]
enum MathError {
    DivisionByZero,
}

fn divide(a: i32, b: i32) -> Result<i32, MathError> {
    if b == 0 {
        Err(MathError::DivisionByZero)
    } else {
        Ok(a / b)
    }
}

fn main() {
    println!("{:?}", divide(8, 2)); // output: Ok(4)
}
```

---

# 9. File Reading and Writing

Most file operations return `Result` because files may be missing, locked, or invalid.

## File method/function index

| Function / method | Return type | Meaning |
|---|---:|---|
| `std::fs::read_to_string(path)` | `Result<String, io::Error>` | read whole file as text |
| `std::fs::write(path, data)` | `Result<(), io::Error>` | write text/bytes to file |
| `File::open(path)` | `Result<File, io::Error>` | open file |
| `file.read_to_string(&mut s)` | `Result<usize, io::Error>` | read into String |
| `std::fs::remove_file(path)` | `Result<(), io::Error>` | delete file |

## Write and read a file

```rust
use std::fs;

fn main() {
    fs::write("demo.txt", "hello rust").expect("write failed");
    let text = fs::read_to_string("demo.txt").expect("read failed");

    println!("{}", text); // output: hello rust
}
```

## Safe file reading with `match`

```rust
use std::fs;

fn main() {
    match fs::read_to_string("missing.txt") {
        Ok(text) => println!("{}", text), // output: file content if file exists
        Err(_) => println!("could not read file"), // output: could not read file
    }
}
```

## File reading with `?`

```rust
use std::fs;
use std::io;

fn read_config(path: &str) -> Result<String, io::Error> {
    let text = fs::read_to_string(path)?;
    Ok(text)
}

fn main() {
    fs::write("config.txt", "mode=dev").expect("write failed");
    let result = read_config("config.txt");

    println!("{:?}", result); // output: Ok("mode=dev")
}
```

---

# 10. JSON Basics with `serde_json`

JSON usually requires external crates.

Add this to `Cargo.toml`:

```toml
[dependencies]
serde = { version = "1", features = ["derive"] }
serde_json = "1"
```

## JSON method/function index

| Function / macro | Return type | Meaning |
|---|---:|---|
| `serde_json::from_str::<T>(text)` | `Result<T, serde_json::Error>` | parse JSON into type |
| `serde_json::to_string(&value)` | `Result<String, serde_json::Error>` | convert value to JSON text |
| `serde_json::to_string_pretty(&value)` | `Result<String, serde_json::Error>` | pretty JSON text |
| `serde_json::json!({...})` | `serde_json::Value` | create JSON value |
| `value["key"]` | `Value` | access JSON key |
| `.as_str()` | `Option<&str>` | get string from JSON value |
| `.as_i64()` | `Option<i64>` | get integer from JSON value |
| `.as_bool()` | `Option<bool>` | get bool from JSON value |

## Parse JSON into `serde_json::Value`

```rust
use serde_json::Value;

fn main() {
    let text = r#"{"name":"Abbas","age":25}"#;
    let value: Value = serde_json::from_str(text).expect("invalid JSON");

    println!("{}", value["name"]); // output: "Abbas"
    println!("{}", value["age"]); // output: 25
}
```

## Extract JSON values safely

```rust
use serde_json::Value;

fn main() {
    let text = r#"{"name":"Abbas","age":25}"#;
    let value: Value = serde_json::from_str(text).expect("invalid JSON");

    let name = value["name"].as_str().unwrap_or("unknown");
    let age = value["age"].as_i64().unwrap_or(0);

    println!("{} is {}", name, age); // output: Abbas is 25
}
```

## Parse JSON into a struct

```rust
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct User {
    name: String,
    age: u32,
}

fn main() {
    let text = r#"{"name":"Abbas","age":25}"#;
    let user: User = serde_json::from_str(text).expect("invalid JSON");

    println!("{}", user.name); // output: Abbas
    println!("{}", user.age); // output: 25
}
```

## Convert struct to JSON

```rust
use serde::Serialize;

#[derive(Serialize)]
struct User {
    name: String,
    age: u32,
}

fn main() {
    let user = User { name: String::from("Abbas"), age: 25 };
    let json = serde_json::to_string(&user).expect("serialize failed");

    println!("{}", json); // output: {"name":"Abbas","age":25}
}
```

---

# 11. Command-Line Arguments

Use `std::env::args()` to read arguments passed to the program.

## CLI method/function index

| Function / method | Return type | Meaning |
|---|---:|---|
| `std::env::args()` | `Args` iterator | command-line arguments |
| `.collect::<Vec<String>>()` | `Vec<String>` | store args in vector |
| `.skip(1)` | iterator | skip program name |
| `.nth(index)` | `Option<T>` | get argument at position |

## Read all args

```rust
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    println!("{:?}", args); // output: includes program path and provided arguments
}
```

## Read first user argument safely

```rust
use std::env;

fn main() {
    let name = env::args().nth(1).unwrap_or(String::from("guest"));

    println!("Hello, {}", name); // output: Hello, guest if no argument is provided
}
```

## Parse numeric argument safely

```rust
use std::env;

fn main() {
    let number = env::args()
        .nth(1)
        .and_then(|text| text.parse::<i32>().ok())
        .unwrap_or(0);

    println!("{}", number); // output: 0 if missing or invalid
}
```

---

# 12. stdout and stderr

Use:

```rust
println!(...);  // output: normal output
 eprintln!(...); // output: error output
```

## Output macro index

| Macro | Stream | Meaning |
|---|---|---|
| `print!()` | stdout | print without newline |
| `println!()` | stdout | print with newline; // output: line printed to stdout |
| `eprint!()` | stderr | error print without newline |
| `eprintln!()` | stderr | error print with newline; // output: line printed to stderr |
| `format!()` | String | build formatted string |

## Normal output

```rust
fn main() {
    println!("success"); // output: success
}
```

## Error output

```rust
fn main() {
    eprintln!("error: invalid input"); // output: stderr: error: invalid input
}
```

---

# 13. Cargo Basics

Cargo manages Rust projects.

## Common Cargo commands

| Command | Meaning |
|---|---|
| `cargo new project_name` | create binary project |
| `cargo new --lib project_name` | create library project |
| `cargo run` | build and run |
| `cargo build` | build only |
| `cargo test` | run tests |
| `cargo check` | check code without producing final binary |
| `cargo fmt` | format code |
| `cargo clippy` | lint code |
| `cargo add crate_name` | add dependency if cargo-edit is installed |

## Common project structure

```text
project/
├── Cargo.toml
└── src/
    ├── main.rs
    └── lib.rs
```

## Binary crate

A binary project has `src/main.rs`:

```rust
fn main() {
    println!("hello"); // output: hello
}
```

Run with:

```bash
cargo run
```

## Library crate

A library usually has `src/lib.rs`:

```rust
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

Test it with `cargo test`.

---

# 14. `Cargo.toml`

`Cargo.toml` stores project metadata and dependencies.

## Basic example

```toml
[package]
name = "practice"
version = "0.1.0"
edition = "2021"

[dependencies]
serde_json = "1"
```

## Dependency examples

```toml
[dependencies]
rand = "0.9"
serde = { version = "1", features = ["derive"] }
serde_json = "1"
```

## Common sections

| Section | Meaning |
|---|---|
| `[package]` | project name, version, edition |
| `[dependencies]` | external crates needed by normal code |
| `[dev-dependencies]` | crates needed only for tests/examples |

---

# 15. Unit Testing

Tests usually go inside `src/lib.rs` or the same file as the code.

## Test macro/index table

| Syntax / macro | Meaning |
|---|---|
| `#[cfg(test)]` | compile only during tests |
| `mod tests {}` | test module |
| `#[test]` | marks a test function |
| `assert!(condition)` | test condition is true |
| `assert_eq!(a, b)` | test values are equal |
| `assert_ne!(a, b)` | test values are not equal |
| `#[should_panic]` | test should panic |

## Simple test

```rust
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
    }
}
```

Run:

```bash
cargo test
```

## `assert!`

```rust
pub fn is_even(n: i32) -> bool {
    n % 2 == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_even() {
        assert!(is_even(4));
    }
}
```

## `#[should_panic]`

```rust
pub fn must_be_positive(n: i32) {
    if n <= 0 {
        panic!("not positive");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn test_panic() {
        must_be_positive(0);
    }
}
```

---

# 16. Integration Tests

Integration tests go in a `tests/` folder.

```text
project/
├── Cargo.toml
├── src/
│   └── lib.rs
└── tests/
    └── basic.rs
```

## `src/lib.rs`

```rust
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

## `tests/basic.rs`

```rust
use practice::add;

#[test]
fn test_add() {
    assert_eq!(add(2, 3), 5);
}
```

---

# 17. Exam Error-Handling Patterns

## Pattern 1: Parse or return default

```rust
fn parse_or_zero(text: &str) -> i32 {
    text.parse::<i32>().unwrap_or(0)
}

fn main() {
    println!("{}", parse_or_zero("42")); // output: 42
    println!("{}", parse_or_zero("abc")); // output: 0
}
```

## Pattern 2: Parse or return `Result`

```rust
fn parse_age(text: &str) -> Result<u32, std::num::ParseIntError> {
    text.parse::<u32>()
}

fn main() {
    println!("{:?}", parse_age("25")); // output: Ok(25)
}
```

## Pattern 3: Read file and count lines

```rust
use std::fs;
use std::io;

fn count_lines(path: &str) -> Result<usize, io::Error> {
    let text = fs::read_to_string(path)?;
    Ok(text.lines().count())
}

fn main() {
    fs::write("lines.txt", "one\ntwo\nthree").expect("write failed");
    let count = count_lines("lines.txt").expect("read failed");

    println!("{}", count); // output: 3
}
```

## Pattern 4: Collect only valid numbers

```rust
fn valid_numbers(text: &str) -> Vec<i32> {
    text.split_whitespace()
        .filter_map(|part| part.parse::<i32>().ok())
        .collect()
}

fn main() {
    let result = valid_numbers("1 two 3 four 5");

    println!("{:?}", result); // output: [1, 3, 5]
}
```

---

# 18. Common Compiler Errors and Fixes

## Error: expected `Result`, found value

Wrong idea:

```rust
fn parse_number(text: &str) -> Result<i32, String> {
    text.parse::<i32>().unwrap_or(0)
}
```

Correct:

```rust
fn parse_number(text: &str) -> Result<i32, String> {
    text.parse::<i32>().map_err(|_| String::from("invalid number"))
}

fn main() {
    println!("{:?}", parse_number("42")); // output: Ok(42)
}
```

## Error: `?` used in function that does not return `Result` or `Option`

Wrong idea:

```rust
fn main() {
    let n = "42".parse::<i32>()?;
}
```

Correct:

```rust
fn parse_number() -> Result<i32, std::num::ParseIntError> {
    let n = "42".parse::<i32>()?;
    Ok(n)
}

fn main() {
    println!("{:?}", parse_number()); // output: Ok(42)
}
```

## Error: using `unwrap()` when value may fail

Risky:

```rust
fn risky(text: &str) -> i32 {
    text.parse::<i32>().unwrap()
}
```

Safer:

```rust
fn safe(text: &str) -> i32 {
    text.parse::<i32>().unwrap_or(0)
}

fn main() {
    println!("{}", safe("abc")); // output: 0
}
```

---

# 19. Quick Return-Type Table

| Method / function | Return type | Subject |
|---|---:|---|
| `Option::is_some()` | `bool` | Option |
| `Option::is_none()` | `bool` | Option |
| `Option::unwrap_or(default)` | `T` | Option |
| `Result::is_ok()` | `bool` | Result |
| `Result::is_err()` | `bool` | Result |
| `Result::unwrap_or(default)` | `T` | Result |
| `parse::<T>()` | `Result<T, E>` | parsing |
| `try_into()` | `Result<T, E>` | conversion |
| `fs::read_to_string(path)` | `Result<String, io::Error>` | files |
| `fs::write(path, data)` | `Result<(), io::Error>` | files |
| `serde_json::from_str::<T>(text)` | `Result<T, serde_json::Error>` | JSON |
| `serde_json::to_string(&value)` | `Result<String, serde_json::Error>` | JSON |
| `env::args()` | `Args` iterator | CLI |
| `assert_eq!(a, b)` | `()` or panic | tests |

---

# 20. Practice Exercises

## Exercise 1 — Safe parse

Write:

```rust
fn parse_or_zero(text: &str) -> i32
```

Expected:

```rust
println!("{}", parse_or_zero("10")); // output: 10
println!("{}", parse_or_zero("abc")); // output: 0
```

## Exercise 2 — Safe divide

Write:

```rust
fn safe_divide(a: i32, b: i32) -> Result<i32, String>
```

Expected:

```rust
println!("{:?}", safe_divide(10, 2)); // output: Ok(5)
println!("{:?}", safe_divide(10, 0)); // output: Err("division by zero")
```

## Exercise 3 — Read file

Write:

```rust
fn read_file(path: &str) -> Result<String, std::io::Error>
```

Expected after writing `hello` into `demo.txt`:

```rust
println!("{:?}", read_file("demo.txt")); // output: Ok("hello")
```

## Exercise 4 — Collect valid numbers

Write:

```rust
fn collect_numbers(text: &str) -> Vec<i32>
```

Expected:

```rust
println!("{:?}", collect_numbers("1 abc 2 def 3")); // output: [1, 2, 3]
```

## Exercise 5 — JSON value extraction

Using `serde_json`, extract the `name` field from:

```json
{"name":"Abbas","age":25}
```

Expected:

```rust
println!("{}", name); // output: Abbas
```

---

End of **cheat_sheet9.md**.
