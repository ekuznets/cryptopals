#![allow(non_snake_case)]
use libpal::pal as libpal;
use std::fs::File;
use std::io::{BufRead, BufReader};
use base64::decode;

use generic_array;
use aes::Aes128;
use aes::cipher::{
    BlockDecrypt, KeyInit
};

fn DecodeFile(file_path: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let mut output_data: Vec<u8> = Vec::new();

    for line in reader.lines() {
        let encoded_line = line?;
        let decoded_data = decode(&encoded_line)?;
        output_data.extend(decoded_data);
    }
    Ok(output_data)
}

fn main()
{
	// Setup Key
	use generic_array::{typenum::U16, GenericArray};
	let phrase = "YELLOW SUBMARINE";
	let bytes = phrase.as_bytes();
	let key: GenericArray<_, U16> = GenericArray::clone_from_slice(&bytes[0..16]);
	let cipher = Aes128::new(&key);

	// Setup Data
	let content = DecodeFile("solutions/data/file7.txt");

	if content.is_err()
	{
		println!("Error reading file: {}", content.err().unwrap());
		return;
	}

	let rawContent = content.unwrap();

	println!("Content size {}", rawContent.len());

	let mut decryptData: Vec<u8> = Vec::new();

	for i in (0..rawContent.len()).step_by(16)
	{
		let mut block: GenericArray<_, U16> = GenericArray::clone_from_slice(&rawContent[i..i+16]);
		cipher.decrypt_block(&mut block);
		let mut vec: Vec<u8> = block.as_slice().try_into().expect("Ok");
		decryptData.append(&mut vec);
	}

	println!("Output: {}", libpal::u8VecToString(&decryptData));
}