#[warn(non_snake_case)]
pub mod base64Tools;
pub mod pal;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn Test_DecryptAES128ECB_SUCCESS() {
		let test_str:Result<Vec<u8>, std::num::ParseIntError> = pal::decode_hex("9D235990830654F0997B73572C56B9A960FA36707E45F499DBA0F25B922301A5");
		let key_str: &str = "YELLOW SUBMARINE";
        let decrypt_msg = pal::DecryptAES128ECB(test_str.unwrap().as_slice(), key_str.as_bytes());
        println!("Decrypted msg: {}", pal::u8VecToString(&decrypt_msg));
    }

	#[test]
	#[should_panic]
	fn Test_DecryptAES128ECB_BAD_TEXT() {
		let test_str:Result<Vec<u8>, std::num::ParseIntError> = pal::decode_hex("9D235990");
		let key_str: &str = "YELLOW SUBMARINE";
        let decrypt_msg = pal::DecryptAES128ECB(test_str.unwrap().as_slice(), key_str.as_bytes());
    }

	#[test]
	#[should_panic]
	fn Test_DecryptAES128ECB_BAD_KEY() {
		let test_str:Result<Vec<u8>, std::num::ParseIntError> = pal::decode_hex("9D235990830654F0997B73572C56B9A960FA36707E45F499DBA0F25B922301A5");
		let key_str: &str = "BadKey";
        let decrypt_msg = pal::DecryptAES128ECB(test_str.unwrap().as_slice(), key_str.as_bytes());
    }
}



