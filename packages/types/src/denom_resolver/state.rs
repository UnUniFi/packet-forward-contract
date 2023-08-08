use cw_storage_plus::Item;

use super::types::Config;

pub const CONFIG: Item<Config> = Item::new("config");
