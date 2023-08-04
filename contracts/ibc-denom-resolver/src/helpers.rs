use crate::types::Memo;

pub trait OptionRefMemo {
    fn serialize_json(&self) -> String;
}

impl OptionRefMemo for Option<&Memo> {
    fn serialize_json(&self) -> String {
        match self {
            Some(memo) => serde_json_wasm::to_string(memo).unwrap(),
            None => "".to_string(),
        }
    }
}
