#![allow(non_snake_case)]
use libpal::pal as libpal;
use generic_array;
use aes::Aes128;
use aes::cipher::{
    BlockDecrypt, KeyInit
};

fn main()
{
	// Setup Key
	// TODO: Create a cipher factory that will given a key will create me an engine
	use generic_array::{typenum::U16, GenericArray};
	let phrase = "YELLOW SUBMARINE";
	let bytes = phrase.as_bytes(); // TODO: This API is ugly, is there a better way?
	let key: GenericArray<_, U16> = GenericArray::clone_from_slice(&bytes[0..16]);
	let cipher = Aes128::new(&key);

	// Setup Data
	let content = libpal::ReadAndDecodeFileByLine("solutions/data/file7.txt");

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