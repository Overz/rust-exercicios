// Esta tarefa requer que um programa em Rust seja desenvolvido para ler um número inteiro e exibir sua tabuada.
// O programa deve ler o número do usuário, calcular o resultado da multiplicação para cada número de 1 a 10 e exibir os resultados na tela.

use crate::{parse_to_i32, read_stdin};

pub fn run() {
	let mut a = String::new();
	read_stdin!(&mut a, "digite o número da tabuada de 1 a 10: ");
	let v = parse_to_i32!(a);

	println!("Tabuada de '{}': ", v);
	for i in 1..11 {
		println!("{} x {} = {}", v, i, v * i);
	}
}
