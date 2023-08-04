use cw_storage_plus::Item;

use crate::types::Config;

pub const CONFIG: Item<Config> = Item::new("config");
