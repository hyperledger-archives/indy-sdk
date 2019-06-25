fn prep(wallet_handle: i32, sender_vk: &str, receipt_vk: &str) {
    let mut file = File::create(FILE).unwrap();

    println!("Enter message");
    let mut message = String::new();
    io::stdin().read_line(&mut message).unwrap();

    let encrypted_msg = crypto::auth_crypt(wallet_handle, &sender_vk, &receipt_vk, message.trim().as_bytes()).wait().unwrap();
    file.write_all(&encrypted_msg).unwrap();
}
