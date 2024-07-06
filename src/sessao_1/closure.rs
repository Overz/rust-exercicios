use std::thread;

fn run_1() {
	let greeting = String::from("Hello");

	let handle = thread::spawn(move || {
		println!("{}", greeting);
	});

	handle.join().unwrap();
}

fn run_2() {
	let mut num = 0;

	{
		let mut add_to_num = move |x| num += x;
		add_to_num(5);
	}

	println!("O valor de num é: {}", num); // Funciona: O valor de num é: 5
}

#[test]
fn test() {
	run_1();
	run_2();
}
