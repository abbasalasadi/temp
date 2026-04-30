fn main() {
    let numbers: Vec<i32> = vec![-1, 2, 3];
    let abs: Vec<i32> = numbers
    .iter()
    .map(|&c| c.abs())
    .collect();
    println!("{:?}", abs); // output: [1, 2, 3]
}