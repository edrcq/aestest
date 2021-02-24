mod protosec;

use hex;
use protosec::decrypt_packet;

fn main() {
    println!("Hello, world!");
    let encryptedNoIV = "b090c2f1e0d78b47b5a8672d3a169634";
    let mut ebuf = hex::decode(encryptedNoIV);
    match ebuf {
        Ok(mut u8arr) => {
            println!("{}", decrypt_packet(&mut u8arr));
        }
        Err(e) => {
            eprintln!("{}", e.to_string());
        }
    }
    let mut buf = [0u8; 4];
    let mut buf2 = [0u8; 29];
    let mut buf3 = [0u8; 32];
    let mut buf4 = [1u8; 512];
    //println!("{}", decrypt_packet(&mut ebuf));
    println!("{}", protosec::decrypt_packet(&mut buf2));
    //println!("{}", protosec::decrypt_packet(&mut buf3));
    //println!("{}", protosec::decrypt_packet(&mut buf4));
}
