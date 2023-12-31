#![allow(non_snake_case)]
use libpal::pal as libpal;

fn BreakRepeatingXorKeyTest(input: &str, key: &str)
{
	let output = libpal::RepeatingKeyXor(input, key);

	assert_eq!(libpal::encode_hex(&output), "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2\
		a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b2028\
		3165286326302e27282f");
	
	println!("output: {:?}", output);
	println!("All tests pass!");
}

fn main()
{
	let input = "Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal";
	let key = "ICE";
	BreakRepeatingXorKeyTest(input, key);
}
