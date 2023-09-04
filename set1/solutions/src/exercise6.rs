#![allow(non_snake_case)]
use libpal::pal as libpal;

fn Tests()
{
	assert_eq!(libpal::ComputeHummingDistanceStr("this is a test", "wokka wokka!!!").unwrap(), 37);
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
	let sizeMod: usize = 5;

	for keylen in MIN_KEYSIZE..MAX_KEYSIZE 	// Find score for each key size
	{
		/* Take 2 chunks of data and finding Hamming distance between them for each keylen,
		then normalize by keylen to obtain the score of how close are two chuncks 
		The best result would likely indicate which keysize is used to protect data.
		Lowesst score means that two chucks are closer to each other bitwise.
		*/
		let chunk1: Vec<u8> = input[0..keylen*sizeFactor*sizeMod].to_vec();
		let chunk2: Vec<u8> = input[keylen*sizeFactor*sizeMod..keylen*sizeFactor*2*sizeMod].to_vec();
		score = (libpal::ComputeHummingDistance(chunk1, chunk2).unwrap() as f32) / (keylen as f32);
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

	let blocks: Vec<Vec<u8>> = libpal::TransposeDataBasedonKeyLen(&rawContent, bestKeyLen);
	let mut MessageKey = Vec::<u8>::new();

	for i in 0..blocks.len()
	{
		let xor_soloution = libpal::CrackSingleXor(&blocks[i]);
		MessageKey.push(xor_soloution.key)
	}

	let stringMessageKey = libpal::u8VecToString(&MessageKey);
	println!("Found Key: {:?}", stringMessageKey);
	let stringText = libpal::u8VecToString(&rawContent);
	let decryptText = libpal::RepeatingKeyXor(&stringText, &stringMessageKey);
	println!("Decrypted Text: {:?}", libpal::u8VecToString(&decryptText));

	Tests();
}
