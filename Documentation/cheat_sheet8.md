# Rust Cheat Sheet 8 — Algorithms, Sorting, Searching, Games, Matrices, and Grids

This is **cheat_sheet8.md**.

This sheet focuses on exam-style algorithm problems in Rust.

It avoids repeating full method sections from earlier sheets. Instead, it shows how to **use Rust syntax, collections, loops, and functions to solve algorithmic exercises**.

---

# 1. Algorithm Thinking in Rust

Most Rust algorithm exercises follow this pattern:

```text
1. Understand input type
2. Choose the right data structure
3. Loop through values
4. Track state with variables
5. Return the expected type
6. Avoid panics unless the exercise allows them
```

## Common algorithm building blocks

| Goal | Common Rust tool |
|---|---|
| Visit every item | `for item in items` |
| Visit index and value | `.iter().enumerate()` |
| Build result vector | `let mut result = Vec::new(); result.push(x);` |
| Filter values | `if condition { ... }` or `.filter(...)` |
| Sort values | `.sort()`, `.sort_by(...)`, `.sort_by_key(...)` |
| Search manually | `for` loop |
| Safe index | `.get(index)` |
| Stack | `Vec<T>` with `.push()` and `.pop()` |
| Queue | `VecDeque<T>` with `.push_back()` and `.pop_front()` |
| Grid | `Vec<Vec<T>>` or `[[T; C]; R]` |
| Count frequencies | `HashMap<T, usize>` |

---

# 2. Basic Algorithm Function Shape

```rust
fn is_even(n: i32) -> bool {
    n % 2 == 0
}

fn main() {
    println!("{}", is_even(10)); // output: true
    println!("{}", is_even(7)); // output: false
}
```

## Common return choices

| Exercise type | Common return type |
|---|---|
| yes/no question | `bool` |
| number answer | `i32`, `u32`, `usize`, `f64` |
| transformed list | `Vec<T>` |
| maybe found | `Option<T>` or `Option<&T>` |
| validation with message | `Result<T, E>` or `String` |
| grid/matrix result | `Vec<Vec<T>>` |

---

# 3. Sorting

Sorting appears in many exercises: ranking scores, cleaning data, ordering names, finding median, and binary search preparation.

## Sorting method table

| Method / Pattern | Return type | Meaning |
|---|---:|---|
| `vec.sort()` | `()` | sort ascending |
| `vec.reverse()` | `()` | reverse current order |
| `vec.sort_by(|a, b| ...)` | `()` | custom comparison |
| `vec.sort_by_key(|x| ...)` | `()` | sort by extracted key |
| `vec.sort_unstable()` | `()` | faster sort, not stable |
| `slice.binary_search(&x)` | `Result<usize, usize>` | search sorted slice |

## Built-in ascending sort

```rust
fn main() {
    let mut numbers = vec![30, 10, 20];
    numbers.sort();

    println!("{:?}", numbers); // output: [10, 20, 30]
}
```

## Descending sort

```rust
fn main() {
    let mut numbers = vec![30, 10, 20];
    numbers.sort();
    numbers.reverse();

    println!("{:?}", numbers); // output: [30, 20, 10]
}
```

## Descending sort with `sort_by`

```rust
fn main() {
    let mut numbers = vec![30, 10, 20];

    numbers.sort_by(|a, b| b.cmp(a));

    println!("{:?}", numbers); // output: [30, 20, 10]
}
```

## Sort strings by length

```rust
fn main() {
    let mut words = vec!["rust", "a", "language"];

    words.sort_by_key(|word| word.len());

    println!("{:?}", words); // output: ["a", "rust", "language"]
}
```

---

# 4. Bubble Sort

Bubble sort repeatedly swaps neighboring values if they are in the wrong order.

## Bubble sort template

```rust
fn bubble_sort(arr: &mut [i32]) {
    let len = arr.len();

    for _ in 0..len {
        for i in 0..len.saturating_sub(1) {
            if arr[i] > arr[i + 1] {
                arr.swap(i, i + 1);
            }
        }
    }
}

fn main() {
    let mut numbers = [3, 2, 5, 1, 4];

    bubble_sort(&mut numbers);

    println!("{:?}", numbers); // output: [1, 2, 3, 4, 5]
}
```

## Methods used

| Method / Syntax | Return type | Meaning |
|---|---:|---|
| `.len()` | `usize` | number of items |
| `.saturating_sub(1)` | `usize` | subtract safely without underflow |
| `.swap(i, j)` | `()` | swap two items |
| `&mut [i32]` | mutable slice | function can sort arrays and vectors |

---

# 5. Insertion Sort

Insertion sort builds a sorted part from left to right.

```rust
fn insertion_sort(arr: &mut [i32]) {
    for i in 1..arr.len() {
        let key = arr[i];
        let mut j = i;

        while j > 0 && arr[j - 1] > key {
            arr[j] = arr[j - 1];
            j -= 1;
        }

        arr[j] = key;
    }
}

fn main() {
    let mut numbers = [5, 2, 4, 1, 3];

    insertion_sort(&mut numbers);

    println!("{:?}", numbers); // output: [1, 2, 3, 4, 5]
}
```

## Insertion sort idea

```text
Take one item.
Move bigger items to the right.
Put the item in its correct place.
```

---

# 6. Searching

Searching means finding whether something exists or where it is.

## Search method table

| Method / Pattern | Return type | Meaning |
|---|---:|---|
| `.contains(&value)` | `bool` | check if value exists |
| `.iter().find(...)` | `Option<&T>` | first matching item |
| `.iter().position(...)` | `Option<usize>` | index of first matching item |
| `.binary_search(&value)` | `Result<usize, usize>` | search sorted slice |
| manual `for` loop | any | most flexible search style |

## Linear search returning index

```rust
fn linear_search(numbers: &[i32], target: i32) -> Option<usize> {
    for (index, value) in numbers.iter().enumerate() {
        if *value == target {
            return Some(index);
        }
    }

    None
}

fn main() {
    let numbers = [10, 20, 30];

    println!("{:?}", linear_search(&numbers, 20)); // output: Some(1)
    println!("{:?}", linear_search(&numbers, 99)); // output: None
}
```

## Using `.position()`

```rust
fn main() {
    let numbers = [10, 20, 30];

    let index = numbers.iter().position(|n| *n == 20);

    println!("{:?}", index); // output: Some(1)
}
```

## Binary search

Binary search requires sorted data.

```rust
fn main() {
    let numbers = [10, 20, 30, 40];

    println!("{:?}", numbers.binary_search(&30)); // output: Ok(2)
    println!("{:?}", numbers.binary_search(&25)); // output: Err(2)
}
```

`Ok(index)` means found. `Err(insert_index)` means not found, and tells where it could be inserted.

---

# 7. Stack Patterns

A stack is last-in, first-out.

Rust usually uses `Vec<T>` as a stack.

## Stack method table

| Method | Return type | Meaning |
|---|---:|---|
| `.push(value)` | `()` | push to top |
| `.pop()` | `Option<T>` | remove from top |
| `.last()` | `Option<&T>` | peek top |
| `.is_empty()` | `bool` | check empty |

## Stack example

```rust
fn main() {
    let mut stack = Vec::new();

    stack.push(10);
    stack.push(20);

    println!("{:?}", stack.pop()); // output: Some(20)
    println!("{:?}", stack.pop()); // output: Some(10)
    println!("{:?}", stack.pop()); // output: None
}
```

## Bracket matching

```rust
fn brackets_are_balanced(text: &str) -> bool {
    let mut stack = Vec::new();

    for ch in text.chars() {
        match ch {
            '(' | '[' | '{' => stack.push(ch),
            ')' => {
                if stack.pop() != Some('(') {
                    return false;
                }
            }
            ']' => {
                if stack.pop() != Some('[') {
                    return false;
                }
            }
            '}' => {
                if stack.pop() != Some('{') {
                    return false;
                }
            }
            _ => {}
        }
    }

    stack.is_empty()
}

fn main() {
    println!("{}", brackets_are_balanced("([])")); // output: true
    println!("{}", brackets_are_balanced("([)]")); // output: false
}
```

---

# 8. Queue Patterns

A queue is first-in, first-out.

Rust uses `VecDeque<T>` for efficient queues.

```rust
use std::collections::VecDeque;

fn main() {
    let mut queue = VecDeque::new();

    queue.push_back("first");
    queue.push_back("second");

    println!("{:?}", queue.pop_front()); // output: Some("first")
    println!("{:?}", queue.pop_front()); // output: Some("second")
    println!("{:?}", queue.pop_front()); // output: None
}
```

## Queue method table

| Method | Return type | Meaning |
|---|---:|---|
| `VecDeque::new()` | `VecDeque<T>` | create queue |
| `.push_back(value)` | `()` | add to back |
| `.pop_front()` | `Option<T>` | remove from front |
| `.front()` | `Option<&T>` | see front |
| `.back()` | `Option<&T>` | see back |

---

# 9. Grid Basics

A grid is usually represented as:

```rust
Vec<Vec<T>>
```

or:

```rust
[[T; COLS]; ROWS]
```

## Grid access

```rust
fn main() {
    let grid = vec![
        vec![1, 2, 3],
        vec![4, 5, 6],
    ];

    println!("{}", grid[0][0]); // output: 1
    println!("{}", grid[1][2]); // output: 6
}
```

## Safe grid access

```rust
fn get_cell(grid: &[Vec<i32>], row: usize, col: usize) -> Option<i32> {
    Some(*grid.get(row)?.get(col)?)
}

fn main() {
    let grid = vec![vec![1, 2], vec![3, 4]];

    println!("{:?}", get_cell(&grid, 1, 1)); // output: Some(4)
    println!("{:?}", get_cell(&grid, 3, 0)); // output: None
}
```

## Grid traversal

```rust
fn main() {
    let grid = vec![vec![1, 2], vec![3, 4]];

    for row in &grid {
        for value in row {
            println!("{}", value); // output: 1 then 2 then 3 then 4
        }
    }
}
```

---

# 10. Neighbor Checking in Grids

Grid problems often need neighboring cells.

## 4-direction neighbors

```rust
fn neighbors4(row: i32, col: i32) -> Vec<(i32, i32)> {
    vec![
        (row - 1, col),
        (row + 1, col),
        (row, col - 1),
        (row, col + 1),
    ]
}

fn main() {
    println!("{:?}", neighbors4(1, 1)); // output: [(0, 1), (2, 1), (1, 0), (1, 2)]
}
```

## Valid neighbor check

```rust
fn is_inside(row: i32, col: i32, rows: i32, cols: i32) -> bool {
    row >= 0 && row < rows && col >= 0 && col < cols
}

fn main() {
    println!("{}", is_inside(1, 1, 3, 3)); // output: true
    println!("{}", is_inside(-1, 0, 3, 3)); // output: false
}
```

---

# 11. Breadth-First Search Pattern

BFS is useful for shortest paths, grids, and spreading problems.

```rust
use std::collections::{HashSet, VecDeque};

fn bfs_order(start: i32, graph: &[(i32, Vec<i32>)]) -> Vec<i32> {
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    let mut order = Vec::new();

    visited.insert(start);
    queue.push_back(start);

    while let Some(node) = queue.pop_front() {
        order.push(node);

        for (key, neighbors) in graph {
            if *key == node {
                for next in neighbors {
                    if visited.insert(*next) {
                        queue.push_back(*next);
                    }
                }
            }
        }
    }

    order
}

fn main() {
    let graph = [
        (1, vec![2, 3]),
        (2, vec![4]),
        (3, vec![]),
        (4, vec![]),
    ];

    println!("{:?}", bfs_order(1, &graph)); // output: [1, 2, 3, 4]
}
```

---

# 12. Matrices

A matrix is a 2D list of numbers.

## Matrix method / pattern table

| Pattern | Meaning |
|---|---|
| `matrix[row][col]` | access cell |
| `matrix.len()` | row count |
| `matrix[0].len()` | column count if non-empty |
| nested `for` loops | traverse matrix |
| `vec![vec![0; cols]; rows]` | create 2D vector |

## Create matrix

```rust
fn main() {
    let matrix = vec![vec![1, 2], vec![3, 4]];

    println!("{:?}", matrix); // output: [[1, 2], [3, 4]]
}
```

## Sum matrix

```rust
fn sum_matrix(matrix: &[Vec<i32>]) -> i32 {
    let mut total = 0;

    for row in matrix {
        for value in row {
            total += value;
        }
    }

    total
}

fn main() {
    let matrix = vec![vec![1, 2], vec![3, 4]];

    println!("{}", sum_matrix(&matrix)); // output: 10
}
```

---

# 13. Matrix Transposition

Transpose swaps rows and columns.

```rust
fn transpose(matrix: &[Vec<i32>]) -> Vec<Vec<i32>> {
    if matrix.is_empty() {
        return Vec::new();
    }

    let rows = matrix.len();
    let cols = matrix[0].len();
    let mut result = vec![vec![0; rows]; cols];

    for r in 0..rows {
        for c in 0..cols {
            result[c][r] = matrix[r][c];
        }
    }

    result
}

fn main() {
    let matrix = vec![vec![1, 2, 3], vec![4, 5, 6]];

    println!("{:?}", transpose(&matrix)); // output: [[1, 4], [2, 5], [3, 6]]
}
```

---

# 14. Matrix Multiplication

Matrix multiplication rule:

```text
result[row][col] = sum of left[row][k] * right[k][col]
```

```rust
fn multiply(a: &[Vec<i32>], b: &[Vec<i32>]) -> Vec<Vec<i32>> {
    let rows = a.len();
    let cols = b[0].len();
    let shared = b.len();

    let mut result = vec![vec![0; cols]; rows];

    for r in 0..rows {
        for c in 0..cols {
            for k in 0..shared {
                result[r][c] += a[r][k] * b[k][c];
            }
        }
    }

    result
}

fn main() {
    let a = vec![vec![1, 2], vec![3, 4]];
    let b = vec![vec![5, 6], vec![7, 8]];

    println!("{:?}", multiply(&a, &b)); // output: [[19, 22], [43, 50]]
}
```

---

# 15. Matrix Determinant Basics

For a 2x2 matrix:

```text
[a b]
[c d]

det = a*d - b*c
```

```rust
fn determinant_2x2(m: [[i32; 2]; 2]) -> i32 {
    m[0][0] * m[1][1] - m[0][1] * m[1][0]
}

fn main() {
    let matrix = [[1, 2], [3, 4]];

    println!("{}", determinant_2x2(matrix)); // output: -2
}
```

---

# 16. Tic-Tac-Toe Checking

A board can be represented as:

```rust
[[char; 3]; 3]
```

## Winner checker

```rust
fn winner(board: [[char; 3]; 3]) -> Option<char> {
    for i in 0..3 {
        if board[i][0] != ' ' && board[i][0] == board[i][1] && board[i][1] == board[i][2] {
            return Some(board[i][0]);
        }

        if board[0][i] != ' ' && board[0][i] == board[1][i] && board[1][i] == board[2][i] {
            return Some(board[0][i]);
        }
    }

    if board[0][0] != ' ' && board[0][0] == board[1][1] && board[1][1] == board[2][2] {
        return Some(board[0][0]);
    }

    if board[0][2] != ' ' && board[0][2] == board[1][1] && board[1][1] == board[2][0] {
        return Some(board[0][2]);
    }

    None
}

fn main() {
    let board = [
        ['X', 'X', 'X'],
        ['O', ' ', 'O'],
        [' ', ' ', ' '],
    ];

    println!("{:?}", winner(board)); // output: Some('X')
}
```

---

# 17. Minesweeper Neighbor Counting

Count mines around each empty cell.

```rust
fn count_mines(grid: &[Vec<char>], row: i32, col: i32) -> i32 {
    let directions = [
        (-1, -1), (-1, 0), (-1, 1),
        (0, -1),           (0, 1),
        (1, -1),  (1, 0),  (1, 1),
    ];

    let rows = grid.len() as i32;
    let cols = grid[0].len() as i32;
    let mut count = 0;

    for (dr, dc) in directions {
        let nr = row + dr;
        let nc = col + dc;

        if nr >= 0 && nr < rows && nc >= 0 && nc < cols {
            if grid[nr as usize][nc as usize] == '*' {
                count += 1;
            }
        }
    }

    count
}

fn main() {
    let grid = vec![
        vec!['*', '.', '.'],
        vec!['.', '.', '.'],
        vec!['.', '*', '.'],
    ];

    println!("{}", count_mines(&grid, 1, 1)); // output: 2
}
```

---

# 18. Chess Queen Movement

Two queens attack each other if they share:

```text
same row
same column
same diagonal
```

```rust
fn queens_attack(a: (i32, i32), b: (i32, i32)) -> bool {
    let same_row = a.0 == b.0;
    let same_col = a.1 == b.1;
    let same_diag = (a.0 - b.0).abs() == (a.1 - b.1).abs();

    same_row || same_col || same_diag
}

fn main() {
    println!("{}", queens_attack((0, 0), (3, 3))); // output: true
    println!("{}", queens_attack((0, 0), (1, 2))); // output: false
}
```

---

# 19. Edit Distance Dynamic Programming

Edit distance measures how many insert/delete/replace operations are needed to change one word into another.

```rust
fn edit_distance(a: &str, b: &str) -> usize {
    let a: Vec<char> = a.chars().collect();
    let b: Vec<char> = b.chars().collect();

    let mut dp = vec![vec![0; b.len() + 1]; a.len() + 1];

    for i in 0..=a.len() {
        dp[i][0] = i;
    }

    for j in 0..=b.len() {
        dp[0][j] = j;
    }

    for i in 1..=a.len() {
        for j in 1..=b.len() {
            if a[i - 1] == b[j - 1] {
                dp[i][j] = dp[i - 1][j - 1];
            } else {
                let insert = dp[i][j - 1];
                let delete = dp[i - 1][j];
                let replace = dp[i - 1][j - 1];

                dp[i][j] = 1 + insert.min(delete).min(replace);
            }
        }
    }

    dp[a.len()][b.len()]
}

fn main() {
    println!("{}", edit_distance("kitten", "sitting")); // output: 3
}
```

---

# 20. Bowling Score Pattern

A simple bowling scoring function needs to handle strikes and spares.

```rust
fn bowling_score(rolls: &[i32]) -> i32 {
    let mut score = 0;
    let mut index = 0;

    for _frame in 0..10 {
        if rolls[index] == 10 {
            score += 10 + rolls[index + 1] + rolls[index + 2];
            index += 1;
        } else if rolls[index] + rolls[index + 1] == 10 {
            score += 10 + rolls[index + 2];
            index += 2;
        } else {
            score += rolls[index] + rolls[index + 1];
            index += 2;
        }
    }

    score
}

fn main() {
    let rolls = vec![10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10, 10];

    println!("{}", bowling_score(&rolls)); // output: 300
}
```

---

# 21. Brainfuck Interpreter Pattern

Brainfuck exercises usually require:

```text
1. memory array
2. pointer
3. instruction pointer
4. loop jump handling for [ and ]
5. output for .
```

## Instruction table

| Symbol | Meaning |
|---|---|
| `>` | move pointer right |
| `<` | move pointer left |
| `+` | increment current byte |
| `-` | decrement current byte |
| `.` | print current byte as char |
| `[` | jump after matching `]` if current byte is 0 |
| `]` | jump back to matching `[` if current byte is not 0 |

## Small pattern for matching brackets

```rust
fn build_bracket_map(code: &[char]) -> Vec<usize> {
    let mut stack = Vec::new();
    let mut jumps = vec![0; code.len()];

    for (i, ch) in code.iter().enumerate() {
        if *ch == '[' {
            stack.push(i);
        } else if *ch == ']' {
            let start = stack.pop().expect("valid brainfuck code");
            jumps[start] = i;
            jumps[i] = start;
        }
    }

    jumps
}

fn main() {
    let code: Vec<char> = "[+]".chars().collect();
    let jumps = build_bracket_map(&code);

    println!("{:?}", jumps); // output: [2, 0, 0]
}
```

---

# 22. Pattern Drawing

Pattern exercises usually require nested loops and string building.

## Stars line

```rust
fn stars(n: usize) -> String {
    "*".repeat(n)
}

fn main() {
    println!("{}", stars(5)); // output: *****
}
```

## Pyramid

```rust
fn pyramid(height: usize) -> Vec<String> {
    let mut result = Vec::new();

    for level in 1..=height {
        let spaces = " ".repeat(height - level);
        let stars = "*".repeat(level * 2 - 1);
        result.push(format!("{}{}", spaces, stars));
    }

    result
}

fn main() {
    println!("{:?}", pyramid(3)); // output: ["  *", " ***", "*****"]
}
```

## Diamond

```rust
fn diamond(size: usize) -> Vec<String> {
    let mut result = Vec::new();

    for level in 1..=size {
        let spaces = " ".repeat(size - level);
        let stars = "*".repeat(level * 2 - 1);
        result.push(format!("{}{}", spaces, stars));
    }

    for level in (1..size).rev() {
        let spaces = " ".repeat(size - level);
        let stars = "*".repeat(level * 2 - 1);
        result.push(format!("{}{}", spaces, stars));
    }

    result
}

fn main() {
    println!("{:?}", diamond(2)); // output: [" *", "***", " *"]
}
```

---

# 23. Display Table Pattern

Table exercises usually require column widths.

```rust
fn display_table(rows: &[(&str, i32)]) -> Vec<String> {
    let name_width = rows.iter().map(|(name, _)| name.len()).max().unwrap_or(0);
    let mut result = Vec::new();

    for (name, value) in rows {
        result.push(format!("{:<width$} | {}", name, value, width = name_width));
    }

    result
}

fn main() {
    let rows = [("apple", 3), ("banana", 10)];

    println!("{:?}", display_table(&rows)); // output: ["apple  | 3", "banana | 10"]
}
```

---

# 24. Common Algorithm Mistakes

## Mistake 1: invalid index

Wrong:

```rust
fn main() {
    let numbers = vec![1, 2, 3];

    // println!("{}", numbers[10]); // output: panic if uncommented
    println!("{}", numbers.len()); // output: 3
}
```

Better:

```rust
fn main() {
    let numbers = vec![1, 2, 3];

    match numbers.get(10) {
        Some(n) => println!("{}", n), // output: value if index exists
        None => println!("not found"), // output: not found
    }
}
```

## Mistake 2: modifying a vector while iterating over it

Wrong idea:

```rust
fn main() {
    let mut numbers = vec![1, 2, 3];

    // for n in &numbers {
    //     numbers.push(*n); // output: compiler error if uncommented
    // }

    println!("{:?}", numbers); // output: [1, 2, 3]
}
```

Better: build a new vector.

```rust
fn main() {
    let numbers = vec![1, 2, 3];
    let mut result = Vec::new();

    for n in &numbers {
        result.push(*n);
        result.push(*n);
    }

    println!("{:?}", result); // output: [1, 1, 2, 2, 3, 3]
}
```

## Mistake 3: comparing references incorrectly

```rust
fn main() {
    let numbers = vec![1, 2, 3];

    let count = numbers.iter().filter(|n| **n > 1).count();

    println!("{}", count); // output: 2
}
```

Cleaner:

```rust
fn main() {
    let numbers = vec![1, 2, 3];

    let count = numbers.iter().copied().filter(|n| *n > 1).count();

    println!("{}", count); // output: 2
}
```

---

# 25. Exam Templates

## Template 1: build transformed vector

```rust
fn transform(numbers: &[i32]) -> Vec<i32> {
    let mut result = Vec::new();

    for n in numbers {
        result.push(*n * 2);
    }

    result
}

fn main() {
    println!("{:?}", transform(&[1, 2, 3])); // output: [2, 4, 6]
}
```

## Template 2: validate every character

```rust
fn only_digits(text: &str) -> bool {
    for ch in text.chars() {
        if !ch.is_ascii_digit() {
            return false;
        }
    }

    true
}

fn main() {
    println!("{}", only_digits("123")); // output: true
    println!("{}", only_digits("12a")); // output: false
}
```

## Template 3: count matches

```rust
fn count_even(numbers: &[i32]) -> usize {
    let mut count = 0;

    for n in numbers {
        if *n % 2 == 0 {
            count += 1;
        }
    }

    count
}

fn main() {
    println!("{}", count_even(&[1, 2, 3, 4])); // output: 2
}
```

## Template 4: grid scan

```rust
fn count_value(grid: &[Vec<i32>], target: i32) -> usize {
    let mut count = 0;

    for row in grid {
        for value in row {
            if *value == target {
                count += 1;
            }
        }
    }

    count
}

fn main() {
    let grid = vec![vec![1, 2], vec![2, 3]];

    println!("{}", count_value(&grid, 2)); // output: 2
}
```

---

# 26. Practice Exercises

## Exercise 1 — Bubble sort

Write:

```rust
fn bubble_sort(values: &mut [i32])
```

Expected:

```rust
fn main() {
    let mut values = [4, 2, 1, 3];
    bubble_sort(&mut values);

    println!("{:?}", values); // output: [1, 2, 3, 4]
}
```

## Exercise 2 — Linear search

Write:

```rust
fn search(values: &[i32], target: i32) -> Option<usize>
```

Expected:

```rust
fn main() {
    println!("{:?}", search(&[10, 20, 30], 30)); // output: Some(2)
    println!("{:?}", search(&[10, 20, 30], 99)); // output: None
}
```

## Exercise 3 — Brackets

Write:

```rust
fn balanced(text: &str) -> bool
```

Expected:

```rust
fn main() {
    println!("{}", balanced("([]{})")); // output: true
    println!("{}", balanced("([)]")); // output: false
}
```

## Exercise 4 — Matrix transpose

Write:

```rust
fn transpose(matrix: &[Vec<i32>]) -> Vec<Vec<i32>>
```

Expected:

```rust
fn main() {
    let matrix = vec![vec![1, 2], vec![3, 4], vec![5, 6]];

    println!("{:?}", transpose(&matrix)); // output: [[1, 3, 5], [2, 4, 6]]
}
```

## Exercise 5 — Queen attack

Write:

```rust
fn queens_attack(a: (i32, i32), b: (i32, i32)) -> bool
```

Expected:

```rust
fn main() {
    println!("{}", queens_attack((2, 2), (5, 5))); // output: true
    println!("{}", queens_attack((2, 2), (3, 4))); // output: false
}
```

## Exercise 6 — Minesweeper count

Write:

```rust
fn count_mines(grid: &[Vec<char>], row: i32, col: i32) -> i32
```

Expected:

```rust
fn main() {
    let grid = vec![vec!['*', '.'], vec!['.', '*']];

    println!("{}", count_mines(&grid, 0, 1)); // output: 2
}
```

---

# 27. Quick Algorithm Checklist

Before solving an algorithm exercise, ask:

```text
1. What is the input type?
2. What is the return type?
3. Do I need a Vec, HashMap, stack, queue, or grid?
4. Can I solve it with a simple loop first?
5. Do I need safe access with get?
6. Am I borrowing, mutating, or moving values?
7. Can the function panic, or should it return Option/Result?
8. Are indexes usize or i32?
9. Do I need to convert between i32 and usize?
10. Are all edge cases handled?
```

---

End of **cheat_sheet8.md**.
