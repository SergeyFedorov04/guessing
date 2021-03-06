extern crate rand;

use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
	println!("Угадай число!");

	let secret_numb = rand::thread_rng().gen_range(1, 101);

	println!("Загадонное число: {}", secret_numb);

	loop {
		println!("Пожалуйста, введите предположение...");

		let mut guess = String::new();

		io::stdin().read_line(&mut guess)
			.expect("Не удалось прочитать строку!");

		let guess: u32 = match guess.trim().parse() {
			Ok(num) => num,
			Err(_) => continue,
		};

		println!("Ваша попытка: {}", guess);

		match guess.cmp(&secret_numb) {
			Ordering::Less		=> println!("Слишком маленькое число!"),
			Ordering::Greater	=> println!("Слишком большое!"),
			Ordering::Equal		=> {
				println!("Вы выиграли!");
				break;
			}
		}
	}
}