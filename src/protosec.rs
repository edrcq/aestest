use std::iter;
use aes::cipher::generic_array::GenericArray;
use aes::cipher::{BlockCipher, NewBlockCipher};
use aes::Aes256;
use std::convert::TryInto;
use std::convert::AsMut;

// Thanks to @malbarbo we can use this helper function (https://stackoverflow.com/a/37679019/147192)
fn clone_into_array<A, T>(slice: &[T]) -> A
    where A: Sized + Default + AsMut<[T]>,
          T: Clone
{
    let mut a = Default::default();
    <A as AsMut<[T]>>::as_mut(&mut a).clone_from_slice(slice);
    a
}

pub fn decrypt_packet(buf : &mut [u8]) -> &'static str {
	let mut v3: Vec<&[u8; 16]> = Vec::new();
	let mut vu8 = split_packet(buf);
	//let mut result = vec![];
	let count = v3.len();
	let mut i = 0;
	while i < count {
		if i < v3.len() {
			let parr: [u8; 16] = *v3[i];
			let mut genAr = GenericArray::new();
			genAr.clone_from(parr);
			//let mut block = GenericArray::clone_from(&parr[0..16]);
		}
		i += 1;
	}



    // key & cipher
    let key = GenericArray::from_slice(&[1, 2, 3, 4, 5, 6, 7, 8, 1, 2, 3, 4, 5, 6, 7, 8, 1, 2, 3, 4, 5, 6, 7, 8, 1, 2, 3, 4, 5, 6, 7, 8]);
    let cipher = Aes256::new(&key);

	// println!("{:?}", v3);
	// println!("Len v3 {}", v3.len());
	//let mut block = GenericArray::clone_from_slice(&v3[0..16]);
    // AES decrypt
	// cipher.decrypt_block(&mut block);

	// let u8b: [u8; 32] = block.as_slice().try_into().unwrap();

	// vu8.extended_from_slice(u8b)
	// std str from utf8 (vu8)

	// println!("{:?}", u8b);

	return "Decrypt Packet: Done";
}

/**
 * Function to split buffer into a Vec of buffer(16)
 */
fn split_packet(buf : &mut [u8]) -> Vec<&[u8]> {
	let mut blocks = Vec::new();

	if buf.len() % 16 != 0 {
		println!("Bad split. Panic soon");
	}
	let count = buf.len() / 16;
	let mut i = 0;
	while i < count {
		let start = 16 * i;
		let endy = 16 * (i + 1);
		let ar = &buf[start..endy];
		//let mut block = GenericArray::clone_from_slice(&ar[0..16]);
		blocks.push(ar);
		i += 1;
	}

	return blocks;
}

fn decrypt_slice32(buf : &mut [u8]) -> &[u8] {
	return buf;
}
