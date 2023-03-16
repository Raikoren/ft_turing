use std::io;

fn main() {
    let mut input: String = String::new();
    println!("Hello, world!");
    io::stdin().read_line(&mut input)
        .expect("bad input");
    println!("{}", input.trim_end());
}
