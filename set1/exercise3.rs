extern crate libpal;

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
			if !libpal::ValidateHumanReadableChar(&(res_ch as char))
			{
				break;
			}
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
		local_max = libpal::CountMessageScore(&list_of_solutions[i]);

		if local_max > abs_max
		{
			abs_max = local_max;
			local_max = 0;
			index = i;
		}
	}

	let s = libpal::u8VecToString(&list_of_solutions[index as usize]);
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
