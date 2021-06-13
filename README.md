# Rust prefix to postfix
### A prefix to postfix notation converter programmed in rust that uses the shunting yard algorithm

## Usage
1. Place the library inside your project, in this case test_rust_prefix_to_postfix. Then add rust_prefix_to_postfix = {path = "rust_prefix_to_postfix to the dependencies like below
```
[package]
name = "test_rust_prefix_to_postfix"
version = "0.1.0"
edition = "2018"


[dependencies]
rust_prefix_to_postfix = {path = "rust_prefix_to_postfix
```
2. Use call the trait in the library and add a new infix operation 
```Rust
use rust_prefix_to_postfix::Operation;

fn main() {
    let op_info_r = rust_prefix_to_postfix::OperationInfo::new(&"122+((2^2)*3-1)-(2^2)+2-sqrt(5*7+3)".to_string());

    if op_info_r.is_ok() {
        let mut op_info = op_info_r.unwrap();
        match op_info.reverse_polish_parsing() {
            Err(e) => println!("{:?}", e),
            Ok(_) => (),
        };
        println!("{:?}", op_info.infix);
        println!("{:?}", op_info.reverse_pn);
    }
}
```
