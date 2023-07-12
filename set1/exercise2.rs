#![allow(non_snake_case)]
extern crate base64Tools;
extern crate libpal;

fn XoRTest()
{
	let input1 = "00";
	let input2 = "01";
	let xor_result = libpal::XoRHexStringOperation(input1, input2);
	assert_eq!(xor_result, "01");

	let input1 = "01";
	let input2 = "01";
	let xor_result = libpal::XoRHexStringOperation(input1, input2);
	assert_eq!(xor_result, "00");

	let input1 = "1c0111001f010100061a024b53535009181c";
	let input2 = "686974207468652062756c6c277320657965";
	let xor_result = libpal::XoRHexStringOperation(input1, input2);
	assert_eq!(xor_result, "746865206b696420646f6e277420706c6179");

	println!("All tests pass!");
}

fn main()
{
	XoRTest();
}
