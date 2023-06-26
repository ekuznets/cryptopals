use std::num::ParseIntError;
use std::collections::HashSet;

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
	return String::from_utf8(vec.clone()).expect("Found invalid UTF-8");
}

// My hacky version to allow only human readable ascii characters
// input char is 0 to 255 out of which only 32 to 126 inclusive are readable
// Does not include ascii extended set of characters
pub fn ValidateHumanReadableChar(ch: &char) -> bool
{
	if (*ch >= ' ') && (*ch <= '~') // 32 is " " and 126 is "~"
	{
		return true;
	}
	{
		return false;
	}
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
	// ToDo: this can be much better but I am lazy to fix
	let mut vowels_set: HashSet::<u8> = HashSet::new();
	vowels_set.insert('a' as u8);
	vowels_set.insert('e' as u8);
	vowels_set.insert('i' as u8);
	vowels_set.insert('o' as u8);
	vowels_set.insert('u' as u8);
	vowels_set.insert('y' as u8);

	let mut counter:i32 = 0;

	for &byte in input 
	{
        let character = byte as char;
		if character.is_alphabetic()
		{
            let lowercase_char = character.to_ascii_lowercase();
			if vowels_set.contains(&(lowercase_char as u8)) 
			{
                counter += 1;
            }
		}
		else if character == ' '
		{
			counter += 1;
		}
		else
		{
			counter -=1;
		}
	}
	if counter < 0
	{
		counter = 0;
	}
	return counter as u8;
}
