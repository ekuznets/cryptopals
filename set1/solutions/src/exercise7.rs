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

	let mut decryptData = Vec::new();

	// Decrypt data with the key using AES128ECB
	for i in (0..rawContent.len()).step_by(16)
	{	
		let decryptedChunk: Vec<u8> = libpal::DecryptAES128ECB(&rawContent[i..i+16], &phrase.as_bytes());
		decryptData.extend(decryptedChunk);
	}
	println!("Output: {}", libpal::u8VecToString(&decryptData));
}
