// Pim
// Faça um programa que recebe um número do usuário e imprime a sequência Pim™ até esse valor. 
// Ex: 
// Input: 13
// Output: 1, 2, 3, pim, 5, 6, 7, pim, 9, 10, 11, pim, 13.
 
// ⭐ Fazer com que o programa depois de cada terceiro pim troque o próximo número por piririm.  
//Ex: 1, 2, 3, pim¹, 5, 6, 7, pim², 9, 10, 11, pim³, piririm, 14, pim

use std::io;

fn main() {
    println!("Digite um numero");
    let mut numero_inicial = String::new();
    io::stdin().read_line(&mut numero_inicial).expect("Erro ao ler o numero");
    let numero_inicial:i64 = numero_inicial.trim().parse().expect("Digite um numero valido");
    let pim = 4;
    let mut numero_atual = 1;
    let mut prox = false;
    let mut pim_atual = 0;
    while numero_atual <= numero_inicial {
        if numero_atual % pim == 0 {
            print!(", pim");
            pim_atual += 1;
            if pim_atual % 3 == 0 {
                prox = true;
                pim_atual = 0;
            }
        } else if prox {
            print!(", piririm");
            prox = false;
        }else{
            if numero_atual == 1 {
                print!("{}", numero_atual);
            } else {
                print!(", {}", numero_atual);
            }    
        }
        numero_atual += 1;
    }
}