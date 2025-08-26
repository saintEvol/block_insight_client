use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

pub trait KvStorage {
    fn save(&self, key: &str, value: impl Serialize);
    fn load<T>(&self, key: &str) -> Option<T>
    where
        T: DeserializeOwned;
}
