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
        Self::from_json(&json_pools)
    }

    pub fn from_json(json: &str) -> Self {
        serde_json::from_str(&json).unwrap()
    }

    pub fn pool_exists(&self, name: &str) -> bool {
       self.0.iter().find(|p| &p.pool == name).is_some()
    }
}