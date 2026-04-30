# Rust Cheat Sheet 7 — HashMap, HashSet, Queues, Trees, and Data Structures

This is **cheat_sheet7.md**.

This sheet focuses on exam-style data structures beyond basic `Vec`.

## Style used in this sheet

- Method tables are placed inside their related subject sections.
- Every `println!` example has `// output:` on the same line whenever practical.
- Method headings show common return types.
- HashMap and HashSet debug order is not stable, so examples avoid relying on printed map/set order.

---

# 1. Imports for Common Data Structures

Most advanced containers are in `std::collections`.

```rust
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::collections::VecDeque;
use std::collections::BinaryHeap;
```

Common combined import:

```rust
use std::collections::{HashMap, HashSet, VecDeque, BinaryHeap};
```

---

# 2. HashMap

A `HashMap<K, V>` stores key-value pairs.

Think:

```text
key -> value
name -> score
word -> count
id -> user
```

Example:

```rust
use std::collections::HashMap;

fn main() {
    let mut scores: HashMap<&str, i32> = HashMap::new();

    scores.insert("Ali", 90);
    scores.insert("Sara", 95);

    println!("{}", scores.len()); // output: 2
    println!("{}", scores.contains_key("Ali")); // output: true
}
```

## HashMap method index

| Method / syntax | Common return type | Meaning |
|---|---:|---|
| `HashMap::new()` | `HashMap<K, V>` | create empty map |
| `.insert(key, value)` | `Option<V>` | add/update value; returns old value if key existed |
| `.get(&key)` | `Option<&V>` | read value safely |
| `.get_mut(&key)` | `Option<&mut V>` | edit value safely |
| `.contains_key(&key)` | `bool` | check if key exists |
| `.remove(&key)` | `Option<V>` | remove key and return value |
| `.entry(key)` | `Entry<K, V>` | insert/update with entry API |
| `.or_insert(value)` | `&mut V` | insert default if missing |
| `.len()` | `usize` | number of pairs |
| `.is_empty()` | `bool` | true if empty |
| `.keys()` | iterator | iterate keys |
| `.values()` | iterator | iterate values |
| `.iter()` | iterator over `(&K, &V)` | borrow pairs |
| `.iter_mut()` | iterator over `(&K, &mut V)` | mutate values |

---

# 3. HashMap Insert and Update

## `.insert(key, value) -> Option<V>`

If the key is new, `.insert()` returns `None`.

If the key already exists, `.insert()` replaces the value and returns `Some(old_value)`.

```rust
use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    let old = scores.insert("Ali", 90);
    println!("{:?}", old); // output: None

    let old = scores.insert("Ali", 100);
    println!("{:?}", old); // output: Some(90)

    println!("{}", scores["Ali"]); // output: 100
}
```

Important:

```rust
scores["Ali"]
```

works if the key exists. It panics if the key is missing. Prefer `.get()` for safe access.

---

# 4. HashMap Safe Access

## `.get(&key) -> Option<&V>`

```rust
use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert("Ali", 90);

    match scores.get("Ali") {
        Some(score) => println!("{}", score), // output: 90
        None => println!("not found"), // output: not found if key missing
    }

    match scores.get("Mona") {
        Some(score) => println!("{}", score), // output: score if found
        None => println!("not found"), // output: not found
    }
}
```

## Common ways to resolve `Option<&V>`

```rust
use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert("Ali", 90);

    let score = scores.get("Ali").unwrap_or(&0);
    println!("{}", score); // output: 90

    let missing = scores.get("Mona").unwrap_or(&0);
    println!("{}", missing); // output: 0
}
```

Important: `.unwrap_or(&0)` uses a reference because `.get()` returns `Option<&V>`.

---

# 5. HashMap Mutable Access

## `.get_mut(&key) -> Option<&mut V>`

Use this when you want to change a value inside the map.

```rust
use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert("Ali", 90);

    if let Some(score) = scores.get_mut("Ali") {
        *score += 10;
    }

    println!("{}", scores["Ali"]); // output: 100
}
```

Why `*score`?

```text
score is &mut i32
*score is the actual i32 value
```

---

# 6. HashMap Entry API

The entry API is excellent for counting, grouping, and inserting defaults.

## Entry method index

| Method / syntax | Common return type | Meaning |
|---|---:|---|
| `.entry(key)` | `Entry<K, V>` | access key slot |
| `.or_insert(value)` | `&mut V` | insert if missing, return mutable reference |
| `.or_insert_with(|| value)` | `&mut V` | lazily insert if missing |
| `.and_modify(|v| ...)` | `Entry<K, V>` | modify if key exists |
| `.or_default()` | `&mut V` | insert default value if missing |

## `.entry(key).or_insert(value) -> &mut V`

Word counter:

```rust
use std::collections::HashMap;

fn main() {
    let text = "rust rust code";
    let mut counts: HashMap<&str, i32> = HashMap::new();

    for word in text.split_whitespace() {
        let count = counts.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{}", counts["rust"]); // output: 2
    println!("{}", counts["code"]); // output: 1
}
```

Shorter counting pattern:

```rust
use std::collections::HashMap;

fn main() {
    let words = ["red", "blue", "red"];
    let mut counts = HashMap::new();

    for word in words {
        *counts.entry(word).or_insert(0) += 1;
    }

    println!("{}", counts["red"]); // output: 2
    println!("{}", counts["blue"]); // output: 1
}
```

## `.and_modify(...).or_insert(...)`

```rust
use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.entry("Ali").and_modify(|score| *score += 10).or_insert(90);
    scores.entry("Ali").and_modify(|score| *score += 10).or_insert(90);

    println!("{}", scores["Ali"]); // output: 100
}
```

Explanation:

```text
First call: Ali missing, insert 90.
Second call: Ali exists, add 10.
```

---

# 7. Iterating HashMap

HashMap order is not guaranteed. Do not depend on the printed order.

## `.iter() -> iterator over (&K, &V)`

```rust
use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert("Ali", 90);
    scores.insert("Sara", 95);

    let total: i32 = scores.values().copied().sum();

    println!("{}", total); // output: 185
}
```

## `.keys() -> iterator over &K`

```rust
use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert("Ali", 90);
    scores.insert("Sara", 95);

    println!("{}", scores.keys().count()); // output: 2
}
```

## `.values() -> iterator over &V`

```rust
use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert("Ali", 90);
    scores.insert("Sara", 95);

    let best = scores.values().max();

    println!("{:?}", best); // output: Some(95)
}
```

---

# 8. HashMap Grouping Pattern

Group items by a key.

```rust
use std::collections::HashMap;

fn main() {
    let names = ["Ali", "Abbas", "Sara", "Sami"];
    let mut groups: HashMap<char, Vec<&str>> = HashMap::new();

    for name in names {
        let first = name.chars().next().unwrap();
        groups.entry(first).or_insert(Vec::new()).push(name);
    }

    println!("{}", groups.get(&'A').unwrap().len()); // output: 2
    println!("{}", groups.get(&'S').unwrap().len()); // output: 2
}
```

Cleaner default version:

```rust
use std::collections::HashMap;

fn main() {
    let names = ["Ali", "Abbas", "Sara"];
    let mut groups: HashMap<char, Vec<&str>> = HashMap::new();

    for name in names {
        let first = name.chars().next().unwrap();
        groups.entry(first).or_default().push(name);
    }

    println!("{}", groups.get(&'A').unwrap().len()); // output: 2
}
```

---

# 9. HashSet

A `HashSet<T>` stores unique values.

Use it when you need:

```text
unique items
membership checks
duplicates removal
set operations
```

```rust
use std::collections::HashSet;

fn main() {
    let mut seen = HashSet::new();

    seen.insert("rust");
    seen.insert("rust");
    seen.insert("go");

    println!("{}", seen.len()); // output: 2
    println!("{}", seen.contains("rust")); // output: true
}
```

## HashSet method index

| Method / syntax | Common return type | Meaning |
|---|---:|---|
| `HashSet::new()` | `HashSet<T>` | create empty set |
| `.insert(value)` | `bool` | true if value was newly inserted |
| `.contains(&value)` | `bool` | check membership |
| `.remove(&value)` | `bool` | remove value if present |
| `.len()` | `usize` | number of unique items |
| `.is_empty()` | `bool` | true if empty |
| `.iter()` | iterator over `&T` | borrow values |
| `.union(&other)` | iterator | all values in either set |
| `.intersection(&other)` | iterator | values in both sets |
| `.difference(&other)` | iterator | values in first but not second |
| `.is_subset(&other)` | `bool` | all values are inside other |
| `.is_superset(&other)` | `bool` | contains all values of other |

---

# 10. HashSet Insert, Contains, Remove

## `.insert(value) -> bool`

```rust
use std::collections::HashSet;

fn main() {
    let mut set = HashSet::new();

    println!("{}", set.insert(10)); // output: true
    println!("{}", set.insert(10)); // output: false
    println!("{}", set.len()); // output: 1
}
```

## `.contains(&value) -> bool`

For numbers:

```rust
use std::collections::HashSet;

fn main() {
    let set: HashSet<i32> = [1, 2, 3].into_iter().collect();

    println!("{}", set.contains(&2)); // output: true
}
```

For `&str`, this also works:

```rust
use std::collections::HashSet;

fn main() {
    let set: HashSet<&str> = ["rust", "go"].into_iter().collect();

    println!("{}", set.contains("rust")); // output: true
}
```

## `.remove(&value) -> bool`

```rust
use std::collections::HashSet;

fn main() {
    let mut set: HashSet<i32> = [1, 2, 3].into_iter().collect();

    println!("{}", set.remove(&2)); // output: true
    println!("{}", set.contains(&2)); // output: false
}
```

---

# 11. HashSet Common Patterns

## Remove duplicates

```rust
use std::collections::HashSet;

fn main() {
    let numbers = [1, 2, 2, 3, 3, 3];
    let unique: HashSet<i32> = numbers.into_iter().collect();

    println!("{}", unique.len()); // output: 3
    println!("{}", unique.contains(&1)); // output: true
}
```

## Check if all characters are unique

```rust
use std::collections::HashSet;

fn all_unique(text: &str) -> bool {
    let mut seen = HashSet::new();

    for c in text.chars() {
        if !seen.insert(c) {
            return false;
        }
    }

    true
}

fn main() {
    println!("{}", all_unique("rust")); // output: true
    println!("{}", all_unique("hello")); // output: false
}
```

## Pangram-style check

```rust
use std::collections::HashSet;

fn has_all_abc(text: &str) -> bool {
    let letters: HashSet<char> = text.chars().collect();

    letters.contains(&'a') && letters.contains(&'b') && letters.contains(&'c')
}

fn main() {
    println!("{}", has_all_abc("cab")); // output: true
    println!("{}", has_all_abc("cat")); // output: false
}
```

---

# 12. BTreeMap and BTreeSet

`BTreeMap` and `BTreeSet` keep keys/items sorted.

Use them when order matters.

```rust
use std::collections::BTreeMap;

fn main() {
    let mut scores = BTreeMap::new();

    scores.insert("Ali", 90);
    scores.insert("Sara", 95);

    println!("{}", scores.keys().next().unwrap()); // output: Ali
}
```

## BTreeMap / BTreeSet method index

| Type | Useful methods | Why use it |
|---|---|---|
| `BTreeMap<K, V>` | `new`, `insert`, `get`, `remove`, `keys`, `values`, `range` | sorted key-value pairs |
| `BTreeSet<T>` | `new`, `insert`, `contains`, `remove`, `range` | sorted unique values |

## BTreeSet sorted unique values

```rust
use std::collections::BTreeSet;

fn main() {
    let numbers = [3, 1, 2, 3];
    let set: BTreeSet<i32> = numbers.into_iter().collect();

    println!("{}", set.first().unwrap()); // output: 1
    println!("{}", set.last().unwrap()); // output: 3
}
```

## `.range(...) -> iterator`

```rust
use std::collections::BTreeSet;

fn main() {
    let set: BTreeSet<i32> = [1, 2, 3, 4, 5].into_iter().collect();
    let middle: Vec<i32> = set.range(2..=4).copied().collect();

    println!("{:?}", middle); // output: [2, 3, 4]
}
```

---

# 13. VecDeque

`VecDeque<T>` is a double-ended queue.

Use it when you need efficient push/pop from both front and back.

```rust
use std::collections::VecDeque;

fn main() {
    let mut queue = VecDeque::new();

    queue.push_back("first");
    queue.push_back("second");

    println!("{:?}", queue.pop_front()); // output: Some("first")
    println!("{:?}", queue.pop_front()); // output: Some("second")
}
```

## VecDeque method index

| Method / syntax | Common return type | Meaning |
|---|---:|---|
| `VecDeque::new()` | `VecDeque<T>` | create empty queue |
| `.push_back(value)` | `()` | add to back |
| `.push_front(value)` | `()` | add to front |
| `.pop_back()` | `Option<T>` | remove from back |
| `.pop_front()` | `Option<T>` | remove from front |
| `.front()` | `Option<&T>` | read front |
| `.back()` | `Option<&T>` | read back |
| `.len()` | `usize` | number of items |
| `.is_empty()` | `bool` | true if empty |

## Queue pattern: first in, first out

```rust
use std::collections::VecDeque;

fn main() {
    let mut queue = VecDeque::new();

    queue.push_back(10);
    queue.push_back(20);
    queue.push_back(30);

    println!("{:?}", queue.pop_front()); // output: Some(10)
    println!("{:?}", queue.pop_front()); // output: Some(20)
}
```

## Stack-like pattern with VecDeque

```rust
use std::collections::VecDeque;

fn main() {
    let mut stack = VecDeque::new();

    stack.push_back(10);
    stack.push_back(20);

    println!("{:?}", stack.pop_back()); // output: Some(20)
    println!("{:?}", stack.pop_back()); // output: Some(10)
}
```

---

# 14. BinaryHeap

`BinaryHeap<T>` is a priority queue.

By default, Rust's `BinaryHeap` is a max-heap: largest value comes out first.

```rust
use std::collections::BinaryHeap;

fn main() {
    let mut heap = BinaryHeap::new();

    heap.push(30);
    heap.push(10);
    heap.push(20);

    println!("{:?}", heap.pop()); // output: Some(30)
    println!("{:?}", heap.pop()); // output: Some(20)
}
```

## BinaryHeap method index

| Method / syntax | Common return type | Meaning |
|---|---:|---|
| `BinaryHeap::new()` | `BinaryHeap<T>` | create empty heap |
| `.push(value)` | `()` | insert item |
| `.pop()` | `Option<T>` | remove highest-priority item |
| `.peek()` | `Option<&T>` | read highest-priority item |
| `.len()` | `usize` | number of items |
| `.is_empty()` | `bool` | true if empty |
| `.into_sorted_vec()` | `Vec<T>` | sorted ascending vector |

## `.peek() -> Option<&T>`

```rust
use std::collections::BinaryHeap;

fn main() {
    let mut heap = BinaryHeap::new();

    heap.push(5);
    heap.push(100);
    heap.push(20);

    println!("{:?}", heap.peek()); // output: Some(100)
    println!("{}", heap.len()); // output: 3
}
```

## `.into_sorted_vec() -> Vec<T>`

```rust
use std::collections::BinaryHeap;

fn main() {
    let heap: BinaryHeap<i32> = [3, 1, 2].into_iter().collect();
    let sorted = heap.into_sorted_vec();

    println!("{:?}", sorted); // output: [1, 2, 3]
}
```

## Min-heap with `Reverse`

Use `std::cmp::Reverse` to make smallest value come out first.

```rust
use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn main() {
    let mut heap = BinaryHeap::new();

    heap.push(Reverse(30));
    heap.push(Reverse(10));
    heap.push(Reverse(20));

    println!("{:?}", heap.pop().unwrap().0); // output: 10
    println!("{:?}", heap.pop().unwrap().0); // output: 20
}
```

---

# 15. Stack Patterns

A stack is last in, first out.

You can use `Vec<T>` as a stack.

```rust
fn main() {
    let mut stack = Vec::new();

    stack.push('(');
    stack.push('[');

    println!("{:?}", stack.pop()); // output: Some('[')
    println!("{:?}", stack.pop()); // output: Some('(')
}
```

## Stack method index with Vec

| Method | Common return type | Meaning |
|---|---:|---|
| `.push(value)` | `()` | push onto stack |
| `.pop()` | `Option<T>` | remove top |
| `.last()` | `Option<&T>` | view top |
| `.is_empty()` | `bool` | check empty |

## Bracket matching pattern

```rust
fn brackets_match(text: &str) -> bool {
    let mut stack = Vec::new();

    for c in text.chars() {
        match c {
            '(' => stack.push(')'),
            '[' => stack.push(']'),
            '{' => stack.push('}'),
            ')' | ']' | '}' => {
                if stack.pop() != Some(c) {
                    return false;
                }
            }
            _ => {}
        }
    }

    stack.is_empty()
}

fn main() {
    println!("{}", brackets_match("([])")); // output: true
    println!("{}", brackets_match("([)]")); // output: false
}
```

---

# 16. Custom Data Structures with Structs

Use structs to group related data.

```rust
struct User {
    id: u32,
    name: String,
    score: i32,
}

fn main() {
    let user = User {
        id: 1,
        name: String::from("Ali"),
        score: 90,
    };

    println!("{} {}", user.id, user.name); // output: 1 Ali
}
```

## Struct data container method index

| Pattern | Meaning |
|---|---|
| `struct Name { fields }` | define data shape |
| `Name { field: value }` | create value |
| `value.field` | read field |
| `value.field = new_value` | edit field if value is mutable |
| `impl Name { ... }` | add methods |

## Struct with methods

```rust
struct Counter {
    value: i32,
}

impl Counter {
    fn new() -> Counter {
        Counter { value: 0 }
    }

    fn increment(&mut self) {
        self.value += 1;
    }

    fn get(&self) -> i32 {
        self.value
    }
}

fn main() {
    let mut counter = Counter::new();
    counter.increment();

    println!("{}", counter.get()); // output: 1
}
```

---

# 17. Trees with Box

Recursive data structures need indirection. `Box<T>` is the common beginner solution.

```rust
#[derive(Debug)]
enum Tree {
    Empty,
    Node {
        value: i32,
        left: Box<Tree>,
        right: Box<Tree>,
    },
}

fn count_nodes(tree: &Tree) -> i32 {
    match tree {
        Tree::Empty => 0,
        Tree::Node { left, right, .. } => 1 + count_nodes(left) + count_nodes(right),
    }
}

fn main() {
    let tree = Tree::Node {
        value: 10,
        left: Box::new(Tree::Empty),
        right: Box::new(Tree::Node {
            value: 20,
            left: Box::new(Tree::Empty),
            right: Box::new(Tree::Empty),
        }),
    };

    println!("{}", count_nodes(&tree)); // output: 2
}
```

## Tree pattern index

| Pattern | Meaning |
|---|---|
| `enum Tree { Empty, Node { ... } }` | recursive tree shape |
| `Box<Tree>` | store recursive child indirectly |
| `match tree` | handle empty vs node |
| recursive function | process left/right children |

---

# 18. Linked List with Box

A simple recursive linked list:

```rust
enum List {
    Empty,
    Node(i32, Box<List>),
}

fn len(list: &List) -> i32 {
    match list {
        List::Empty => 0,
        List::Node(_, next) => 1 + len(next),
    }
}

fn main() {
    let list = List::Node(
        10,
        Box::new(List::Node(
            20,
            Box::new(List::Empty),
        )),
    );

    println!("{}", len(&list)); // output: 2
}
```

---

# 19. Data Structure Choice Table

| Need | Good choice |
|---|---|
| unique values | `HashSet<T>` |
| key-value lookup | `HashMap<K, V>` |
| sorted unique values | `BTreeSet<T>` |
| sorted key-value pairs | `BTreeMap<K, V>` |
| first-in-first-out queue | `VecDeque<T>` |
| priority queue | `BinaryHeap<T>` |
| stack | `Vec<T>` |
| recursive tree/list | `Box<T>` with enum |
| group by key | `HashMap<K, Vec<V>>` |
| count frequency | `HashMap<T, i32>` |

---

# 20. Utilizing Collections in Loops

Collections become most useful when combined with loops. The main choice is whether the loop should read, mutate, or consume the collection.

## Collection loop method table

| Pattern | Common item type | Meaning | Collection usable after? |
|---|---|---|---|
| `for item in &vec` | `&T` | read values | yes |
| `for item in vec.iter()` | `&T` | read values | yes |
| `for item in &mut vec` | `&mut T` | edit values | yes |
| `for item in vec.iter_mut()` | `&mut T` | edit values | yes |
| `for item in vec` | `T` | move/consume values | no, unless items are `Copy` and array behavior applies |
| `while let Some(x) = vec.pop()` | `T` | remove from end until empty | yes, but emptied |
| `while let Some(x) = queue.pop_front()` | `T` | queue processing | yes, but changed |
| `while let Some(x) = heap.pop()` | `T` | priority processing | yes, but changed |

## Loop over `Vec` by reference

Use this when you only want to read the values.

```rust
fn main() {
    let numbers = vec![10, 20, 30];
    let mut total = 0;

    for n in &numbers {
        total += *n;
    }

    println!("total = {}", total); // output: total = 60
    println!("len = {}", numbers.len()); // output: len = 3
}
```

## Loop over `Vec` mutably

Use this when you want to edit every value in place.

```rust
fn main() {
    let mut numbers = vec![1, 2, 3];

    for n in &mut numbers {
        *n *= 10;
    }

    println!("{:?}", numbers); // output: [10, 20, 30]
}
```

## Consume a `Vec` in a loop

Use this when you do not need the vector after the loop.

```rust
fn main() {
    let words = vec![String::from("red"), String::from("blue")];
    let mut total_len = 0;

    for word in words {
        total_len += word.len();
    }

    println!("total length = {}", total_len); // output: total length = 7
}
```

After `for word in words`, the vector is moved and cannot be used again.

## Loop over `HashMap` by reference

`HashMap` order is not stable, so avoid relying on printed order. For exam logic, calculate totals or search values.

```rust
use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert("Ali", 80);
    scores.insert("Sara", 90);

    let mut total = 0;

    for score in scores.values() {
        total += *score;
    }

    println!("total = {}", total); // output: total = 170
}
```

## Loop over `HashMap` key-value pairs

Use `for (key, value) in &map` when reading both keys and values.

```rust
use std::collections::HashMap;

fn main() {
    let mut prices = HashMap::new();
    prices.insert("book", 3);
    prices.insert("pen", 2);

    let mut total = 0;

    for (_item, price) in &prices {
        total += *price;
    }

    println!("total = {}", total); // output: total = 5
}
```

## Mutate `HashMap` values in a loop

Use `.values_mut()` when you only need to edit values.

```rust
use std::collections::HashMap;

fn main() {
    let mut stock = HashMap::new();
    stock.insert("apple", 5);
    stock.insert("banana", 10);

    for quantity in stock.values_mut() {
        *quantity += 1;
    }

    let total: i32 = stock.values().copied().sum();
    println!("total = {}", total); // output: total = 17
}
```

## Loop over `BTreeMap` when order matters

Use `BTreeMap` when you want sorted key order.

```rust
use std::collections::BTreeMap;

fn main() {
    let mut ages = BTreeMap::new();
    ages.insert("Sara", 20);
    ages.insert("Ali", 18);

    for (name, age) in &ages {
        println!("{} = {}", name, age); // output: Ali = 18 then Sara = 20
    }
}
```

## Loop over `HashSet`

Use a set when you care about uniqueness. `HashSet` order is not stable, so examples should avoid depending on order.

```rust
use std::collections::HashSet;

fn main() {
    let numbers: HashSet<i32> = [1, 2, 2, 3].into_iter().collect();
    let mut total = 0;

    for n in &numbers {
        total += *n;
    }

    println!("unique count = {}", numbers.len()); // output: unique count = 3
    println!("total = {}", total); // output: total = 6
}
```

## Loop over `VecDeque` as a queue

Use `pop_front()` for first-in-first-out processing.

```rust
use std::collections::VecDeque;

fn main() {
    let mut queue = VecDeque::from(["A", "B", "C"]);

    while let Some(name) = queue.pop_front() {
        println!("served {}", name); // output: served A then served B then served C
    }

    println!("empty = {}", queue.is_empty()); // output: empty = true
}
```

## Loop over `BinaryHeap` as a priority queue

`BinaryHeap` pops the largest item first by default.

```rust
use std::collections::BinaryHeap;

fn main() {
    let mut heap = BinaryHeap::from([3, 10, 5]);

    while let Some(priority) = heap.pop() {
        println!("{}", priority); // output: 10 then 5 then 3
    }
}
```

## Stack loop with `Vec`

Use `push()` and `pop()` for last-in-first-out stack behavior.

```rust
fn main() {
    let mut stack = Vec::new();
    stack.push('a');
    stack.push('b');
    stack.push('c');

    while let Some(ch) = stack.pop() {
        println!("{}", ch); // output: c then b then a
    }
}
```

## Avoid removing while iterating by reference

This is a common beginner mistake. Use `.retain()` when you want to remove items based on a condition.

```rust
fn main() {
    let mut numbers = vec![1, 2, 3, 4];

    numbers.retain(|n| *n % 2 == 0);

    println!("{:?}", numbers); // output: [2, 4]
}
```

## Collect keys first, then mutate map

When you need to remove from a map based on values, collect the keys first.

```rust
use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert("Ali", 40);
    scores.insert("Sara", 90);
    scores.insert("Omar", 30);

    let to_remove: Vec<&str> = scores
        .iter()
        .filter(|(_name, score)| **score < 50)
        .map(|(name, _score)| *name)
        .collect();

    for name in to_remove {
        scores.remove(name);
    }

    println!("remaining = {}", scores.len()); // output: remaining = 1
}
```

## Loop decision rule

| Goal | Good pattern |
|---|---|
| Read values | `for x in &collection` |
| Edit values | `for x in &mut collection` |
| Consume values | `for x in collection` |
| Remove from end | `while let Some(x) = vec.pop()` |
| Process queue | `while let Some(x) = queue.pop_front()` |
| Process priority | `while let Some(x) = heap.pop()` |
| Remove by condition | `.retain(...)` |
| Mutate map values | `for value in map.values_mut()` |
| Ordered map loop | use `BTreeMap` |

---

# 21. Common Exam Patterns

## Frequency counter

```rust
use std::collections::HashMap;

fn count_letters(text: &str) -> HashMap<char, i32> {
    let mut counts = HashMap::new();

    for c in text.chars() {
        *counts.entry(c).or_insert(0) += 1;
    }

    counts
}

fn main() {
    let counts = count_letters("banana");

    println!("{}", counts[&'a']); // output: 3
    println!("{}", counts[&'b']); // output: 1
}
```

## Two-sum existence with HashSet

```rust
use std::collections::HashSet;

fn has_pair_sum(numbers: &[i32], target: i32) -> bool {
    let mut seen = HashSet::new();

    for &n in numbers {
        let needed = target - n;

        if seen.contains(&needed) {
            return true;
        }

        seen.insert(n);
    }

    false
}

fn main() {
    println!("{}", has_pair_sum(&[2, 7, 11], 9)); // output: true
    println!("{}", has_pair_sum(&[1, 2, 3], 10)); // output: false
}
```

## Group words by length

```rust
use std::collections::HashMap;

fn group_by_len(words: &[&str]) -> HashMap<usize, Vec<String>> {
    let mut groups: HashMap<usize, Vec<String>> = HashMap::new();

    for word in words {
        groups.entry(word.len()).or_default().push(word.to_string());
    }

    groups
}

fn main() {
    let groups = group_by_len(&["hi", "go", "rust"]);

    println!("{}", groups.get(&2).unwrap().len()); // output: 2
    println!("{}", groups.get(&4).unwrap().len()); // output: 1
}
```

## Queue simulation

```rust
use std::collections::VecDeque;

fn main() {
    let mut queue = VecDeque::from(["Ali", "Sara"]);

    queue.push_back("Mona");

    println!("{:?}", queue.pop_front()); // output: Some("Ali")
    println!("{:?}", queue.front()); // output: Some("Sara")
}
```

## Priority processing

```rust
use std::collections::BinaryHeap;

fn main() {
    let mut tasks = BinaryHeap::new();

    tasks.push(3);
    tasks.push(10);
    tasks.push(1);

    println!("{:?}", tasks.pop()); // output: Some(10)
}
```

---

# 22. Common Compiler Errors and Fixes

## Error: expected reference, found value

Wrong:

```rust
use std::collections::HashSet;

fn main() {
    let set: HashSet<i32> = [1, 2, 3].into_iter().collect();

    // println!("{}", set.contains(2)); // output: compiler error
}
```

Correct:

```rust
use std::collections::HashSet;

fn main() {
    let set: HashSet<i32> = [1, 2, 3].into_iter().collect();

    println!("{}", set.contains(&2)); // output: true
}
```

## Error: cannot mutate value from `.get()`

Wrong idea:

```rust
use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert("Ali", 90);

    // scores.get("Ali").unwrap() += 10; // output: compiler error
}
```

Correct:

```rust
use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert("Ali", 90);

    if let Some(score) = scores.get_mut("Ali") {
        *score += 10;
    }

    println!("{}", scores["Ali"]); // output: 100
}
```

## Error: HashMap debug order changes

Wrong expectation:

```rust
use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert("b", 2);
    map.insert("a", 1);

    println!("{}", map.len()); // output: 2
}
```

Do not rely on:

```rust
println!("{:?}", map); // output: order not guaranteed
```

because order is not guaranteed.

Use `BTreeMap` if order matters.

```rust
use std::collections::BTreeMap;

fn main() {
    let mut map = BTreeMap::new();
    map.insert("b", 2);
    map.insert("a", 1);

    println!("{}", map.keys().next().unwrap()); // output: a
}
```

---

# 23. Practice Exercises

## Exercise 1 — Count words

Write:

```rust
fn count_words(text: &str) -> HashMap<String, i32>
```

Rules:

1. Split by whitespace.
2. Count each word.
3. Return a `HashMap<String, i32>`.

Expected:

```rust
let counts = count_words("rust rust code");
println!("{}", counts["rust"]); // output: 2
println!("{}", counts["code"]); // output: 1
```

## Exercise 2 — Unique numbers

Write:

```rust
fn unique_count(numbers: &[i32]) -> usize
```

Expected:

```rust
println!("{}", unique_count(&[1, 2, 2, 3])); // output: 3
```

## Exercise 3 — Queue

Use `VecDeque`.

Expected behavior:

```rust
println!("{:?}", first_served); // output: Some("Ali")
println!("{:?}", second_served); // output: Some("Sara")
```

## Exercise 4 — Highest priority

Use `BinaryHeap`.

Expected:

```rust
println!("{:?}", heap.pop()); // output: Some(100)
```

## Exercise 5 — Group by first letter

Write:

```rust
fn group_by_first(names: &[&str]) -> HashMap<char, Vec<String>>
```

Expected:

```rust
let groups = group_by_first(&["Ali", "Abbas", "Sara"]);
println!("{}", groups.get(&'A').unwrap().len()); // output: 2
println!("{}", groups.get(&'S').unwrap().len()); // output: 1
```

---

# 24. Final Memory Rules

```text
HashMap = key -> value
HashSet = unique values
BTreeMap / BTreeSet = sorted map/set
VecDeque = queue from front/back
BinaryHeap = priority queue
Vec = simple stack
Box = recursive structures
```

Most useful exam patterns:

```text
counting     -> HashMap<T, i32>
unique check -> HashSet<T>
grouping     -> HashMap<K, Vec<V>>
queue        -> VecDeque<T>
priority     -> BinaryHeap<T>
tree/list    -> enum + Box<T>
```
