#![allow(non_snake_case)]
extern crate base64Tools;
extern crate libpal;

// Simple UnitTest suite to test that basics works
fn HexToBase64Test()
{
	let mut input = "010000";
	let mut base64_result = libpal::HexStringToBase64(input);
	assert_eq!(base64_result, "AQAA");

	let mut input = "0100";
	let mut base64_result = libpal::HexStringToBase64(input);
	assert_eq!(base64_result, "AQA=");

	let mut input = "01";
	let mut base64_result = libpal::HexStringToBase64(input);
	assert_eq!(base64_result, "AQ==");

	input = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
	base64_result = libpal::HexStringToBase64(input);
	assert_eq!(base64_result, "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t");

	println!("All tests pass!");
}

fn main()
{
	HexToBase64Test();
}
