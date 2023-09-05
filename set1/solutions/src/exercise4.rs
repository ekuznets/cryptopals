#![allow(non_snake_case)]
use libpal::pal as libpal;
use std::fs::read_to_string;

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename) 
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(String::from)  // make each slice into a string
        .collect()  // gather them together into a vector
}

fn CrackSingleXorTest()
{
	// Cargo sets the executable directory to be you current directory
	let lines =  read_lines("solutions/data/file4.txt");

	println!("Total Candidates {}", lines.len());

	let mut maxScore = 0;
	let mut index = 0;
	let mut solution : String = "".to_string();

	for i in 0..lines.len()
	{
		let byte_stream = libpal::decode_hex(&lines[i]).unwrap();
		let sol: libpal::XorCrackSolution = libpal::CrackSingleXor(&byte_stream);
		if sol.text != ""
		{
			if sol.score > maxScore
			{
				maxScore = sol.score;
				solution = sol.text;
				index = i;
			}
		}
	}
	println!("Id: {}, Message {}, Score {}", index, solution, maxScore);
}

fn main()
{
	CrackSingleXorTest();
}
