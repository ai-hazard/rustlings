// clippy2.rs
// Execute `rustlings hint clippy2` or use the `hint` watch subcommand for a hint.

fn main() {
    let mut res = 42;
    let option = Some(12);
    res += match option {
        Some(x) => x,
        _ => 0
    };
    
    println!("{}", res);
}
