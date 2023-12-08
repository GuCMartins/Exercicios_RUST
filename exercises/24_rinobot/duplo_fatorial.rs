// Fatorial duplo
// Faça uma função que receba um inteiro e calcule o fatorial duplo™ desse número. 
// Ex: 5!! = (3!)! = (3x2x1)! = 6! = 720

// Obs: Esse exercício antes de tudo é uma piada. Todavia é interessante para melhor entendimento 
// de tipos de dados em Rust.

use std::io;

fn fatorial(n: i64) -> i64 {
    println!("Calculando {}!!", n);
    let mut fatorial = 1;
    let mut aux = n;
    while aux > 0 {
        fatorial *= aux;
        aux -= 1;
    }
    fatorial
}

fn main() {
    println!("Digite um numero");
    let mut numero_inicial = String::new();
    io::stdin().read_line(&mut numero_inicial).expect("Erro ao ler o numero");
    let mut resultado:i64 = numero_inicial.trim().parse().expect("Digite um numero valido");
    resultado = fatorial(resultado);
    resultado = fatorial(resultado);
    println!("{}!! = {}", numero_inicial.trim(), resultado);
}
