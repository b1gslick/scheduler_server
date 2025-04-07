use chrono::prelude::*;
use tracing::info;

extern crate redis;
use redis::Commands;
use redis_pool::{RedisPool, SingleRedisPool};

#[derive(Clone)]
pub struct CacheStore {
    pub pool: SingleRedisPool,
}

impl CacheStore {
    pub async fn new(db_url: &str) -> Result<Self, redis::RedisError> {
        info!("{}", db_url);
        let client = redis::Client::open(db_url).unwrap();
        let pool = RedisPool::from(client);
        Ok(CacheStore { pool })
    }

    pub async fn set_value(
        &mut self,
        key_name: String,
        value: DateTime<Utc>,
    ) -> Result<(), redis::RedisError> {
        self.pool
            .get_connection()
            .unwrap()
            .set(key_name, value.timestamp_nanos_opt().unwrap())
    }

    pub async fn get_value(&mut self, key_name: String) -> Result<i64, redis::RedisError> {
        self.pool.get_connection().unwrap().get(key_name)
    }

    pub async fn delete_value(&mut self, key_name: String) -> Result<(), redis::RedisError> {
        self.pool.get_connection().unwrap().del(key_name)
    }
}
