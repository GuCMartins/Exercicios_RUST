// Raiz
// Faça um programa que recebe 1 valor do usuário e imprime a raiz quadrada dele.

use std::io;
use std::f64; // Import the f64 module for the sqrt function

fn main(){
    println!("Digite um numero");
    let mut numero = String::new();
    io::stdin().read_line(&mut numero).expect("Erro ao ler o numero");
    let numero:i64 = numero.trim().parse().expect("Digite um numero valido");
    let raiz = f64::sqrt(numero as f64); // Calculate the square root of the number
    println!("A raiz quadrada de {} é {}", numero, raiz);
}