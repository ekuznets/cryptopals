#![allow(non_snake_case)]
extern crate base64;
pub use crate::base64Tools;

use std::num::ParseIntError;
use std::collections::HashSet;
use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader};
use base64::Engine;
use generic_array;
use aes::Aes128;
use aes::cipher::{
    BlockDecrypt, KeyInit
};

// Takes Hex String as input and returns Base64 String
pub fn HexStringToBase64(input: &str) -> String
{
	let byte_vector = decode_hex(input);
	let output = base64Tools::HexToBase64(&byte_vector.unwrap());
	let string: String = output.iter().cloned().collect();
	return string;
}

/* Basic XOR gate principle:
A B   Output
0 0 -> 0
0 1 -> 1
1 0 -> 1
1 1 -> 0 */
pub fn XoRHexStringOperation(input1 :&str, input2: &str) -> String
{
	let parced_input1 = decode_hex(input1).unwrap();
	let parced_input2 = decode_hex(input2).unwrap();
	let mut output = Vec::new();

	for i in 0..parced_input1.len()
	{
		output.push(parced_input1[i] ^ parced_input2[i]);
	}
	let string: String = encode_hex(&output);
	return string;
}

// Convect Hex-String into Byte-Vector
//TODO: reports error if operation failed
pub fn decode_hex(HexString: &str) -> Result<Vec<u8>, ParseIntError> 
{
	(0..HexString.len())
		.step_by(2)
		.map(|i| u8::from_str_radix(&HexString[i..i + 2], 16))
		.collect()
}

// Convert Byte-Vector to Hex-String
pub fn encode_hex(vec: &Vec<u8>) -> String
{
	let hex : String = vec.iter()
		.map(|b| format!("{:02x}", b).to_string())
		.collect::<String>();
	return hex;
}

// Constract and returns a String from a Vector of Bytes with a copy
pub fn u8VecToString(vec: &Vec<u8>) -> String
{
	let string = unsafe {
		String::from_utf8_unchecked(vec.clone())
	};
	return string;
}

// Take a StringView and returns a Vector of Bytes
pub fn StrToU8Vec(input: &str) -> Vec<u8>
{
	let mut output = Vec::new();
	for ch in input.chars()
	{
		output.push(ch as u8);
	}
	return output;
}

// My hacky version to allow only human readable ascii characters
// input char is 0 to 255 out of which only 32 to 126 inclusive are readable
// Does not include ascii extended set of characters
pub fn ValidateHumanReadableChar(ch: &char) -> bool
{
	return (*ch >= ' ') && (*ch <= '~');
}

// Message Scoring function:
/*
	- Human text usually has about 40% vowels letters where as randomly you would
	expect only 6/26 ~ 22%
	- Also human text usually has ' ' space between words 
	but in random string it would have a really small probability to occure.
	- Also, human text has little of special characters. 
	
	Thus we can capitalize on that:
	* Each vowels = 1 point
	* Each space = 1 point
	* Each special character = -1 point
	* Else it is 0 points

	- I could also add Capital letter huristic but lets not bother for now.
	Capital letters would also give negative 1 point, since they are rare.

	This is not 100% great algorithm but it would do most of the text and thus good enough.
*/
pub fn CountMessageScore(input: &Vec<u8>) -> u8
{
	// Set of Vowels which is our score points
	let vowels_set: HashSet<u8> = vec![('a' as u8), ('e' as u8), ('i' as u8), ('o' as u8), ('u' as u8), ('y' as u8)].into_iter().collect();
	let mut counter:i32 = 0;

	for &byte in input 
	{
		let character = byte as char;
		if character.is_alphabetic()
		{
			let lowercase_char = character.to_ascii_lowercase(); // Dont bother with capital letters
			// Vowels adds 1 score, else it is a 0 score
			if vowels_set.contains(&(lowercase_char as u8)) 
			{
				counter += 1;
			}
		}
		else if character == ' ' // space is a 1 point
		{
			counter += 1;
		}
		// Takes all not renderable character, numbers and special characters
		else
		{
			counter -=1;
		}
	}
	// Lets leave 0 score as String that is not even worth considering
	if counter < 0
	{
		counter = 0;
	}
	return counter as u8;
}

pub struct XorCrackSolution
{
	pub text: String,
	pub score: u8,
	pub key: u8,
}

impl fmt::Debug for XorCrackSolution {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Key {}, Score: {}, Text: {}", self.key as char, self.key, self.text)
    }
}

// Cracks Single Character XOR using bruteforce method
pub fn CrackSingleXor(byte_stream: &Vec<u8>) -> XorCrackSolution
{
	let mut list_of_solutions = Vec::new();

	let i:u8 = 0;

	for i in 0..255
	{
		let mut msg_array = Vec::new();
		for j in 0..byte_stream.len()
		{
			let res_ch = byte_stream[j] ^ i;
			msg_array.push(res_ch);
		}

		if byte_stream.len() == msg_array.len()
		{
			list_of_solutions.push(msg_array);
		}
	}

	let mut local_max = 0;
	let mut abs_max = 0;
	let mut index = 0;

	for i in 0..list_of_solutions.len()
	{
		// Pass each possible solution to the scoring function
		local_max = CountMessageScore(&list_of_solutions[i]);

		if local_max > abs_max
		{
			abs_max = local_max;
			local_max = 0;
			index = i;
		}
	}

	let mut sol = XorCrackSolution 
	{
		text: "".to_string(),
		score: 0,
		key: 0,
	};

	if list_of_solutions.len() == 0
	{
		return sol;
	}
	else
	{
		sol.text = u8VecToString(&list_of_solutions[index as usize]);
		sol.score = abs_max;
		sol.key = index as u8;
		return sol;
	}
}

pub fn RepeatingKeyXor(input: &str, key: &str) -> Vec<u8>
{
	let input_vec = StrToU8Vec(input);
	let key_vec = StrToU8Vec(key);

	let mut output = Vec::new();
	let mut key_index = 0;

	for i in 0..input_vec.len()
	{
		output.push(input_vec[i] ^ key_vec[key_index]);
		key_index += 1;
		if key_index == key_vec.len()
		{
			key_index = 0;
		}
	}
	return output;
}

pub fn ReadAndDecodeFileByLine(file_path: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
	// Configure file
	let file = File::open(file_path)?;
	let reader = BufReader::new(file);

	// Configure Base64 decoder
	use base64::{alphabet, engine::{self, general_purpose}};
	const CUSTOM_ENGINE: engine::GeneralPurpose =
		engine::GeneralPurpose::new(&alphabet::STANDARD, general_purpose::PAD);

	// Read every line and decode the output using build base64 from crate
	let mut output_data: Vec<u8> = Vec::new();
	for line in reader.lines() {
		let encoded_line = line?;
		let decoded_data = Engine::decode(&CUSTOM_ENGINE,&encoded_line)?;
		output_data.extend(decoded_data);
	}
	Ok(output_data)
}

/*
	Takes input data and trancates it into chunks of KeyLen size.
	Then each element index with-in KeyLen placed into a 
	corresponding vector.
	Returns: vectors where each indexed element is grouped togeter
 */
pub fn TransposeDataBasedonKeyLen(input: &Vec<u8>, keyLen: usize) -> Vec<Vec<u8>>
{
	let mut blocks: Vec<Vec<u8>> = Vec::new();
	blocks.resize(keyLen, Vec::new());
	let mut contentIndex = 0;
	let mut keyIndex = 0;
	while contentIndex != input.len()
	{
		blocks[keyIndex].push(input[contentIndex]);
		contentIndex+=1;
		keyIndex = contentIndex % keyLen;
	}
	return blocks;
}

pub fn ComputeHummingDistanceStr(input1: &str,input2: &str) -> Result<u32, &'static str> 
{
	// Validate size is the same
	if input1.len() != input2.len()
	{
		return Err("String Length Missmatch!");
	}
	let vec1: Vec<_> = StrToU8Vec(input1);
	let vec2: Vec<_> = StrToU8Vec(input2);
	return ComputeHummingDistance(vec1, vec2);
}

pub fn ComputeHummingDistance(input1: Vec<u8>, input2: Vec<u8>) -> Result<u32, &'static str> 
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

// Given data and the key we can decrypt the data 
// assuming it is encrypted with AES128ECB
// Dont use this function, only keep as a reference in favour of DecryptAES128ECB
pub fn DecryptAES128ECB_LAZY(input: &Vec<u8>, key: &Vec<u8>) -> Vec<u8>
{
	use generic_array::{typenum::U16, GenericArray};
	let cipher = Aes128::new(&GenericArray::from_slice(key));

	let mut decryptData: Vec<u8> = Vec::new();
	for i in (0..input.len()).step_by(16)
	{
		let mut block: GenericArray<_, U16> = GenericArray::clone_from_slice(&input[i..i+16]);
		cipher.decrypt_block(&mut block);
		let mut vec: Vec<u8> = block.as_slice().try_into().expect("Ok");
		decryptData.append(&mut vec);
	}
	return decryptData;
}

pub fn DecryptAES128ECB(input: &[u8], key: &[u8]) -> Vec<u8>
{
	// WE only want to deal with 128bit chunks = 16 bytes
	assert!(input.len() % 16 == 0, "Input data is not 128bit aligned!");
	assert!(key.len() == 16, "Key Must be 16 bytes");
	use generic_array::{typenum::U16, GenericArray};
	 // TODO: Test how ineficient this is to be created for each block
	let cipher = Aes128::new(&GenericArray::from_slice(key));
	let mut decryptData: Vec<u8> = Vec::new();
	let mut block: GenericArray<_, U16> = GenericArray::clone_from_slice(&input[0..16]);
	cipher.decrypt_block(&mut block);
	let mut vec: Vec<u8> = block.as_slice().try_into().expect("Ok");
	decryptData.append(&mut vec);

	return decryptData;
}