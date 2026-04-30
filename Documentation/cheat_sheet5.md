# Rust Cheat Sheet 5 — Ownership, Borrowing, References, `Box`, and `RefCell`

This is **cheat_sheet5.md**.

This sheet focuses on how Rust manages data:

- ownership
- move vs copy
- borrowing with `&`
- mutable borrowing with `&mut`
- dereferencing with `*`
- cloning
- function parameter choices
- ownership in loops
- returning owned values and references
- `Box<T>`
- recursive data with `Box`
- `RefCell<T>` and interior mutability
- common ownership compiler errors and fixes

Style rules used:

- Every `println!` line includes `// output:` on the same line.
- Method/function headings show return type notation where useful.
- Tables are placed near the related subject.

---

# 1. The Main Ownership Idea

Rust has one big rule:

```text
Each value has one owner.
When the owner goes out of scope, the value is dropped.
```

For simple values like numbers, Rust usually copies the value.

For heap values like `String` and `Vec`, Rust usually moves ownership.

```rust
fn main() {
    let x = 10;
    let y = x;

    println!("x = {}, y = {}", x, y); // output: x = 10, y = 10
}
```

This works because `i32` is `Copy`.

```rust
fn main() {
    let name = String::from("Abbas");
    let other = name;

    println!("{}", other); // output: Abbas
}
```

Here `name` moved into `other`.

After the move, `name` cannot be used.

---

# 2. Move vs Copy

## Move

A move transfers ownership.

```rust
fn main() {
    let a = String::from("Rust");
    let b = a;

    println!("{}", b); // output: Rust
}
```

`a` is no longer valid after:

```rust
let b = a;
```

## Copy

Copy duplicates simple stack values automatically.

```rust
fn main() {
    let a = 5;
    let b = a;

    println!("a = {}, b = {}", a, b); // output: a = 5, b = 5
}
```

## Common `Copy` types

| Type | Usually `Copy`? | Notes |
|---|---:|---|
| `i32`, `u32`, `usize` | yes | integers copy easily |
| `f64` | yes | floats copy easily |
| `bool` | yes | true/false copy easily |
| `char` | yes | single character |
| `&T` | yes | shared references are copyable |
| `String` | no | heap allocation, moves by default |
| `Vec<T>` | no | heap allocation, moves by default |

---

# 3. Scope and Drop

When a variable leaves its scope, Rust drops it automatically.

```rust
fn main() {
    {
        let name = String::from("Abbas");
        println!("{}", name); // output: Abbas
    }

    println!("done"); // output: done
}
```

`name` is dropped at the end of the inner block.

---

# 4. Borrowing with `&`

Borrowing means:

```text
Let code use a value without taking ownership.
```

## `&value -> &T`

| Syntax | Meaning | Type example |
|---|---|---|
| `x` | actual value | `i32` |
| `&x` | reference to value | `&i32` |
| `name` | actual owned string | `String` |
| `&name` | reference to string | `&String` |

```rust
fn print_name(name: &String) {
    println!("{}", name); // output: Abbas
}

fn main() {
    let name = String::from("Abbas");
    print_name(&name);

    println!("{}", name); // output: Abbas
}
```

Because `print_name` borrows `name`, `main` can still use `name` after the call.

## Prefer `&str` for read-only text

```rust
fn print_text(text: &str) {
    println!("{}", text); // output: Rust
}

fn main() {
    let word = String::from("Rust");
    print_text(&word);
}
```

`&str` is more flexible than `&String`.

---

# 5. Mutable Borrowing with `&mut`

Use `&mut` when a function needs to change the original value.

## `&mut value -> &mut T`

| Syntax | Meaning |
|---|---|
| `let mut x` | variable can change |
| `&mut x` | give mutable access to another function/block |
| `param: &mut T` | function accepts mutable reference |

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

You need both:

```rust
let mut x = 10;
add_one(&mut x);
```

---

# 6. Dereferencing with `*`

`*reference` means:

```text
Get the value behind the reference.
```

## `*reference -> T`

```rust
fn main() {
    let x = 10;
    let r = &x;

    println!("{}", *r + 5); // output: 15
}
```

When changing through `&mut`, dereference first:

```rust
fn double(n: &mut i32) {
    *n *= 2;
}

fn main() {
    let mut x = 7;
    double(&mut x);

    println!("{}", x); // output: 14
}
```

---

# 7. Function Parameter Choices

This table is very important for exercises.

| Parameter | Meaning | Call with | Use when |
|---|---|---|---|
| `T` | take ownership/copy | `value` | function owns or consumes value |
| `&T` | read-only borrow | `&value` | function only reads |
| `&mut T` | mutable borrow | `&mut value` | function changes original |
| `&str` | borrowed text | `"text"` or `&string` | reading text |
| `&[T]` | borrowed list/slice | `&array` or `&vec` | reading list |

## Take ownership: `T`

```rust
fn consume(text: String) {
    println!("{}", text); // output: hello
}

fn main() {
    let s = String::from("hello");
    consume(s);
}
```

After `consume(s)`, `s` cannot be used.

## Read-only borrow: `&T`

```rust
fn show(text: &String) {
    println!("{}", text); // output: hello
}

fn main() {
    let s = String::from("hello");
    show(&s);

    println!("{}", s); // output: hello
}
```

## Mutable borrow: `&mut T`

```rust
fn add_exclamation(text: &mut String) {
    text.push('!');
}

fn main() {
    let mut s = String::from("hello");
    add_exclamation(&mut s);

    println!("{}", s); // output: hello!
}
```

---

# 8. Borrowing Rules

Rust allows either:

```text
Many immutable references
```

or:

```text
One mutable reference
```

But not both at the same time.

## Many read-only references are okay

```rust
fn main() {
    let x = 10;
    let a = &x;
    let b = &x;

    println!("{} {}", a, b); // output: 10 10
}
```

## One mutable reference is okay

```rust
fn main() {
    let mut x = 10;
    let a = &mut x;

    *a += 5;

    println!("{}", a); // output: 15
}
```

## Mutable borrow ends after last use

```rust
fn main() {
    let mut x = 10;

    let a = &mut x;
    *a += 1;
    println!("{}", a); // output: 11

    let b = &x;
    println!("{}", b); // output: 11
}
```

This works because `a` is not used after its last print statement.

---

# 9. `String` Ownership Patterns

## Move a `String`

```rust
fn main() {
    let a = String::from("Rust");
    let b = a;

    println!("{}", b); // output: Rust
}
```

## Borrow a `String`

```rust
fn print_len(text: &String) {
    println!("{}", text.len()); // output: 4
}

fn main() {
    let a = String::from("Rust");
    print_len(&a);

    println!("{}", a); // output: Rust
}
```

## Best read-only text parameter: `&str`

```rust
fn print_len(text: &str) {
    println!("{}", text.len()); // output: 4
}

fn main() {
    let a = String::from("Rust");
    print_len(&a);
}
```

## Return a new `String`

```rust
fn shout(text: &str) -> String {
    text.to_uppercase()
}

fn main() {
    let result = shout("rust");

    println!("{}", result); // output: RUST
}
```

---

# 10. `Vec` Ownership Patterns

## Move a vector

```rust
fn consume(numbers: Vec<i32>) {
    println!("{:?}", numbers); // output: [1, 2, 3]
}

fn main() {
    let numbers = vec![1, 2, 3];
    consume(numbers);
}
```

After `consume(numbers)`, the original `numbers` cannot be used.

## Borrow a vector as slice: `&[T]`

```rust
fn sum(numbers: &[i32]) -> i32 {
    numbers.iter().copied().sum()
}

fn main() {
    let numbers = vec![1, 2, 3];

    println!("{}", sum(&numbers)); // output: 6
    println!("{:?}", numbers); // output: [1, 2, 3]
}
```

Use `&[i32]` when you only need to read a list.

## Mutate a vector through function

```rust
fn add_number(numbers: &mut Vec<i32>) {
    numbers.push(4);
}

fn main() {
    let mut numbers = vec![1, 2, 3];
    add_number(&mut numbers);

    println!("{:?}", numbers); // output: [1, 2, 3, 4]
}
```

---

# 11. `Clone`

`.clone() -> T`

Clone creates a real duplicate.

| Method | Common return type | Meaning |
|---|---|---|
| `.clone()` | same type | duplicate value |
| `.to_owned()` | owned value | often converts borrowed to owned |
| `.to_string()` | `String` | convert displayable/text to String |

```rust
fn main() {
    let a = String::from("Rust");
    let b = a.clone();

    println!("a = {}, b = {}", a, b); // output: a = Rust, b = Rust
}
```

For vectors:

```rust
fn main() {
    let a = vec![1, 2, 3];
    let b = a.clone();

    println!("{:?}", a); // output: [1, 2, 3]
    println!("{:?}", b); // output: [1, 2, 3]
}
```

Beginner rule:

```text
Use clone when you really need two owned copies.
Prefer borrowing when possible.
```

---

# 12. `Copy`

`Copy` types are duplicated automatically.

```rust
fn main() {
    let a = 10;
    let b = a;

    println!("{} {}", a, b); // output: 10 10
}
```

You can derive `Copy` for simple structs when all fields are `Copy`.

```rust
#[derive(Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p1 = Point { x: 3, y: 4 };
    let p2 = p1;

    println!("{}, {}", p1.x, p2.y); // output: 3, 4
}
```

A struct containing `String` cannot be `Copy`.

---

# 13. Borrowing in Loops

## `for n in numbers` — moves/consumes collection

```rust
fn main() {
    let numbers = vec![1, 2, 3];

    for n in numbers {
        println!("{}", n); // output: 1 then 2 then 3
    }
}
```

After this loop, `numbers` is moved.

## `for n in &numbers` — borrows collection

```rust
fn main() {
    let numbers = vec![1, 2, 3];

    for n in &numbers {
        println!("{}", n); // output: 1 then 2 then 3
    }

    println!("{:?}", numbers); // output: [1, 2, 3]
}
```

## `for n in &mut numbers` — mutably borrows collection

```rust
fn main() {
    let mut numbers = vec![1, 2, 3];

    for n in &mut numbers {
        *n *= 2;
    }

    println!("{:?}", numbers); // output: [2, 4, 6]
}
```

## Loop ownership table

| Loop form | Item type | Collection usable after? | Use when |
|---|---|---:|---|
| `for x in values` | `T` | no, for non-Copy collections | consume/move values |
| `for x in &values` | `&T` | yes | read values |
| `for x in &mut values` | `&mut T` | yes | edit values |

---

# 14. `iter`, `iter_mut`, and `into_iter`

| Method | Common item type | Meaning |
|---|---|---|
| `.iter()` | `&T` | borrow each item |
| `.iter_mut()` | `&mut T` | mutably borrow each item |
| `.into_iter()` | `T` | consume and move each item |

## `.iter() -> Iterator<Item = &T>`

```rust
fn main() {
    let numbers = vec![1, 2, 3];
    let total: i32 = numbers.iter().copied().sum();

    println!("{}", total); // output: 6
}
```

## `.iter_mut() -> Iterator<Item = &mut T>`

```rust
fn main() {
    let mut numbers = vec![1, 2, 3];

    numbers.iter_mut().for_each(|n| *n += 10);

    println!("{:?}", numbers); // output: [11, 12, 13]
}
```

## `.into_iter() -> Iterator<Item = T>`

```rust
fn main() {
    let names = vec![String::from("Ali"), String::from("Sara")];

    for name in names.into_iter() {
        println!("{}", name); // output: Ali then Sara
    }
}
```

---

# 15. Returning Owned Values

Returning owned values is usually simple and safe.

```rust
fn make_name() -> String {
    String::from("Abbas")
}

fn main() {
    let name = make_name();

    println!("{}", name); // output: Abbas
}
```

You can take ownership and return ownership back.

```rust
fn add_exclamation(mut text: String) -> String {
    text.push('!');
    text
}

fn main() {
    let text = String::from("hello");
    let result = add_exclamation(text);

    println!("{}", result); // output: hello!
}
```

---

# 16. Returning References

Returning references is possible only when the returned reference points to data that still exists.

## Good: return reference from parameter

```rust
fn first_word<'a>(text: &'a str) -> &'a str {
    text.split_whitespace().next().unwrap_or("")
}

fn main() {
    let text = "hello rust";
    let first = first_word(text);

    println!("{}", first); // output: hello
}
```

The returned `&str` comes from the input `text`.

## Better beginner approach: return owned `String`

If lifetimes feel hard, return an owned value.

```rust
fn first_word_owned(text: &str) -> String {
    text.split_whitespace().next().unwrap_or("").to_string()
}

fn main() {
    let first = first_word_owned("hello rust");

    println!("{}", first); // output: hello
}
```

---

# 17. `Box<T>`

`Box<T>` stores a value on the heap.

## `Box::new(value) -> Box<T>`

| Method/function | Return type | Meaning |
|---|---|---|
| `Box::new(value)` | `Box<T>` | allocate value on heap |
| `*boxed` | `T` or access to `T` | dereference box |

```rust
fn main() {
    let boxed = Box::new(10);

    println!("{}", boxed); // output: 10
    println!("{}", *boxed + 5); // output: 15
}
```

A `Box<T>` owns its value.

When the box is dropped, the heap value is also dropped.

---

# 18. Why Use `Box<T>`?

Common uses:

```text
1. Put large data on the heap.
2. Create recursive types.
3. Store trait objects later.
```

Simple example:

```rust
struct User {
    name: String,
}

fn main() {
    let user = Box::new(User {
        name: String::from("Abbas"),
    });

    println!("{}", user.name); // output: Abbas
}
```

Rust can automatically dereference boxes in many field/method accesses.

---

# 19. Recursive Data with `Box`

Without `Box`, recursive types have infinite size.

Use `Box` to store the recursive part on the heap.

```rust
#[derive(Debug)]
enum List {
    Node(i32, Box<List>),
    Empty,
}

fn main() {
    let list = List::Node(
        1,
        Box::new(List::Node(
            2,
            Box::new(List::Empty),
        )),
    );

    println!("{:?}", list); // output: Node(1, Node(2, Empty))
}
```

This pattern appears in linked lists, trees, and recursive structures.

---

# 20. `RefCell<T>`

`RefCell<T>` allows mutation checked at runtime instead of compile time.

It is useful for **interior mutability**.

## `RefCell` method table

| Method | Common return type | Meaning |
|---|---|---|
| `RefCell::new(value)` | `RefCell<T>` | create RefCell |
| `.borrow()` | `Ref<T>` | immutable borrow at runtime |
| `.borrow_mut()` | `RefMut<T>` | mutable borrow at runtime |
| `.into_inner()` | `T` | consume RefCell and return value |

```rust
use std::cell::RefCell;

fn main() {
    let value = RefCell::new(10);

    *value.borrow_mut() += 5;

    println!("{}", value.borrow()); // output: 15
}
```

With `RefCell`, borrow rule violations cause a runtime panic instead of a compile-time error.

---

# 21. `RefCell<Vec<T>>`

```rust
use std::cell::RefCell;

fn main() {
    let numbers = RefCell::new(vec![1, 2, 3]);

    numbers.borrow_mut().push(4);

    println!("{:?}", numbers.borrow()); // output: [1, 2, 3, 4]
}
```

This lets you mutate the vector through the `RefCell` even when the `RefCell` variable itself is not declared `mut`.

---

# 22. `RefCell` Borrowing Rules

At runtime, `RefCell` still enforces:

```text
Many immutable borrows OR one mutable borrow.
```

Good:

```rust
use std::cell::RefCell;

fn main() {
    let value = RefCell::new(10);

    let a = value.borrow();
    let b = value.borrow();

    println!("{} {}", a, b); // output: 10 10
}
```

Good:

```rust
use std::cell::RefCell;

fn main() {
    let value = RefCell::new(10);

    {
        let mut a = value.borrow_mut();
        *a += 1;
        println!("{}", a); // output: 11
    }

    println!("{}", value.borrow()); // output: 11
}
```

The mutable borrow ends at the end of the block.

---

# 23. `Rc<RefCell<T>>` Preview

Sometimes you need shared ownership plus mutation.

That usually uses:

```rust
Rc<RefCell<T>>
```

`Rc` means reference-counted shared ownership.

`RefCell` means interior mutability.

```rust
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let shared = Rc::new(RefCell::new(5));
    let a = Rc::clone(&shared);
    let b = Rc::clone(&shared);

    *a.borrow_mut() += 10;
    *b.borrow_mut() += 20;

    println!("{}", shared.borrow()); // output: 35
}
```

This is useful in trees, graphs, and shared mutable state.

---

# 24. Common Ownership Errors and Fixes

## Error: value borrowed after move

Wrong idea:

```rust
fn take(text: String) {
    println!("{}", text); // output: text value
}
```

If you pass a `String` by value, it moves.

Fix by borrowing:

```rust
fn take(text: &str) {
    println!("{}", text); // output: hello
}

fn main() {
    let text = String::from("hello");
    take(&text);

    println!("{}", text); // output: hello
}
```

## Error: cannot borrow immutable variable as mutable

Fix by adding `mut` to the original variable.

```rust
fn add(text: &mut String) {
    text.push('!');
}

fn main() {
    let mut text = String::from("hello");
    add(&mut text);

    println!("{}", text); // output: hello!
}
```

## Error: expected `&T`, found `T`

Fix by adding `&`.

```rust
fn show(n: &i32) {
    println!("{}", n); // output: 10
}

fn main() {
    let x = 10;
    show(&x);
}
```

## Error: expected `T`, found `&T`

Fix by dereferencing or copying.

```rust
fn double(n: i32) -> i32 {
    n * 2
}

fn main() {
    let x = 10;
    let r = &x;

    println!("{}", double(*r)); // output: 20
}
```

---

# 25. Exam Patterns

## Pattern 1: read a string and return a new string

```rust
fn clean(text: &str) -> String {
    text.trim().to_lowercase()
}

fn main() {
    println!("{}", clean("  RUST  ")); // output: rust
}
```

## Pattern 2: read a slice and return a new vector

```rust
fn positives(numbers: &[i32]) -> Vec<i32> {
    numbers.iter().copied().filter(|n| *n > 0).collect()
}

fn main() {
    println!("{:?}", positives(&[-2, 0, 3, 5])); // output: [3, 5]
}
```

## Pattern 3: mutate a vector in place

```rust
fn double_all(numbers: &mut Vec<i32>) {
    for n in numbers {
        *n *= 2;
    }
}

fn main() {
    let mut numbers = vec![1, 2, 3];
    double_all(&mut numbers);

    println!("{:?}", numbers); // output: [2, 4, 6]
}
```

## Pattern 4: return ownership after modifying

```rust
fn sorted(mut numbers: Vec<i32>) -> Vec<i32> {
    numbers.sort();
    numbers
}

fn main() {
    let numbers = vec![3, 1, 2];
    let result = sorted(numbers);

    println!("{:?}", result); // output: [1, 2, 3]
}
```

---

# 26. Quick Decision Table

| Goal | Best parameter | Return type idea |
|---|---|---|
| Read text | `&str` | depends |
| Create changed text | `&str` | `String` |
| Edit original string | `&mut String` | `()` |
| Read list | `&[T]` | depends |
| Create changed list | `&[T]` | `Vec<T>` |
| Edit original vector | `&mut Vec<T>` | `()` |
| Consume vector and return changed vector | `Vec<T>` | `Vec<T>` |
| Share data without changing | `&T` | depends |
| Change data in function | `&mut T` | usually `()` |
| Need recursive type | `Box<T>` | owned recursive data |
| Need runtime borrow checks | `RefCell<T>` | interior mutability |

---

# 27. Practice Exercises

## Exercise 1 — Borrow text

Write:

```rust
fn print_twice(text: &str)
```

Expected:

```rust
fn main() {
    print_twice("Rust");
}
```

Output:

```text
Rust
Rust
```

## Exercise 2 — Mutate number

Write:

```rust
fn add_ten(n: &mut i32)
```

Expected:

```rust
fn main() {
    let mut x = 5;
    add_ten(&mut x);
    println!("{}", x); // output: 15
}
```

## Exercise 3 — Borrow slice

Write:

```rust
fn sum(numbers: &[i32]) -> i32
```

Expected:

```rust
fn main() {
    println!("{}", sum(&[1, 2, 3])); // output: 6
}
```

## Exercise 4 — Mutate vector

Write:

```rust
fn remove_negatives(numbers: &mut Vec<i32>)
```

Expected:

```rust
fn main() {
    let mut numbers = vec![-1, 2, -3, 4];
    remove_negatives(&mut numbers);
    println!("{:?}", numbers); // output: [2, 4]
}
```

## Exercise 5 — Clone only when needed

Create two owned `String` values from one original string using `.clone()`.

Expected:

```rust
fn main() {
    let a = String::from("Rust");
    let b = a.clone();
    println!("{} {}", a, b); // output: Rust Rust
}
```

---

End of cheat_sheet5.md.
