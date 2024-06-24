use std::collections::LinkedList;

use crate::{parse_to_i32, read_stdin};

pub fn run() {
	println!("Digite 0 para parar");
	println!("Digite um número positivo por vez para somar");

	let mut inputs = LinkedList::<String>::new();
	let mut input = String::new();
	loop {
		read_stdin!(&mut input, "Digite um número: ");

		if input.eq("0") {
			break;
		}

		inputs.push_back(input.clone());
		input.clear();
	}

	let mut s = 0;
	for input in inputs {
		s += parse_to_i32!(input);
	}

	println!("total: {}", s);
}
