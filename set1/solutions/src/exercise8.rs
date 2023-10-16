#![allow(non_snake_case)]
use libpal::pal as libpal;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashSet;

// TODO: This functions could be extanded since it needs to read Hex data from file
// but now we read each hex character as a byte but would be much better to
// combine to hex to produce a byte. I will keep this for future work.
fn ReadRawFile(file_path: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let mut output_data: Vec<u8> = Vec::new();
    for line in reader.lines() {
        output_data.extend(libpal::StrToU8Vec(&line.unwrap()));
    }
    Ok(output_data)
}

/*
	How this attack works:
	Assumptions: when dealing with AES in ECB mode we know that the same input will produce the same output
	So we will be reading files and track hashes of each block from the file as an entry.
	We assuming that file will have two itentical entries that are encrypted.
	Once we process every block then we will end up with matching encrypted blocks.
	Thus we can assume we detected ECB mode.
*/
fn main()
{
	// Since we are trying to crack AES in ECB we settle for AES to be 128bits = 16 bytes
	let BLOCK_SIZE = 16;
	let rawData = ReadRawFile("solutions/data/file8.txt");
	let data = rawData.unwrap(); // unwraping moves data
	println!("Data len {} bytes", data.len()/2);
	let mut uniqueBlocks = HashSet::new(); // storing unique bloack so we can compare for duplicates 

	// Since data is in Hex format which means 4 bits per symbol then we have to multiply by 2 to make 32hex=16 bytes block size
	for i in (0..data.len()).step_by(BLOCK_SIZE*2)
	{
		let block = &data[i..i+BLOCK_SIZE*2];
		let res = uniqueBlocks.insert(block);
		if !res
		{
			println!("Found repeated block {:?} num {} out of {}", block, i / BLOCK_SIZE, data.len() / BLOCK_SIZE);
		}
	}
}
