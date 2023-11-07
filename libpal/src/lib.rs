#[warn(non_snake_case)]
pub mod base64Tools;
pub mod pal;

#[cfg(test)]
mod tests {
    use super::*;
	#[test]
	fn Test_EncryptDecrypt_AES128ECB_SAMETEXT() {
		let text: &str = "Zalupa1234567890";
		let key_str: &str = "YELLOW SUBMARINE";
		let ecnrypt_message = pal::EncryptAES128ECB(text.as_bytes(), key_str.as_bytes());
        let decrypt_msg = pal::DecryptAES128ECB(ecnrypt_message.as_slice(), key_str.as_bytes());

		assert_eq!(text.as_bytes(), decrypt_msg.as_slice());
        println!("Encrypted and then decrypted msg: {}", pal::u8VecToString(&decrypt_msg));
    }

	#[test]
	fn Test_EncryptDecrypt_AES128CBC_LARGETEXT_SAMETEXT()
	{
		const BLOCK_SIZE : usize = 16;
		let text: &str = "This text will be really long and juicy so it will take an effort for this computer to break it!";
		let key_str: &str = "YELLOW SUBMARINE";
		let IV: &str = "1111111111111111";
		
		let mut cipher_text: Vec<u8> = Vec::new();
		let mut IV_vec = pal::StrToU8Vec(IV);

		// Encrypting logic with CBC
		for i in (0..text.len()).step_by(BLOCK_SIZE)
		{
			let xored_block = pal::XoRHexVectorOperation(text[i..i+BLOCK_SIZE].as_bytes(), &IV_vec);
			let mut cipher_block = pal::EncryptAES128ECB(&xored_block, key_str.as_bytes());
			IV_vec = cipher_block.clone(); // IV for next block is resulted cipher_block of this operation. TODO: This could be better done for performance to be kept in mind, do I have to clone ??
			cipher_text.append(&mut cipher_block);
		}

		let mut plain_text: Vec<u8> = Vec::new();
		IV_vec = pal::StrToU8Vec(IV);

		// Decrypting logic with CBC
		for i in (0..cipher_text.len()).step_by(BLOCK_SIZE)
		{
			let decrypted_block = pal::DecryptAES128ECB(&cipher_text[i..i+BLOCK_SIZE], key_str.as_bytes());
			let mut plain_text_block = pal::XoRHexVectorOperation(&decrypted_block, &IV_vec);
			IV_vec = cipher_text[i..i+BLOCK_SIZE].to_vec(); // IV for the next block is the cipher_block. First block can be found by only Key and IV
			plain_text.append(&mut plain_text_block);
		}

		println!("Encrypted and then decrypted msg: {}", pal::u8VecToString(&plain_text));
		assert_eq!(text.as_bytes(), plain_text.as_slice());

    }

}



