# Rust Cheat Sheet 10 — Macros, Closures, Advanced Iterators, and Exam Strategy

This is **cheat_sheet10.md**.

This sheet focuses on stronger Rust patterns that often appear near the end of exam-style exercises:

- closures
- passing closures to functions
- advanced iterator methods
- custom iterators
- `macro_rules!`
- formatting macros
- `Drop`
- reading tests quickly
- exam workflow and debugging strategy

This sheet avoids repeating full method sections already covered in earlier cheat sheets.

---

# 1. Quick Index

```text
1. Closure syntax
2. Closure type inference
3. Closure captures
4. move closures
5. Passing closures to functions
6. Fn, FnMut, FnOnce basics
7. Returning closures basics
8. Advanced iterator method index
9. fold
10. reduce
11. flat_map
12. flatten
13. zip
14. chain
15. take, skip, step_by
16. rev
17. scan
18. inspect
19. peekable
20. Custom Iterator implementation
21. next()
22. macro_rules! basics
23. Macro arguments
24. Macro repetition
25. Macro map pattern
26. Macro calculator pattern
27. Formatting macros
28. Debugging macros
29. Drop behavior
30. Reading exam tests
31. Inferring expected function signatures
32. Fast exam workflow
33. Common advanced mistakes and fixes
34. Practice exercises
```

---

# 2. Closures

A closure is like a small anonymous function.

## Closure syntax index

| Syntax | Meaning | Example |
|---|---|---|
| `|x| x + 1` | one parameter | `let add_one = |x| x + 1;` |
| `|a, b| a + b` | two parameters | `let add = |a, b| a + b;` |
| `|x: i32| -> i32 { x + 1 }` | typed closure | explicit types |
| `{ ... }` | closure body block | multiple lines |

## Basic closure

```rust
fn main() {
    let add_one = |x| x + 1;

    println!("{}", add_one(5)); // output: 6
}
```

## Closure with two parameters

```rust
fn main() {
    let add = |a, b| a + b;

    println!("{}", add(2, 3)); // output: 5
}
```

## Closure with explicit types

```rust
fn main() {
    let square = |n: i32| -> i32 {
        n * n
    };

    println!("{}", square(4)); // output: 16
}
```

---

# 3. Closure Type Inference

Rust usually guesses closure parameter and return types from usage.

```rust
fn main() {
    let double = |n| n * 2;

    println!("{}", double(10)); // output: 20
}
```

Once Rust decides the closure type, it stays fixed.

```rust
fn main() {
    let identity = |x| x;

    println!("{}", identity(10)); // output: 10
    // println!("{}", identity("hello")); // output: compiler error if uncommented
}
```

Why?

```text
After identity(10), Rust treats identity as a closure that takes i32.
```

---

# 4. Closure Captures

Closures can use variables from outside.

## Capture by shared borrow

```rust
fn main() {
    let name = String::from("Abbas");

    let greet = || {
        println!("Hello, {}", name); // output: Hello, Abbas
    };

    greet();
    println!("{}", name); // output: Abbas
}
```

The closure only reads `name`, so `name` can still be used after.

## Capture by mutable borrow

```rust
fn main() {
    let mut count = 0;

    let mut increase = || {
        count += 1;
        println!("count = {}", count); // output: count = 1 then count = 2
    };

    increase();
    increase();
}
```

Why `let mut increase`?

```text
The closure changes captured state, so calling the closure mutates the closure itself.
```

## Capture by move

```rust
fn main() {
    let name = String::from("Abbas");

    let show = move || {
        println!("{}", name); // output: Abbas
    };

    show();
    // println!("{}", name); // output: compiler error if uncommented, name moved
}
```

`move` forces the closure to take ownership of captured values.

---

# 5. Passing Closures to Functions

## Closure trait index

| Trait | Meaning | Common situation |
|---|---|---|
| `Fn` | reads captured values | closure does not mutate or consume captures |
| `FnMut` | mutates captured values | closure changes outside state |
| `FnOnce` | consumes captured values | closure can only run once |

## Function accepts a closure: `Fn`

```rust
fn apply<F>(value: i32, f: F) -> i32
where
    F: Fn(i32) -> i32,
{
    f(value)
}

fn main() {
    let result = apply(5, |x| x * 2);

    println!("{}", result); // output: 10
}
```

## Function accepts a mutating closure: `FnMut`

```rust
fn repeat_three_times<F>(mut f: F)
where
    F: FnMut(),
{
    f();
    f();
    f();
}

fn main() {
    let mut count = 0;

    repeat_three_times(|| {
        count += 1;
    });

    println!("{}", count); // output: 3
}
```

## Function accepts a one-time closure: `FnOnce`

```rust
fn consume_once<F>(f: F)
where
    F: FnOnce(),
{
    f();
}

fn main() {
    let text = String::from("owned");

    consume_once(move || {
        println!("{}", text); // output: owned
    });
}
```

---

# 6. Returning Closures Basics

Use `impl Fn(...) -> ...` when returning a closure.

```rust
fn make_adder(amount: i32) -> impl Fn(i32) -> i32 {
    move |x| x + amount
}

fn main() {
    let add_five = make_adder(5);

    println!("{}", add_five(10)); // output: 15
}
```

Why `move`?

```text
The returned closure must own amount because make_adder ends before the closure is used.
```

---

# 7. Advanced Iterator Method Index

| Method | Common return type | Meaning |
|---|---|---|
| `.fold(init, f)` | value | accumulate into one value |
| `.reduce(f)` | `Option<T>` | combine items without initial value |
| `.flat_map(f)` | iterator | map each item to many items and flatten |
| `.flatten()` | iterator | flatten nested iterators/options/results |
| `.zip(other)` | iterator of pairs | combine two iterators side by side |
| `.chain(other)` | iterator | append one iterator after another |
| `.take(n)` | iterator | first `n` items |
| `.skip(n)` | iterator | ignore first `n` items |
| `.step_by(n)` | iterator | take every nth item |
| `.rev()` | iterator | reverse iterator direction |
| `.scan(state, f)` | iterator | stateful transformation |
| `.inspect(f)` | iterator | debug items without changing them |
| `.peekable()` | `Peekable<I>` | look at next item without consuming |
| `.next()` | `Option<T>` | get next item from iterator |

---

# 8. `.fold(init, closure) -> value`

`.fold()` is used to build one result from many items.

```rust
fn main() {
    let numbers = vec![1, 2, 3, 4];

    let total = numbers.iter().fold(0, |acc, n| acc + n);

    println!("{}", total); // output: 10
}
```

## Product with fold

```rust
fn main() {
    let numbers = vec![2, 3, 4];

    let product = numbers.iter().fold(1, |acc, n| acc * n);

    println!("{}", product); // output: 24
}
```

## Build a string with fold

```rust
fn main() {
    let words = vec!["Rust", "is", "fun"];

    let sentence = words.iter().fold(String::new(), |mut acc, word| {
        if !acc.is_empty() {
            acc.push(' ');
        }
        acc.push_str(word);
        acc
    });

    println!("{}", sentence); // output: Rust is fun
}
```

---

# 9. `.reduce(closure) -> Option<T>`

`.reduce()` combines items, but it has no starting value.

Because the iterator may be empty, it returns `Option<T>`.

```rust
fn main() {
    let numbers = vec![5, 2, 9, 1];

    let max = numbers.into_iter().reduce(|a, b| a.max(b));

    println!("{:?}", max); // output: Some(9)
}
```

## Resolve `.reduce()` with `unwrap_or`

```rust
fn main() {
    let numbers: Vec<i32> = vec![];

    let total = numbers.into_iter().reduce(|a, b| a + b).unwrap_or(0);

    println!("{}", total); // output: 0
}
```

---

# 10. `.flat_map() -> iterator`

`.flat_map()` is useful when one input item becomes many output items.

```rust
fn main() {
    let words = vec!["hi", "rust"];

    let letters: Vec<char> = words
        .iter()
        .flat_map(|word| word.chars())
        .collect();

    println!("{:?}", letters); // output: ['h', 'i', 'r', 'u', 's', 't']
}
```

## Split sentences into words

```rust
fn main() {
    let lines = vec!["hello rust", "learn fast"];

    let words: Vec<&str> = lines
        .iter()
        .flat_map(|line| line.split_whitespace())
        .collect();

    println!("{:?}", words); // output: ["hello", "rust", "learn", "fast"]
}
```

---

# 11. `.flatten() -> iterator`

`.flatten()` removes one level of nesting.

## Flatten nested vectors

```rust
fn main() {
    let nested = vec![vec![1, 2], vec![3, 4]];

    let flat: Vec<i32> = nested.into_iter().flatten().collect();

    println!("{:?}", flat); // output: [1, 2, 3, 4]
}
```

## Flatten `Option`s

```rust
fn main() {
    let values = vec![Some(1), None, Some(3)];

    let numbers: Vec<i32> = values.into_iter().flatten().collect();

    println!("{:?}", numbers); // output: [1, 3]
}
```

---

# 12. `.zip(other) -> iterator of pairs`

`.zip()` pairs items from two iterators.

```rust
fn main() {
    let names = vec!["Ali", "Sara"];
    let scores = vec![90, 80];

    for (name, score) in names.iter().zip(scores.iter()) {
        println!("{} = {}", name, score); // output: Ali = 90 then Sara = 80
    }
}
```

## Build pairs with collect

```rust
fn main() {
    let a = vec![1, 2, 3];
    let b = vec![10, 20, 30];

    let pairs: Vec<(i32, i32)> = a.into_iter().zip(b.into_iter()).collect();

    println!("{:?}", pairs); // output: [(1, 10), (2, 20), (3, 30)]
}
```

---

# 13. `.chain(other) -> iterator`

`.chain()` joins two iterators end-to-end.

```rust
fn main() {
    let a = vec![1, 2];
    let b = vec![3, 4];

    let joined: Vec<i32> = a.into_iter().chain(b.into_iter()).collect();

    println!("{:?}", joined); // output: [1, 2, 3, 4]
}
```

---

# 14. `.take()`, `.skip()`, `.step_by()`

## Method table

| Method | Meaning | Example |
|---|---|---|
| `.take(n)` | keep first `n` items | first 3 |
| `.skip(n)` | ignore first `n` items | skip first 2 |
| `.step_by(n)` | take every nth item | every 2nd item |

## Take

```rust
fn main() {
    let numbers = vec![1, 2, 3, 4, 5];

    let first_three: Vec<i32> = numbers.iter().copied().take(3).collect();

    println!("{:?}", first_three); // output: [1, 2, 3]
}
```

## Skip

```rust
fn main() {
    let numbers = vec![1, 2, 3, 4, 5];

    let after_two: Vec<i32> = numbers.iter().copied().skip(2).collect();

    println!("{:?}", after_two); // output: [3, 4, 5]
}
```

## Step by

```rust
fn main() {
    let numbers = vec![1, 2, 3, 4, 5, 6];

    let every_two: Vec<i32> = numbers.iter().copied().step_by(2).collect();

    println!("{:?}", every_two); // output: [1, 3, 5]
}
```

---

# 15. `.rev() -> reversed iterator`

```rust
fn main() {
    let numbers = vec![1, 2, 3];

    let reversed: Vec<i32> = numbers.iter().copied().rev().collect();

    println!("{:?}", reversed); // output: [3, 2, 1]
}
```

Useful for reverse traversal without changing the original collection.

```rust
fn main() {
    let text = "rust";
    let reversed: String = text.chars().rev().collect();

    println!("{}", reversed); // output: tsur
}
```

---

# 16. `.scan(state, closure) -> iterator`

`.scan()` is like `.map()`, but it keeps state.

```rust
fn main() {
    let numbers = vec![1, 2, 3, 4];

    let partial_sums: Vec<i32> = numbers
        .into_iter()
        .scan(0, |state, n| {
            *state += n;
            Some(*state)
        })
        .collect();

    println!("{:?}", partial_sums); // output: [1, 3, 6, 10]
}
```

This is useful for exam tasks like partial sums.

---

# 17. `.inspect() -> iterator`

`.inspect()` is useful for debugging iterator chains.

```rust
fn main() {
    let result: Vec<i32> = vec![1, 2, 3]
        .into_iter()
        .inspect(|n| println!("before = {}", n)) // output: before = 1 then before = 2 then before = 3
        .map(|n| n * 2)
        .collect();

    println!("{:?}", result); // output: [2, 4, 6]
}
```

Use `.inspect()` while debugging, then remove it when the solution is clean.

---

# 18. `.peekable() -> Peekable<I>`

`.peekable()` lets you look at the next item without consuming it.

```rust
fn main() {
    let mut chars = "ab".chars().peekable();

    println!("{:?}", chars.peek()); // output: Some('a')
    println!("{:?}", chars.next()); // output: Some('a')
    println!("{:?}", chars.next()); // output: Some('b')
}
```

Useful in parsers and interpreters.

---

# 19. `next() -> Option<T>`

Iterators produce values one by one using `.next()`.

```rust
fn main() {
    let mut iter = vec![10, 20].into_iter();

    println!("{:?}", iter.next()); // output: Some(10)
    println!("{:?}", iter.next()); // output: Some(20)
    println!("{:?}", iter.next()); // output: None
}
```

Because `.next()` may run out of items, it returns `Option<T>`.

---

# 20. Custom Iterator Implementation

To create a custom iterator, implement the `Iterator` trait and define `next()`.

## Custom counter

```rust
struct Counter {
    current: u32,
    end: u32,
}

impl Counter {
    fn new(end: u32) -> Counter {
        Counter { current: 0, end }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current < self.end {
            self.current += 1;
            Some(self.current)
        } else {
            None
        }
    }
}

fn main() {
    let values: Vec<u32> = Counter::new(3).collect();

    println!("{:?}", values); // output: [1, 2, 3]
}
```

Key parts:

```rust
type Item = u32;
fn next(&mut self) -> Option<Self::Item>
```

---

# 21. Macro Basics

Macros generate code before compilation.

Common built-in macros:

| Macro | Meaning |
|---|---|
| `println!` // output: prints a line | print line |
| `format!` | create `String` |
| `vec!` | create vector |
| `panic!` | stop program with error |
| `assert!` | test boolean condition |
| `assert_eq!` | test equality |

Macros use `!`:

```rust
println!("hello"); // output: hello
```

---

# 22. `macro_rules!` Basic Syntax

```rust
macro_rules! say_hello {
    () => {
        println!("hello"); // output: hello
    };
}

fn main() {
    say_hello!();
}
```

Pattern:

```rust
macro_rules! name {
    (pattern) => {
        generated_code
    };
}
```

---

# 23. Macro Arguments

## Expression argument: `$x:expr`

```rust
macro_rules! square {
    ($x:expr) => {
        $x * $x
    };
}

fn main() {
    println!("{}", square!(4)); // output: 16
}
```

## Identifier argument: `$name:ident`

```rust
macro_rules! make_zero {
    ($name:ident) => {
        let $name = 0;
    };
}

fn main() {
    make_zero!(score);

    println!("{}", score); // output: 0
}
```

## Type argument: `$t:ty`

```rust
macro_rules! default_value {
    ($t:ty) => {
        <$t>::default()
    };
}

fn main() {
    let x: i32 = default_value!(i32);

    println!("{}", x); // output: 0
}
```

---

# 24. Macro Fragment Specifier Index

| Fragment | Meaning | Example input |
|---|---|---|
| `expr` | expression | `1 + 2`, `x`, `foo()` |
| `ident` | identifier | `name`, `score` |
| `ty` | type | `i32`, `Vec<String>` |
| `stmt` | statement | `let x = 1;` |
| `block` | block | `{ x + 1 }` |
| `pat` | pattern | `Some(x)` |
| `literal` | literal | `5`, `"hi"` |
| `tt` | token tree | flexible raw tokens |

---

# 25. Macro Repetition

Use:

```rust
$( pattern ),*
```

Meaning:

```text
repeat pattern zero or more times, separated by commas
```

## Sum macro

```rust
macro_rules! sum {
    ($($x:expr),*) => {
        0 $(+ $x)*
    };
}

fn main() {
    println!("{}", sum!(1, 2, 3)); // output: 6
}
```

## Vector macro style

```rust
macro_rules! my_vec {
    ($($x:expr),*) => {
        {
            let mut temp = Vec::new();
            $(temp.push($x);)*
            temp
        }
    };
}

fn main() {
    let numbers = my_vec![1, 2, 3];

    println!("{:?}", numbers); // output: [1, 2, 3]
}
```

---

# 26. Macro Map Pattern

This pattern is useful for exercises like `macro_map`.

```rust
use std::collections::HashMap;

macro_rules! map {
    ($($key:expr => $value:expr),* $(,)?) => {
        {
            let mut m = HashMap::new();
            $(m.insert($key, $value);)*
            m
        }
    };
}

fn main() {
    let scores = map! {
        "Ali" => 90,
        "Sara" => 80,
    };

    println!("{:?}", scores.get("Ali")); // output: Some(90)
}
```

Important part:

```rust
$(,)?
```

allows an optional trailing comma.

---

# 27. Macro Calculator Pattern

This pattern matches different operators.

```rust
macro_rules! calc {
    ($a:expr, +, $b:expr) => {
        $a + $b
    };
    ($a:expr, -, $b:expr) => {
        $a - $b
    };
    ($a:expr, *, $b:expr) => {
        $a * $b
    };
    ($a:expr, /, $b:expr) => {
        $a / $b
    };
}

fn main() {
    println!("{}", calc!(10, +, 5)); // output: 15
    println!("{}", calc!(10, -, 5)); // output: 5
    println!("{}", calc!(10, *, 5)); // output: 50
    println!("{}", calc!(10, /, 5)); // output: 2
}
```

---

# 28. Formatting Macros

| Macro | Return / behavior | Use |
|---|---|---|
| `println!()` // output: stdout line | prints to stdout | user output |
| `eprintln!()` // output: stderr line | prints to stderr | error/debug output |
| `format!()` | `String` | build string |
| `write!()` | `Result` | write formatted text into buffer |
| `writeln!()` | `Result` | write formatted text with newline |

## `format!() -> String`

```rust
fn main() {
    let name = "Abbas";
    let text = format!("Hello, {}", name);

    println!("{}", text); // output: Hello, Abbas
}
```

## Debug formatting

```rust
fn main() {
    let numbers = vec![1, 2, 3];

    println!("{:?}", numbers); // output: [1, 2, 3]
}
```

## Pretty debug formatting

```rust
fn main() {
    let pair = (1, 2);

    println!("{:#?}", pair); // output: pretty debug view of (1, 2)
}
```

---

# 29. Drop Behavior

`Drop` runs automatically when a value goes out of scope.

```rust
struct Tracker {
    name: String,
}

impl Drop for Tracker {
    fn drop(&mut self) {
        println!("dropping {}", self.name); // output: dropping file
    }
}

fn main() {
    let _t = Tracker {
        name: String::from("file"),
    };
}
```

Output happens at the end of `main` when `_t` is dropped.

## Manual early drop

```rust
fn main() {
    let text = String::from("hello");

    drop(text);

    println!("dropped"); // output: dropped
    // println!("{}", text); // output: compiler error if uncommented, text moved into drop
}
```

---

# 30. Exam Strategy: Read Tests First

Many Rust exercises give you a function signature in tests.

Look for:

```text
1. Function name
2. Parameter types
3. Return type
4. Expected output
5. Ownership clues
```

Example test:

```rust
assert_eq!(double(4), 8);
```

You infer:

```rust
fn double(n: i32) -> i32 {
    n * 2
}
```

Example test:

```rust
assert_eq!(clean_name(" Abbas "), "abbas");
```

You infer:

```rust
fn clean_name(name: &str) -> String {
    name.trim().to_lowercase()
}
```

---

# 31. Inferring Function Signatures

## If test passes a string literal

```rust
my_func("hello")
```

Good parameter:

```rust
fn my_func(text: &str)
```

## If test passes a vector by reference

```rust
my_func(&numbers)
```

Good parameter:

```rust
fn my_func(numbers: &[i32])
```

## If function must modify input

```rust
add_item(&mut values)
```

Good parameter:

```rust
fn add_item(values: &mut Vec<i32>)
```

## If function returns a new vector

```rust
fn clean(values: &[i32]) -> Vec<i32>
```

## If function may not find value

```rust
fn find_item(values: &[i32]) -> Option<i32>
```

---

# 32. Fast Exam Workflow

Use this order:

```text
1. Read instructions.
2. Read tests / main example.
3. Write the exact function signature.
4. Start with the simplest working solution.
5. Run cargo test.
6. Fix compiler errors first.
7. Then fix logic errors.
8. Add edge cases.
9. Clean code only after it passes.
```

---

# 33. Debugging Compiler Errors

## Error: ambiguous numeric type

Wrong:

```rust
fn main() {
    let numbers = vec![-1, 2, 3];
    let fixed: Vec<i32> = numbers.iter().map(|&n| n.abs()).collect();

    println!("{:?}", fixed); // output: compiler error if type is ambiguous
}
```

Correct:

```rust
fn main() {
    let numbers: Vec<i32> = vec![-1, 2, 3];
    let fixed: Vec<i32> = numbers.iter().map(|&n| n.abs()).collect();

    println!("{:?}", fixed); // output: [1, 2, 3]
}
```

## Error: expected `T`, found `&T`

Usually caused by `.iter()`.

```rust
fn main() {
    let numbers = vec![1, 2, 3];

    let copied: Vec<i32> = numbers.iter().copied().collect();

    println!("{:?}", copied); // output: [1, 2, 3]
}
```

## Error: value borrowed after move

Wrong idea:

```rust
fn main() {
    let names = vec![String::from("Ali")];

    for name in names {
        println!("{}", name); // output: Ali
    }

    // println!("{:?}", names); // output: compiler error if uncommented, names moved
}
```

Fix by borrowing:

```rust
fn main() {
    let names = vec![String::from("Ali")];

    for name in &names {
        println!("{}", name); // output: Ali
    }

    println!("{:?}", names); // output: ["Ali"]
}
```

---

# 34. Common Exam Patterns

## Pattern: transform collection

```rust
fn double_all(numbers: &[i32]) -> Vec<i32> {
    numbers.iter().map(|n| n * 2).collect()
}

fn main() {
    println!("{:?}", double_all(&[1, 2, 3])); // output: [2, 4, 6]
}
```

## Pattern: filter collection

```rust
fn positives(numbers: &[i32]) -> Vec<i32> {
    numbers.iter().copied().filter(|n| *n > 0).collect()
}

fn main() {
    println!("{:?}", positives(&[-1, 2, 0, 3])); // output: [2, 3]
}
```

## Pattern: fold into answer

```rust
fn sum_squares(numbers: &[i32]) -> i32 {
    numbers.iter().fold(0, |acc, n| acc + n * n)
}

fn main() {
    println!("{}", sum_squares(&[1, 2, 3])); // output: 14
}
```

## Pattern: parse with `filter_map`

```rust
fn parse_numbers(texts: &[&str]) -> Vec<i32> {
    texts.iter().filter_map(|text| text.parse::<i32>().ok()).collect()
}

fn main() {
    println!("{:?}", parse_numbers(&["1", "a", "3"])); // output: [1, 3]
}
```

---

# 35. Practice Exercises

## Exercise 1 — Closure

Write a function:

```rust
fn apply_twice<F>(value: i32, f: F) -> i32
where
    F: Fn(i32) -> i32
```

Expected:

```rust
println!("{}", apply_twice(3, |x| x + 2)); // output: 7
```

## Exercise 2 — Fold

Write:

```rust
fn product(numbers: &[i32]) -> i32
```

Expected:

```rust
println!("{}", product(&[2, 3, 4])); // output: 24
```

## Exercise 3 — Flatten

Write:

```rust
fn flatten_numbers(numbers: Vec<Vec<i32>>) -> Vec<i32>
```

Expected:

```rust
println!("{:?}", flatten_numbers(vec![vec![1, 2], vec![3]])); // output: [1, 2, 3]
```

## Exercise 4 — Zip

Write:

```rust
fn pair_names_scores(names: Vec<&str>, scores: Vec<i32>) -> Vec<(&str, i32)>
```

Expected:

```rust
println!("{:?}", pair_names_scores(vec!["Ali", "Sara"], vec![90, 80])); // output: [("Ali", 90), ("Sara", 80)]
```

## Exercise 5 — Macro

Create a macro:

```rust
square!(5)
```

Expected:

```rust
println!("{}", square!(5)); // output: 25
```

## Exercise 6 — Map macro

Create a macro:

```rust
map! { "a" => 1, "b" => 2 }
```

Expected:

```rust
println!("{:?}", m.get("a")); // output: Some(1)
```

---

# 36. Final Exam Reminders

```text
1. Make types explicit when Rust says ambiguous.
2. Use &str for read-only string parameters.
3. Use &[T] for read-only array/vector parameters.
4. Use &mut T only when you must modify the original.
5. Use .iter().copied() for numeric iterator exercises.
6. Use match or unwrap_or for Option/Result when failure is possible.
7. Read tests before writing code.
8. Fix compiler errors before improving style.
9. Prefer simple passing code over clever code during exams.
10. After passing, simplify only if there is time.
```

End of cheat_sheet10.md.
