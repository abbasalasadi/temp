# Rust Cheat Sheet 2 — Strings and Text Processing

This is **cheat_sheet2.md** in the exam-based Rust cheat-sheet series.

Focus: **all common string and text-processing syntax, methods, and patterns**.

This sheet avoids repeating full beginner syntax from `cheat_sheet1.md`. It focuses on practical text work for exercises such as capitalization, username checks, ciphers, prefixes, initials, anagrams, pangrams, Pig Latin, ROT ciphers, URL formatting, and sentence transformations.

## Style used in this sheet

- Method headings show common return types, for example `.trim() -> &str`.
- Every executable `println!` line includes `// output:` beside it whenever practical.
- Examples prefer simple exam-style functions.

---

# 1. Text Types: `&str`, `String`, `&String`

## Method index — text types and parameter choices

| Syntax / type | Common type | Changes original? | Main use |
|---|---:|---:|---|
| `"text"` | `&str` | No | string literal / borrowed text |
| `String` | `String` | Can be changed if `mut` | owned, growable text |
| `&String` | `&String` | No | borrowed `String`, usually less flexible than `&str` |
| `&str` parameter | `&str` | No | best parameter type when only reading text |
| `String` return | `String` | No | return new owned text |
| `&mut String` parameter | `&mut String` | Yes | modify original string |


## `&str` — borrowed string slice

Use `&str` when a function only needs to read text.

```rust
fn show(text: &str) {
    println!("{}", text); // output: Abbas
}

fn main() {
    show("Abbas");
}
```

## `String` — owned, growable text

Use `String` when you need to own, build, edit, or return text.

```rust
fn main() {
    let mut text = String::from("Rust");
    text.push('!');

    println!("{}", text); // output: Rust!
}
```

## `&String` — borrowed `String`

Usually avoid `&String` in function parameters. Prefer `&str` because it accepts both string literals and borrowed `String`s.

```rust
fn show_good(text: &str) {
    println!("{}", text); // output: Hasan
}

fn show_less_flexible(text: &String) {
    println!("{}", text); // output: Hasan
}

fn main() {
    let name = String::from("Hasan");

    show_good(&name);
    show_less_flexible(&name);
}
```

Rule:

```rust
fn read_text(text: &str) {}
fn return_text(text: &str) -> String { text.to_string() }
fn edit_original(text: &mut String) { text.push('!'); }
```

---

# 2. Creating Strings

## Method index — string creation

| Syntax / method | Common return type | Changes original? | Main use |
|---|---:|---:|---|
| `String::new()` | `String` | No | create empty owned string |
| `String::from(s)` | `String` | No | convert `&str` to owned `String` |
| `s.to_string()` | `String` | No | convert many values to `String` |
| `s.to_owned()` | `String` | No | commonly convert `&str` to `String` |
| `format!(...)` | `String` | No | build formatted text |


## String literal: `"text" -> &str`

```rust
fn main() {
    let text = "hello";
    println!("{}", text); // output: hello
}
```

## `String::new() -> String`

Creates an empty `String`.

```rust
fn main() {
    let text = String::new();
    println!("{}", text); // output: empty line
}
```

## `String::from(&str) -> String`

```rust
fn main() {
    let text = String::from("hello");
    println!("{}", text); // output: hello
}
```

## `.to_string() -> String`

```rust
fn main() {
    let text = "hello".to_string();
    println!("{}", text); // output: hello
}
```

## `.to_owned() -> String`

Commonly used to convert `&str` to owned `String`.

```rust
fn main() {
    let text = "hello".to_owned();
    println!("{}", text); // output: hello
}
```

---

# 3. Length and Emptiness

## Method index — length and emptiness

| Method | Common return type | Changes original? | Main use |
|---|---:|---:|---|
| `s.len()` | `usize` | No | byte length |
| `s.chars().count()` | `usize` | No | character count |
| `s.is_empty()` | `bool` | No | check empty text |


## `.len() -> usize`

Returns the length in **bytes**, not always characters.

```rust
fn main() {
    let text = "hello";
    println!("{}", text.len()); // output: 5
}
```

Unicode example:

```rust
fn main() {
    let text = "hé";
    println!("{}", text.len()); // output: 3
    println!("{}", text.chars().count()); // output: 2
}
```

## `.chars().count() -> usize`

Counts Unicode scalar values. This is often closer to “character count” than `.len()`.

```rust
fn main() {
    let text = "rust";
    println!("{}", text.chars().count()); // output: 4
}
```

## `.is_empty() -> bool`

```rust
fn main() {
    let text = "";
    println!("{}", text.is_empty()); // output: true
}
```

---

# 4. Editing a `String`

## Method index — editing a mutable `String`

These require a mutable `String`, for example `let mut text = String::from("Rust");`.

| Method | Common return type | Changes original? | Main use |
|---|---:|---:|---|
| `s.push(c)` | `()` | Yes | add one `char` to end |
| `s.push_str(t)` | `()` | Yes | add `&str` to end |
| `s.insert(i, c)` | `()` | Yes | insert one `char` at byte index |
| `s.insert_str(i, t)` | `()` | Yes | insert `&str` at byte index |
| `s.remove(i)` | `char` | Yes | remove and return char at byte index |
| `s.pop()` | `Option<char>` | Yes | remove and return last char if it exists |
| `s.clear()` | `()` | Yes | remove all text |


These methods change the original `String`, so the variable must be `mut`.

## `.push(char) -> ()`

Adds one character.

```rust
fn main() {
    let mut text = String::from("Hi");
    text.push('!');

    println!("{}", text); // output: Hi!
}
```

## `.push_str(&str) -> ()`

Adds text.

```rust
fn main() {
    let mut text = String::from("Hello");
    text.push_str(" Rust");

    println!("{}", text); // output: Hello Rust
}
```

## `.insert(index, char) -> ()`

Inserts one character at a byte index.

```rust
fn main() {
    let mut text = String::from("Rst");
    text.insert(1, 'u');

    println!("{}", text); // output: Rust
}
```

## `.insert_str(index, &str) -> ()`

Inserts text at a byte index.

```rust
fn main() {
    let mut text = String::from("Rust");
    text.insert_str(4, " language");

    println!("{}", text); // output: Rust language
}
```

## `.remove(index) -> char`

Removes and returns the character at a byte index.

```rust
fn main() {
    let mut text = String::from("Rust!");
    let removed = text.remove(4);

    println!("{}", removed); // output: !
    println!("{}", text); // output: Rust
}
```

## `.pop() -> Option<char>`

Removes the last character if the string is not empty.

```rust
fn main() {
    let mut text = String::from("Rust!");

    match text.pop() {
        Some(c) => println!("removed = {}", c), // output: removed = !
        None => println!("empty"), // output: empty if text is empty
    }

    println!("{}", text); // output: Rust
}
```

## `.clear() -> ()`

Removes all text.

```rust
fn main() {
    let mut text = String::from("Rust");
    text.clear();

    println!("{}", text); // output: empty line
}
```

---

# 5. Cleaning Whitespace

## Method index — trimming and cleaning edges

| Method | Common return type | Changes original? | Main use |
|---|---:|---:|---|
| `s.trim()` | `&str` | No | remove whitespace from both sides |
| `s.trim_start()` | `&str` | No | remove whitespace from start |
| `s.trim_end()` | `&str` | No | remove whitespace from end |
| `s.trim_matches(p)` | `&str` | No | remove matching pattern from both sides |
| `s.trim_start_matches(p)` | `&str` | No | remove pattern from start |
| `s.trim_end_matches(p)` | `&str` | No | remove pattern from end |


## `.trim() -> &str`

Removes whitespace from both sides.

```rust
fn main() {
    let text = "  hello  ";
    println!("{}", text.trim()); // output: hello
}
```

## `.trim_start() -> &str`

Removes whitespace from the start.

```rust
fn main() {
    let text = "  hello  ";
    println!("{}", text.trim_start()); // output: hello  
}
```

## `.trim_end() -> &str`

Removes whitespace from the end.

```rust
fn main() {
    let text = "  hello  ";
    println!("{}", text.trim_end()); // output:   hello
}
```

## `.trim_matches(pattern) -> &str`

Removes matching characters from both sides.

```rust
fn main() {
    let text = "---rust---";
    println!("{}", text.trim_matches('-')); // output: rust
}
```

## `.trim_start_matches(pattern) -> &str`

```rust
fn main() {
    let text = "---rust---";
    println!("{}", text.trim_start_matches('-')); // output: rust---
}
```

## `.trim_end_matches(pattern) -> &str`

```rust
fn main() {
    let text = "---rust---";
    println!("{}", text.trim_end_matches('-')); // output: ---rust
}
```

---

# 6. Case Conversion

## Method index — case conversion

| Method | Common return type | Changes original? | Main use |
|---|---:|---:|---|
| `s.to_uppercase()` | `String` | No | Unicode uppercase |
| `s.to_lowercase()` | `String` | No | Unicode lowercase |
| `s.to_ascii_uppercase()` | `String` | No | ASCII uppercase |
| `s.to_ascii_lowercase()` | `String` | No | ASCII lowercase |
| `c.to_ascii_uppercase()` | `char` | No | uppercase ASCII char |
| `c.to_ascii_lowercase()` | `char` | No | lowercase ASCII char |


## `.to_uppercase() -> String`

Unicode-aware uppercase.

```rust
fn main() {
    let text = "rust";
    println!("{}", text.to_uppercase()); // output: RUST
}
```

## `.to_lowercase() -> String`

Unicode-aware lowercase.

```rust
fn main() {
    let text = "RUST";
    println!("{}", text.to_lowercase()); // output: rust
}
```

## `.to_ascii_uppercase() -> String`

ASCII-only uppercase.

```rust
fn main() {
    let text = "rust123";
    println!("{}", text.to_ascii_uppercase()); // output: RUST123
}
```

## `.to_ascii_lowercase() -> String`

ASCII-only lowercase.

```rust
fn main() {
    let text = "RUST123";
    println!("{}", text.to_ascii_lowercase()); // output: rust123
}
```

## Character case methods

For `char`, ASCII methods are common in exercises.

```rust
fn main() {
    let c = 'a';
    println!("{}", c.to_ascii_uppercase()); // output: A
}
```

```rust
fn main() {
    let c = 'Z';
    println!("{}", c.to_ascii_lowercase()); // output: z
}
```

---

# 7. Checking Text

## Method index — text and character checks

| Method | Common return type | Changes original? | Main use |
|---|---:|---:|---|
| `s.contains(p)` | `bool` | No | check if text contains pattern |
| `s.starts_with(p)` | `bool` | No | check prefix |
| `s.ends_with(p)` | `bool` | No | check suffix |
| `s.is_ascii()` | `bool` | No | check if all bytes are ASCII |
| `c.is_alphabetic()` | `bool` | No | Unicode alphabetic check |
| `c.is_ascii_alphabetic()` | `bool` | No | ASCII letter check |
| `c.is_ascii_uppercase()` | `bool` | No | ASCII uppercase check |
| `c.is_ascii_lowercase()` | `bool` | No | ASCII lowercase check |
| `c.is_ascii_digit()` | `bool` | No | ASCII digit check |
| `c.is_numeric()` | `bool` | No | Unicode numeric check |
| `c.is_whitespace()` | `bool` | No | whitespace check |


## `.contains(pattern) -> bool`

```rust
fn main() {
    let text = "hello rust";
    println!("{}", text.contains("rust")); // output: true
}
```

## `.starts_with(pattern) -> bool`

```rust
fn main() {
    let text = "main.rs";
    println!("{}", text.starts_with("main")); // output: true
}
```

## `.ends_with(pattern) -> bool`

```rust
fn main() {
    let text = "main.rs";
    println!("{}", text.ends_with(".rs")); // output: true
}
```

## `.is_ascii() -> bool`

```rust
fn main() {
    let text = "Rust123";
    println!("{}", text.is_ascii()); // output: true
}
```

## `char` checking methods

```rust
fn main() {
    let c = 'A';

    println!("{}", c.is_alphabetic()); // output: true
    println!("{}", c.is_ascii_alphabetic()); // output: true
    println!("{}", c.is_ascii_uppercase()); // output: true
    println!("{}", c.is_ascii_lowercase()); // output: false
    println!("{}", c.is_ascii_digit()); // output: false
}
```

```rust
fn main() {
    let c = '7';

    println!("{}", c.is_numeric()); // output: true
    println!("{}", c.is_ascii_digit()); // output: true
    println!("{}", c.is_whitespace()); // output: false
}
```

---

# 8. Replacing Text

## Method index — replacing text

| Method | Common return type | Changes original? | Main use |
|---|---:|---:|---|
| `s.replace(a, b)` | `String` | No | replace all matches |
| `s.replacen(a, b, n)` | `String` | No | replace first `n` matches |


## `.replace(from, to) -> String`

Replaces all matches and returns a new `String`.

```rust
fn main() {
    let text = "hello rust rust";
    let fixed = text.replace("rust", "Rust");

    println!("{}", fixed); // output: hello Rust Rust
}
```

Important: `.replace()` does not change the original by itself.

```rust
fn main() {
    let text = "hello rust";
    text.replace("rust", "Rust");

    println!("{}", text); // output: hello rust
}
```

Correct assignment:

```rust
fn main() {
    let text = "hello rust";
    let text = text.replace("rust", "Rust");

    println!("{}", text); // output: hello Rust
}
```

## `.replacen(from, to, count) -> String`

Replaces only the first `count` matches.

```rust
fn main() {
    let text = "rust rust rust";
    let fixed = text.replacen("rust", "Rust", 2);

    println!("{}", fixed); // output: Rust Rust rust
}
```

---

# 9. Splitting Text

## Method index — splitting text

| Method | Common return type | Changes original? | Main use |
|---|---:|---:|---|
| `s.split(p)` | iterator over `&str` | No | split by pattern |
| `s.split_whitespace()` | iterator over `&str` | No | split words and ignore repeated whitespace |
| `s.split_once(p)` | `Option<(&str, &str)>` | No | split once from left |
| `s.rsplit_once(p)` | `Option<(&str, &str)>` | No | split once from right |
| `s.lines()` | iterator over `&str` | No | split by lines |


## `.split_whitespace() -> iterator over &str`

Splits by spaces, tabs, and newlines, ignoring repeated whitespace.

```rust
fn main() {
    let text = "hello   rust\nlanguage";
    let words: Vec<&str> = text.split_whitespace().collect();

    println!("{:?}", words); // output: ["hello", "rust", "language"]
}
```

## `.split(pattern) -> iterator over &str`

```rust
fn main() {
    let text = "red,green,blue";
    let colors: Vec<&str> = text.split(',').collect();

    println!("{:?}", colors); // output: ["red", "green", "blue"]
}
```

## `.split_once(pattern) -> Option<(&str, &str)>`

Splits only once.

```rust
fn main() {
    let text = "name=Abbas";

    match text.split_once('=') {
        Some((key, value)) => println!("{} -> {}", key, value), // output: name -> Abbas
        None => println!("no separator"), // output: no separator if '=' is missing
    }
}
```

## `.rsplit_once(pattern) -> Option<(&str, &str)>`

Splits once from the right side.

```rust
fn main() {
    let text = "folder/file.txt";

    match text.rsplit_once('/') {
        Some((folder, file)) => println!("{} | {}", folder, file), // output: folder | file.txt
        None => println!("no slash"), // output: no slash if '/' is missing
    }
}
```

## `.lines() -> iterator over &str`

```rust
fn main() {
    let text = "one\ntwo\nthree";
    let lines: Vec<&str> = text.lines().collect();

    println!("{:?}", lines); // output: ["one", "two", "three"]
}
```

---

# 10. Iterating Over Text

## Method index — character, byte, and index iteration

| Method chain | Common return type | Changes original? | Main use |
|---|---:|---:|---|
| `s.chars()` | iterator over `char` | No | process characters |
| `s.bytes()` | iterator over `u8` | No | process ASCII/bytes |
| `s.char_indices()` | iterator over `(usize, char)` | No | get byte index with char |
| `s.chars().enumerate()` | iterator over `(usize, char)` | No | get count index with char |
| `s.chars().next()` | `Option<char>` | No | get first char safely |
| `s.chars().rev()` | iterator over `char` | No | reverse character order |
| `s.chars().collect::<String>()` | `String` | No | build string from chars |
| `s.chars().collect::<Vec<char>>()` | `Vec<char>` | No | build vector of chars |


## `.chars() -> iterator over char`

```rust
fn main() {
    let text = "abc";
    let letters: Vec<char> = text.chars().collect();

    println!("{:?}", letters); // output: ['a', 'b', 'c']
}
```

## `.bytes() -> iterator over u8`

Useful for ASCII exercises and ciphers.

```rust
fn main() {
    let text = "ABC";
    let bytes: Vec<u8> = text.bytes().collect();

    println!("{:?}", bytes); // output: [65, 66, 67]
}
```

## `.char_indices() -> iterator over (usize, char)`

Gives byte index and character.

```rust
fn main() {
    let text = "abc";

    for (i, c) in text.char_indices() {
        println!("{}:{}", i, c); // output: 0:a then 1:b then 2:c
    }
}
```

## `.enumerate()` with `.chars()`

Gives count index, not byte index.

```rust
fn main() {
    let text = "abc";

    for (i, c) in text.chars().enumerate() {
        println!("{}:{}", i, c); // output: 0:a then 1:b then 2:c
    }
}
```

---

# 11. Searching Text

## Method index — searching text

| Method | Common return type | Changes original? | Main use |
|---|---:|---:|---|
| `s.find(p)` | `Option<usize>` | No | first byte index of pattern |
| `s.rfind(p)` | `Option<usize>` | No | last byte index of pattern |
| `s.matches(p)` | iterator over `&str` | No | all matching substrings |


## `.find(pattern) -> Option<usize>`

Returns the byte index of the first match.

```rust
fn main() {
    let text = "hello rust";
    println!("{:?}", text.find("rust")); // output: Some(6)
}
```

Handling `Option`:

```rust
fn main() {
    let text = "hello rust";

    match text.find("rust") {
        Some(index) => println!("found at {}", index), // output: found at 6
        None => println!("not found"), // output: not found if missing
    }
}
```

## `.rfind(pattern) -> Option<usize>`

Finds from the right.

```rust
fn main() {
    let text = "rust and rust";
    println!("{:?}", text.rfind("rust")); // output: Some(9)
}
```

## `.matches(pattern) -> iterator over &str`

Finds all matches.

```rust
fn main() {
    let text = "rust and rust";
    let found: Vec<&str> = text.matches("rust").collect();

    println!("{:?}", found); // output: ["rust", "rust"]
}
```

---

# 12. Prefixes and Suffixes

## Method index — prefix and suffix handling

| Method | Common return type | Changes original? | Main use |
|---|---:|---:|---|
| `s.strip_prefix(p)` | `Option<&str>` | No | remove prefix if present |
| `s.strip_suffix(p)` | `Option<&str>` | No | remove suffix if present |


## `.strip_prefix(prefix) -> Option<&str>`

```rust
fn main() {
    let text = "user_abbas";

    match text.strip_prefix("user_") {
        Some(name) => println!("{}", name), // output: abbas
        None => println!("missing prefix"), // output: missing prefix if prefix not found
    }
}
```

## `.strip_suffix(suffix) -> Option<&str>`

```rust
fn main() {
    let text = "main.rs";

    match text.strip_suffix(".rs") {
        Some(name) => println!("{}", name), // output: main
        None => println!("missing suffix"), // output: missing suffix if suffix not found
    }
}
```

---

# 13. Joining, Concatenation, and Formatting

## Method index — joining and formatting

| Method / syntax | Common return type | Changes original? | Main use |
|---|---:|---:|---|
| `String + &str` | `String` | Moves left side | concatenate owned `String` with borrowed text |
| `format!(...)` | `String` | No | build formatted text |
| `parts.concat()` | `String` | No | join parts without separator |
| `parts.join(sep)` | `String` | No | join parts with separator |
| `s.repeat(n)` | `String` | No | repeat text `n` times |


## `String + &str -> String`

```rust
fn main() {
    let a = String::from("Hello, ");
    let b = "Rust";
    let result = a + b;

    println!("{}", result); // output: Hello, Rust
}
```

Important: `a + b` moves `a`, so `a` cannot be used after.

## `format!() -> String`

Best general way to combine values into a string.

```rust
fn main() {
    let first = "Abbas";
    let last = "Hasan";
    let full = format!("{} {}", first, last);

    println!("{}", full); // output: Abbas Hasan
}
```

## `.concat() -> String`

Concatenates a list of string slices.

```rust
fn main() {
    let parts = ["Rust", " ", "is", " ", "fun"];
    let text = parts.concat();

    println!("{}", text); // output: Rust is fun
}
```

## `.join(separator) -> String`

Joins strings with a separator.

```rust
fn main() {
    let words = ["Rust", "is", "fun"];
    let text = words.join("-");

    println!("{}", text); // output: Rust-is-fun
}
```

## `.repeat(n) -> String`

Repeats text.

```rust
fn main() {
    let text = "ha".repeat(3);
    println!("{}", text); // output: hahaha
}
```

---

# 14. String Slicing Warning

String indexes are byte indexes, not character indexes.

This is okay for ASCII:

```rust
fn main() {
    let text = "Rust";
    let part = &text[0..2];

    println!("{}", part); // output: Ru
}
```

Be careful with Unicode:

```rust
fn main() {
    let text = "hé";
    println!("{}", text); // output: hé
    // let bad = &text[0..1]; // output: panic if uncommented, invalid char boundary
}
```

Safer character access:

```rust
fn main() {
    let text = "hé";
    let first = text.chars().next();

    println!("{:?}", first); // output: Some('h')
}
```

---

# 15. Converting Between Text and Numbers

## Method index — text and number conversion

| Method | Common return type | Changes original? | Main use |
|---|---:|---:|---|
| `s.parse::<T>()` | `Result<T, E>` | No | convert text into a type such as `i32` |
| `value.to_string()` | `String` | No | convert number/bool/char to text |
| `format!(...)` | `String` | No | format values into text |


## `.parse::<T>() -> Result<T, E>`

```rust
fn main() {
    let text = "42";
    let n = text.parse::<i32>().expect("not a number");

    println!("{}", n + 1); // output: 43
}
```

Safe version:

```rust
fn main() {
    let text = "abc";

    match text.parse::<i32>() {
        Ok(n) => println!("{}", n), // output: parsed number if valid
        Err(_) => println!("not a number"), // output: not a number
    }
}
```

## `.to_string() -> String`

```rust
fn main() {
    let n = 42;
    let text = n.to_string();

    println!("{}", text); // output: 42
}
```

---

# 16. Common Text-Processing Function Patterns

## Return cleaned text

```rust
fn clean_name(name: &str) -> String {
    name.trim().to_lowercase().replace(' ', "_")
}

fn main() {
    println!("{}", clean_name("  Abbas Hasan  ")); // output: abbas_hasan
}
```

## Mutate original string

```rust
fn add_exclamation(text: &mut String) {
    text.push('!');
}

fn main() {
    let mut text = String::from("Rust");
    add_exclamation(&mut text);

    println!("{}", text); // output: Rust!
}
```

## Count words

```rust
fn count_words(text: &str) -> usize {
    text.split_whitespace().count()
}

fn main() {
    println!("{}", count_words("hello rust language")); // output: 3
}
```

## Reverse characters

```rust
fn reverse_text(text: &str) -> String {
    text.chars().rev().collect()
}

fn main() {
    println!("{}", reverse_text("rust")); // output: tsur
}
```

## Keep only alphabetic characters

```rust
fn only_letters(text: &str) -> String {
    text.chars().filter(|c| c.is_alphabetic()).collect()
}

fn main() {
    println!("{}", only_letters("ab12-c!")); // output: abc
}
```

## Remove spaces

```rust
fn remove_spaces(text: &str) -> String {
    text.chars().filter(|c| !c.is_whitespace()).collect()
}

fn main() {
    println!("{}", remove_spaces("a b  c")); // output: abc
}
```

---

# 17. Capitalization Patterns

## Capitalize first character

```rust
fn capitalize_first(text: &str) -> String {
    let mut chars = text.chars();

    match chars.next() {
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
        None => String::new(),
    }
}

fn main() {
    println!("{}", capitalize_first("rust")); // output: Rust
}
```

## Capitalize each word

```rust
fn title_case(text: &str) -> String {
    text.split_whitespace()
        .map(capitalize_first)
        .collect::<Vec<String>>()
        .join(" ")
}

fn capitalize_first(text: &str) -> String {
    let mut chars = text.chars();

    match chars.next() {
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
        None => String::new(),
    }
}

fn main() {
    println!("{}", title_case("hello rust language")); // output: Hello Rust Language
}
```

---

# 18. Initials Pattern

```rust
fn initials(name: &str) -> String {
    name.split_whitespace()
        .filter_map(|word| word.chars().next())
        .map(|c| c.to_ascii_uppercase())
        .collect()
}

fn main() {
    println!("{}", initials("abbas hasan")); // output: AH
}
```

Useful methods:

| Method | Return type | Use |
|---|---|---|
| `.split_whitespace()` | iterator over `&str` | get words |
| `.chars().next()` | `Option<char>` | get first character |
| `.filter_map(...)` | iterator | ignore `None`, keep `Some` values |
| `.collect::<String>()` | `String` | build text from chars |

---

# 19. Anagram Pattern

```rust
fn is_anagram(a: &str, b: &str) -> bool {
    let mut a_chars: Vec<char> = a.to_lowercase().chars().filter(|c| c.is_alphabetic()).collect();
    let mut b_chars: Vec<char> = b.to_lowercase().chars().filter(|c| c.is_alphabetic()).collect();

    a_chars.sort();
    b_chars.sort();

    a_chars == b_chars
}

fn main() {
    println!("{}", is_anagram("listen", "silent")); // output: true
    println!("{}", is_anagram("rust", "trust")); // output: false
}
```

---

# 20. Pangram Pattern

A pangram contains every letter from `a` to `z`.

```rust
fn is_pangram(text: &str) -> bool {
    ('a'..='z').all(|letter| text.to_lowercase().contains(letter))
}

fn main() {
    println!("{}", is_pangram("the quick brown fox jumps over the lazy dog")); // output: true
}
```

More efficient version:

```rust
use std::collections::HashSet;

fn is_pangram(text: &str) -> bool {
    let letters: HashSet<char> = text
        .to_lowercase()
        .chars()
        .filter(|c| c.is_ascii_lowercase())
        .collect();

    letters.len() == 26
}

fn main() {
    println!("{}", is_pangram("the quick brown fox jumps over the lazy dog")); // output: true
}
```

---

# 21. ROT Cipher Pattern

Common for `rot`, `rot21`, and Caesar-style exercises.

```rust
fn rotate_char(c: char, shift: u8) -> char {
    if c.is_ascii_lowercase() {
        let base = b'a';
        let rotated = (c as u8 - base + shift) % 26 + base;
        rotated as char
    } else if c.is_ascii_uppercase() {
        let base = b'A';
        let rotated = (c as u8 - base + shift) % 26 + base;
        rotated as char
    } else {
        c
    }
}

fn rot(text: &str, shift: u8) -> String {
    text.chars().map(|c| rotate_char(c, shift)).collect()
}

fn main() {
    println!("{}", rot("abc XYZ!", 2)); // output: cde ZAB!
}
```

---

# 22. Pig Latin Pattern

Simple version: if word starts with vowel, add `ay`; otherwise move first char to end and add `ay`.

```rust
fn pig_latin_word(word: &str) -> String {
    let vowels = "aeiouAEIOU";
    let mut chars = word.chars();

    match chars.next() {
        Some(first) if vowels.contains(first) => format!("{}ay", word),
        Some(first) => format!("{}{}ay", chars.as_str(), first),
        None => String::new(),
    }
}

fn main() {
    println!("{}", pig_latin_word("apple")); // output: appleay
    println!("{}", pig_latin_word("rust")); // output: ustray
}
```

---

# 23. URL Formatting Pattern

```rust
fn to_url(text: &str) -> String {
    text.trim()
        .to_lowercase()
        .split_whitespace()
        .collect::<Vec<&str>>()
        .join("-")
}

fn main() {
    println!("{}", to_url("  Hello Rust Language  ")); // output: hello-rust-language
}
```

---

# 24. Profanity / Word Replacement Pattern

```rust
fn censor(text: &str, bad_word: &str) -> String {
    text.split_whitespace()
        .map(|word| if word == bad_word { "***" } else { word })
        .collect::<Vec<&str>>()
        .join(" ")
}

fn main() {
    println!("{}", censor("hello bad rust", "bad")); // output: hello *** rust
}
```

---

# 25. Prefix Deletion Pattern

```rust
fn delete_prefix(text: &str, prefix: &str) -> String {
    match text.strip_prefix(prefix) {
        Some(rest) => rest.to_string(),
        None => text.to_string(),
    }
}

fn main() {
    println!("{}", delete_prefix("user_abbas", "user_")); // output: abbas
    println!("{}", delete_prefix("abbas", "user_")); // output: abbas
}
```

Shorter version:

```rust
fn delete_prefix(text: &str, prefix: &str) -> String {
    text.strip_prefix(prefix).unwrap_or(text).to_string()
}

fn main() {
    println!("{}", delete_prefix("user_abbas", "user_")); // output: abbas
}
```

---

# 26. Username Check Pattern

Example rules:

1. Length must be 3 to 12 characters.
2. Only ASCII letters, digits, and `_` are allowed.
3. Must not start with a digit.

```rust
fn valid_username(name: &str) -> bool {
    let len = name.chars().count();

    if len < 3 || len > 12 {
        return false;
    }

    let mut chars = name.chars();

    match chars.next() {
        Some(first) if first.is_ascii_digit() => return false,
        Some(_) => {}
        None => return false,
    }

    name.chars().all(|c| c.is_ascii_alphanumeric() || c == '_')
}

fn main() {
    println!("{}", valid_username("abbas_01")); // output: true
    println!("{}", valid_username("1abbas")); // output: false
}
```

---

# 27. Common `Option` Results in String Work

## Method index — methods that return `Option` or `Result`

| Method | Return type | Why not direct value? | Common fix |
|---|---:|---|---|
| `s.pop()` | `Option<char>` | string may be empty | `match`, `if let`, `unwrap_or` |
| `s.chars().next()` | `Option<char>` | string may be empty | `match`, `if let`, `unwrap_or` |
| `s.find(p)` | `Option<usize>` | pattern may not exist | `match`, `if let`, `unwrap_or` |
| `s.rfind(p)` | `Option<usize>` | pattern may not exist | `match`, `if let`, `unwrap_or` |
| `s.strip_prefix(p)` | `Option<&str>` | prefix may not exist | `match`, `unwrap_or(s)` |
| `s.strip_suffix(p)` | `Option<&str>` | suffix may not exist | `match`, `unwrap_or(s)` |
| `s.split_once(p)` | `Option<(&str, &str)>` | separator may not exist | `match`, `if let` |
| `s.parse::<T>()` | `Result<T, E>` | parsing may fail | `match`, `if let Ok`, `expect` |


Many text methods return `Option` because the value may not exist.

| Method | Return type | Why Option? |
|---|---|---|
| `.chars().next()` | `Option<char>` | string may be empty |
| `.find(pattern)` | `Option<usize>` | pattern may not exist |
| `.rfind(pattern)` | `Option<usize>` | pattern may not exist |
| `.strip_prefix(prefix)` | `Option<&str>` | prefix may not exist |
| `.strip_suffix(suffix)` | `Option<&str>` | suffix may not exist |
| `.split_once(pattern)` | `Option<(&str, &str)>` | separator may not exist |
| `.pop()` | `Option<char>` | string may be empty |

## Resolve with `match`

```rust
fn main() {
    let text = "rust";

    match text.chars().next() {
        Some(c) => println!("{}", c), // output: r
        None => println!("empty"), // output: empty if text is empty
    }
}
```

## Resolve with `if let`

```rust
fn main() {
    let text = "main.rs";

    if let Some(name) = text.strip_suffix(".rs") {
        println!("{}", name); // output: main
    }
}
```

## Resolve with `.unwrap_or(default)`

```rust
fn main() {
    let text = "main.rs";
    let name = text.strip_suffix(".txt").unwrap_or(text);

    println!("{}", name); // output: main.rs
}
```

## Resolve with `.expect(message)`

Use only when missing value should stop the program.

```rust
fn main() {
    let text = "main.rs";
    let name = text.strip_suffix(".rs").expect("expected .rs file");

    println!("{}", name); // output: main
}
```

---

# 28. Common Mistakes and Fixes

## Mistake 1: expecting `.push()` to return a `String`

Wrong:

```rust
fn main() {
    let mut text = String::from("Hi");
    let result = text.push('!');

    println!("{:?}", result); // output: ()
}
```

Correct:

```rust
fn main() {
    let mut text = String::from("Hi");
    text.push('!');

    println!("{}", text); // output: Hi!
}
```

## Mistake 2: forgetting that `.replace()` returns a new `String`

Wrong:

```rust
fn main() {
    let text = String::from("hello rust");
    text.replace("rust", "Rust");

    println!("{}", text); // output: hello rust
}
```

Correct:

```rust
fn main() {
    let text = String::from("hello rust");
    let text = text.replace("rust", "Rust");

    println!("{}", text); // output: hello Rust
}
```

## Mistake 3: using `&String` when `&str` is better

Less flexible:

```rust
fn show(text: &String) {
    println!("{}", text); // output: text passed as String
}
```

Better:

```rust
fn show(text: &str) {
    println!("{}", text); // output: text passed as &str or String reference
}
```

## Mistake 4: indexing strings by character position

Wrong idea:

```rust
fn main() {
    let text = "rust";
    // println!("{}", text[0]); // output: compiler error if uncommented
    println!("{}", text); // output: rust
}
```

Correct:

```rust
fn main() {
    let text = "rust";
    let first = text.chars().next();

    println!("{:?}", first); // output: Some('r')
}
```

---

# 29. Quick Return-Type Table

| Method / Syntax | Common return type | Notes |
|---|---|---|
| `"text"` | `&str` | string literal |
| `String::new()` | `String` | empty owned text |
| `String::from(s)` | `String` | owned text |
| `s.to_string()` | `String` | owned text |
| `s.to_owned()` | `String` | owned text |
| `s.len()` | `usize` | bytes |
| `s.chars().count()` | `usize` | character count |
| `s.is_empty()` | `bool` | empty check |
| `s.push(c)` | `()` | mutates original `String` |
| `s.push_str(t)` | `()` | mutates original `String` |
| `s.insert(i, c)` | `()` | mutates original `String` |
| `s.insert_str(i, t)` | `()` | mutates original `String` |
| `s.remove(i)` | `char` | mutates original `String` |
| `s.pop()` | `Option<char>` | mutates original `String` |
| `s.clear()` | `()` | mutates original `String` |
| `s.trim()` | `&str` | borrowed cleaned slice |
| `s.trim_matches(p)` | `&str` | borrowed cleaned slice |
| `s.to_uppercase()` | `String` | new text |
| `s.to_lowercase()` | `String` | new text |
| `s.contains(p)` | `bool` | check |
| `s.starts_with(p)` | `bool` | check |
| `s.ends_with(p)` | `bool` | check |
| `s.replace(a, b)` | `String` | new text |
| `s.replacen(a, b, n)` | `String` | new text |
| `s.split(p)` | iterator over `&str` | often collect into `Vec<&str>` |
| `s.split_whitespace()` | iterator over `&str` | words |
| `s.lines()` | iterator over `&str` | lines |
| `s.chars()` | iterator over `char` | characters |
| `s.bytes()` | iterator over `u8` | bytes |
| `s.char_indices()` | iterator over `(usize, char)` | byte index + char |
| `s.find(p)` | `Option<usize>` | first match index |
| `s.rfind(p)` | `Option<usize>` | last match index |
| `s.matches(p)` | iterator over `&str` | all matching substrings |
| `s.strip_prefix(p)` | `Option<&str>` | remove prefix if present |
| `s.strip_suffix(p)` | `Option<&str>` | remove suffix if present |
| `s.split_once(p)` | `Option<(&str, &str)>` | split once |
| `parts.concat()` | `String` | join without separator |
| `parts.join(sep)` | `String` | join with separator |
| `s.repeat(n)` | `String` | repeat text |
| `s.parse::<T>()` | `Result<T, E>` | text to type |

---

# 30. Practice Exercises

## Exercise 1 — Clean name

Write:

```rust
fn clean_name(name: &str) -> String
```

Rules:

1. Trim spaces.
2. Convert to lowercase.
3. Replace spaces with `_`.

Expected:

```rust
fn main() {
    println!("{}", clean_name("  Abbas Hasan  ")); // output: abbas_hasan
}
```

## Exercise 2 — Username validation

Write:

```rust
fn valid_username(name: &str) -> bool
```

Rules:

1. Length must be 3 to 12 characters.
2. Only ASCII letters, digits, and `_` are allowed.
3. Must not start with a digit.

Expected:

```rust
fn main() {
    println!("{}", valid_username("abbas_01")); // output: true
    println!("{}", valid_username("1abbas")); // output: false
}
```

## Exercise 3 — Initials

Write:

```rust
fn initials(name: &str) -> String
```

Expected:

```rust
fn main() {
    println!("{}", initials("abbas hasan")); // output: AH
}
```

## Exercise 4 — Anagram

Write:

```rust
fn is_anagram(a: &str, b: &str) -> bool
```

Expected:

```rust
fn main() {
    println!("{}", is_anagram("listen", "silent")); // output: true
    println!("{}", is_anagram("rust", "trust")); // output: false
}
```

## Exercise 5 — ROT cipher

Write:

```rust
fn rot(text: &str, shift: u8) -> String
```

Expected:

```rust
fn main() {
    println!("{}", rot("abc XYZ!", 2)); // output: cde ZAB!
}
```

## Exercise 6 — Delete prefix

Write:

```rust
fn delete_prefix(text: &str, prefix: &str) -> String
```

Expected:

```rust
fn main() {
    println!("{}", delete_prefix("user_abbas", "user_")); // output: abbas
    println!("{}", delete_prefix("abbas", "user_")); // output: abbas
}
```
