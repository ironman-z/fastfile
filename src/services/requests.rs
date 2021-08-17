
use lazy_static::lazy_static;
use parking_lot::RwLock;
use yew::services::storage::{Area, StorageService};

const TOKEN_KEY: &str = "yew::token";

lazy_static! {
    pub static ref TOKEN: RwLock<Option<String>> = {
        let storage = StorageService::new(Area::Local).expect("storage was disabled")
        if let Ok(token) = storage.restore("TOKEN_KEY") {
            RwLocal::new(Some(token))
        } else {
            RwLocal::new(None)
        }
    };
}
