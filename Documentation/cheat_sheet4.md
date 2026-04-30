# Rust Cheat Sheet 4 — Numeric Algorithms and Math Problems

This is **cheat_sheet4.md**.

Focus: numeric manipulation, math algorithms, and exam-style number problems.

This sheet avoids repeating full basic syntax from earlier sheets. It focuses on **how to solve numeric exercises** in Rust.

---

## Style used in this sheet

- Every executable `println!` line includes `// output:` on the same line.
- Method tables are placed inside their related subject sections.
- Method headings show common return types.
- Examples prefer clear exam-style functions.

---

# 1. Numeric Type Choice

Choosing the right numeric type avoids many Rust errors.

## Numeric type index

| Type | Use when | Notes |
|---|---|---|
| `i32` | normal signed integers | good default for exercises |
| `i64` | larger signed integers | safer for bigger calculations |
| `u32` | non-negative integers | used by `.pow()` exponent |
| `usize` | indexes, lengths, counts | returned by `.len()` |
| `f64` | decimal calculations | common default for floats |
| `bool` | true/false result | useful for checks |

## Examples

```rust
fn main() {
    let score: i32 = -10;
    let count: usize = 3;
    let price: f64 = 12.5;

    println!("{}", score); // output: -10
    println!("{}", count); // output: 3
    println!("{}", price); // output: 12.5
}
```

## Type ambiguity with numeric methods

Some numeric methods require Rust to know the exact type.

```rust
fn main() {
    let x: i32 = -5;
    println!("{}", x.abs()); // output: 5
}
```

If Rust says **ambiguous numeric type**, add a type annotation.

```rust
fn main() {
    let numbers: Vec<i32> = vec![-1, 2, -3];
    let fixed: Vec<i32> = numbers.iter().copied().map(|n| n.abs()).collect();

    println!("{:?}", fixed); // output: [1, 2, 3]
}
```

---

# 2. Numeric Operators

## Operator index

| Operation | Syntax | Common return type | Notes |
|---|---|---|---|
| add | `a + b` | same numeric type | both sides usually same type |
| subtract | `a - b` | same numeric type | can go negative only on signed types |
| multiply | `a * b` | same numeric type | watch overflow |
| divide | `a / b` | same numeric type | integer division removes decimals |
| remainder | `a % b` | same numeric type | useful for even/odd and digits |

## Example

```rust
fn main() {
    let a = 10;
    let b = 3;

    println!("{}", a + b); // output: 13
    println!("{}", a - b); // output: 7
    println!("{}", a * b); // output: 30
    println!("{}", a / b); // output: 3
    println!("{}", a % b); // output: 1
}
```

## Integer division vs decimal division

```rust
fn main() {
    let a = 10;
    let b = 3;

    println!("{}", a / b); // output: 3
    println!("{}", a as f64 / b as f64); // output: 3.3333333333333335
}
```

---

# 3. Common Numeric Methods

## Method index

| Method | Common return type | Meaning |
|---|---|---|
| `.abs()` | signed number | positive distance from zero |
| `.pow(exp: u32)` | number | integer power |
| `.powi(exp: i32)` | float | float raised to integer power |
| `.powf(exp: f64)` | float | float raised to float power |
| `.sqrt()` | float | square root |
| `.min(x)` | same type | smaller value |
| `.max(x)` | same type | bigger value |
| `.clamp(min, max)` | same type | force inside range |
| `.round()` | float | nearest whole number |
| `.floor()` | float | round down |
| `.ceil()` | float | round up |
| `.trunc()` | float | remove decimal part |
| `.fract()` | float | decimal part |
| `.is_positive()` | `bool` | greater than zero |
| `.is_negative()` | `bool` | less than zero |

## Signed integer methods

```rust
fn main() {
    let x: i32 = -12;

    println!("{}", x.abs()); // output: 12
    println!("{}", x.is_negative()); // output: true
    println!("{}", x.is_positive()); // output: false
}
```

## Power methods

```rust
fn main() {
    let base: i32 = 2;
    let exp: u32 = 3;

    println!("{}", base.pow(exp)); // output: 8
}
```

```rust
fn main() {
    let x = 2.0_f64;

    println!("{}", x.powi(3)); // output: 8
    println!("{}", x.powf(0.5)); // output: 1.4142135623730951
}
```

## Min, max, and clamp

```rust
fn main() {
    let score = 120;

    println!("{}", score.min(100)); // output: 100
    println!("{}", score.max(0)); // output: 120
    println!("{}", score.clamp(0, 100)); // output: 100
}
```

## Float rounding methods

```rust
fn main() {
    let x = 3.75_f64;

    println!("{}", x.round()); // output: 4
    println!("{}", x.floor()); // output: 3
    println!("{}", x.ceil()); // output: 4
    println!("{}", x.trunc()); // output: 3
    println!("{}", x.fract()); // output: 0.75
}
```

## Decimal-place helper

Rust `.round()`, `.floor()`, and `.ceil()` do not take decimal places directly.

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

# 4. Type Conversion in Numeric Problems

## Conversion index

| Syntax / method | Common return type | Use |
|---|---|---|
| `x as f64` | `f64` | simple numeric cast |
| `x as i32` | `i32` | simple numeric cast |
| `x.try_into()` | `Result<T, E>` | safer integer conversion |
| `"42".parse::<i32>()` | `Result<i32, E>` | text to number |
| `x.to_string()` | `String` | number to text |
| `format!("{}", x)` | `String` | formatted number text |

## Convert for decimal division

```rust
fn main() {
    let total = 7;
    let count = 2;

    let average = total as f64 / count as f64;

    println!("{}", average); // output: 3.5
}
```

## Convert exponent for `.pow()`

```rust
fn main() {
    let base: i32 = 3;
    let exp: i32 = 4;

    println!("{}", base.pow(exp as u32)); // output: 81
}
```

## Safe conversion with `try_into`

```rust
use std::convert::TryInto;

fn main() {
    let x: i32 = 5;
    let y: u32 = x.try_into().expect("must be non-negative");

    println!("{}", y); // output: 5
}
```

---

# 5. Even, Odd, Divisibility, and Remainders

## Remainder pattern index

| Pattern | Meaning | Example |
|---|---|---|
| `n % 2 == 0` | even number | `4` |
| `n % 2 != 0` | odd number | `5` |
| `n % d == 0` | divisible by `d` | `12 % 3 == 0` |
| `n % 10` | last digit | `123 % 10 == 3` |
| `n / 10` | remove last digit | `123 / 10 == 12` |

## Even / odd

```rust
fn is_even(n: i32) -> bool {
    n % 2 == 0
}

fn main() {
    println!("{}", is_even(8)); // output: true
    println!("{}", is_even(7)); // output: false
}
```

## Divisibility

```rust
fn is_divisible_by(n: i32, d: i32) -> bool {
    d != 0 && n % d == 0
}

fn main() {
    println!("{}", is_divisible_by(12, 3)); // output: true
    println!("{}", is_divisible_by(12, 5)); // output: false
}
```

## Digit extraction

```rust
fn main() {
    let mut n = 1234;

    println!("{}", n % 10); // output: 4
    n /= 10;
    println!("{}", n); // output: 123
}
```

---

# 6. Min, Max, Sum, Product, and Average

## Iterator calculation index

| Method | Common return type | Notes |
|---|---|---|
| `.sum()` | numeric type | often needs type annotation |
| `.product()` | numeric type | multiplies all items |
| `.min()` | `Option<T>` | empty iterator returns `None` |
| `.max()` | `Option<T>` | empty iterator returns `None` |
| `.count()` | `usize` | counts items |

## Sum and product

```rust
fn main() {
    let numbers = [2, 3, 4];

    let sum: i32 = numbers.iter().copied().sum();
    let product: i32 = numbers.iter().copied().product();

    println!("{}", sum); // output: 9
    println!("{}", product); // output: 24
}
```

## Minimum and maximum

```rust
fn main() {
    let numbers = [10, 4, 30, 2];

    let min = numbers.iter().copied().min();
    let max = numbers.iter().copied().max();

    println!("{:?}", min); // output: Some(2)
    println!("{:?}", max); // output: Some(30)
}
```

## Resolve `Option` from `.min()` and `.max()`

```rust
fn smallest(numbers: &[i32]) -> String {
    match numbers.iter().copied().min() {
        Some(n) => format!("min = {}", n),
        None => String::from("empty"),
    }
}

fn main() {
    println!("{}", smallest(&[5, 2, 9])); // output: min = 2
    println!("{}", smallest(&[])); // output: empty
}
```

## Average

```rust
fn average(numbers: &[i32]) -> Option<f64> {
    if numbers.is_empty() {
        return None;
    }

    let total: i32 = numbers.iter().copied().sum();
    Some(total as f64 / numbers.len() as f64)
}

fn main() {
    println!("{:?}", average(&[10, 20, 30])); // output: Some(20.0)
    println!("{:?}", average(&[])); // output: None
}
```

---

# 7. Factorial

Factorial means:

```text
5! = 5 * 4 * 3 * 2 * 1 = 120
```

## Factorial pattern index

| Pattern | Use |
|---|---|
| `for n in 1..=x` | multiply from 1 to x |
| `.product()` | compact factorial |
| `u64` | safer than `u32` for larger factorials |
| `checked_mul()` | safer overflow detection |

## Loop version

```rust
fn factorial(n: u64) -> u64 {
    let mut result = 1;

    for i in 1..=n {
        result *= i;
    }

    result
}

fn main() {
    println!("{}", factorial(5)); // output: 120
}
```

## Iterator version

```rust
fn factorial(n: u64) -> u64 {
    (1..=n).product()
}

fn main() {
    println!("{}", factorial(6)); // output: 720
}
```

## Checked factorial

```rust
fn checked_factorial(n: u64) -> Option<u64> {
    let mut result = 1_u64;

    for i in 1..=n {
        result = result.checked_mul(i)?;
    }

    Some(result)
}

fn main() {
    println!("{:?}", checked_factorial(5)); // output: Some(120)
}
```

---

# 8. Fibonacci and Lucas Numbers

Fibonacci pattern:

```text
0, 1, 1, 2, 3, 5, 8, 13
```

Lucas pattern:

```text
2, 1, 3, 4, 7, 11, 18
```

## Sequence method index

| Pattern | Meaning |
|---|---|
| `prev` and `curr` | track last two values |
| `for _ in 0..n` | repeat n times |
| tuple assignment | update two values cleanly |

## Fibonacci

```rust
fn fibonacci(n: usize) -> u64 {
    let mut prev = 0;
    let mut curr = 1;

    for _ in 0..n {
        let next = prev + curr;
        prev = curr;
        curr = next;
    }

    prev
}

fn main() {
    println!("{}", fibonacci(0)); // output: 0
    println!("{}", fibonacci(1)); // output: 1
    println!("{}", fibonacci(7)); // output: 13
}
```

## Lucas

```rust
fn lucas(n: usize) -> u64 {
    let mut prev = 2;
    let mut curr = 1;

    for _ in 0..n {
        let next = prev + curr;
        prev = curr;
        curr = next;
    }

    prev
}

fn main() {
    println!("{}", lucas(0)); // output: 2
    println!("{}", lucas(1)); // output: 1
    println!("{}", lucas(5)); // output: 11
}
```

---

# 9. Prime Numbers

A prime number is greater than 1 and divisible only by 1 and itself.

## Prime pattern index

| Pattern | Use |
|---|---|
| `n < 2` | not prime |
| `n == 2` | prime |
| `n % d == 0` | found divisor |
| `d * d <= n` | only check up to square root |
| `while` loop | efficient divisor checking |

## Prime checker

```rust
fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }

    let mut d = 2;

    while d * d <= n {
        if n % d == 0 {
            return false;
        }
        d += 1;
    }

    true
}

fn main() {
    println!("{}", is_prime(1)); // output: false
    println!("{}", is_prime(2)); // output: true
    println!("{}", is_prime(17)); // output: true
    println!("{}", is_prime(21)); // output: false
}
```

## Next prime

```rust
fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }

    let mut d = 2;
    while d * d <= n {
        if n % d == 0 {
            return false;
        }
        d += 1;
    }

    true
}

fn next_prime(mut n: u64) -> u64 {
    n += 1;

    while !is_prime(n) {
        n += 1;
    }

    n
}

fn main() {
    println!("{}", next_prime(17)); // output: 19
}
```

## Previous prime

```rust
fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }

    let mut d = 2;
    while d * d <= n {
        if n % d == 0 {
            return false;
        }
        d += 1;
    }

    true
}

fn previous_prime(mut n: u64) -> Option<u64> {
    if n <= 2 {
        return None;
    }

    n -= 1;

    while n >= 2 {
        if is_prime(n) {
            return Some(n);
        }
        n -= 1;
    }

    None
}

fn main() {
    println!("{:?}", previous_prime(20)); // output: Some(19)
    println!("{:?}", previous_prime(2)); // output: None
}
```

---

# 10. Armstrong Numbers

An Armstrong number equals the sum of its digits raised to the number of digits.

Example:

```text
153 = 1^3 + 5^3 + 3^3
```

## Armstrong pattern index

| Pattern | Use |
|---|---|
| `.to_string()` | count digits |
| `.chars()` | process digits |
| `.to_digit(10)` | convert char digit to number |
| `.pow(exp)` | raise each digit |
| `.sum()` | add powered digits |

```rust
fn is_armstrong(n: u32) -> bool {
    let text = n.to_string();
    let power = text.len() as u32;

    let total: u32 = text
        .chars()
        .map(|c| c.to_digit(10).unwrap().pow(power))
        .sum();

    total == n
}

fn main() {
    println!("{}", is_armstrong(153)); // output: true
    println!("{}", is_armstrong(154)); // output: false
}
```

---

# 11. Digit Algorithms

Many exam numeric exercises use digit processing.

## Digit processing index

| Method / pattern | Common return type | Use |
|---|---|---|
| `n % 10` | integer | last digit |
| `n / 10` | integer | remove last digit |
| `.to_string()` | `String` | convert number to text |
| `.chars()` | iterator | process each digit as char |
| `.to_digit(10)` | `Option<u32>` | char digit to number |
| `.rev()` | iterator | reverse digit order |

## Sum digits with arithmetic

```rust
fn sum_digits(mut n: u64) -> u64 {
    let mut total = 0;

    while n > 0 {
        total += n % 10;
        n /= 10;
    }

    total
}

fn main() {
    println!("{}", sum_digits(1234)); // output: 10
}
```

## Sum digits with strings

```rust
fn sum_digits(n: u64) -> u32 {
    n.to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .sum()
}

fn main() {
    println!("{}", sum_digits(1234)); // output: 10
}
```

## Reverse digits

```rust
fn reverse_digits(mut n: u64) -> u64 {
    let mut result = 0;

    while n > 0 {
        result = result * 10 + n % 10;
        n /= 10;
    }

    result
}

fn main() {
    println!("{}", reverse_digits(1234)); // output: 4321
}
```

---

# 12. Luhn Algorithm

The Luhn algorithm is used to validate identification numbers.

## Luhn pattern index

| Step | Rust pattern |
|---|---|
| Remove spaces | `.filter(|c| !c.is_whitespace())` |
| Check digits | `.to_digit(10)` |
| Reverse digits | `.rev()` |
| Double every second digit | `.enumerate()` |
| If doubled > 9 subtract 9 | `if doubled > 9 { doubled - 9 }` |
| Valid if sum % 10 == 0 | `sum % 10 == 0` |

```rust
fn is_luhn_valid(code: &str) -> bool {
    let digits: Option<Vec<u32>> = code
        .chars()
        .filter(|c| !c.is_whitespace())
        .map(|c| c.to_digit(10))
        .collect();

    let Some(digits) = digits else {
        return false;
    };

    if digits.len() <= 1 {
        return false;
    }

    let sum: u32 = digits
        .iter()
        .rev()
        .enumerate()
        .map(|(i, &digit)| {
            if i % 2 == 1 {
                let doubled = digit * 2;
                if doubled > 9 { doubled - 9 } else { doubled }
            } else {
                digit
            }
        })
        .sum();

    sum % 10 == 0
}

fn main() {
    println!("{}", is_luhn_valid("4539 3195 0343 6467")); // output: true
    println!("{}", is_luhn_valid("8273 1232 7352 0569")); // output: false
}
```

---

# 13. Roman Numerals

Roman numerals are often solved with repeated subtraction.

## Roman numeral index

| Value | Symbol |
|---:|---|
| 1000 | `M` |
| 900 | `CM` |
| 500 | `D` |
| 400 | `CD` |
| 100 | `C` |
| 90 | `XC` |
| 50 | `L` |
| 40 | `XL` |
| 10 | `X` |
| 9 | `IX` |
| 5 | `V` |
| 4 | `IV` |
| 1 | `I` |

```rust
fn to_roman(mut n: u32) -> String {
    let values = [
        (1000, "M"),
        (900, "CM"),
        (500, "D"),
        (400, "CD"),
        (100, "C"),
        (90, "XC"),
        (50, "L"),
        (40, "XL"),
        (10, "X"),
        (9, "IX"),
        (5, "V"),
        (4, "IV"),
        (1, "I"),
    ];

    let mut result = String::new();

    for &(value, symbol) in &values {
        while n >= value {
            result.push_str(symbol);
            n -= value;
        }
    }

    result
}

fn main() {
    println!("{}", to_roman(9)); // output: IX
    println!("{}", to_roman(58)); // output: LVIII
    println!("{}", to_roman(1994)); // output: MCMXCIV
}
```

---

# 14. Reverse Polish Notation

Reverse Polish notation uses a stack.

Example:

```text
"3 4 +" = 7
"5 1 2 + 4 * + 3 -" = 14
```

## RPN method index

| Method / pattern | Use |
|---|---|
| `.split_whitespace()` | tokenize expression |
| `.parse::<i32>()` | parse numbers |
| `Vec::new()` | stack |
| `.push()` | push number |
| `.pop()` | pop operands |
| `match token` | handle operators |

```rust
fn rpn(expr: &str) -> Option<i32> {
    let mut stack: Vec<i32> = Vec::new();

    for token in expr.split_whitespace() {
        match token {
            "+" | "-" | "*" | "/" => {
                let b = stack.pop()?;
                let a = stack.pop()?;

                let value = match token {
                    "+" => a + b,
                    "-" => a - b,
                    "*" => a * b,
                    "/" => {
                        if b == 0 {
                            return None;
                        }
                        a / b
                    }
                    _ => unreachable!(),
                };

                stack.push(value);
            }
            number => {
                let value = number.parse::<i32>().ok()?;
                stack.push(value);
            }
        }
    }

    if stack.len() == 1 {
        stack.pop()
    } else {
        None
    }
}

fn main() {
    println!("{:?}", rpn("3 4 +")); // output: Some(7)
    println!("{:?}", rpn("5 1 2 + 4 * + 3 -")); // output: Some(14)
    println!("{:?}", rpn("1 +")); // output: None
}
```

---

# 15. Temperature and Unit Conversion

## Conversion pattern index

| Pattern | Meaning |
|---|---|
| `c * 9.0 / 5.0 + 32.0` | Celsius to Fahrenheit |
| `(f - 32.0) * 5.0 / 9.0` | Fahrenheit to Celsius |
| `kmh / 3.6` | km/h to m/s |
| `ms * 3.6` | m/s to km/h |

```rust
fn c_to_f(c: f64) -> f64 {
    c * 9.0 / 5.0 + 32.0
}

fn f_to_c(f: f64) -> f64 {
    (f - 32.0) * 5.0 / 9.0
}

fn main() {
    println!("{}", c_to_f(0.0)); // output: 32
    println!("{}", f_to_c(212.0)); // output: 100
}
```

```rust
fn kmh_to_ms(kmh: f64) -> f64 {
    kmh / 3.6
}

fn ms_to_kmh(ms: f64) -> f64 {
    ms * 3.6
}

fn main() {
    println!("{}", kmh_to_ms(36.0)); // output: 10
    println!("{}", ms_to_kmh(10.0)); // output: 36
}
```

---

# 16. Number Spelling and Ordinals

Number spelling is usually solved with `match`.

## Spelling pattern index

| Pattern | Use |
|---|---|
| `match n` | map numbers to words |
| arrays of words | lookup by index |
| `format!()` | combine words |
| `% 10` and `/ 10` | split tens and ones |

## Simple spelling

```rust
fn spell_digit(n: u32) -> &'static str {
    match n {
        0 => "zero",
        1 => "one",
        2 => "two",
        3 => "three",
        4 => "four",
        5 => "five",
        6 => "six",
        7 => "seven",
        8 => "eight",
        9 => "nine",
        _ => "unknown",
    }
}

fn main() {
    println!("{}", spell_digit(7)); // output: seven
}
```

## Ordinal suffix

```rust
fn ordinal(n: u32) -> String {
    let suffix = if (11..=13).contains(&(n % 100)) {
        "th"
    } else {
        match n % 10 {
            1 => "st",
            2 => "nd",
            3 => "rd",
            _ => "th",
        }
    };

    format!("{}{}", n, suffix)
}

fn main() {
    println!("{}", ordinal(1)); // output: 1st
    println!("{}", ordinal(2)); // output: 2nd
    println!("{}", ordinal(11)); // output: 11th
    println!("{}", ordinal(23)); // output: 23rd
}
```

---

# 17. Safe Arithmetic and Overflow

Rust integer overflow can panic in debug builds or wrap in release builds, depending on context.

## Safe arithmetic method index

| Method | Common return type | Meaning |
|---|---|---|
| `.checked_add(x)` | `Option<T>` | `None` on overflow |
| `.checked_sub(x)` | `Option<T>` | `None` on overflow |
| `.checked_mul(x)` | `Option<T>` | `None` on overflow |
| `.checked_div(x)` | `Option<T>` | `None` on division by zero or overflow |
| `.saturating_add(x)` | `T` | clamp at numeric limit |
| `.wrapping_add(x)` | `T` | wrap around on overflow |

## Checked arithmetic

```rust
fn safe_multiply(a: u32, b: u32) -> Option<u32> {
    a.checked_mul(b)
}

fn main() {
    println!("{:?}", safe_multiply(10, 20)); // output: Some(200)
}
```

## Checked division

```rust
fn safe_divide(a: i32, b: i32) -> Option<i32> {
    a.checked_div(b)
}

fn main() {
    println!("{:?}", safe_divide(10, 2)); // output: Some(5)
    println!("{:?}", safe_divide(10, 0)); // output: None
}
```

---

# 18. Common Numeric Exercise Templates

## Template 1 — Count values matching a numeric condition

```rust
fn count_positive(numbers: &[i32]) -> usize {
    numbers.iter().filter(|&&n| n > 0).count()
}

fn main() {
    println!("{}", count_positive(&[-1, 2, 3, -4])); // output: 2
}
```

## Template 2 — Transform numeric vector

```rust
fn abs_values(numbers: &[i32]) -> Vec<i32> {
    numbers.iter().copied().map(|n| n.abs()).collect()
}

fn main() {
    println!("{:?}", abs_values(&[-1, 2, -3])); // output: [1, 2, 3]
}
```

## Template 3 — Filter numeric vector

```rust
fn valid_scores(scores: &[i32]) -> Vec<i32> {
    scores
        .iter()
        .copied()
        .filter(|score| *score >= 0 && *score <= 100)
        .collect()
}

fn main() {
    println!("{:?}", valid_scores(&[-5, 50, 120, 80])); // output: [50, 80]
}
```

## Template 4 — Clamp numeric vector

```rust
fn clamp_scores(scores: &[i32]) -> Vec<i32> {
    scores.iter().copied().map(|score| score.clamp(0, 100)).collect()
}

fn main() {
    println!("{:?}", clamp_scores(&[-5, 50, 120])); // output: [0, 50, 100]
}
```

## Template 5 — Return `Option` for empty input

```rust
fn max_score(scores: &[i32]) -> Option<i32> {
    scores.iter().copied().max()
}

fn main() {
    println!("{:?}", max_score(&[10, 90, 30])); // output: Some(90)
    println!("{:?}", max_score(&[])); // output: None
}
```

---

# 19. Common Numeric Compiler Errors and Fixes

## Error: ambiguous numeric type

Wrong:

```rust
fn main() {
    let x = -5;
    // println!("{}", x.abs()); // output: compiler error if uncommented
}
```

Correct:

```rust
fn main() {
    let x: i32 = -5;
    println!("{}", x.abs()); // output: 5
}
```

## Error: `.pow()` expects `u32`

Wrong idea:

```rust
fn main() {
    let base: i32 = 2;
    let power: i32 = 3;

    println!("{}", base.pow(power as u32)); // output: 8
}
```

Correct pattern:

```rust
fn main() {
    let base: i32 = 2;
    let power: u32 = 3;

    println!("{}", base.pow(power)); // output: 8
}
```

## Error: integer division removes decimals

```rust
fn main() {
    let a = 5;
    let b = 2;

    println!("{}", a / b); // output: 2
    println!("{}", a as f64 / b as f64); // output: 2.5
}
```

## Error: `min()` and `max()` on iterators return `Option`

```rust
fn main() {
    let numbers = [4, 9, 2];
    let max = numbers.iter().copied().max().unwrap_or(0);

    println!("{}", max); // output: 9
}
```

---

# 20. Quick Method Return-Type Table

| Method / pattern | Common return type |
|---|---|
| `a + b` | number |
| `a - b` | number |
| `a * b` | number |
| `a / b` | number |
| `a % b` | number |
| `x.abs()` | signed number |
| `x.pow(exp)` | number |
| `x.powi(exp)` | float |
| `x.powf(exp)` | float |
| `x.sqrt()` | float |
| `x.round()` | float |
| `x.floor()` | float |
| `x.ceil()` | float |
| `x.trunc()` | float |
| `x.fract()` | float |
| `x.min(y)` | same type |
| `x.max(y)` | same type |
| `x.clamp(a, b)` | same type |
| `x.is_positive()` | `bool` |
| `x.is_negative()` | `bool` |
| `iter.sum()` | numeric type |
| `iter.product()` | numeric type |
| `iter.min()` | `Option<T>` |
| `iter.max()` | `Option<T>` |
| `checked_add()` | `Option<T>` |
| `checked_sub()` | `Option<T>` |
| `checked_mul()` | `Option<T>` |
| `checked_div()` | `Option<T>` |

---

# 21. Practice Exercises

## Exercise 1 — Fix score

Write:

```rust
fn fix_score(score: i32) -> i32
```

Rules:

1. Add `10`.
2. Clamp between `0` and `100`.

Expected:

```rust
println!("{}", fix_score(95)); // output: 100
println!("{}", fix_score(-20)); // output: 0
```

## Exercise 2 — Sum digits

Write:

```rust
fn sum_digits(n: u64) -> u64
```

Expected:

```rust
println!("{}", sum_digits(1234)); // output: 10
```

## Exercise 3 — Prime checker

Write:

```rust
fn is_prime(n: u64) -> bool
```

Expected:

```rust
println!("{}", is_prime(17)); // output: true
println!("{}", is_prime(21)); // output: false
```

## Exercise 4 — Factorial

Write:

```rust
fn factorial(n: u64) -> u64
```

Expected:

```rust
println!("{}", factorial(5)); // output: 120
```

## Exercise 5 — Average

Write:

```rust
fn average(numbers: &[i32]) -> Option<f64>
```

Expected:

```rust
println!("{:?}", average(&[10, 20, 30])); // output: Some(20.0)
println!("{:?}", average(&[])); // output: None
```

## Exercise 6 — Roman numeral

Write:

```rust
fn to_roman(n: u32) -> String
```

Expected:

```rust
println!("{}", to_roman(58)); // output: LVIII
```

## Exercise 7 — RPN

Write:

```rust
fn rpn(expr: &str) -> Option<i32>
```

Expected:

```rust
println!("{:?}", rpn("3 4 +")); // output: Some(7)
println!("{:?}", rpn("1 +")); // output: None
```

---

End of **cheat_sheet4.md**.
