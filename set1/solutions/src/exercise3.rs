#![allow(non_snake_case)]
use libpal::pal as libpal;

fn CrackXorTest()
{
	let message: libpal::XorCrackSolution = libpal::CrackXor("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736");
	println!("{} - {}", message.text, message.score);
	assert_eq!(message.text, "Cooking MC's like a pound of bacon");

	let message2: libpal::XorCrackSolution = libpal::CrackXor("E38AC6C5DCCF8AD9C5C7CF8AC9C5CCCCCFCF8AC3C48ADEC2CF8AC7C5D8C4C3C4CD8A9898928B");
	println!("{} - {}", message2.text, message2.score);
	assert_eq!(message2.text, "I love some coffee in the morning 228!");

	println!("All tests pass!");
}

fn main()
{
	CrackXorTest();
}
