use cw_storage_plus::Item;
use ibc_denom_resolver::types::Config;

pub const CONFIG: Item<Config> = Item::new("config");
