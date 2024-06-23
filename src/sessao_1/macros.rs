#[macro_export]
macro_rules! parse_to_i32 {
	($v:expr) => {
		if let Ok(val) = $v.trim().parse::<i32>() { val as i32 }
		else { eprintln!("Erro ao converter '{}' para i32", $v); 0 }
	};
}

#[macro_export]
macro_rules! read_stdin {
	($v:expr, $msg:expr) => {
		{
			use std::io::{self, Write};

			print!("{}", $msg);
			let _ = io::stdout().flush();

			io::stdin().read_line($v).expect("Erro lendo valor digitado no terminal");

			*$v = $v.trim().to_string();
		}
	};
}
