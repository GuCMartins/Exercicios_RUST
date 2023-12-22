// variables3.rs
//
// Execute `rustlings hint variables3` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE

use std::io;

fn main() {
    println!("Digite um numero");
    let mut x = String::new();
    io::stdin().read_line(&mut x).expect("Erro ao ler o numero");
    let x:i32 = x.trim().parse().expect("Digite um numero valido");
    println!("Mambo Number {}", x);
}
