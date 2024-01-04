// move_semantics6.rs
//
// You can't change anything except adding or removing references.
//
// Execute `rustlings hint move_semantics6` or use the `hint` watch subcommand
// for a hint.

// I AM NOT DONE

fn main() {
    let data = "Rust is great!".to_string();

    //In Rust, when a variable is passed to a function, the function takes ownership of that variable,
    //and the original variable can no longer be used. 

    get_char(&data);//passa como referencia e nao troca o dono da variavel, podendo ser usada de novo

    string_uppercase(data);//passa como dono e nao pode ser usada de novo, sendo alterada permanentemente
}

// Should not take ownership
fn get_char(data: &String) -> char {
    data.chars().last().unwrap()//unwrap retorna o valor do Option, que é o ultimo char da string
}

// Should take ownership
fn string_uppercase(mut data: String) {
    data = data.to_uppercase();//altera a string para uppercase

    println!("{}", data);//printa a string
}
