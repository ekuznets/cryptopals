#![allow(non_snake_case)]
use libpal::pal as libpal;

const BLOCK_SIZE: usize = 16;

fn Produce_Padding_Bits(padding_needed: usize) -> Vec<u8>
{
	let mut padding_bits = Vec::new();
	// Number of bits needed is also the value of each padding bit
	padding_bits.resize(padding_needed, padding_needed as u8);
	return padding_bits;
}

// this functions does calculation of how many padding bits are needed
// then it will generate and return them
// User of this API is responsible to append padding bits to the message
fn Produce_PKCS7_Padding(input: &Vec<u8>) -> Vec<u8>
{
	let mut padding_bits: Vec<u8> = Vec::new();
	let padding_needed = BLOCK_SIZE - (input.len() & 15); // HACK
	let padding_bits = Produce_Padding_Bits(padding_needed);
	return padding_bits;
}

fn Strip_PKCS7_Padding(input: &mut Vec<u8>) -> Vec<u8>
{
	let padding_value: usize = input[input.len()-1].into();
	input.truncate(input.len() - padding_value);
	return input.to_vec();
}

// When using AES128 "YELLOW SUBMARINE" text will be padded to two blocks
// This is because it is already 16 bytes and in which case extra 16 will be added.
fn Test_Proper()
{
	let text = "YELLOW SUBMARINE";
	let padding = Produce_PKCS7_Padding(&text.as_bytes().to_vec());
	let text_final = libpal::u8VecToString(&[text.as_bytes(), &padding].concat());
	println!("Padding: {}", libpal::u8VecToString(&padding));
	println!("Padded message: {}", text_final);
	assert_eq!(padding.len(), BLOCK_SIZE);
	assert_eq!(text_final.len(), BLOCK_SIZE*2);
	assert_eq!(text_final, "YELLOW SUBMARINE\x10\x10\x10\x10\x10\x10\x10\x10\x10\x10\x10\x10\x10\x10\x10\x10");
	
}

// This test is needed only to prove that example in the exercise is working
fn Test_Improper()
{
	let text = "YELLOW SUBMARINE";
	let padding = Produce_Padding_Bits(4);
	let text_final = libpal::u8VecToString(&[text.as_bytes(), &padding].concat());
	println!("Padding: {}", libpal::u8VecToString(&padding));
	println!("Padded message: {}", text_final);
	assert_eq!(padding.len(), padding.len());
	assert_eq!(text_final.len(), text.len() + padding.len());
	assert_eq!(text_final, "YELLOW SUBMARINE\x04\x04\x04\x04");
}

// This test is needed only to prove that example in the exercise is working
fn Test_Inverse()
{
	let text = "YELLOW SUBMARINE";
	let mut padding = Produce_PKCS7_Padding(&text.as_bytes().to_vec());
	let mut padded_text = libpal::StrToU8Vec(text);
	padded_text.append(&mut padding);
	padded_text = Strip_PKCS7_Padding(&mut padded_text);
	// After adding and string padding, text should remain the same
	assert_eq!(libpal::u8VecToString(&padded_text), text);
}

fn main()
{
	Test_Proper();
	Test_Improper();
	Test_Inverse();
}
