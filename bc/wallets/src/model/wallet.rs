use super::*;
use serde::export::fmt::Debug;
use std::fmt;

pub mod chain;

#[repr(C)]
pub struct Mnemonic {
    pub status: StatusCode,
    pub mnid: String,
    pub mn: Vec<u8>,
}

#[repr(C)]
#[derive(Default)]
pub struct Wallet {
    pub status: StatusCode,
    pub wallet_id: String,
    pub wallet_type:i64,
    //这个值不会存在Null 的情况
    pub wallet_name: Option<String>,
    pub display_chain_id:i64,
    pub selected:bool,
    pub create_time:String,
    pub eee_chain: Option<chain::EeeChain>,
    pub eth_chain: Option<chain::EthChain>,
    pub btc_chain: Option<chain::BtcChain>,
}

impl fmt::Debug for Wallet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Wallet {{ status: {}, wallet_id: {},wallet_type:{},wallet_name:{:?} }}", self.status.clone() as u32, self.wallet_id,self.wallet_type,self.wallet_name)
    }
}











