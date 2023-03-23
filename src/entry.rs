use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OrdDomain {
    // pub s_id: usize,
    pub wallet_id: String,
    pub dom_name: String,
    pub dom_type: String,
    pub dom_state: i32,
    pub inscribe_id: String,
    pub tx_hash: String,
    // pub img_url: String,
    pub expire_time: i64,
    pub cost_fee: f64,
    pub out_wallet: String,
    pub fee_rate: i64,
    pub create_time: i64,
    pub update_time: i64,
}