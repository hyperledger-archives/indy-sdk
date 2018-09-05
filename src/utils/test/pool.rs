use pool::Pool;
use serde_json;

#[derive(Deserialize)]
struct PoolItem {
    pool: String
}

#[derive(Deserialize)]
pub struct PoolList(Vec<PoolItem>);

impl PoolList {
    pub fn new() -> Self {
        let json_pools = Pool::list().unwrap();
        serde_json::from_str(&json_pools).unwrap()
    }

    pub fn pool_exists(&self, name: String) -> bool {
       self.0.iter().find(|p| p.pool == name).is_some()
    }
}