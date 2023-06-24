/* Basic XOR gate principle:
A B   Output
0 0 -> 0
0 1 -> 1
1 0 -> 1
1 1 -> 0
*/

fn XoROperation(input1 :&str, input2: &str) -> String
{
	let parced_input1 = libpal::decode_hex(input1).unwrap();
	let parced_input2 = libpal::decode_hex(input2).unwrap();
	let mut output = Vec::new();

	for i in 0..parced_input1.len()
	{
		output.push(parced_input1[i] ^ parced_input2[i]);
	}
	let string: String = libpal::encode_hex(&output);
	return string;
}

// Simple UnitTest suite to test that basics works
fn XoRTest()
{
	let input1 = "01";
	let input2 = "01";
	let mut xor_result = XoROperation(input1, input2);
	assert_eq!(xor_result, "00");

	let input1 = "1c0111001f010100061a024b53535009181c";
	let input2 = "686974207468652062756c6c277320657965";
	let mut xor_result = XoROperation(input1, input2);
	assert_eq!(xor_result, "746865206b696420646f6e277420706c6179");

	println!("All tests pass!");
}

fn main()
{
	XoRTest();
}