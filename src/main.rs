use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Adivinhe o número!");

    println!("Por favor, insira o seu palpite: ");

    let numero_secreto = rand::thread_rng().gen_range(1..=100);

    let mut numero = String::new();

    io::stdin()
    .read_line(&mut numero)
    .expect("Erro ao ler a linha.");

    let numero: u32 = numero.trim().parse().expect("Por favor digite um número!");

    //println!("O número secreto é {numeroSecreto}");
    match numero.cmp(&numero_secreto) {
        Ordering::Less => println!("Muito baixo!"),
        Ordering::Greater => println!("Muito alto!"),
        Ordering::Equal => println!("Você ganhou!!!"),
        
    }
    println!("Você adivinhou {numero}")
}
