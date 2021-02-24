use std::iter;
use aes::cipher::generic_array::GenericArray;
use aes::cipher::{BlockCipher, NewBlockCipher};
use aes::Aes256;

pub fn decrypt_packet(buf : &mut [u8]) -> &'static str {
	let mut v3 = vec![];
	//let mut result = vec![];
	let size = buf.len();

	if (size < 32) {
		let sbuf = &buf[..size];
		let s2buf = vec![0; 32 - size];
		v3.extend_from_slice(sbuf);
		v3.extend_from_slice(&s2buf);

	}
	else if size == 32 {
		let sbuf = &buf[..size];
		v3.extend_from_slice(sbuf);
	}
	else {
		let sbuf = &buf[0..32];
		let sbuf2 = &buf[32..64];
		v3.extend_from_slice(sbuf);
		v3.extend_from_slice(&sbuf2);
		return "Nope";
	}

	println!("{:?}", v3);
	println!("Len v3 {}", v3.len());
	let mut block = GenericArray::clone_from_slice(&v3[0..32]);

    // key & cipher
    let key = GenericArray::from_slice(&[1, 2, 3, 4, 5, 6, 7, 8, 1, 2, 3, 4, 5, 6, 7, 8, 1, 2, 3, 4, 5, 6, 7, 8, 1, 2, 3, 4, 5, 6, 7, 8]);
    let cipher = Aes256::new(&key);

    // AES decrypt
    cipher.decrypt_block(&mut block);

	println!("{:?}", v3);

	return "Decrypt Packet: Done";
}

fn decrypt_slice32(buf : &mut [u8]) -> &[u8] {
	return buf;
}
