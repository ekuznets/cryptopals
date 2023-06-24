use std::num::ParseIntError;

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
