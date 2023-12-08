// Semáforo
// Faça uma função que recebe uma cor de semáforo (enum) e troca a variável de velocidade 
// do carro de acordo com a cor.

// ⭐ (Pré requisitos: struct, vetor) → representar o carro como tendo velocidade e deslocamento. 
// Receber um vetor de cores de semáforo também com o tempo que o semáforo ficou em cada cor. 
// Calcular assim a evolução do deslocamento do veículo

use std::io;
use std::collections::HashMap;

struct Carro{
    velocidadeVermelho: i64,//1
    velocidadeVerde: i64,//2
    velocidadeAmarelo: i64,//3
    deslocamento: i64,
}

fn deslocamento(carro: Carro, cor:i32, tempo: i64) -> i64{
    if cor == 1 {
        carro.deslocamento += carro.velocidadeVermelho * tempo;
    } else if cor == 2 {
        carro.deslocamento += carro.velocidadeVerde * tempo;
    } else {
        carro.deslocamento += carro.velocidadeAmarelo * tempo;
    }
}

fn main(){
    let mut carro = Carro{velocidadeVermelho: 0,velocidadeVerde: 0, velocidadeAmarelo: 0, deslocamento: 0};
    let mut semaforo = HashMap::new();
    
    println!("Digite a velocidade do carro para o semaforo vermelho");
    let mut velocidade = String::new();
    io::stdin().read_line(&mut velocidade).expect("Erro ao ler a velocidade");
    let velocidade = velocidade.trim();
    let velocidade = velocidade.parse::<i64>().expect("Digite um numero valido");
    carro.velocidadeVermelho = velocidade;

    println!("Digite a velocidade do carro para o semaforo verde");
    let mut velocidade = String::new();
    io::stdin().read_line(&mut velocidade).expect("Erro ao ler a velocidade");
    let velocidade = velocidade.trim();
    let velocidade = velocidade.parse::<i64>().expect("Digite um numero valido");
    carro.velocidadeVerde = velocidade;

    println!("Digite a velocidade do carro para o semaforo amarelo");
    let mut velocidade = String::new();
    io::stdin().read_line(&mut velocidade).expect("Erro ao ler a velocidade");
    let velocidade = velocidade.trim();
    let velocidade = velocidade.parse::<i64>().expect("Digite um numero valido");
    carro.velocidadeAmarelo = velocidade;

    println!("Digite quantas cores o semaforo apresentou no total");
    let mut num_cores = String::new();
    io::stdin().read_line(&mut num_cores).expect("Erro ao ler a quantidade de cores");
    let num_cores = num_cores.trim();
    let mut cores = num_cores.parse::<i64>().expect("Digite um numero valido");

    for i in 0..cores {
        println!("Digite a cor do semaforo (1 = vermelho, 2 = verde, 3 = amarelo)");
        let mut cor = String::new();
        io::stdin().read_line(&mut cor).expect("Erro ao ler a cor");
        let cor = cor.trim();
        let cor = cor.parse::<i32>().expect("Digite um numero valido");
        println!("Digite o tempo que o semaforo ficou nessa cor");
        let mut tempo = String::new();
        io::stdin().read_line(&mut tempo).expect("Erro ao ler o tempo");
        let tempo = tempo.trim();
        let tempo = tempo.parse::<i64>().expect("Digite um numero valido");
        semaforo.insert(cor, tempo);
    }

    for (cor, tempo) in semaforo.iter() {
        deslocamento(carro, cor, tempo);
    }

    println!("O carro andou {} metros", carro.deslocamento);
}


