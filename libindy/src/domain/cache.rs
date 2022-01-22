#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PurgeOptions {
    pub max_age: Option<i32>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GetCacheOptions {
    pub no_cache: Option<bool>,     // Skip usage of cache,
    pub no_update: Option<bool>,    // Use only cached data, do not try to update.
    pub no_store: Option<bool>,     // Skip storing fresh data if updated
    pub min_fresh: Option<i32>,     // Return cached data if not older than this many seconds. -1 means do not check age.
}