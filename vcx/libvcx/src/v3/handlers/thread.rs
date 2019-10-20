pub struct Thread {
    pub thid: String,
    pub pthid: String,
    pub sender_order: u64,
    pub received_orders: HashMap<String, u64>
}