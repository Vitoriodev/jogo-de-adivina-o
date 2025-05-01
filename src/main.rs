use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("jogo de adivinação");

    let numero_secreto = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("digite um palpite.");

        let mut palpite = String::new();

        io::stdin()
            .read_line(&mut palpite)
            .expect("falha ao ler o palpite");

        let palpite: u32 = match palpite.trim().parse() {
            Ok(num) => num, //quando for um numero vai continuar o codigo
            Err(_) => {
                println!("coloque apenas numeros");
                continue; //ser não for um tipo numerico vai retonar
            }
        };

        print!("voce disse {palpite} ");

        match palpite.cmp(&numero_secreto) {
            Ordering::Less => println!("Muito baixo!"),
            Ordering::Greater => println!("Muito alto!"),
            Ordering::Equal => {
                println!("Você acertou!");
                break;
            }
        }
    }
}
