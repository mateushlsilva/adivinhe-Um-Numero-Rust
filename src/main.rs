use std::io;
use rand::Rng;

fn main() {
    println!("Adivinhe o número!");

    println!("Por favor, insira o seu palpite: ");

    let numeroSecreto = rand::thread_rng().gen_range(1..=100);

    let mut numero = String::new();

    io::stdin()
    .read_line(&mut numero)
    .expect("Erro ao ler a linha.");

    println!("O número secreto é {numeroSecreto}");

    println!("Você adivinhou {numero}")
}
