use std::{num::ParseIntError};

const LOOKUP_TABLE: [char; 64] = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P','Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '+', '/'];
const PADDING: char = '=';

fn EncodeChunk(vec: &Vec<u8>) -> Vec<char>
{
	let mut output = Vec::new();

	match vec.len(){
		3=> {
			println!("Process 3 bytes");
			output.push(LOOKUP_TABLE[((vec[0] & 0b11111100) >> 2) as usize]);
			println!("{:?}", output);
			output.push(LOOKUP_TABLE[(((vec[0] & 0b00000011) << 4) | ((vec[1] & 0b11110000)) >> 4) as usize]);
			println!("{:?}", output);
			output.push(LOOKUP_TABLE[(((vec[1] & 0b00001111) << 2) | (vec[2] & 0b11000000) >> 6 ) as usize]);
			println!("{:?}", output);
			output.push(LOOKUP_TABLE[(vec[2] & 0b00111111) as usize]);

		},
		2=> {
			println!("Process 2 bytes");
			output.push(LOOKUP_TABLE[((vec[0] & 0b11111100) >> 2) as usize]);
			output.push(LOOKUP_TABLE[(((vec[0] & 0b00000011) << 4) + ((vec[1] & 0b11110000)) >> 2) as usize]);
			output.push(LOOKUP_TABLE[((vec[1] & 0b00001111) << 2 ) as usize]);
			output.push(PADDING);
		},
		1=>{
			println!("Process 1 bytes");
			output.push(LOOKUP_TABLE[((vec[0] & 0b11111100)  >> 2) as usize]);
			output.push(LOOKUP_TABLE[((vec[0] & 0b00000011) << 4) as usize]);
			output.push(PADDING);
			output.push(PADDING);
		},
		_=>println!("Is Empty!")
	}
	return output;
}


pub fn decode_hex(s: &str) -> Result<Vec<u8>, ParseIntError> {
	(0..s.len())
		.step_by(2)
		.map(|i| u8::from_str_radix(&s[i..i + 2], 16))
		.collect()
}
pub fn hex_to_base64(s: &Vec<u8>) -> Result<Vec<char>, ParseIntError> {

	let mut output = Vec::new();
	
	for i in (0..s.len()).step_by(3)
	{
		let mut chunk = &s[i..i+3].to_vec();
		println!("{:?} len is: {} i {}", chunk, chunk.len(), i);
		output.extend(EncodeChunk(chunk));
	}
	return Ok(output);
}

fn HexToBase64(input: &str) -> String
{
	let byte_vector = decode_hex(input);
	println!("{:?}", byte_vector);
	let output = hex_to_base64(&byte_vector.unwrap());
	let string: String = output.unwrap().iter().cloned().collect();
	return string;
}

fn HexToBase64Test()
{
	let mut input = "010000";
	let mut base64_result = HexToBase64(input);
	assert_eq!(base64_result, "AQAA");

	input = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
	base64_result = HexToBase64(input);
	assert_eq!(base64_result, "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t");
}

fn main()
{
	HexToBase64Test();
}