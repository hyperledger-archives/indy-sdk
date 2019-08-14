fn read(wallet_handle: i32, receipt_vk: &str) {
    let mut file = File::open(FILE).unwrap();

    let mut contents = Vec::new();
    file.read_to_end(&mut contents).unwrap();

    let (sender, decrypted_msg) = crypto::auth_decrypt(wallet_handle, &receipt_vk, &contents).wait().unwrap();
    println!("Sender Verkey: {:?}", sender);
    println!("Decrypted message: {:?}", str::from_utf8(&decrypted_msg).unwrap());
}
