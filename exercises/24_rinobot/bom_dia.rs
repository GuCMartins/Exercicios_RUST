// Bom dia
// Faça um programa que recebe um nome do usuário e imprime uma mensagem de bom dia para esse nome. 
// Ex.: Bom dia, Helton

// ⭐ Fazer com que o programa responda de acordo com a hora do dia. De noite quero receber meu boa noite.

extern crate chrono;
use chrono::prelude::*;
use std::io;

fn main() {
    println!("Qual o seu nome?");
    let mut nome = String::new();
    io::stdin().read_line(&mut nome).expect("Erro ao ler o nome");//recebe o nome e associa a variavel 
    let time = Local::now().hour();
    if  time < 12 {
        println!("Bom dia, {}", nome);
    } else if time < 18 {
        println!("Boa tarde, {}", nome);
    } else {
        println!("Boa noite, {}", nome);
    }
}