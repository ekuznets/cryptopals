#![allow(non_snake_case)]
use libpal::pal as libpal;

fn ComputeHummingDistanceStr(input1: &str,input2: &str) -> Result<u32, &'static str> 
{
	// Validate size is the same
	if input1.len() != input2.len()
	{
		return Err("String Length Missmatch!");
	}
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
	let mut lowestscore: f32 = 9999.0; // High result on purpose
	let sizeMod = 5;
	// Find score for each key len
	for keylen in MIN_KEYSIZE..MAX_KEYSIZE
	{
		/* Take 2 chunks of data and try finding Hamming distance between them for each keylen, normalize by keylen */
		
		// Take 2 slices
		let chunk1: Vec<u8> = input[0..keylen*sizeFactor*sizeMod].to_vec();
		let chunk2: Vec<u8> = input[keylen*sizeFactor*sizeMod..keylen*sizeFactor*2*sizeMod].to_vec();
		score = (ComputeHummingDistance(chunk1, chunk2).unwrap() as f32) / (keylen as f32);
		println!("Score is: {}", score);
		if lowestscore > score
		{
			lowestscore = score;
			bestkeylen = keylen;
		}
	}

	return bestkeylen;
}



fn main()
{
	let content = libpal::ReadAndDecodeFileByLine("solutions/data/file6.txt");

	if content.is_err()
	{
		println!("Error reading file: {}", content.err().unwrap());
		return;
	}

	let rawContent = content.unwrap();

	let bestKeyLen = GuessKeySize(&rawContent);
	println!("Best key len is: {}", bestKeyLen);

	// Peform Transpose
	let blocks: Vec<Vec<u8>> = libpal::TransposeDataBasedonKeyLen(&rawContent, bestKeyLen);

	let mut MessageKey = Vec::<u8>::new();

	for i in 0..blocks.len()
	{
		let xor_soloution = libpal::CrackSingleXor(&blocks[i]);
		MessageKey.push(xor_soloution.key)
	}

	let stringMessageKey = libpal::u8VecToString(&MessageKey);
	println!("Found Key: {:?}", stringMessageKey);
	let stringRawContent = libpal::u8VecToString(&rawContent);
	let decryptText = libpal::RepeatingKeyXor(&stringRawContent, &stringMessageKey);
	println!("Decrypted Text: {:?}", libpal::u8VecToString(&decryptText));

	Tests();
}
