use std::sync::RwLock;
use dioxus::logger::tracing::error;
use once_cell::sync::Lazy;
use crate::storage::kv_storage::KvStorage;
use serde::Serialize;
use serde::de::DeserializeOwned;

pub static LOCAL_STORAGE: Lazy<LocalStorageProvider> = Lazy::new(|| {
    LocalStorageProvider
});

pub trait LocalStorage: Serialize + DeserializeOwned {
    fn key() -> &'static str;
    fn save(&self, ) {
        LOCAL_STORAGE.save(Self::key(), serde_json::to_string(self).unwrap())
    }

    fn load() -> Option<Self> {
        let ret = LOCAL_STORAGE.load::<String>(Self::key());
        if let Some(ret) = ret {
            match serde_json::from_str(&ret) {
                Ok(ret) => Some(ret),
                Err(e) => {
                    error!("加载本地数据: {}时出错: {:?}", Self::key(), e);
                    None
                }
            }
        } else {
            None
        }

    }
}

// #[cfg(target_arch = "wasm32")]
#[derive(Copy, Clone)]
pub struct LocalStorageProvider;

#[cfg(not(target_arch = "wasm32"))]
impl KvStorage for LocalStorageProvider {
    fn save(&self, key: &str, value: impl Serialize) {
        todo!("save local storage")
    }

    fn load<T>(&self, key: &str) -> Option<T>
    where
        T: DeserializeOwned
    {
        todo!("load local storage")
    }
}

impl LocalStorageProvider {
    pub fn use_context_provider() -> Self {
        dioxus::prelude::use_context_provider(|| LocalStorageProvider)
    }

    pub fn use_context() -> Self {
        dioxus::prelude::use_context()
    }
}

#[cfg(target_arch = "wasm32")]
impl KvStorage for LocalStorageProvider {
    fn save(&self, key: &str, value: impl Serialize) {
        use web_sys::window;
        let storage = window()
            .unwrap()
            .local_storage()
            .expect("failed to get local_storage")
            .unwrap();
        let value = serde_json::to_string(&value).expect("failed to serialize value");
        storage
            .set_item(key, value.as_str())
            .expect("failed to set item");
    }

    fn load<T>(&self, key: &str) -> Option<T>
    where
        T: DeserializeOwned,
    {
        use web_sys::window;
        let storage = window()
            .unwrap()
            .local_storage()
            .expect("failed to get local_storage")
            .unwrap();
        let ret = storage.get_item(key).expect("failed to get key");
        if let Some(ret) = ret {
            Some(serde_json::from_str::<T>(&ret).expect("failed to deserialize value"))
        } else {
            None
        }

    }
}
