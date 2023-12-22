// variables2.rs
//
// Execute `rustlings hint variables2` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE

use std::io;

fn main() {
    println!("Digite um numero");
    let mut x = String::new();
    io::stdin().read_line(&mut x).expect("Erro ao ler o numero");
    let x:i64 = x.trim().parse().expect("Digite um numero valido");

    if x == 10 {
        println!("x is ten!");
    } else {
        println!("x is not ten!");
    }
}
