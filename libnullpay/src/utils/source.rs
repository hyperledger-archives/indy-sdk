pub fn to_source(payment_address: &str, seq_no: i32) -> Option<String> {
    let addr_parts: Vec<&str> = payment_address.split(":").collect();

    if addr_parts.len() != 3 {
        return None
    }

    let mut source_parts: Vec<String> = Vec::new();
    source_parts.push(addr_parts.get(0).unwrap().to_string());
    source_parts.push(addr_parts.get(1).unwrap().to_string());
    source_parts.push(seq_no.to_string() + "_" + addr_parts.get(2).unwrap());

    Some(source_parts.join(":"))
}

pub fn from_source(source: &str) -> Option<(i32, String)> {
    let source_parts: Vec<&str> = source.split(":").collect();

    if source_parts.len() != 3 {
        return None
    }

    let last = source_parts.get(2).unwrap();
    let last_parts: Vec<&str> = last.split("_").collect();

    if last_parts.len() != 2 {
        return None
    }

    let seq_no = match last_parts.get(0).unwrap().to_string().parse() {
        Ok(v) => v,
        Err(_) => return None
    };

    let mut recipient_parts = Vec::new();
    recipient_parts.push(source_parts.get(0).unwrap().to_string());
    recipient_parts.push(source_parts.get(1).unwrap().to_string());
    recipient_parts.push(last_parts.get(1).unwrap().to_string());

    Some((seq_no, recipient_parts.join(":")))
}
