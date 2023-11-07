
#![allow(non_snake_case)]
use libpal::pal as libpal;

fn main()
{
    const BLOCK_SIZE: usize = 16;
	// Extract data from file
	let content = libpal::ReadAndDecodeFileByLine("solutions/data/file10.txt");
	if content.is_err()
	{
		println!("Error reading file: {}", content.err().unwrap());
		return;
	}

	let cipher_text = content.unwrap();
    println!("Size of the cipher_text {}", cipher_text.len());

    let key_str: &str = "YELLOW SUBMARINE";
    let mut plain_text: Vec<u8> = Vec::new();
    let mut IV_vec = vec![0; BLOCK_SIZE];

    // Decrypting logic with CBC
    for i in (0..cipher_text.len()).step_by(BLOCK_SIZE)
    {
        let decrypted_block = libpal::DecryptAES128ECB(&cipher_text[i..i+BLOCK_SIZE], key_str.as_bytes());
        let mut plain_text_block = libpal::XoRHexVectorOperation(&decrypted_block, &IV_vec);
        IV_vec = cipher_text[i..i+BLOCK_SIZE].to_vec(); // IV for the next block is the cipher_block. First block can be found by only Key and IV
        plain_text.append(&mut plain_text_block);
    }

    println!("PlainText: {:?}", libpal::u8VecToString(&plain_text));

}