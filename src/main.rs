fn main() {
    let numbers = [3, 9, 10, 1337, 404];

    for &num in &numbers {
        println!("{} -> 0b{:b}", num, num);
    }
}
