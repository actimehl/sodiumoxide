extern crate sodiumoxide;

use sodiumoxide::crypto::asymmetricbox;

fn main() {
    sodiumoxide::init();
  
    let (pk1, sk1) = asymmetricbox::gen_keypair();
    let (pk2, sk2) = asymmetricbox::gen_keypair();
    let nonce = asymmetricbox::gen_nonce();
  
    let msg = "Hello World!";
    println!("Message: {}", msg);
    
    let crypted_msg = asymmetricbox::seal(msg.as_bytes(), &nonce, &pk2, &sk1);
    let decrypted_msg_opt = asymmetricbox::open(&crypted_msg, &nonce, &pk1, &sk2);
  
    match decrypted_msg_opt {
        Some(ref decrypted_msg) =>
            match std::str::from_utf8(&decrypted_msg) {
                Ok(ref decrypted_msg_str) =>
                    println!("Decrypted message: {}", decrypted_msg_str),
                Err(e) =>
                    println!("Decryped text is not valid utf-8!!! {:?}", e),
        },
        None => println!("Decryption failed!!!"),
    }
}

