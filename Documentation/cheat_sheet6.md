# Rust Cheat Sheet 6 — Structs, Enums, Traits, Generics, and Lifetimes

This is **cheat_sheet6.md**.

Focus: Rust custom types and type-system patterns used heavily in exam exercises.

This sheet avoids repeating full method sections from earlier sheets. It focuses on:

- structs
- tuple structs
- unit structs
- `impl`
- associated functions
- methods with `self`, `&self`, and `&mut self`
- enums
- pattern matching with enums
- traits
- derive macros
- generics
- trait bounds
- lifetimes

Style:

- Method/function headings show return types where useful.
- Each `println!` example line includes `// output:` on the same line.
- Examples are small and exam-focused.

---

# 1. Structs

A struct groups related values under one type.

## Struct syntax

```rust
struct User {
    name: String,
    age: u32,
}
```

Pattern:

```rust
struct TypeName {
    field_name: FieldType,
}
```

---

## Struct creation

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

---

## Struct field access

```rust
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p = Point { x: 3, y: 4 };

    println!("{}", p.x); // output: 3
    println!("{}", p.y); // output: 4
}
```

---

## Mutable struct

To change fields, the variable must be mutable.

```rust
struct User {
    name: String,
    age: u32,
}

fn main() {
    let mut user = User {
        name: String::from("Abbas"),
        age: 25,
    };

    user.age = 26;

    println!("{}", user.age); // output: 26
}
```

---

## Field init shorthand

If variable names match field names, use shorthand.

```rust
struct User {
    name: String,
    age: u32,
}

fn build_user(name: String, age: u32) -> User {
    User { name, age }
}

fn main() {
    let user = build_user(String::from("Abbas"), 25);

    println!("{} {}", user.name, user.age); // output: Abbas 25
}
```

---

## Struct update syntax

Use fields from another struct.

```rust
struct User {
    name: String,
    age: u32,
    active: bool,
}

fn main() {
    let user1 = User {
        name: String::from("Abbas"),
        age: 25,
        active: true,
    };

    let user2 = User {
        name: String::from("Ali"),
        ..user1
    };

    println!("{} {}", user2.name, user2.age); // output: Ali 25
    println!("{}", user2.active); // output: true
}
```

Important: fields like `String` may move from the old struct when using `..user1`.

---

## Struct method/index table

| Pattern | Meaning | Return type |
|---|---|---|
| `Type { field: value }` | create struct | `Type` |
| `value.field` | access field | field type |
| `value.field = new_value` | change field | `()` |
| `Type { field, ..old }` | struct update | `Type` |
| `#[derive(Debug)]` | allow debug print | compile-time generated impl |
| `#[derive(Clone)]` | allow `.clone()` | compile-time generated impl |

---

# 2. Tuple Structs

Tuple structs have named type but unnamed fields.

## Tuple struct syntax

```rust
struct Color(u8, u8, u8);

fn main() {
    let red = Color(255, 0, 0);

    println!("{}", red.0); // output: 255
    println!("{}", red.1); // output: 0
    println!("{}", red.2); // output: 0
}
```

Pattern:

```rust
struct TypeName(Type1, Type2, Type3);
```

Access fields with:

```rust
value.0
value.1
```

---

## Common use: new type wrapper

```rust
struct UserId(u32);

fn main() {
    let id = UserId(1001);

    println!("{}", id.0); // output: 1001
}
```

Tuple structs help avoid mixing values that are both numbers but have different meanings.

---

# 3. Unit Structs

Unit structs have no fields.

```rust
struct Marker;

fn main() {
    let _m = Marker;

    println!("created"); // output: created
}
```

Useful for marker types and simple trait implementations.

---

# 4. Debug Printing for Structs

Normal `{}` printing does not work unless you implement `Display`.

For learning and debugging, use `#[derive(Debug)]`.

## `#[derive(Debug)]`

```rust
#[derive(Debug)]
struct User {
    name: String,
    age: u32,
}

fn main() {
    let user = User {
        name: String::from("Abbas"),
        age: 25,
    };

    println!("{:?}", user); // output: User { name: "Abbas", age: 25 }
}
```

Pretty debug:

```rust
#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p = Point { x: 3, y: 4 };

    println!("{:#?}", p); // output: Point { x: 3, y: 4 } pretty-printed across lines
}
```

---

# 5. `impl` Blocks

`impl` attaches functions and methods to a type.

```rust
struct Rectangle {
    width: i32,
    height: i32,
}

impl Rectangle {
    fn area(&self) -> i32 {
        self.width * self.height
    }
}

fn main() {
    let r = Rectangle {
        width: 5,
        height: 4,
    };

    println!("{}", r.area()); // output: 20
}
```

Pattern:

```rust
impl TypeName {
    fn method(&self) -> ReturnType {
        ...
    }
}
```

---

# 6. Associated Functions

Associated functions belong to a type, not to one existing value.

They do **not** take `self`.

## `Type::new(...) -> Type`

```rust
struct User {
    name: String,
    age: u32,
}

impl User {
    fn new(name: String, age: u32) -> User {
        User { name, age }
    }
}

fn main() {
    let user = User::new(String::from("Abbas"), 25);

    println!("{} {}", user.name, user.age); // output: Abbas 25
}
```

Common pattern:

```rust
Type::new(...)
```

Examples:

```rust
String::new()
Vec::new()
User::new(...)
```

---

## `Self` inside impl

`Self` means the current type.

```rust
struct Counter {
    value: i32,
}

impl Counter {
    fn new() -> Self {
        Self { value: 0 }
    }
}

fn main() {
    let c = Counter::new();

    println!("{}", c.value); // output: 0
}
```

---

## Associated function table

| Pattern | Meaning | Call syntax | Return type |
|---|---|---|---|
| `fn new() -> Self` | create value | `Type::new()` | `Self` |
| `fn from_x(x: T) -> Self` | create from input | `Type::from_x(x)` | `Self` |
| `fn helper(...) -> T` | type-level helper | `Type::helper(...)` | any type |
| no `self` parameter | associated function | `Type::function()` | declared return |

---

# 7. Methods with `&self`

Use `&self` when the method only reads the value.

```rust
struct User {
    name: String,
}

impl User {
    fn greet(&self) {
        println!("Hello, {}", self.name); // output: Hello, Abbas
    }
}

fn main() {
    let user = User {
        name: String::from("Abbas"),
    };

    user.greet();
}
```

Pattern:

```rust
fn method_name(&self)
```

Call:

```rust
value.method_name()
```

---

# 8. Methods with `&mut self`

Use `&mut self` when the method changes the value.

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
    c.increment();

    println!("{}", c.value); // output: 2
}
```

Rules:

```rust
let mut value = Type { ... };
value.method_that_uses_mut_self();
```

---

# 9. Methods with `self`

Use `self` when the method consumes the value.

```rust
struct User {
    name: String,
}

impl User {
    fn into_name(self) -> String {
        self.name
    }
}

fn main() {
    let user = User {
        name: String::from("Abbas"),
    };

    let name = user.into_name();

    println!("{}", name); // output: Abbas
}
```

After `into_name`, `user` is moved and cannot be used.

---

## Method receiver table

| Receiver | Meaning | Can use value after call? | Example |
|---|---|---|---|
| `&self` | read-only borrow | yes | `user.greet()` |
| `&mut self` | mutable borrow | yes after borrow ends | `counter.increment()` |
| `self` | takes ownership | no | `user.into_name()` |

---

# 10. Getters and Setters

Rust does not automatically create getters/setters. You write them when useful.

```rust
struct Player {
    name: String,
    score: i32,
}

impl Player {
    fn score(&self) -> i32 {
        self.score
    }

    fn add_score(&mut self, points: i32) {
        self.score += points;
    }
}

fn main() {
    let mut p = Player {
        name: String::from("Abbas"),
        score: 10,
    };

    p.add_score(5);

    println!("{}", p.score()); // output: 15
    println!("{}", p.name); // output: Abbas
}
```

---

# 11. Structs with Methods — Exam Pattern

```rust
#[derive(Debug)]
struct Student {
    name: String,
    grades: Vec<i32>,
}

impl Student {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            grades: Vec::new(),
        }
    }

    fn add_grade(&mut self, grade: i32) {
        self.grades.push(grade);
    }

    fn average(&self) -> f64 {
        if self.grades.is_empty() {
            return 0.0;
        }

        let total: i32 = self.grades.iter().copied().sum();
        total as f64 / self.grades.len() as f64
    }
}

fn main() {
    let mut s = Student::new("Abbas");

    s.add_grade(80);
    s.add_grade(100);

    println!("{}", s.name); // output: Abbas
    println!("{}", s.average()); // output: 90
}
```

---

# 12. Enums

Enums represent a value that can be one of several variants.

## Basic enum

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
        Direction::Down => println!("down"), // output: down if selected
        Direction::Left => println!("left"), // output: left if selected
        Direction::Right => println!("right"), // output: right if selected
    }
}
```

Pattern:

```rust
EnumName::Variant
```

---

## Enum with data

```rust
enum Message {
    Quit,
    Write(String),
    Move { x: i32, y: i32 },
}

fn main() {
    let msg = Message::Move { x: 3, y: 4 };

    match msg {
        Message::Quit => println!("quit"), // output: quit if selected
        Message::Write(text) => println!("{}", text), // output: message text if selected
        Message::Move { x, y } => println!("move {}, {}", x, y), // output: move 3, 4
    }
}
```

Enum variants can carry:

```rust
Variant
Variant(T)
Variant { field: T }
```

---

## Enum method with impl

```rust
enum Status {
    Active,
    Inactive,
    Blocked,
}

impl Status {
    fn message(&self) -> &str {
        match self {
            Status::Active => "active",
            Status::Inactive => "inactive",
            Status::Blocked => "blocked",
        }
    }
}

fn main() {
    let status = Status::Active;

    println!("{}", status.message()); // output: active
}
```

---

## Enum method/index table

| Pattern | Meaning | Example |
|---|---|---|
| `Enum::Variant` | simple variant | `Direction::Up` |
| `Enum::Variant(value)` | tuple-like variant | `Message::Write(text)` |
| `Enum::Variant { field }` | struct-like variant | `Move { x, y }` |
| `match enum_value` | handle variants | `match status { ... }` |
| `impl Enum` | add methods | `status.message()` |

---

# 13. `Option<T>` and `Result<T, E>` Are Enums

These are standard library enums.

## Option

```rust
enum Option<T> {
    Some(T),
    None,
}
```

Example:

```rust
fn main() {
    let value = Some(10);

    match value {
        Some(n) => println!("{}", n), // output: 10
        None => println!("none"), // output: none if value is None
    }
}
```

---

## Result

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

Example:

```rust
fn main() {
    let result = "42".parse::<i32>();

    match result {
        Ok(n) => println!("{}", n), // output: 42
        Err(_) => println!("error"), // output: error if parse fails
    }
}
```

---

# 14. Pattern Matching with Enums

## Matching by value

```rust
enum Grade {
    A,
    B,
    C,
    Fail,
}

fn message(grade: Grade) -> &'static str {
    match grade {
        Grade::A => "excellent",
        Grade::B => "good",
        Grade::C => "pass",
        Grade::Fail => "try again",
    }
}

fn main() {
    println!("{}", message(Grade::B)); // output: good
}
```

---

## Matching with data

```rust
enum Shape {
    Circle(f64),
    Rectangle(f64, f64),
}

fn area(shape: Shape) -> f64 {
    match shape {
        Shape::Circle(r) => 3.14 * r * r,
        Shape::Rectangle(w, h) => w * h,
    }
}

fn main() {
    println!("{}", area(Shape::Rectangle(4.0, 5.0))); // output: 20
}
```

---

## `if let` with enum

Use `if let` when you only care about one variant.

```rust
enum Message {
    Quit,
    Write(String),
}

fn main() {
    let msg = Message::Write(String::from("hello"));

    if let Message::Write(text) = msg {
        println!("{}", text); // output: hello
    }
}
```

---

# 15. Traits

A trait defines shared behavior.

## Trait syntax

```rust
trait Speak {
    fn speak(&self) -> String;
}
```

Implement it for a type:

```rust
trait Speak {
    fn speak(&self) -> String;
}

struct Dog;

impl Speak for Dog {
    fn speak(&self) -> String {
        String::from("woof")
    }
}

fn main() {
    let dog = Dog;

    println!("{}", dog.speak()); // output: woof
}
```

Pattern:

```rust
impl TraitName for TypeName {
    fn method(&self) -> ReturnType {
        ...
    }
}
```

---

## Trait method/index table

| Pattern | Meaning | Example |
|---|---|---|
| `trait Name { ... }` | define behavior | `trait Speak` |
| `impl Trait for Type` | implement behavior | `impl Speak for Dog` |
| `fn method(&self)` | required method | `fn speak(&self)` |
| `value.method()` | call trait method | `dog.speak()` |
| `T: Trait` | generic constraint | `fn show<T: Debug>(x: T)` |

---

# 16. Default Trait Methods

Traits can provide default method bodies.

```rust
trait Describe {
    fn name(&self) -> &str;

    fn describe(&self) -> String {
        format!("name = {}", self.name())
    }
}

struct User {
    name: String,
}

impl Describe for User {
    fn name(&self) -> &str {
        &self.name
    }
}

fn main() {
    let user = User {
        name: String::from("Abbas"),
    };

    println!("{}", user.describe()); // output: name = Abbas
}
```

The type only implements `name`; it gets `describe` for free.

---

# 17. Traits as Function Parameters

## `impl Trait`

```rust
use std::fmt::Debug;

fn print_debug(value: impl Debug) {
    println!("{:?}", value); // output: value printed with Debug
}

fn main() {
    print_debug(10); // output: 10
    print_debug("hello"); // output: "hello"
}
```

## Generic trait bound

```rust
use std::fmt::Debug;

fn print_debug<T: Debug>(value: T) {
    println!("{:?}", value); // output: value printed with Debug
}

fn main() {
    print_debug(20); // output: 20
}
```

These are similar for simple cases.

---

## Multiple trait bounds

```rust
use std::fmt::Debug;

fn duplicate_and_print<T: Clone + Debug>(value: T) {
    let copy = value.clone();

    println!("{:?}", copy); // output: cloned value printed with Debug
}

fn main() {
    duplicate_and_print(String::from("Rust")); // output: "Rust"
}
```

---

## `where` clause

Use `where` for cleaner generic bounds.

```rust
use std::fmt::Debug;

fn show_pair<T, U>(a: T, b: U)
where
    T: Debug,
    U: Debug,
{
    println!("{:?} {:?}", a, b); // output: 10 "Rust"
}

fn main() {
    show_pair(10, "Rust");
}
```

---

# 18. Common Derive Macros

Derive macros generate trait implementations.

## Derive index table

| Derive | Purpose | Enables |
|---|---|---|
| `Debug` | debug printing | `{:?}` formatting |
| `Clone` | explicit copying | `.clone()` |
| `Copy` | implicit copy for small/simple values | use after assignment |
| `PartialEq` | equality comparison | `==`, `!=` |
| `Eq` | full equality marker | stronger equality |
| `PartialOrd` | partial ordering | `<`, `>`, `.partial_cmp()` |
| `Ord` | full ordering | `.sort()` |
| `Default` | default value | `Type::default()` |

---

## `#[derive(Debug)]`

```rust
#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p = Point { x: 1, y: 2 };

    println!("{:?}", p); // output: Point { x: 1, y: 2 }
}
```

---

## `#[derive(Clone)]`

```rust
#[derive(Clone)]
struct User {
    name: String,
}

fn main() {
    let a = User {
        name: String::from("Abbas"),
    };

    let b = a.clone();

    println!("{}", b.name); // output: Abbas
}
```

---

## `#[derive(Copy, Clone)]`

Only use `Copy` when all fields are `Copy`.

```rust
#[derive(Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let a = Point { x: 1, y: 2 };
    let b = a;

    println!("{} {}", a.x, b.y); // output: 1 2
}
```

Without `Copy`, assigning `a` to `b` may move `a`.

---

## `#[derive(PartialEq)]`

```rust
#[derive(PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let a = Point { x: 1, y: 2 };
    let b = Point { x: 1, y: 2 };

    println!("{}", a == b); // output: true
}
```

---

## `#[derive(Default)]`

```rust
#[derive(Default)]
struct Settings {
    volume: u32,
    dark_mode: bool,
}

fn main() {
    let s = Settings::default();

    println!("{} {}", s.volume, s.dark_mode); // output: 0 false
}
```

---

# 19. Generics

Generics let code work with different types.

## Generic function

```rust
fn first<T>(items: &[T]) -> Option<&T> {
    items.first()
}

fn main() {
    let numbers = [10, 20, 30];

    match first(&numbers) {
        Some(n) => println!("{}", n), // output: 10
        None => println!("empty"), // output: empty if slice is empty
    }
}
```

Pattern:

```rust
fn function_name<T>(arg: T) -> T
```

---

## Generic struct

```rust
struct Pair<T> {
    first: T,
    second: T,
}

fn main() {
    let pair = Pair {
        first: 10,
        second: 20,
    };

    println!("{} {}", pair.first, pair.second); // output: 10 20
}
```

---

## Generic enum

```rust
enum MyOption<T> {
    Some(T),
    None,
}

fn main() {
    let value = MyOption::Some(5);

    match value {
        MyOption::Some(n) => println!("{}", n), // output: 5
        MyOption::None => println!("none"), // output: none if selected
    }
}
```

---

## Multiple generic types

```rust
struct Pair<T, U> {
    first: T,
    second: U,
}

fn main() {
    let pair = Pair {
        first: "age",
        second: 25,
    };

    println!("{} {}", pair.first, pair.second); // output: age 25
}
```

---

# 20. Trait Bounds with Generics

Generic code sometimes needs to know what operations are allowed.

## Bound with `Debug`

```rust
use std::fmt::Debug;

fn show<T: Debug>(value: T) {
    println!("{:?}", value); // output: value printed with Debug
}

fn main() {
    show(vec![1, 2, 3]); // output: [1, 2, 3]
}
```

---

## Bound with `PartialOrd + Copy`

```rust
fn max_value<T: PartialOrd + Copy>(a: T, b: T) -> T {
    if a > b {
        a
    } else {
        b
    }
}

fn main() {
    println!("{}", max_value(10, 20)); // output: 20
}
```

Why `Copy`?

Because we return either `a` or `b` by value.

---

## Bound with `Clone`

```rust
fn duplicate<T: Clone>(value: T) -> (T, T) {
    (value.clone(), value)
}

fn main() {
    let pair = duplicate(String::from("Rust"));

    println!("{} {}", pair.0, pair.1); // output: Rust Rust
}
```

---

## Trait bound table

| Bound | Needed for | Example |
|---|---|---|
| `T: Debug` | `{:?}` printing | debug formatting |
| `T: Clone` | `.clone()` | `x.clone()` |
| `T: Copy` | implicit copy | return `a` or `b` easily |
| `T: PartialEq` | `==`, `!=` | `a == b` |
| `T: PartialOrd` | `<`, `>`, `<=`, `>=` | `a > b` |
| `T: Default` | default value | `T::default()` |

---

# 21. Implementing Methods for Generic Structs

```rust
struct Boxed<T> {
    value: T,
}

impl<T> Boxed<T> {
    fn new(value: T) -> Self {
        Self { value }
    }

    fn get(&self) -> &T {
        &self.value
    }
}

fn main() {
    let b = Boxed::new(42);

    println!("{}", b.get()); // output: 42
}
```

Pattern:

```rust
impl<T> Type<T> {
    ...
}
```

---

## Generic impl with trait bound

```rust
use std::fmt::Debug;

struct Boxed<T> {
    value: T,
}

impl<T: Debug> Boxed<T> {
    fn show(&self) {
        println!("{:?}", self.value); // output: 42
    }
}

fn main() {
    let b = Boxed { value: 42 };

    b.show();
}
```

Only types that implement `Debug` can use `show`.

---

# 22. Lifetimes — Beginner Meaning

Lifetimes tell Rust how long references are valid.

You usually need explicit lifetimes when a function returns a reference connected to an input reference.

---

## Example that does not need explicit lifetime

```rust
fn print_text(text: &str) {
    println!("{}", text); // output: hello
}

fn main() {
    print_text("hello");
}
```

Rust can understand this automatically.

---

## Function returning one input reference

```rust
fn first_word(text: &str) -> &str {
    text.split_whitespace().next().unwrap_or("")
}

fn main() {
    println!("{}", first_word("hello rust")); // output: hello
}
```

Rust can infer this lifetime.

---

## Function returning one of two references

This needs a named lifetime.

```rust
fn longer<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() >= b.len() {
        a
    } else {
        b
    }
}

fn main() {
    let a = "rust";
    let b = "language";

    println!("{}", longer(a, b)); // output: language
}
```

Meaning:

```rust
'a
```

says:

```text
The returned reference lives as long as both input references are valid.
```

---

## Lifetime syntax table

| Syntax | Meaning |
|---|---|
| `&T` | reference with inferred lifetime |
| `&'a T` | reference with named lifetime `'a` |
| `fn f<'a>(x: &'a T) -> &'a T` | output reference tied to input |
| `struct S<'a> { field: &'a str }` | struct stores a reference |

---

# 23. Structs with References and Lifetimes

If a struct stores references, it usually needs a lifetime.

```rust
struct UserRef<'a> {
    name: &'a str,
}

fn main() {
    let name = "Abbas";
    let user = UserRef { name };

    println!("{}", user.name); // output: Abbas
}
```

Pattern:

```rust
struct Type<'a> {
    field: &'a T,
}
```

Beginner advice: prefer owning `String` in structs unless the exercise asks for references.

---

## Owned struct vs borrowed struct

Owned:

```rust
struct User {
    name: String,
}

fn main() {
    let user = User {
        name: String::from("Abbas"),
    };

    println!("{}", user.name); // output: Abbas
}
```

Borrowed:

```rust
struct UserRef<'a> {
    name: &'a str,
}

fn main() {
    let name = String::from("Abbas");
    let user = UserRef { name: &name };

    println!("{}", user.name); // output: Abbas
}
```

---

# 24. Common Modeling Patterns

## Pattern 1: constructor + methods

```rust
struct Wallet {
    balance: i32,
}

impl Wallet {
    fn new() -> Self {
        Self { balance: 0 }
    }

    fn deposit(&mut self, amount: i32) {
        self.balance += amount;
    }

    fn balance(&self) -> i32 {
        self.balance
    }
}

fn main() {
    let mut wallet = Wallet::new();

    wallet.deposit(50);

    println!("{}", wallet.balance()); // output: 50
}
```

---

## Pattern 2: enum for state

```rust
enum OrderStatus {
    Pending,
    Paid,
    Shipped,
}

impl OrderStatus {
    fn can_cancel(&self) -> bool {
        match self {
            OrderStatus::Pending => true,
            OrderStatus::Paid => true,
            OrderStatus::Shipped => false,
        }
    }
}

fn main() {
    let status = OrderStatus::Shipped;

    println!("{}", status.can_cancel()); // output: false
}
```

---

## Pattern 3: trait for shared behavior

```rust
trait Area {
    fn area(&self) -> f64;
}

struct Rectangle {
    width: f64,
    height: f64,
}

impl Area for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}

fn main() {
    let r = Rectangle {
        width: 4.0,
        height: 5.0,
    };

    println!("{}", r.area()); // output: 20
}
```

---

# 25. Common Compiler Errors and Fixes

## Error: cannot print struct with `{}`

Wrong:

```rust
struct User {
    name: String,
}

fn main() {
    let user = User {
        name: String::from("Abbas"),
    };

    // println!("{}", user); // output: compiler error
}
```

Fix with `Debug` and `{:?}`:

```rust
#[derive(Debug)]
struct User {
    name: String,
}

fn main() {
    let user = User {
        name: String::from("Abbas"),
    };

    println!("{:?}", user); // output: User { name: "Abbas" }
}
```

---

## Error: method not found

Possible reasons:

- method is not inside an `impl`
- trait providing method is not imported
- receiver type is wrong
- method uses `&mut self` but value is not mutable

Example fix:

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

---

## Error: binary operation `==` cannot be applied

Need `PartialEq`.

```rust
#[derive(PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let a = Point { x: 1, y: 2 };
    let b = Point { x: 1, y: 2 };

    println!("{}", a == b); // output: true
}
```

---

## Error: generic type cannot be printed

Need a trait bound.

```rust
use std::fmt::Debug;

fn show<T: Debug>(value: T) {
    println!("{:?}", value); // output: value printed with Debug
}

fn main() {
    show(10); // output: 10
}
```

---

## Error: missing lifetime specifier

A function returning a reference from multiple references may need a lifetime.

```rust
fn longer<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() >= b.len() {
        a
    } else {
        b
    }
}

fn main() {
    println!("{}", longer("abc", "abcdef")); // output: abcdef
}
```

---

# 26. Quick Syntax Summary

| Subject | Syntax | Meaning |
|---|---|---|
| struct | `struct User { name: String }` | custom data type |
| tuple struct | `struct Point(i32, i32);` | named tuple type |
| unit struct | `struct Marker;` | type with no fields |
| impl | `impl User { ... }` | functions/methods for type |
| associated fn | `fn new() -> Self` | called with `Type::new()` |
| method read | `fn show(&self)` | reads value |
| method edit | `fn edit(&mut self)` | changes value |
| method consume | `fn into_x(self)` | takes ownership |
| enum | `enum Status { Active }` | one of many variants |
| trait | `trait Speak { ... }` | shared behavior |
| impl trait | `impl Speak for Dog` | implement behavior |
| generic fn | `fn f<T>(x: T)` | works with many types |
| trait bound | `T: Debug` | generic type must support behavior |
| lifetime | `<'a>` | connects reference lifetimes |

---

# 27. Practice Exercises

## Exercise 1 — Struct and method

Create:

```rust
struct Rectangle {
    width: i32,
    height: i32,
}
```

Add method:

```rust
fn area(&self) -> i32
```

Expected:

```rust
println!("{}", r.area()); // output: 20
```

---

## Exercise 2 — Mutable method

Create `Counter` with:

```rust
value: i32
```

Add:

```rust
fn new() -> Self
fn increment(&mut self)
fn value(&self) -> i32
```

Expected:

```rust
println!("{}", c.value()); // output: 2
```

---

## Exercise 3 — Enum match

Create enum:

```rust
enum TrafficLight {
    Red,
    Yellow,
    Green,
}
```

Write:

```rust
fn action(light: TrafficLight) -> &'static str
```

Expected:

```rust
println!("{}", action(TrafficLight::Green)); // output: go
```

---

## Exercise 4 — Trait

Create trait:

```rust
trait Speak {
    fn speak(&self) -> String;
}
```

Implement it for:

```rust
struct Cat;
```

Expected:

```rust
println!("{}", cat.speak()); // output: meow
```

---

## Exercise 5 — Generic max

Write:

```rust
fn max_value<T: PartialOrd + Copy>(a: T, b: T) -> T
```

Expected:

```rust
println!("{}", max_value(3, 8)); // output: 8
```

---

## Exercise 6 — Lifetime

Write a function:

```rust
fn longer<'a>(a: &'a str, b: &'a str) -> &'a str
```

Expected:

```rust
println!("{}", longer("rust", "language")); // output: language
```

---

End of cheat_sheet6.md.
