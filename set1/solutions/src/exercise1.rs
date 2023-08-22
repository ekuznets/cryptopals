#![allow(non_snake_case)]
use libpal::pal as libpal;

// Simple UnitTest suite to test that basics works
fn HexToBase64Test()
{
	{
		let input = "010000";
		let base64_result = libpal::HexStringToBase64(input);
		assert_eq!(base64_result, "AQAA");
	}

	{
		let input = "0100";
		let base64_result = libpal::HexStringToBase64(input);
		assert_eq!(base64_result, "AQA=");
	}

	{
		let input = "01";
		let base64_result = libpal::HexStringToBase64(input);
		assert_eq!(base64_result, "AQ==");
	}

	let input = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
	let base64_result = libpal::HexStringToBase64(input);
	assert_eq!(base64_result, "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t");

	println!("All tests pass!");
}

fn main()
{
	HexToBase64Test();
}
