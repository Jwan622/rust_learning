fn main() {
    let numbers = vec![1, 2, 3];
    let incremented: Vec<u32> = numbers.iter().map(|&x| x + 1).collect(); // this does not need type annotation
    println!("{:?}", incremented);
}