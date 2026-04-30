# Rust Cheat Sheet 3 — Collections, Iterators, and Transformations

This is **cheat_sheet3.md**.

Focus:

```text
Collections, iterators, transformations, safe access, and common data-processing patterns.
```

This sheet does **not** repeat the full basics from cheat_sheet1 or the full string methods from cheat_sheet2.
It focuses on working with **many values**.

Style used in this sheet:

- Method index tables are placed inside the related subject section.
- Method headings show common return types.
- Every `println!` line has `// output:` beside it whenever practical.
- Examples prefer exam-friendly Rust patterns.

---

# 1. Collection Overview

Rust has several common ways to store multiple values.

| Type | Size | Owns data? | Growable? | Common use |
|---|---:|---:|---:|---|
| Array `[T; N]` | fixed | yes | no | known number of items |
| Slice `&[T]` | dynamic view | no | no | function parameters, borrowed list |
| Vector `Vec<T>` | dynamic | yes | yes | growable list |
| Tuple `(A, B, C)` | fixed | yes | no | mixed values |

Example:

```rust
fn main() {
    let arr = [1, 2, 3];
    let slice = &arr[..];
    let vec = vec![4, 5, 6];
    let pair = (10, "Rust");

    println!("{:?}", arr); // output: [1, 2, 3]
    println!("{:?}", slice); // output: [1, 2, 3]
    println!("{:?}", vec); // output: [4, 5, 6]
    println!("{} {}", pair.0, pair.1); // output: 10 Rust
}
```

---

# 2. Arrays

An array has a fixed length known at compile time.

## Array method index

| Syntax / Method | Common return type | Meaning |
|---|---|---|
| `[a, b, c]` | `[T; N]` | create array |
| `[value; count]` | `[T; N]` | repeated value array |
| `arr[index]` | `T` or `&T` depending context | direct index access, can panic |
| `arr.len()` | `usize` | number of items |
| `arr.is_empty()` | `bool` | true if length is 0 |
| `arr.contains(&value)` | `bool` | checks if value exists |
| `arr.get(index)` | `Option<&T>` | safe index access |
| `arr.first()` | `Option<&T>` | first item safely |
| `arr.last()` | `Option<&T>` | last item safely |
| `arr.iter()` | iterator over `&T` | borrow each item |
| `arr.to_vec()` | `Vec<T>` | convert array/slice to vector |

## Array creation

```rust
fn main() {
    let numbers: [i32; 3] = [10, 20, 30];

    println!("{:?}", numbers); // output: [10, 20, 30]
    println!("{}", numbers[0]); // output: 10
}
```

## Repeated values

```rust
fn main() {
    let zeros = [0; 5];

    println!("{:?}", zeros); // output: [0, 0, 0, 0, 0]
}
```

## Array length and checking

```rust
fn main() {
    let numbers = [10, 20, 30];

    println!("{}", numbers.len()); // output: 3
    println!("{}", numbers.is_empty()); // output: false
    println!("{}", numbers.contains(&20)); // output: true
}
```

## Safe array access with `.get()`

```rust
fn main() {
    let numbers = [10, 20, 30];

    match numbers.get(1) {
        Some(n) => println!("{}", n), // output: 20
        None => println!("not found"), // output: not found if index is missing
    }

    match numbers.get(9) {
        Some(n) => println!("{}", n), // output: value if found
        None => println!("not found"), // output: not found
    }
}
```

---

# 3. Slices

A slice is a borrowed view of part of an array or vector.

Most collection functions should accept slices when they only need to read values.

```rust
fn total(numbers: &[i32]) -> i32 {
    numbers.iter().copied().sum()
}
```

This accepts both arrays and vectors.

## Slice method index

| Syntax / Method | Common return type | Meaning |
|---|---|---|
| `&items[..]` | `&[T]` | whole slice |
| `&items[start..end]` | `&[T]` | partial slice, excludes end |
| `&items[..end]` | `&[T]` | from start to before end |
| `&items[start..]` | `&[T]` | from start to end |
| `slice.len()` | `usize` | number of items |
| `slice.is_empty()` | `bool` | true if empty |
| `slice.contains(&value)` | `bool` | checks if value exists |
| `slice.get(index)` | `Option<&T>` | safe access |
| `slice.first()` | `Option<&T>` | first item safely |
| `slice.last()` | `Option<&T>` | last item safely |
| `slice.iter()` | iterator over `&T` | borrow each item |
| `slice.to_vec()` | `Vec<T>` | copy/clone into vector |
| `slice.split_at(index)` | `(&[T], &[T])` | split into two slices |

## Slice ranges

```rust
fn main() {
    let numbers = [10, 20, 30, 40, 50];

    println!("{:?}", &numbers[1..4]); // output: [20, 30, 40]
    println!("{:?}", &numbers[..3]); // output: [10, 20, 30]
    println!("{:?}", &numbers[2..]); // output: [30, 40, 50]
    println!("{:?}", &numbers[..]); // output: [10, 20, 30, 40, 50]
}
```

## Slice function parameter

```rust
fn sum_numbers(numbers: &[i32]) -> i32 {
    numbers.iter().copied().sum()
}

fn main() {
    let arr = [1, 2, 3];
    let vec = vec![4, 5, 6];

    println!("{}", sum_numbers(&arr)); // output: 6
    println!("{}", sum_numbers(&vec)); // output: 15
}
```

## Split a slice

```rust
fn main() {
    let numbers = [10, 20, 30, 40];
    let (left, right) = numbers.split_at(2);

    println!("{:?}", left); // output: [10, 20]
    println!("{:?}", right); // output: [30, 40]
}
```

---

# 4. Vectors

A vector is a growable list.

## Vec method index

| Syntax / Method | Common return type | Meaning |
|---|---|---|
| `Vec::new()` | `Vec<T>` | empty vector |
| `vec![...]` | `Vec<T>` | vector with values |
| `vec.len()` | `usize` | number of items |
| `vec.is_empty()` | `bool` | true if empty |
| `vec.push(value)` | `()` | add to end |
| `vec.pop()` | `Option<T>` | remove last safely |
| `vec.insert(index, value)` | `()` | insert at index, can panic |
| `vec.remove(index)` | `T` | remove and return item, can panic |
| `vec.clear()` | `()` | remove all items |
| `vec.contains(&value)` | `bool` | checks if value exists |
| `vec.get(index)` | `Option<&T>` | safe access |
| `vec.first()` | `Option<&T>` | first item safely |
| `vec.last()` | `Option<&T>` | last item safely |
| `vec.sort()` | `()` | sort ascending |
| `vec.reverse()` | `()` | reverse current order |
| `vec.append(&mut other)` | `()` | move all items from other into vec |
| `vec.extend(iterable)` | `()` | add many items |
| `vec.retain(predicate)` | `()` | keep matching items |
| `vec.dedup()` | `()` | remove consecutive duplicates |
| `vec.truncate(len)` | `()` | shorten vector |
| `vec.resize(len, value)` | `()` | change length |
| `vec.swap(a, b)` | `()` | swap two indexes |
| `vec.swap_remove(index)` | `T` | fast remove by swapping with last |

## Create and push

```rust
fn main() {
    let mut numbers = Vec::new();

    numbers.push(10);
    numbers.push(20);

    println!("{:?}", numbers); // output: [10, 20]
}
```

## Create with `vec![]`

```rust
fn main() {
    let numbers = vec![10, 20, 30];

    println!("{:?}", numbers); // output: [10, 20, 30]
}
```

## Convert array to vector with `.to_vec()`

```rust
fn main() {
    let arr = [10, 20, 30];
    let mut numbers = arr.to_vec();

    numbers.push(40);

    println!("{:?}", numbers); // output: [10, 20, 30, 40]
}
```

## `.pop() -> Option<T>`

```rust
fn main() {
    let mut numbers = vec![10, 20, 30];
    let last = numbers.pop();

    println!("{:?}", last); // output: Some(30)
    println!("{:?}", numbers); // output: [10, 20]
}
```

Resolve `Some(...)` with `match`:

```rust
fn main() {
    let mut numbers = vec![10, 20];

    match numbers.pop() {
        Some(n) => println!("removed = {}", n), // output: removed = 20
        None => println!("empty vector"), // output: empty vector if no item exists
    }
}
```

## Insert and remove

```rust
fn main() {
    let mut numbers = vec![10, 30];

    numbers.insert(1, 20);
    let removed = numbers.remove(0);

    println!("removed = {}", removed); // output: removed = 10
    println!("{:?}", numbers); // output: [20, 30]
}
```

## Sort and reverse

```rust
fn main() {
    let mut numbers = vec![30, 10, 20];

    numbers.sort();
    println!("{:?}", numbers); // output: [10, 20, 30]

    numbers.reverse();
    println!("{:?}", numbers); // output: [30, 20, 10]
}
```

## Append vs extend

`append` moves values from another vector and leaves it empty.

```rust
fn main() {
    let mut a = vec![1, 2];
    let mut b = vec![3, 4];

    a.append(&mut b);

    println!("{:?}", a); // output: [1, 2, 3, 4]
    println!("{:?}", b); // output: []
}
```

`extend` adds items from any iterable.

```rust
fn main() {
    let mut numbers = vec![1, 2];

    numbers.extend([3, 4, 5]);

    println!("{:?}", numbers); // output: [1, 2, 3, 4, 5]
}
```

## Retain values

```rust
fn main() {
    let mut numbers = vec![1, 2, 3, 4, 5, 6];

    numbers.retain(|n| *n % 2 == 0);

    println!("{:?}", numbers); // output: [2, 4, 6]
}
```

## Dedup consecutive duplicates

```rust
fn main() {
    let mut numbers = vec![1, 1, 2, 2, 2, 3, 1, 1];

    numbers.dedup();

    println!("{:?}", numbers); // output: [1, 2, 3, 1]
}
```

Important: `.dedup()` only removes **consecutive** duplicates.

## Truncate and resize

```rust
fn main() {
    let mut numbers = vec![1, 2, 3, 4, 5];

    numbers.truncate(3);
    println!("{:?}", numbers); // output: [1, 2, 3]

    numbers.resize(5, 0);
    println!("{:?}", numbers); // output: [1, 2, 3, 0, 0]
}
```

## Swap and swap_remove

```rust
fn main() {
    let mut numbers = vec![10, 20, 30, 40];

    numbers.swap(0, 2);
    println!("{:?}", numbers); // output: [30, 20, 10, 40]

    let removed = numbers.swap_remove(1);
    println!("{}", removed); // output: 20
    println!("{:?}", numbers); // output: [30, 40, 10]
}
```

---

# 5. Tuples

Tuples store a fixed number of values, and the values can have different types.

## Tuple method / syntax index

| Syntax | Common return type | Meaning |
|---|---|---|
| `(a, b, c)` | tuple | create tuple |
| `tuple.0` | field type | access first item |
| `tuple.1` | field type | access second item |
| `let (a, b) = tuple` | destructuring | extract values |

## Tuple access

```rust
fn main() {
    let person = ("Abbas", 25, true);

    println!("{}", person.0); // output: Abbas
    println!("{}", person.1); // output: 25
    println!("{}", person.2); // output: true
}
```

## Tuple destructuring

```rust
fn main() {
    let point = (10, 20);
    let (x, y) = point;

    println!("x = {}, y = {}", x, y); // output: x = 10, y = 20
}
```

Tuples are common with `.enumerate()`:

```rust
fn main() {
    let names = ["Ali", "Sara"];

    for (index, name) in names.iter().enumerate() {
        println!("{}: {}", index, name); // output: 0: Ali then 1: Sara
    }
}
```

---

# 6. Iteration Styles

Rust gives you three main iteration styles.

## Iteration style index

| Syntax | Item type usually | Collection usable after? | Meaning |
|---|---|---:|---|
| `for x in collection` | `T` | often no for `Vec<T>` | move/consume values |
| `for x in &collection` | `&T` | yes | borrow values |
| `for x in &mut collection` | `&mut T` | yes | mutably borrow values |
| `collection.iter()` | `&T` | yes | borrow values |
| `collection.iter_mut()` | `&mut T` | yes | mutate values |
| `collection.into_iter()` | `T` | no for `Vec<T>` | move values |

## Borrowing iteration

```rust
fn main() {
    let numbers = vec![1, 2, 3];

    for n in &numbers {
        println!("{}", n); // output: 1 then 2 then 3
    }

    println!("{:?}", numbers); // output: [1, 2, 3]
}
```

## Mutable iteration

```rust
fn main() {
    let mut numbers = vec![1, 2, 3];

    for n in &mut numbers {
        *n *= 2;
    }

    println!("{:?}", numbers); // output: [2, 4, 6]
}
```

## Consuming iteration

```rust
fn main() {
    let numbers = vec![1, 2, 3];

    for n in numbers.into_iter() {
        println!("{}", n); // output: 1 then 2 then 3
    }
}
```

After `into_iter()` on a vector, the vector is moved and cannot be used again.

---

# 7. Iterator Adapters

Iterator adapters transform or filter data. They are lazy until a consuming method like `.collect()`, `.sum()`, or `.count()` is called.

## Iterator adapter index

| Method | Common return type | Meaning |
|---|---|---|
| `.iter()` | iterator over `&T` | borrow each item |
| `.iter_mut()` | iterator over `&mut T` | mutably borrow each item |
| `.into_iter()` | iterator over `T` | consume/move each item |
| `.copied()` | iterator over `T` | copy from `&T`, for `Copy` types |
| `.cloned()` | iterator over `T` | clone from `&T`, for `Clone` types |
| `.map(f)` | iterator | transform each item |
| `.filter(f)` | iterator | keep matching items |
| `.filter_map(f)` | iterator | transform and skip `None` |
| `.enumerate()` | iterator over `(usize, item)` | add index |
| `.zip(other)` | iterator over pairs | combine two iterators |
| `.chain(other)` | iterator | join iterators end-to-end |
| `.rev()` | iterator | reverse iteration |
| `.take(n)` | iterator | keep first n items |
| `.skip(n)` | iterator | skip first n items |
| `.collect::<Vec<T>>()` | `Vec<T>` | build a collection |

## `.iter()` and `.copied()`

```rust
fn main() {
    let numbers = vec![1, 2, 3];

    let copied: Vec<i32> = numbers.iter().copied().collect();

    println!("{:?}", copied); // output: [1, 2, 3]
}
```

Without `.copied()`, the result would be `Vec<&i32>`.

## `.cloned()` for `String`

```rust
fn main() {
    let names = vec![String::from("Ali"), String::from("Sara")];

    let cloned: Vec<String> = names.iter().cloned().collect();

    println!("{:?}", cloned); // output: ["Ali", "Sara"]
}
```

## `.map()` transform values

```rust
fn main() {
    let numbers = vec![1, 2, 3];

    let doubled: Vec<i32> = numbers
        .iter()
        .copied()
        .map(|n| n * 2)
        .collect();

    println!("{:?}", doubled); // output: [2, 4, 6]
}
```

## `.filter()` keep values

```rust
fn main() {
    let numbers = vec![1, 2, 3, 4, 5, 6];

    let evens: Vec<i32> = numbers
        .iter()
        .copied()
        .filter(|n| *n % 2 == 0)
        .collect();

    println!("{:?}", evens); // output: [2, 4, 6]
}
```

Alternative pattern:

```rust
fn main() {
    let numbers = vec![1, 2, 3, 4];

    let bigger: Vec<i32> = numbers
        .iter()
        .copied()
        .filter(|&n| n > 2)
        .collect();

    println!("{:?}", bigger); // output: [3, 4]
}
```

## `.filter_map()` transform and remove invalid items

```rust
fn main() {
    let texts = vec!["10", "abc", "30"];

    let numbers: Vec<i32> = texts
        .iter()
        .filter_map(|text| text.parse::<i32>().ok())
        .collect();

    println!("{:?}", numbers); // output: [10, 30]
}
```

`.ok()` converts `Result<T, E>` into `Option<T>`:

```text
Ok(value) -> Some(value)
Err(_)    -> None
```

## `.enumerate()` add indexes

```rust
fn main() {
    let names = vec!["Abbas", "Ali", "Sara"];

    for (index, name) in names.iter().enumerate() {
        println!("{}: {}", index, name); // output: 0: Abbas then 1: Ali then 2: Sara
    }
}
```

## `.zip()` combine two lists

```rust
fn main() {
    let names = vec!["Ali", "Sara"];
    let scores = vec![90, 95];

    for (name, score) in names.iter().zip(scores.iter()) {
        println!("{} = {}", name, score); // output: Ali = 90 then Sara = 95
    }
}
```

## `.chain()` join iterators

```rust
fn main() {
    let a = vec![1, 2];
    let b = vec![3, 4];

    let result: Vec<i32> = a.iter().chain(b.iter()).copied().collect();

    println!("{:?}", result); // output: [1, 2, 3, 4]
}
```

## `.rev()`, `.take()`, `.skip()`

```rust
fn main() {
    let numbers = vec![1, 2, 3, 4, 5];

    let result: Vec<i32> = numbers
        .iter()
        .copied()
        .rev()
        .skip(1)
        .take(3)
        .collect();

    println!("{:?}", result); // output: [4, 3, 2]
}
```

---

# 8. Iterator Consumers

Consumers produce a final value from an iterator.

## Iterator consumer index

| Method | Common return type | Meaning |
|---|---|---|
| `.collect::<Vec<T>>()` | `Vec<T>` | build vector |
| `.sum()` | `T` | add all items |
| `.product()` | `T` | multiply all items |
| `.count()` | `usize` | count items |
| `.min()` | `Option<T>` or `Option<&T>` | smallest item |
| `.max()` | `Option<T>` or `Option<&T>` | biggest item |
| `.find(f)` | `Option<T>` or `Option<&T>` | first matching item |
| `.position(f)` | `Option<usize>` | index of first match |
| `.any(f)` | `bool` | true if any item matches |
| `.all(f)` | `bool` | true if all items match |

## `.sum()`

```rust
fn main() {
    let numbers = vec![1, 2, 3, 4];

    let total: i32 = numbers.iter().copied().sum();

    println!("{}", total); // output: 10
}
```

## `.product()`

```rust
fn main() {
    let numbers = vec![2, 3, 4];

    let result: i32 = numbers.iter().copied().product();

    println!("{}", result); // output: 24
}
```

## `.count()`

```rust
fn main() {
    let numbers = vec![1, 2, 3, 4, 5];

    let count = numbers.iter().copied().filter(|n| *n > 2).count();

    println!("{}", count); // output: 3
}
```

## `.min()` and `.max()`

```rust
fn main() {
    let numbers = vec![4, 1, 9, 2];

    let min = numbers.iter().copied().min();
    let max = numbers.iter().copied().max();

    println!("{:?}", min); // output: Some(1)
    println!("{:?}", max); // output: Some(9)
}
```

Resolve with `unwrap_or`:

```rust
fn main() {
    let numbers: Vec<i32> = Vec::new();

    let max = numbers.iter().copied().max().unwrap_or(0);

    println!("{}", max); // output: 0
}
```

## `.find()`

```rust
fn main() {
    let numbers = vec![1, 3, 6, 8];

    let found = numbers.iter().copied().find(|n| *n % 2 == 0);

    println!("{:?}", found); // output: Some(6)
}
```

## `.position()`

```rust
fn main() {
    let numbers = vec![10, 20, 30];

    let index = numbers.iter().position(|n| *n == 20);

    println!("{:?}", index); // output: Some(1)
}
```

## `.any()` and `.all()`

```rust
fn main() {
    let numbers = vec![2, 4, 6];

    let has_even = numbers.iter().any(|n| *n % 2 == 0);
    let all_even = numbers.iter().all(|n| *n % 2 == 0);

    println!("{}", has_even); // output: true
    println!("{}", all_even); // output: true
}
```

---

# 9. Safe Access and `Option`

Many collection methods return `Option` because the value might not exist.

## Option-returning collection methods

| Method | Common return type | Why Option? |
|---|---|---|
| `vec.pop()` | `Option<T>` | vector may be empty |
| `items.get(index)` | `Option<&T>` | index may not exist |
| `items.first()` | `Option<&T>` | collection may be empty |
| `items.last()` | `Option<&T>` | collection may be empty |
| `iter.find(...)` | `Option<T>` or `Option<&T>` | match may not exist |
| `iter.position(...)` | `Option<usize>` | match may not exist |
| `iter.min()` | `Option<T>` or `Option<&T>` | iterator may be empty |
| `iter.max()` | `Option<T>` or `Option<&T>` | iterator may be empty |

## Resolve `Option` with `match`

```rust
fn main() {
    let numbers = vec![10, 20, 30];

    match numbers.get(1) {
        Some(n) => println!("value = {}", n), // output: value = 20
        None => println!("not found"), // output: not found if missing
    }
}
```

## Resolve `Option` with `if let`

```rust
fn main() {
    let numbers = vec![10, 20, 30];

    if let Some(n) = numbers.first() {
        println!("first = {}", n); // output: first = 10
    }
}
```

## Resolve `Option` with `.unwrap_or(default)`

```rust
fn main() {
    let numbers: Vec<i32> = Vec::new();

    let first = numbers.first().copied().unwrap_or(0);

    println!("{}", first); // output: 0
}
```

## Resolve `Option` with `.expect(message)`

```rust
fn main() {
    let numbers = vec![10, 20, 30];

    let first = numbers.first().expect("expected at least one number");

    println!("{}", first); // output: 10
}
```

Use `.expect()` only when missing value should be a serious error.

---

# 10. Building New Collections

You can build new collections with loops or iterators.

## Build method index

| Pattern | Return / result | Use when |
|---|---|---|
| `let mut out = Vec::new(); out.push(x);` | `Vec<T>` | beginner-friendly, complex logic |
| `.map(...).collect()` | `Vec<T>` | transform every item |
| `.filter(...).collect()` | `Vec<T>` | keep some items |
| `.filter_map(...).collect()` | `Vec<T>` | transform and skip invalid items |

## Build with loop and `push`

```rust
fn double_numbers(numbers: &[i32]) -> Vec<i32> {
    let mut result = Vec::new();

    for n in numbers {
        result.push(*n * 2);
    }

    result
}

fn main() {
    let result = double_numbers(&[1, 2, 3]);

    println!("{:?}", result); // output: [2, 4, 6]
}
```

## Build with `.map()` and `.collect()`

```rust
fn double_numbers(numbers: &[i32]) -> Vec<i32> {
    numbers.iter().copied().map(|n| n * 2).collect()
}

fn main() {
    let result = double_numbers(&[1, 2, 3]);

    println!("{:?}", result); // output: [2, 4, 6]
}
```

## Build with `.filter()`

```rust
fn positive_numbers(numbers: &[i32]) -> Vec<i32> {
    numbers
        .iter()
        .copied()
        .filter(|n| *n > 0)
        .collect()
}

fn main() {
    let result = positive_numbers(&[-2, 0, 3, 5]);

    println!("{:?}", result); // output: [3, 5]
}
```

## Build with `.filter_map()`

```rust
fn parse_numbers(texts: &[&str]) -> Vec<i32> {
    texts
        .iter()
        .filter_map(|text| text.parse::<i32>().ok())
        .collect()
}

fn main() {
    let result = parse_numbers(&["10", "x", "30"]);

    println!("{:?}", result); // output: [10, 30]
}
```

---

# 11. Mutating Existing Collections

Sometimes you do not want to create a new vector. You want to change the existing one.

## Mutation method index

| Pattern / Method | Return type | Meaning |
|---|---|---|
| `for x in &mut vec` | `()` | mutate each item manually |
| `vec.iter_mut()` | iterator over `&mut T` | mutate through iterator |
| `vec.retain(f)` | `()` | remove non-matching items |
| `vec.sort()` | `()` | sort in place |
| `vec.reverse()` | `()` | reverse in place |
| `vec.clear()` | `()` | empty vector |

## Mutate with loop

```rust
fn main() {
    let mut numbers = vec![1, 2, 3];

    for n in &mut numbers {
        *n += 10;
    }

    println!("{:?}", numbers); // output: [11, 12, 13]
}
```

## Mutate with `.iter_mut()`

```rust
fn main() {
    let mut numbers = vec![1, 2, 3];

    numbers.iter_mut().for_each(|n| *n *= 2);

    println!("{:?}", numbers); // output: [2, 4, 6]
}
```

## Remove invalid values with `.retain()`

```rust
fn main() {
    let mut scores = vec![90, -10, 50, 120, 75];

    scores.retain(|score| *score >= 0 && *score <= 100);

    println!("{:?}", scores); // output: [90, 50, 75]
}
```

---

# 12. Sorting and Searching

Sorting/searching appears often in exam exercises.

## Sorting/searching method index

| Method / Pattern | Common return type | Meaning |
|---|---|---|
| `vec.sort()` | `()` | ascending sort |
| `vec.sort_by(...)` | `()` | custom comparator |
| `vec.sort_by_key(...)` | `()` | sort by extracted key |
| `vec.reverse()` | `()` | reverse current order |
| `slice.binary_search(&value)` | `Result<usize, usize>` | search sorted slice |
| `iter.find(...)` | `Option<T>` or `Option<&T>` | first matching item |
| `iter.position(...)` | `Option<usize>` | first matching index |

## Sort ascending and descending

```rust
fn main() {
    let mut numbers = vec![4, 1, 9, 2];

    numbers.sort();
    println!("{:?}", numbers); // output: [1, 2, 4, 9]

    numbers.reverse();
    println!("{:?}", numbers); // output: [9, 4, 2, 1]
}
```

## Sort by key

```rust
fn main() {
    let mut words = vec!["rust", "go", "javascript"];

    words.sort_by_key(|word| word.len());

    println!("{:?}", words); // output: ["go", "rust", "javascript"]
}
```

## Binary search

`binary_search` requires the data to be sorted.

```rust
fn main() {
    let numbers = vec![10, 20, 30, 40];

    match numbers.binary_search(&30) {
        Ok(index) => println!("found at {}", index), // output: found at 2
        Err(index) => println!("could insert at {}", index), // output: insertion index if missing
    }
}
```

## Search with `.find()`

```rust
fn main() {
    let numbers = vec![5, 7, 10, 12];

    let first_even = numbers.iter().copied().find(|n| *n % 2 == 0);

    println!("{:?}", first_even); // output: Some(10)
}
```

## Search index with `.position()`

```rust
fn main() {
    let names = vec!["Ali", "Sara", "Zainab"];

    let index = names.iter().position(|name| *name == "Sara");

    println!("{:?}", index); // output: Some(1)
}
```

---

# 13. Common Transformation Templates

## Template 1: Keep valid scores and sort descending

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
    let scores = [90, -5, 120, 75, 60];
    let result = clean_scores(&scores);

    println!("{:?}", result); // output: [90, 75, 60]
}
```

## Template 2: Add bonus and clamp

```rust
fn add_bonus(scores: &[i32]) -> Vec<i32> {
    scores
        .iter()
        .copied()
        .map(|score| (score + 10).clamp(0, 100))
        .collect()
}

fn main() {
    let scores = [95, 60, -20];
    let result = add_bonus(&scores);

    println!("{:?}", result); // output: [100, 70, 0]
}
```

## Template 3: Count matching items

```rust
fn count_even(numbers: &[i32]) -> usize {
    numbers.iter().filter(|n| **n % 2 == 0).count()
}

fn main() {
    let numbers = [1, 2, 3, 4, 6];

    println!("{}", count_even(&numbers)); // output: 3
}
```

Alternative with `.copied()`:

```rust
fn count_even(numbers: &[i32]) -> usize {
    numbers.iter().copied().filter(|n| *n % 2 == 0).count()
}

fn main() {
    let numbers = [1, 2, 3, 4, 6];

    println!("{}", count_even(&numbers)); // output: 3
}
```

## Template 4: Find maximum safely

```rust
fn max_or_zero(numbers: &[i32]) -> i32 {
    numbers.iter().copied().max().unwrap_or(0)
}

fn main() {
    println!("{}", max_or_zero(&[3, 9, 2])); // output: 9
    println!("{}", max_or_zero(&[])); // output: 0
}
```

## Template 5: Convert vector of references to owned values

```rust
fn main() {
    let numbers = vec![1, 2, 3];

    let refs: Vec<&i32> = numbers.iter().collect();
    let values: Vec<i32> = numbers.iter().copied().collect();

    println!("{:?}", refs); // output: [1, 2, 3]
    println!("{:?}", values); // output: [1, 2, 3]
}
```

---

# 14. Common Confusions with `&`, `*`, `.iter()`, and `.filter()`

## Why `.contains(&20)`?

```rust
fn main() {
    let numbers = vec![10, 20, 30];

    println!("{}", numbers.contains(&20)); // output: true
}
```

`.contains()` expects `&T`, so you pass `&20`.

## Why `*n` in filters?

```rust
fn main() {
    let numbers = vec![1, 2, 3, 4];

    let result: Vec<i32> = numbers
        .iter()
        .copied()
        .filter(|n| *n > 2)
        .collect();

    println!("{:?}", result); // output: [3, 4]
}
```

After `.copied()`, the iterator item is `i32`, but `.filter()` passes `&i32` to the closure, so `n` is `&i32` inside the closure.

You can write this instead:

```rust
fn main() {
    let numbers = vec![1, 2, 3, 4];

    let result: Vec<i32> = numbers
        .iter()
        .copied()
        .filter(|&n| n > 2)
        .collect();

    println!("{:?}", result); // output: [3, 4]
}
```

## `map` does not have the same closure reference issue as `filter`

```rust
fn main() {
    let numbers = vec![1, 2, 3];

    let doubled: Vec<i32> = numbers
        .iter()
        .copied()
        .map(|n| n * 2)
        .collect();

    println!("{:?}", doubled); // output: [2, 4, 6]
}
```

`map` receives the item directly. `filter` receives a reference to the item.

---

# 15. Common Compiler Errors and Fixes

## Error: `can't call method abs on ambiguous numeric type`

Wrong:

```rust
fn main() {
    let numbers = vec![-1, 2, 3];
    let abs: Vec<i32> = numbers.iter().map(|&n| n.abs()).collect();

    println!("{:?}", abs); // output: compiler error before running
}
```

Fix by specifying the vector type:

```rust
fn main() {
    let numbers: Vec<i32> = vec![-1, 2, 3];
    let abs: Vec<i32> = numbers.iter().map(|&n| n.abs()).collect();

    println!("{:?}", abs); // output: [1, 2, 3]
}
```

Or use `.copied()`:

```rust
fn main() {
    let numbers: Vec<i32> = vec![-1, 2, 3];
    let abs: Vec<i32> = numbers.iter().copied().map(|n| n.abs()).collect();

    println!("{:?}", abs); // output: [1, 2, 3]
}
```

## Error: expected `Vec<i32>`, found `Vec<&i32>`

Wrong:

```rust
fn main() {
    let numbers = vec![1, 2, 3];
    let result: Vec<i32> = numbers.iter().collect();

    println!("{:?}", result); // output: compiler error before running
}
```

Fix:

```rust
fn main() {
    let numbers = vec![1, 2, 3];
    let result: Vec<i32> = numbers.iter().copied().collect();

    println!("{:?}", result); // output: [1, 2, 3]
}
```

## Error: cannot borrow as mutable

Wrong:

```rust
fn main() {
    let numbers = vec![1, 2, 3];
    numbers.push(4);

    println!("{:?}", numbers); // output: compiler error before running
}
```

Fix:

```rust
fn main() {
    let mut numbers = vec![1, 2, 3];
    numbers.push(4);

    println!("{:?}", numbers); // output: [1, 2, 3, 4]
}
```

---

# 16. Quick Return-Type Table

| Method / Syntax | Common return type | Notes |
|---|---|---|
| `[1, 2, 3]` | `[i32; 3]` | fixed array |
| `vec![1, 2, 3]` | `Vec<i32>` | growable vector |
| `&items[..]` | `&[T]` | slice |
| `arr.to_vec()` | `Vec<T>` | copy/clone into Vec |
| `vec.push(x)` | `()` | changes vector |
| `vec.pop()` | `Option<T>` | may be empty |
| `items.get(i)` | `Option<&T>` | safe index |
| `items.first()` | `Option<&T>` | may be empty |
| `items.last()` | `Option<&T>` | may be empty |
| `vec.sort()` | `()` | changes vector |
| `vec.reverse()` | `()` | changes vector |
| `vec.retain(f)` | `()` | changes vector |
| `iter.map(f)` | iterator | lazy transform |
| `iter.filter(f)` | iterator | lazy filter |
| `iter.collect::<Vec<T>>()` | `Vec<T>` | builds Vec |
| `iter.sum()` | `T` | may need type annotation |
| `iter.count()` | `usize` | count items |
| `iter.find(f)` | `Option<T>` or `Option<&T>` | first match |
| `iter.position(f)` | `Option<usize>` | index of match |
| `iter.any(f)` | `bool` | any match |
| `iter.all(f)` | `bool` | all match |

---

# 17. Practice Exercises

## Exercise 1 — Convert array to vector

Write a program that starts with:

```rust
let arr = [3, 1, 4];
```

Requirements:

1. Convert it to a vector.
2. Push `2`.
3. Sort it.
4. Print the vector.

Expected:

```rust
println!("{:?}", result); // output: [1, 2, 3, 4]
```

## Exercise 2 — Keep even numbers

Write:

```rust
fn even_numbers(numbers: &[i32]) -> Vec<i32>
```

Expected:

```rust
println!("{:?}", even_numbers(&[1, 2, 3, 4, 5, 6])); // output: [2, 4, 6]
```

## Exercise 3 — Double positive numbers

Write:

```rust
fn double_positive(numbers: &[i32]) -> Vec<i32>
```

Rules:

1. Keep only positive numbers.
2. Double each one.

Expected:

```rust
println!("{:?}", double_positive(&[-2, 3, 0, 5])); // output: [6, 10]
```

## Exercise 4 — Safe access

Write:

```rust
fn get_item(numbers: &[i32], index: usize) -> String
```

Expected:

```rust
println!("{}", get_item(&[10, 20, 30], 1)); // output: item = 20
println!("{}", get_item(&[10, 20, 30], 9)); // output: not found
```

## Exercise 5 — Find first even

Write:

```rust
fn first_even(numbers: &[i32]) -> Option<i32>
```

Expected:

```rust
println!("{:?}", first_even(&[1, 3, 6, 8])); // output: Some(6)
println!("{:?}", first_even(&[1, 3, 5])); // output: None
```

## Exercise 6 — Clean scores

Write:

```rust
fn clean_scores(scores: &[i32]) -> Vec<i32>
```

Rules:

1. Keep only scores from `0` to `100`.
2. Add `10` bonus.
3. Clamp to `100`.
4. Sort descending.

Expected:

```rust
println!("{:?}", clean_scores(&[90, -5, 50, 120, 95])); // output: [100, 100, 60]
```

---

End of cheat_sheet3.md.
