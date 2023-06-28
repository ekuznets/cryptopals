extern crate libpal;
use std::fs::read_to_string;

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename) 
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(String::from)  // make each slice into a string
        .collect()  // gather them together into a vector
}

fn CrackXorTest()
{
	let lines =  read_lines("set1/file4.txt");

	println!("Total Candidates {}", lines.len());

	let mut maxScore = 0;
	let mut index = 0;
	let mut solution : String = "".to_string();

	for i in 0..lines.len()
	{
		let sol: libpal::XorCrackSolution = libpal::CrackXor(&lines[i]);
		if(sol.text != "")
		{
			if(sol.score > maxScore)
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
	CrackXorTest();
}
