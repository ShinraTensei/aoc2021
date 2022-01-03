use crate::util;

pub fn sol() {
	let input = util::load_input("./inputs/day1.txt");   
    
    //Tests
    let test_vec = vec![199,200,208,210,200,207,240,269,260,263];
    assert!(part1(&test_vec) == 7);
    assert!(part2(&test_vec) == 5);

	//Solutions
	println!("Day1 Part1: {}", part1(&input));
	println!("Day1 Part2: {}", part2(&input));
}

fn part1(input: &Vec<i32>) -> i32 {
	let mut solution = 0;
	for i in 0..input.len()-1 {
		if input[i] < input[i+1] {
			solution += 1;
		}	
	}
	return solution;
}

fn part2(input: &Vec<i32>) -> i32 {
	
	let mut sum2 = 0;
	let mut solution = 0;
	for i in 0..input.len()-2 {
		let sum1 = input[i] + input[i+1] + input[i+2];
		if sum1 > sum2 {
			solution += 1;
		}
		sum2 = sum1;
	}
	return solution-1;
}
