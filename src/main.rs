// ASCII CHARACTERS
// Numbers				48 - 57
// Lowercase letters	97 - 122
// Uppercase letters	65 - 90
// Symbols				33 - 47

use rand::Rng;
mod clear_screen;

fn generate_password(pwd_length: u8, include_uppercase: bool, include_numbers: bool, include_symbols: bool) -> String {
	// Create new vector
	let mut options_vec: Vec<u8> = Vec::new();

	// Always have lowercase letters
	options_vec.push(0);

	// add options as passed in
	if include_uppercase {options_vec.push(1)};
	if include_numbers {options_vec.push(2)};
	if include_symbols { options_vec.push(3)};

	// Iterate over the length of password required and generate a char and for each position and concat into string
	let value: String = (0..pwd_length).map(|_| char::from(
		match options_vec[rand::thread_rng().gen_range(0..options_vec.len())] {
			0 => rand::thread_rng().gen_range(97..=122),	// Lowercase letters
			1 => rand::thread_rng().gen_range(65..=90),		// Uppercase letters
			2 => rand::thread_rng().gen_range(48..=57),		// Numbers
			_ => rand::thread_rng().gen_range(33..=47)		// Symbols
		}
	)).collect();
	return value;
}

fn main() {
	clear_screen::clr();
	println!("Password: {}", generate_password(16, true, false, false));
}
