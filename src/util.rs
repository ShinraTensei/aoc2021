use std::fs;

pub fn load_input(filepath: &str) -> Vec<i32> {
	let input = fs::read_to_string(filepath).expect("Something went wrong reading the file");
	
	let mut tmp: Vec<i32> = Vec::new();
	let input_split = input.trim().split("\n");
	
	for i in input_split {
		tmp.push(i.parse::<i32>().unwrap());
	}
	return tmp;
}
