use std::iter;
use std::convert::TryInto;

use openssl::aes::{AesKey, KeyError, aes_ige};
use openssl::symm::Mode;
use hex::{FromHex, ToHex};


pub fn decrypt_pack(buf : &mut [u8]) -> &'static str {
	return "Hello world";
}
