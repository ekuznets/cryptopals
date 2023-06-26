extern crate libpal;
use std::collections::HashSet;

fn ValidateHumanReadable(ch: &char) -> bool
{
	if ((*ch as u8) >= 32) && ((*ch as u8) <= 126) // 32 is " " and 126 is "~"
	{
		return true;
	}
	{
		return false;
	}
}

fn CountMessageScore(input: &Vec<u8>) -> u8
{
	let mut vowels_set: HashSet::<char> = HashSet::new();
	vowels_set.insert('a');
	vowels_set.insert('e');
	vowels_set.insert('i');
	vowels_set.insert('o');
	vowels_set.insert('u');
	vowels_set.insert('y');

	let mut counter:i32 = 0;

	for &byte in input 
	{
        let character = byte as char;
		if character.is_alphabetic()
		{
            let lowercase_char = character.to_ascii_lowercase();
			if vowels_set.contains(&lowercase_char) 
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
	
	if(counter < 0)
	{
		counter = 0;
	}

	let s = String::from_utf8(input.clone()).expect("Found invalid UTF-8");
	return counter as u8;
}

fn Calculate(HexString: &str) -> String
{
	let byte_stream = libpal::decode_hex(&HexString).unwrap();

	let mut list_of_solutions = Vec::new();

	let mut i:u8 = 0;

	for i in 0..255
	{
		let mut msg_array = Vec::new();

		for j in 0..byte_stream.len()
		{
			let res_ch = byte_stream[j] ^ i;
			if !ValidateHumanReadable(&(res_ch as char))
			{
				break;
			}
			msg_array.push(res_ch);
		}

		//println!("Potential Solution for i: {}", i);
		//let s = String::from_utf8(msg_array).expect("Found invalid UTF-8");
		//println!("{:?}", s);

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
		local_max = CountMessageScore(&list_of_solutions[i]);

		if local_max > abs_max
		{
			abs_max = local_max;
			local_max = 0;
			index = i;

			let s = String::from_utf8(list_of_solutions[i as usize].clone()).expect("Found invalid UTF-8");
			//println!("Number of vowels: {} for {:?}",i, s);
		}
	}

	let s = String::from_utf8(list_of_solutions[index as usize].clone()).expect("Found invalid UTF-8");
	//println!("{:?}", s);

	return s;
}

fn CrackXorTest()
{
	let message = Calculate("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736");
	println!("{}", message);
}

fn main()
{
	CrackXorTest();
}
