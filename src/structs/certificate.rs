use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Certificate {
    pub issuer_ca_id: u64,
    pub issuer_name: String,
    pub common_name: String,
    pub name_value: String,
    pub id: u64,
    pub entry_timestamp: String,
    pub not_before: String,
    pub not_after: String,
    pub serial_number: String,
    pub result_count: u64,
}
