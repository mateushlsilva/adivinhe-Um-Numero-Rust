use std::io;

fn main() {
    println!("Adivinhe o número!");

    println!("Por favor, insira o seu palpite: ");

    let mut numero = String::new();

    io::stdin()
    .read_line(&mut numero)
    .expect("Erro ao ler a linha.");

    println!("Você adivinhou {numero}")
}
