# Convert Decimal Numbers to Binary in Rust  

This repository demonstrates how to convert decimal numbers to binary in **Rust**, similar to Python's `bin()` function.  

## Usage  

Compile and run the Rust program:  

```sh
cargo run
```
## Example Output
```sh
3 -> 0b11
9 -> 0b1001 
10 -> 0b1010
1337 -> 0b10100111001
404 -> 0b110010100
```
## Rust provides the `format!` macro to generate binary string representations using `"{:b}"`.
Example:
```rust
fn main() {
    let num = 10;
    println!("Binary: 0b{:b}", num);
}

```
## Running the Code
1. Ensure you have Rust and Cargo installed.
2. Clone this repository:
```sh
git clone https://github.com/cypriansakwa/Convert_Decimal_Numbers_to_Binary_in_Rust.git
cd Convert_Decimal_Numbers_to_Binary_in_Rust
```
4. Run the program:
```sh
cargo run
```

   
