#![allow(non_snake_case)]
// Look up table that has all 64 value for encoding plus the '=' sign for the end piece 
const LOOKUP_TABLE: [char; 64] = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P','Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '+', '/'];
const PADDING: char = '=';

// Process 1 to 3 bytes into 4 symbols of Base64
fn EncodeChunk(vec: &Vec<u8>) -> Vec<char>
{
	let mut output = Vec::new();

	match vec.len(){
		3=> {
			//println!("Process 3 bytes");
			output.push(LOOKUP_TABLE[((vec[0] & 0b11111100) >> 2) as usize]);
			//println!("{:?}", output);
			output.push(LOOKUP_TABLE[(((vec[0] & 0b00000011) << 4) | ((vec[1] & 0b11110000)) >> 4) as usize]);
			//println!("{:?}", output);
			output.push(LOOKUP_TABLE[(((vec[1] & 0b00001111) << 2) | (vec[2] & 0b11000000) >> 6 ) as usize]);
			//println!("{:?}", output);
			output.push(LOOKUP_TABLE[(vec[2] & 0b00111111) as usize]);

		},
		2=> {
			//println!("Process 2 bytes");
			output.push(LOOKUP_TABLE[((vec[0] & 0b11111100) >> 2) as usize]);
			output.push(LOOKUP_TABLE[(((vec[0] & 0b00000011) << 4) | ((vec[1] & 0b11110000)) >> 2) as usize]);
			output.push(LOOKUP_TABLE[((vec[1] & 0b00001111) << 2 ) as usize]);
			output.push(PADDING);
		},
		1=>{
			//println!("Process 1 bytes");
			output.push(LOOKUP_TABLE[((vec[0] & 0b11111100)  >> 2) as usize]);
			output.push(LOOKUP_TABLE[((vec[0] & 0b00000011) << 4) as usize]);
			output.push(PADDING);
			output.push(PADDING);
		},
		_=>println!("Is Empty!")
	}
	return output;
}

// Takes a Vector of Hex Bytes and returns a Vector of Base64 symbols
pub fn HexToBase64(s: &Vec<u8>) -> Vec<char> 
{
	let mut output = Vec::new();
	for i in (0..s.len()).step_by(3)
	{
		let end = std::cmp::min(3, s.len() - i);
		let mut chunk = &s[i..i + end].to_vec();
		output.extend(EncodeChunk(chunk));
	}
	return output;
}