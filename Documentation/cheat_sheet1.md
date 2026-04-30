# Rust Cheat Sheet 1 — Core Rust Syntax and Built-in Types

This is **cheat_sheet1.md** in the exam-based Rust cheat-sheet series.

Focus: **Rust program structure, variables, basic data types, numeric basics, string basics, containers, functions, control flow, loops, references, Option/Result basics, structs, enums, and common syntax symbols.**

Style used:

- Method headings show common return types, such as `.len() -> usize`.
- `println!` examples include `// output:` on the same line whenever practical.
- This sheet focuses on core syntax. Later sheets go deeper into strings, collections, algorithms, ownership, traits, modules, and advanced exam patterns.

---

## 1. Rust Program Structure

A runnable Rust program starts from `fn main()`.

```rust
fn main() {
    println!("Hello, Rust!"); // output: Hello, Rust!
}
```

Basic shape:

```rust
fn main() {
    // code starts here
}
```

Library exercises often use `src/lib.rs` and expected functions instead of `fn main()`:

```rust
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

---

## 2. Comments

```rust
fn main() {
    // Single-line comment

    /*
       Multi-line comment
    */

    println!("comments are ignored"); // output: comments are ignored
}
```

---

## 3. Variables: `let`, `mut`, `const`, Shadowing

### Immutable variable

```rust
fn main() {
    let x = 5;
    println!("{}", x); // output: 5
}
```

### Mutable variable

Use `mut` when the variable value will change.

```rust
fn main() {
    let mut x = 5;
    x = 10;

    println!("{}", x); // output: 10
}
```

### Constant

Constants must have a type annotation.

```rust
const MAX_SCORE: i32 = 100;

fn main() {
    println!("{}", MAX_SCORE); // output: 100
}
```

### Shadowing

Shadowing creates a new variable with the same name.

```rust
fn main() {
    let value = "42";
    let value: i32 = value.parse().expect("not a number");

    println!("{}", value + 1); // output: 43
}
```

Shadowing can change the type. `mut` changes the value but not the type.

---

## 4. Type Annotations and Type Inference

Rust often guesses the type.

```rust
fn main() {
    let x = 10;
    println!("{}", x); // output: 10
}
```

You can make the type clear:

```rust
fn main() {
    let x: i32 = 10;
    let y: f64 = 2.5;

    println!("{}", x); // output: 10
    println!("{}", y); // output: 2.5
}
```

Use type annotations when Rust says the type is ambiguous.

```rust
fn main() {
    let numbers: Vec<i32> = vec![-1, 2, 3];
    let abs_values: Vec<i32> = numbers.iter().map(|&n| n.abs()).collect();

    println!("{:?}", abs_values); // output: [1, 2, 3]
}
```

---

## 5. Basic Data Types

### Integers

Common integer types:

| Type | Meaning |
|---|---|
| `i32` | signed integer, common default |
| `u32` | unsigned integer |
| `usize` | index and length type |
| `i64` | larger signed integer |
| `u64` | larger unsigned integer |

```rust
fn main() {
    let a: i32 = -10;
    let b: u32 = 10;
    let index: usize = 0;

    println!("{}", a); // output: -10
    println!("{}", b); // output: 10
    println!("{}", index); // output: 0
}
```

### Floats

Common float types:

| Type | Meaning |
|---|---|
| `f32` | 32-bit decimal |
| `f64` | 64-bit decimal, common default |

```rust
fn main() {
    let price: f64 = 10.75;
    println!("{}", price); // output: 10.75
}
```

### Boolean

```rust
fn main() {
    let active: bool = true;
    println!("{}", active); // output: true
}
```

### Character

A `char` uses single quotes.

```rust
fn main() {
    let letter: char = 'A';
    println!("{}", letter); // output: A
}
```

### String slice: `&str`

Usually borrowed text or string literal.

```rust
fn main() {
    let name: &str = "Abbas";
    println!("{}", name); // output: Abbas
}
```

### Owned string: `String`

Owned, growable text.

```rust
fn main() {
    let name: String = String::from("Abbas");
    println!("{}", name); // output: Abbas
}
```

---

## 6. Numeric Operators and Core Methods

### Operators

| Operation | Syntax | Return type usually |
|---|---|---|
| Add | `a + b` | same numeric type |
| Subtract | `a - b` | same numeric type |
| Multiply | `a * b` | same numeric type |
| Divide | `a / b` | same numeric type |
| Remainder | `a % b` | same numeric type |

```rust
fn main() {
    let a = 10;
    let b = 3;

    println!("add = {}", a + b); // output: add = 13
    println!("subtract = {}", a - b); // output: subtract = 7
    println!("multiply = {}", a * b); // output: multiply = 30
    println!("divide = {}", a / b); // output: divide = 3
    println!("remainder = {}", a % b); // output: remainder = 1
}
```

Integer division removes the decimal part.

```rust
fn main() {
    println!("{}", 10 / 3); // output: 3
}
```

Decimal division needs float values.

```rust
fn main() {
    println!("{}", 10 as f64 / 3 as f64); // output: 3.3333333333333335
}
```

### `.abs() -> signed number`

Returns the positive value of a signed number.

```rust
fn main() {
    let x: i32 = -10;
    println!("{}", x.abs()); // output: 10
}
```

### `.pow(exp: u32) -> number`

Raises an integer to a power.

```rust
fn main() {
    let base: i32 = 2;
    let power: u32 = 3;

    println!("{}", base.pow(power)); // output: 8
}
```

If the power is `i32`, convert it:

```rust
fn main() {
    let base: i32 = 2;
    let power: i32 = 3;

    println!("{}", base.pow(power as u32)); // output: 8
}
```

### `.min(other) -> same type`

Returns the smaller value.

```rust
fn main() {
    let x = 10;
    println!("{}", x.min(5)); // output: 5
}
```

### `.max(other) -> same type`

Returns the bigger value.

```rust
fn main() {
    let x = 10;
    println!("{}", x.max(20)); // output: 20
}
```

### `.clamp(min, max) -> same type`

Forces a value to stay between minimum and maximum.

```rust
fn main() {
    let score = 120;
    println!("{}", score.clamp(0, 100)); // output: 100
}
```

```rust
fn main() {
    let score = -10;
    println!("{}", score.clamp(0, 100)); // output: 0
}
```

### `.is_positive() -> bool`

```rust
fn main() {
    let x = 5;
    println!("{}", x.is_positive()); // output: true
}
```

### `.is_negative() -> bool`

```rust
fn main() {
    let x = -5;
    println!("{}", x.is_negative()); // output: true
}
```

### Float methods

| Method | Return type | Meaning |
|---|---|---|
| `.round()` | float | nearest whole number |
| `.floor()` | float | round down |
| `.ceil()` | float | round up |
| `.sqrt()` | float | square root |

```rust
fn main() {
    let x = 3.6_f64;

    println!("{}", x.round()); // output: 4
    println!("{}", x.floor()); // output: 3
    println!("{}", x.ceil()); // output: 4
}
```

```rust
fn main() {
    let x = 25.0_f64;
    println!("{}", x.sqrt()); // output: 5
}
```

### Decimal places helper

Rust `.round()`, `.floor()`, and `.ceil()` do not take decimal-place arguments. Use multiply, round, divide.

```rust
fn round_to(value: f64, decimals: i32) -> f64 {
    let factor = 10_f64.powi(decimals);
    (value * factor).round() / factor
}

fn main() {
    println!("{}", round_to(3.4567, 2)); // output: 3.46
}
```

---

## 7. Type Conversion Basics

### `as -> target numeric type`

Simple numeric conversion.

```rust
fn main() {
    let x: i32 = 10;
    let y: f64 = x as f64;

    println!("{}", y); // output: 10
}
```

Warning: converting negative signed numbers into unsigned numbers using `as` can produce unexpected large values.

### `.try_into() -> Result<T, E>`

Safer conversion.

```rust
use std::convert::TryInto;

fn main() {
    let x: i32 = 10;
    let y: u32 = x.try_into().expect("cannot convert");

    println!("{}", y); // output: 10
}
```

### `.to_string() -> String`

```rust
fn main() {
    let score = 100;
    let text = score.to_string();

    println!("{}", text); // output: 100
}
```

### `.parse::<T>() -> Result<T, E>`

```rust
fn main() {
    let text = "42";
    let number = text.parse::<i32>().expect("not a number");

    println!("{}", number + 1); // output: 43
}
```

Safe handling:

```rust
fn main() {
    let text = "abc";
    let result = text.parse::<i32>();

    match result {
        Ok(n) => println!("{}", n), // output: parsed number if valid
        Err(_) => println!("not a number"), // output: not a number
    }
}
```

---

## 8. String Basics

Detailed string/text-processing patterns are in `cheat_sheet2.md`. This sheet keeps the core syntax and most common methods.

### `&str` vs `String` vs `&String`

Use `&str` for function parameters when only reading text.

```rust
fn show(text: &str) {
    println!("{}", text); // output: text passed to show
}

fn main() {
    let literal = "Abbas";
    let owned = String::from("Hasan");

    show(literal); // output: Abbas
    show(&owned); // output: Hasan
}
```

Less flexible:

```rust
fn show_string(text: &String) {
    println!("{}", text); // output: String passed to show_string
}

fn main() {
    let owned = String::from("Hasan");
    show_string(&owned); // output: Hasan
}
```

### String creation

| Syntax | Return type |
|---|---|
| `String::new()` | `String` |
| `String::from("text")` | `String` |
| `"text".to_string()` | `String` |
| `format!(...)` | `String` |

```rust
fn main() {
    let a = String::new();
    let b = String::from("hello");
    let c = "rust".to_string();
    let d = format!("{} {}", b, c);

    println!("{}", a); // output: empty line
    println!("{}", d); // output: hello rust
}
```

### String editing

| Method | Return type | Meaning |
|---|---|---|
| `.push(char)` | `()` | changes original string |
| `.push_str(&str)` | `()` | changes original string |
| `.replace(from, to)` | `String` | returns new string |

```rust
fn main() {
    let mut text = String::from("Hi");
    text.push('!');
    text.push_str(" Rust");

    println!("{}", text); // output: Hi! Rust
}
```

```rust
fn main() {
    let text = "hello rust";
    let fixed = text.replace("rust", "Rust");

    println!("{}", fixed); // output: hello Rust
}
```

### String checking and cleaning

| Method | Return type |
|---|---|
| `.len()` | `usize` |
| `.is_empty()` | `bool` |
| `.trim()` | `&str` |
| `.contains(pattern)` | `bool` |
| `.starts_with(pattern)` | `bool` |
| `.ends_with(pattern)` | `bool` |

```rust
fn main() {
    let text = "  hello rust  ";

    println!("{}", text.len()); // output: 14
    println!("{}", text.trim()); // output: hello rust
    println!("{}", text.contains("rust")); // output: true
    println!("{}", text.trim().starts_with("hello")); // output: true
    println!("{}", text.trim().ends_with("rust")); // output: true
}
```

### Case conversion

| Method | Return type | Note |
|---|---|---|
| `.to_uppercase()` | `String` | Unicode-aware |
| `.to_lowercase()` | `String` | Unicode-aware |
| `.to_ascii_uppercase()` | `String` | ASCII-only |
| `.to_ascii_lowercase()` | `String` | ASCII-only |

```rust
fn main() {
    let text = "Rust";

    println!("{}", text.to_uppercase()); // output: RUST
    println!("{}", text.to_lowercase()); // output: rust
}
```

### Basic splitting and joining

| Method | Return type |
|---|---|
| `.split_whitespace()` | iterator |
| `.split(pattern)` | iterator |
| `.lines()` | iterator |
| `.chars()` | iterator |
| `.join(separator)` | `String` |

```rust
fn main() {
    let text = "hello   rust";
    let words: Vec<&str> = text.split_whitespace().collect();

    println!("{:?}", words); // output: ["hello", "rust"]
}
```

```rust
fn main() {
    let words = ["Rust", "is", "fun"];
    let sentence = words.join(" ");

    println!("{}", sentence); // output: Rust is fun
}
```

### String concatenation

Rust does not support `&str + &str` directly.

```rust
fn main() {
    let a = String::from("Hello, ");
    let b = "Rust";

    let result = a + b;
    println!("{}", result); // output: Hello, Rust
}
```

Use `format!` when you want to keep original values available.

```rust
fn main() {
    let first = "Abbas";
    let last = "Hasan";
    let full = format!("{} {}", first, last);

    println!("{}", full); // output: Abbas Hasan
}
```

---

## 9. Arrays

Arrays have fixed size.

```rust
fn main() {
    let numbers: [i32; 3] = [10, 20, 30];

    println!("{:?}", numbers); // output: [10, 20, 30]
    println!("{}", numbers[0]); // output: 10
}
```

### Repeated values: `[value; count] -> array`

```rust
fn main() {
    let zeros = [0; 5];
    println!("{:?}", zeros); // output: [0, 0, 0, 0, 0]
}
```

### Common array/slice methods

| Method | Return type |
|---|---|
| `.len()` | `usize` |
| `.is_empty()` | `bool` |
| `.contains(&value)` | `bool` |
| `.to_vec()` | `Vec<T>` |

```rust
fn main() {
    let numbers = [10, 20, 30];

    println!("{}", numbers.len()); // output: 3
    println!("{}", numbers.is_empty()); // output: false
    println!("{}", numbers.contains(&20)); // output: true
    println!("{:?}", numbers.to_vec()); // output: [10, 20, 30]
}
```

---

## 10. Slices

A slice is a borrowed view of part of an array or vector.

```rust
fn main() {
    let numbers = [10, 20, 30, 40, 50];
    let part = &numbers[1..4];

    println!("{:?}", part); // output: [20, 30, 40]
}
```

Ranges:

```rust
fn main() {
    let numbers = [10, 20, 30, 40, 50];

    println!("{:?}", &numbers[..3]); // output: [10, 20, 30]
    println!("{:?}", &numbers[2..]); // output: [30, 40, 50]
    println!("{:?}", &numbers[..]); // output: [10, 20, 30, 40, 50]
}
```

Slice function parameter:

```rust
fn sum(numbers: &[i32]) -> i32 {
    numbers.iter().copied().sum()
}

fn main() {
    let arr = [1, 2, 3];
    let vec = vec![4, 5, 6];

    println!("{}", sum(&arr)); // output: 6
    println!("{}", sum(&vec)); // output: 15
}
```

Use `&[T]` when a function only needs to read a list.

---

## 11. Vectors

Vectors are growable lists.

### `Vec::new() -> Vec<T>`

```rust
fn main() {
    let mut numbers: Vec<i32> = Vec::new();
    numbers.push(10);

    println!("{:?}", numbers); // output: [10]
}
```

### `vec![...] -> Vec<T>`

```rust
fn main() {
    let numbers = vec![10, 20, 30];
    println!("{:?}", numbers); // output: [10, 20, 30]
}
```

### Core vector methods

| Method | Return type | Meaning |
|---|---|---|
| `.push(value)` | `()` | add to end |
| `.pop()` | `Option<T>` | remove last item |
| `.insert(index, value)` | `()` | insert at index |
| `.remove(index)` | `T` | remove and return item |
| `.clear()` | `()` | remove all |
| `.sort()` | `()` | sort ascending |
| `.reverse()` | `()` | reverse current order |
| `.append(&mut other)` | `()` | move all items from other |
| `.extend(iterable)` | `()` | add many items |
| `.get(index)` | `Option<&T>` | safe access |
| `.first()` | `Option<&T>` | first item |
| `.last()` | `Option<&T>` | last item |

```rust
fn main() {
    let mut numbers = vec![10, 20];
    numbers.push(30);

    println!("{:?}", numbers); // output: [10, 20, 30]
}
```

```rust
fn main() {
    let mut numbers = vec![10, 20, 30];
    let last = numbers.pop();

    println!("{:?}", last); // output: Some(30)
    println!("{:?}", numbers); // output: [10, 20]
}
```

```rust
fn main() {
    let mut numbers = vec![30, 10, 20];
    numbers.sort();
    numbers.reverse();

    println!("{:?}", numbers); // output: [30, 20, 10]
}
```

```rust
fn main() {
    let mut a = vec![1, 2];
    let mut b = vec![3, 4];

    a.append(&mut b);

    println!("{:?}", a); // output: [1, 2, 3, 4]
    println!("{:?}", b); // output: []
}
```

Safe access:

```rust
fn main() {
    let numbers = vec![10, 20, 30];

    match numbers.get(1) {
        Some(n) => println!("{}", n), // output: 20
        None => println!("not found"), // output: not found if index is missing
    }
}
```

---

## 12. Tuples

Tuples group values that may have different types.

```rust
fn main() {
    let person = ("Abbas", 25, true);

    println!("{}", person.0); // output: Abbas
    println!("{}", person.1); // output: 25
    println!("{}", person.2); // output: true
}
```

Destructuring:

```rust
fn main() {
    let point = (10, 20);
    let (x, y) = point;

    println!("x = {}, y = {}", x, y); // output: x = 10, y = 20
}
```

Function returning tuple:

```rust
fn min_max(a: i32, b: i32) -> (i32, i32) {
    (a.min(b), a.max(b))
}

fn main() {
    println!("{:?}", min_max(10, 3)); // output: (3, 10)
}
```

---

## 13. Iteration Basics

Detailed iterator transformations are in `cheat_sheet3.md`. These are the core forms.

### `for item in collection`

May move/consume owned collections.

```rust
fn main() {
    let numbers = [1, 2, 3];

    for n in numbers {
        println!("{}", n); // output: 1 then 2 then 3
    }
}
```

### `for item in &collection`

Borrows values.

```rust
fn main() {
    let numbers = vec![1, 2, 3];

    for n in &numbers {
        println!("{}", n); // output: 1 then 2 then 3
    }

    println!("{:?}", numbers); // output: [1, 2, 3]
}
```

### `.iter() -> iterator over &T`

```rust
fn main() {
    let numbers = vec![1, 2, 3];

    for n in numbers.iter() {
        println!("{}", n); // output: 1 then 2 then 3
    }
}
```

### `.iter_mut() -> iterator over &mut T`

```rust
fn main() {
    let mut numbers = vec![1, 2, 3];

    for n in numbers.iter_mut() {
        *n *= 2;
    }

    println!("{:?}", numbers); // output: [2, 4, 6]
}
```

### `.into_iter() -> iterator over T`

Consumes a `Vec`.

```rust
fn main() {
    let numbers = vec![1, 2, 3];

    for n in numbers.into_iter() {
        println!("{}", n); // output: 1 then 2 then 3
    }
}
```

### Common iterator methods

| Method | Common return type | Meaning |
|---|---|---|
| `.map(...)` | iterator | transform items |
| `.filter(...)` | iterator | keep matching items |
| `.collect::<Vec<T>>()` | `Vec<T>` | build collection |
| `.copied()` | iterator over `T` | copy `&T` into `T` |
| `.cloned()` | iterator over `T` | clone `&T` into `T` |
| `.enumerate()` | iterator over `(usize, item)` | add index |
| `.sum()` | `T` | add items |
| `.product()` | `T` | multiply items |
| `.count()` | `usize` | count items |
| `.find(...)` | `Option<item>` | first matching item |
| `.any(...)` | `bool` | at least one matches |
| `.all(...)` | `bool` | all match |

```rust
fn main() {
    let numbers = vec![1, 2, 3];
    let doubled: Vec<i32> = numbers.iter().copied().map(|n| n * 2).collect();

    println!("{:?}", doubled); // output: [2, 4, 6]
}
```

```rust
fn main() {
    let numbers = vec![1, 2, 3, 4];
    let evens: Vec<i32> = numbers.iter().copied().filter(|n| *n % 2 == 0).collect();

    println!("{:?}", evens); // output: [2, 4]
}
```

```rust
fn main() {
    let numbers = vec![1, 2, 3, 4];

    println!("{}", numbers.iter().copied().sum::<i32>()); // output: 10
    println!("{}", numbers.iter().copied().product::<i32>()); // output: 24
}
```

```rust
fn main() {
    let names = vec!["Abbas", "Ali"];

    for (index, name) in names.iter().enumerate() {
        println!("{}: {}", index, name); // output: 0: Abbas then 1: Ali
    }
}
```

---

## 14. Functions

### Function with no parameter and no return

```rust
fn greet() {
    println!("Hello"); // output: Hello
}

fn main() {
    greet();
}
```

### Function with parameters

```rust
fn greet_name(name: &str) {
    println!("Hello, {}", name); // output: Hello, Abbas if name is Abbas
}

fn main() {
    greet_name("Abbas");
}
```

### Function with return value

No semicolon on the final expression.

```rust
fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    println!("{}", add(2, 3)); // output: 5
}
```

### Explicit `return`

```rust
fn add(a: i32, b: i32) -> i32 {
    return a + b;
}

fn main() {
    println!("{}", add(2, 3)); // output: 5
}
```

### Function parameter patterns

| Parameter | Meaning |
|---|---|
| `value: T` | function takes ownership or copies |
| `value: &T` | function borrows read-only |
| `value: &mut T` | function borrows and can mutate |
| `text: &str` | good for reading text |
| `items: &[T]` | good for reading arrays/vectors |

```rust
fn add_score(scores: &mut Vec<i32>) {
    scores.push(100);
}

fn main() {
    let mut scores = vec![50, 60];
    add_score(&mut scores);

    println!("{:?}", scores); // output: [50, 60, 100]
}
```

---

## 15. `if` Statements

### Basic `if`

```rust
fn main() {
    let n = 10;

    if n > 0 {
        println!("positive"); // output: positive
    } else if n < 0 {
        println!("negative"); // output: negative if n is negative
    } else {
        println!("zero"); // output: zero if n is zero
    }
}
```

### `if` as expression

```rust
fn main() {
    let age = 20;

    let status = if age >= 18 {
        "adult"
    } else {
        "minor"
    };

    println!("{}", status); // output: adult
}
```

Both branches must return the same type.

---

## 16. `match`

### Basic `match`

```rust
fn main() {
    let number = 2;

    match number {
        1 => println!("one"), // output: one if number is 1
        2 => println!("two"), // output: two
        _ => println!("other"), // output: other if no branch matches
    }
}
```

### `match` as expression

```rust
fn main() {
    let number = 2;

    let word = match number {
        1 => "one",
        2 => "two",
        _ => "other",
    };

    println!("{}", word); // output: two
}
```

---

## 17. Loops

### `for`

```rust
fn main() {
    for n in 1..=3 {
        println!("{}", n); // output: 1 then 2 then 3
    }
}
```

### `while`

```rust
fn main() {
    let mut n = 3;

    while n > 0 {
        println!("{}", n); // output: 3 then 2 then 1
        n -= 1;
    }
}
```

### `loop`

```rust
fn main() {
    let mut n = 1;

    loop {
        println!("{}", n); // output: 1
        break;
    }
}
```

### `break` with value

```rust
fn main() {
    let mut n = 1;

    let result = loop {
        n *= 2;

        if n > 10 {
            break n;
        }
    };

    println!("{}", result); // output: 16
}
```

### `continue` and `break`

```rust
fn main() {
    for n in 1..=5 {
        if n == 2 {
            continue;
        }

        if n == 5 {
            break;
        }

        println!("{}", n); // output: 1 then 3 then 4
    }
}
```

---

## 18. References and Borrowing Syntax

### `&value -> &T`

Borrow a value.

```rust
fn main() {
    let x = 10;
    let r = &x;

    println!("{}", r); // output: 10
}
```

### `*reference -> T`

Dereference a reference.

```rust
fn main() {
    let x = 10;
    let r = &x;

    println!("{}", *r + 1); // output: 11
}
```

### `&mut value -> &mut T`

Mutable borrow.

```rust
fn add_one(n: &mut i32) {
    *n += 1;
}

fn main() {
    let mut x = 10;
    add_one(&mut x);

    println!("{}", x); // output: 11
}
```

Rule table:

| Situation | Use |
|---|---|
| Function expects `T` | pass `value` |
| Function expects `&T` | pass `&value` |
| Function expects `&mut T` | pass `&mut value` |
| You have `&T` but need `T` | use `*reference` |
| Function only reads | use `&` |
| Function changes original value | use `&mut` |

---

## 19. Option Basics

`Option<T>` is used when a value may or may not exist.

```rust
Some(value)
None
```

Common methods that return `Option`:

| Method | Common return type |
|---|---|
| `vec.pop()` | `Option<T>` |
| `vec.get(index)` | `Option<&T>` |
| `vec.first()` | `Option<&T>` |
| `vec.last()` | `Option<&T>` |
| `iter.find(...)` | `Option<T>` or `Option<&T>` |

### Handle `Option` with `match`

```rust
fn main() {
    let numbers = vec![10, 20, 30];

    match numbers.get(1) {
        Some(n) => println!("{}", n), // output: 20
        None => println!("not found"), // output: not found if index is missing
    }
}
```

### Handle `Option` with `if let`

```rust
fn main() {
    let numbers = vec![10, 20, 30];

    if let Some(n) = numbers.first() {
        println!("{}", n); // output: 10
    }
}
```

### `.unwrap_or(default) -> T`

```rust
fn main() {
    let numbers = vec![10, 20];
    let value = numbers.get(10).copied().unwrap_or(0);

    println!("{}", value); // output: 0
}
```

### `.expect(message) -> T or panic`

```rust
fn main() {
    let numbers = vec![10, 20];
    let value = numbers.first().expect("empty list");

    println!("{}", value); // output: 10
}
```

---

## 20. Result Basics

`Result<T, E>` is used when an operation can succeed or fail.

```rust
Ok(value)
Err(error)
```

Common methods that return `Result`:

| Method | Common return type |
|---|---|
| `"42".parse::<i32>()` | `Result<i32, ParseIntError>` |
| `x.try_into()` | `Result<T, E>` |
| file operations | `Result<T, E>` |

### Handle `Result` with `match`

```rust
fn main() {
    let result = "42".parse::<i32>();

    match result {
        Ok(n) => println!("number = {}", n), // output: number = 42
        Err(_) => println!("not a number"), // output: not a number if parse fails
    }
}
```

### Handle `Result` with `if let`

```rust
fn main() {
    let result = "42".parse::<i32>();

    if let Ok(n) = result {
        println!("{}", n); // output: 42
    }
}
```

### `.unwrap_or(default) -> T`

```rust
fn main() {
    let n = "abc".parse::<i32>().unwrap_or(0);

    println!("{}", n); // output: 0
}
```

### `.expect(message) -> T or panic`

```rust
fn main() {
    let n = "42".parse::<i32>().expect("not a number");

    println!("{}", n); // output: 42
}
```

---

## 21. Struct Basics

Structs group named fields.

```rust
struct User {
    name: String,
    age: u32,
}

fn main() {
    let user = User {
        name: String::from("Abbas"),
        age: 25,
    };

    println!("{} is {}", user.name, user.age); // output: Abbas is 25
}
```

### Updating fields

The variable must be mutable.

```rust
struct Counter {
    value: i32,
}

fn main() {
    let mut counter = Counter { value: 0 };
    counter.value += 1;

    println!("{}", counter.value); // output: 1
}
```

### Struct update syntax

```rust
struct User {
    name: String,
    age: u32,
}

fn main() {
    let user1 = User {
        name: String::from("Abbas"),
        age: 25,
    };

    let user2 = User {
        name: String::from("Ali"),
        ..user1
    };

    println!("{} is {}", user2.name, user2.age); // output: Ali is 25
}
```

---

## 22. `impl` Blocks and Methods

### Associated function: `Type::new() -> Type`

No `self` parameter.

```rust
struct Counter {
    value: i32,
}

impl Counter {
    fn new() -> Counter {
        Counter { value: 0 }
    }
}

fn main() {
    let c = Counter::new();
    println!("{}", c.value); // output: 0
}
```

### Method with `&self`

Reads the value.

```rust
struct User {
    name: String,
}

impl User {
    fn greet(&self) {
        println!("Hello, {}", self.name); // output: Hello, Abbas if name is Abbas
    }
}

fn main() {
    let user = User {
        name: String::from("Abbas"),
    };

    user.greet();
}
```

### Method with `&mut self`

Changes the value.

```rust
struct Counter {
    value: i32,
}

impl Counter {
    fn increment(&mut self) {
        self.value += 1;
    }
}

fn main() {
    let mut c = Counter { value: 0 };
    c.increment();

    println!("{}", c.value); // output: 1
}
```

### Method with `self`

Takes ownership.

```rust
struct Message {
    text: String,
}

impl Message {
    fn consume(self) {
        println!("{}", self.text); // output: hello
    }
}

fn main() {
    let msg = Message {
        text: String::from("hello"),
    };

    msg.consume();
}
```

---

## 23. Enum Basics

Enums define possible variants.

```rust
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn main() {
    let direction = Direction::Up;

    match direction {
        Direction::Up => println!("up"), // output: up
        Direction::Down => println!("down"), // output: down if direction is Down
        Direction::Left => println!("left"), // output: left if direction is Left
        Direction::Right => println!("right"), // output: right if direction is Right
    }
}
```

### Enum with data

```rust
enum Message {
    Quit,
    Write(String),
    Move { x: i32, y: i32 },
}

fn main() {
    let msg = Message::Write(String::from("hello"));

    match msg {
        Message::Quit => println!("quit"), // output: quit if message is Quit
        Message::Write(text) => println!("{}", text), // output: hello
        Message::Move { x, y } => println!("{} {}", x, y), // output: x y if message is Move
    }
}
```

Remember: `Option` and `Result` are also enums.

```rust
Option<T> = Some(T) or None
Result<T, E> = Ok(T) or Err(E)
```

---

## 24. Common Rust Syntax Symbols

| Syntax | Meaning | Example |
|---|---|---|
| `fn` | define function | `fn main()` |
| `let` | create variable | `let x = 5;` |
| `mut` | allow change | `let mut x = 5;` |
| `const` | constant | `const N: i32 = 10;` |
| `:` | type annotation | `let x: i32 = 5;` |
| `->` | function return type | `fn add() -> i32` |
| `;` | end statement | `let x = 5;` |
| `{}` | block / formatting placeholder | `if x > 0 {}` and formatting placeholder `{}` |
| `()` | parameters or call | `add(1, 2)` |
| `[]` | array / indexing | `[1, 2, 3]` |
| `.` | method call | `text.trim()` |
| `::` | path / type / module | `String::new()` |
| `!` | macro call | macro calls end with `!` |
| `&` | borrow | `&x` |
| `&mut` | mutable borrow | `&mut x` |
| `*` | dereference | `*n` |
| `=>` | match arm | `Some(x) => x` |
| `..` | range / struct update | `1..5`, `..user` |
| `..=` | inclusive range | `1..=5` |
| `_` | ignore / default pattern | `_ => ...` |

---

## 25. Common Beginner Errors and Quick Fixes

### Ambiguous numeric type

Problem:

```rust
let numbers = vec![-1, 2, 3];
let abs_values: Vec<i32> = numbers.iter().map(|&n| n.abs()).collect();
```

Fix:

```rust
let numbers: Vec<i32> = vec![-1, 2, 3];
```

### `push()` returns `()`

Wrong idea:

```rust
let result = numbers.push(4);
```

Correct:

```rust
numbers.push(4);
println!("{:?}", numbers); // output: updated vector
```

### `replace()` returns a new `String`

Wrong idea:

```rust
clean.replace("rust", "Rust");
```

Correct:

```rust
clean = clean.replace("rust", "Rust");
```

### `.contains()` expects a reference

```rust
fn main() {
    let numbers = vec![1, 2, 3];
    println!("{}", numbers.contains(&2)); // output: true
}
```

### Function expects `&T`

```rust
fn print_number(n: &i32) {
    println!("{}", n); // output: borrowed number
}

fn main() {
    let x = 10;
    print_number(&x);
}
```

---

## 26. Quick Practice

### Practice 1 — Fix score

```rust
fn fix_score(score: i32) -> i32 {
    (score + 10).clamp(0, 100)
}

fn main() {
    println!("{}", fix_score(95)); // output: 100
    println!("{}", fix_score(-20)); // output: 0
}
```

### Practice 2 — Clean name

```rust
fn clean_name(name: &str) -> String {
    name.trim().to_lowercase().replace(" ", "_")
}

fn main() {
    println!("{}", clean_name("  Abbas Hasan  ")); // output: abbas_hasan
}
```

### Practice 3 — Clean scores

```rust
fn clean_scores(scores: &[i32]) -> Vec<i32> {
    let mut result: Vec<i32> = scores
        .iter()
        .copied()
        .filter(|score| *score >= 0 && *score <= 100)
        .collect();

    result.sort();
    result.reverse();

    result
}

fn main() {
    println!("{:?}", clean_scores(&[90, -5, 120, 75])); // output: [90, 75]
}
```

### Practice 4 — Safe item access

```rust
fn get_item(items: &[i32], index: usize) -> String {
    match items.get(index) {
        Some(item) => format!("item = {}", item),
        None => String::from("not found"),
    }
}

fn main() {
    println!("{}", get_item(&[10, 20, 30], 1)); // output: item = 20
    println!("{}", get_item(&[10, 20, 30], 9)); // output: not found
}
```

---

End of `cheat_sheet1.md`.
