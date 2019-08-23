#[serde(rename_all = "camelCase")]
#[derive(Debug, Deserialize, Serialize)]
pub struct PurgeOptions {
    pub max_age: Option<i32>,
}

#[serde(rename_all = "camelCase")]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct GetCacheOptions {
    pub no_cache: Option<bool>,     // Skip usage of cache,
    pub no_update: Option<bool>,    // Use only cached data, do not try to update.
    pub no_store: Option<bool>,     // Skip storing fresh data if updated
    pub min_fresh: Option<i32>,     // Return cached data if not older than this many seconds. -1 means do not check age.
}