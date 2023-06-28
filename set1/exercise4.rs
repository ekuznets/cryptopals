extern crate libpal;
use std::fs;

fn ReadFromFile(file_path: String) -> String
{
	println!("In file {}", file_path);
	let contents = fs::read_to_string(file_path)
		.expect("Should have been able to read the file");
	println!("With text:\n{contents}");

	return contents;
}

fn CrackXorTest()
{
	let hexStream = ReadFromFile("set1/file4.txt".to_string());
	let message = libpal::CrackXor(&hexStream);
	println!("{}", message);
}

fn main()
{
	CrackXorTest();
}
