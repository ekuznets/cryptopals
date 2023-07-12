#![allow(non_snake_case)]
extern crate base64Tools;

use std::num::ParseIntError;
use std::collections::HashSet;

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
pub fn decode_hex(s: &str) -> Result<Vec<u8>, ParseIntError> 
{
	(0..s.len())
		.step_by(2)
		.map(|i| u8::from_str_radix(&s[i..i + 2], 16))
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
	Human text usually has about 40% vowels letters where as randomly you would
	expect only 6/26 ~ 22%
	Also human text usually has ' ' space between words but in random string space
	would have a really small probability to occure.
	Also, human text has little of special characters.
	Thus we can capitalize on that:
	Each vowels = 1 point
	Each space = 1 point
	Each special character = -1 point
	Else it is 0 points

	I could also add Capital letter huristic but lets not bother for now.
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
			let lowercase_char = character.to_ascii_lowercase();
			// Vowels adds 1 score, else it is a 0 score
			if vowels_set.contains(&(lowercase_char as u8)) 
			{
				counter += 1;
			}
		}
		else if character == ' '
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
}

// Cracks Single Character XOR using bruteforce method
pub fn CrackXor(HexString: &str) -> XorCrackSolution
{
	let byte_stream = decode_hex(HexString).unwrap();
	let mut list_of_solutions = Vec::new();

	let mut i:u8 = 0;

	for i in 0..255
	{
		let mut msg_array = Vec::new();
		for j in 0..byte_stream.len()
		{
			let res_ch = byte_stream[j] ^ i;
			msg_array.push(res_ch);
		}

		let mut counter = 0;
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
	};

	if list_of_solutions.len() == 0
	{
		return sol;
	}
	else
	{
		sol.text = u8VecToString(&list_of_solutions[index as usize]);
		sol.score = abs_max;
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
