pub fn to_utxo(payment_address: &str, seq_no: i32) -> Option<String> {
    let addr_parts: Vec<&str> = payment_address.split(":").collect();

    if addr_parts.len() != 3 {
        return None
    }

    let mut utxo_parts: Vec<String> = Vec::new();
    utxo_parts.push(addr_parts.get(0).unwrap().to_string());
    utxo_parts.push(addr_parts.get(1).unwrap().to_string());
    utxo_parts.push(seq_no.to_string() + "_" + addr_parts.get(2).unwrap());

    Some(utxo_parts.join(":"))
}

pub fn from_utxo(utxo: &str) -> Option<(i32, String)> {
    let utxo_parts: Vec<&str> = utxo.split(":").collect();

    if utxo_parts.len() != 3 {
        return None
    }

    let last = utxo_parts.get(2).unwrap();
    let last_parts: Vec<&str> = last.split("_").collect();

    if last_parts.len() != 2 {
        return None
    }

    let seq_no = match last_parts.get(0).unwrap().to_string().parse() {
        Ok(v) => v,
        Err(_) => return None
    };

    let mut address_parts = Vec::new();
    address_parts.push(utxo_parts.get(0).unwrap().to_string());
    address_parts.push(utxo_parts.get(1).unwrap().to_string());
    address_parts.push(last_parts.get(1).unwrap().to_string());

    Some((seq_no, address_parts.join(":")))
}
