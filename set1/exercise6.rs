#![allow(non_snake_case)]
extern crate base64Tools;
extern crate libpal;

//extern crate base64;
use base64::{Engine as _, alphabet, engine::{self, general_purpose}};
use std::fs;

// Computes Humming Distance between 2 string chat by char or returns error why it cannot be done

// TODO: this can be improved with a look up table or a better version for improved efficiency.
// This is a hot spot function

fn ComputeHummingDistanceStr(input1: &str,input2: &str) -> Result<u32, &'static str> 
{
	let vec1: Vec<_> = libpal::StrToU8Vec(input1);
	let vec2: Vec<_> = libpal::StrToU8Vec(input2);
	return ComputeHummingDistance(vec1, vec2);
}

fn ComputeHummingDistance(input1: Vec<u8>, input2: Vec<u8>) -> Result<u32, &'static str> 
{
	// Validate size is the same
	if input1.len() != input2.len()
	{
		return Err("String Length Missmatch!");
	}

	let mut distance: u32 = 0;

	for i in 0..input1.len()
	{
		let res = (input1[i] as u8) as u32 ^ (input2[i] as u8) as u32;
		distance += res.count_ones();
	}

	return Ok(distance);
}

fn Tests()
{
	assert_eq!(ComputeHummingDistanceStr("this is a test", "wokka wokka!!!").unwrap(), 37);
	println!("All tests pass!");
}

const MIN_KEYSIZE: usize = 2;
const MAX_KEYSIZE: usize = 40;

fn GuessKeySize(input: &Vec<u8>) -> usize
{
	let sizeFactor: usize = 4;
	let mut bestkeylen = MIN_KEYSIZE;
	let mut score: f32 = 0.0;
	let mut bestscore: f32 = 0.0;
	// Find score for each key len
	for keylen in MIN_KEYSIZE..MAX_KEYSIZE
	{
		/* Take 2 chunks of data and try finding Hamming distance between them for each keylen, normalize by keylen */
		
		// Take 2 slices
		let chunk1: Vec<u8> = input[0..keylen*sizeFactor].to_vec();
		let chunk2: Vec<u8> = input[keylen*sizeFactor..keylen*sizeFactor*2].to_vec();
		score = (ComputeHummingDistance(chunk1, chunk2).unwrap() as f32) / (keylen as f32);
		println!("Score is: {}", score);
		if bestscore < score
		{
			bestscore = score;
			bestkeylen = keylen;
		}
	}

	return bestkeylen;
}

fn read_file_vec(filepath: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
	let data = fs::read(filepath)?;
	Ok(data)
}

fn main()
{
	let contentBase64 = read_file_vec("set1/file6.txt");
	let bytes = general_purpose::STANDARD.decode(contentBase64.unwrap()).unwrap();
	println!("{:?}", bytes);

	Tests();
}
