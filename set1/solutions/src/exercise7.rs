#![allow(non_snake_case)]
use libpal::pal as libpal;

fn main()
{
	// Setup Data
	let phrase = "YELLOW SUBMARINE";
	let content = libpal::ReadAndDecodeFileByLine("solutions/data/file7.txt");

	if content.is_err()
	{
		println!("Error reading file: {}", content.err().unwrap());
		return;
	}

	let rawContent = content.unwrap();
	println!("Content size {}", rawContent.len());

	// Decrypt data with the key using AES128ECB
	let decryptData: Vec<u8> = libpal::DecryptAES128ECB(&rawContent, &libpal::StrToU8Vec(phrase));
	println!("Output: {}", libpal::u8VecToString(&decryptData));
}