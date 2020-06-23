#[macro_use]
extern crate serde_derive;

use tiny_keccak::Keccak;

mod crypto;
pub mod error;

pub use crypto::{Sr25519, Ed25519, Crypto};

use std::collections::HashMap;

pub trait Keccak256<T> {
    fn keccak256(&self) -> T
        where T: Sized;
}

impl<T> Keccak256<[u8; 32]> for T where T: AsRef<[u8]> {
    fn keccak256(&self) -> [u8; 32] {
        let mut keccak = Keccak::new_keccak256();
        let mut result = [0u8; 32];
        keccak.update(self.as_ref());
        keccak.finalize(&mut result);
        result
    }
}

/// 用于传递账户信息解码后的结果，这里使用默认的单位？
#[derive(Clone, Debug, Default)]
pub struct EeeAccountInfo {
    pub nonce: u32,
    pub refcount: u32,
    pub free: String,
    //为避免java不支持u128类型格式，全部转换为String格式
    pub reserved: String,
    pub misc_frozen: String,
    pub fee_frozen: String,
}

//设计成option，当交易为设置区块时间的交易，就不存在签名
#[derive(Default, Debug)]
pub struct TransferDetail {
    pub index: Option<u32>,
    //签名账户对应的交易nonce
    pub from: Option<String>,
    //签名账户 从哪个账户转出balance
    pub to: Option<String>,
    //目的账户，接收balance的账户
    pub value: Option<u128>,
    pub hash: Option<String>,
    pub timestamp: Option<u64>,
}

pub fn transfer(_mnemonic: &str, _to: &str, _amount: &str, _genesis_hash: &[u8], _index: u32, _runtime_version: u32) -> Result<String, error::Error> {
    unimplemented!()
}

pub fn tx_sign(_mnemonic: &str, _genesis_hash: &[u8], _index: u32, _func_data: &[u8], _version: u32) -> Result<String, error::Error> {
    unimplemented!()
}

pub fn account_info_key(_account_id: &str) -> Result<String, error::Error> {
    unimplemented!()
}

pub fn decode_account_info(_info: &str) -> Result<EeeAccountInfo, error::Error> {
    unimplemented!()
}

// 通知事件数据 使用hex 方式编码的字符串,将通知事件的结果与交易关联起来
pub fn event_decode(_event_data: &str, _blockhash: &str, _account: &str) -> HashMap<u32, bool> {
    unimplemented!()
}

pub fn decode_extrinsics(_extrinsics_json: &str, _target_account: &str) -> Result<HashMap<u32, TransferDetail>, error::Error> {
    unimplemented!()
}

#[test]
fn decode_extrinsics_test() {
    let data = r#"["0x280402000b00adc5647201","0x3d0284d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d0154c32d047e9629b33ad92c12920b60e722ab60aa9138e94a1f98ddd8c4e81f742eb8769f0177a011bcec4d6febf2290121b94ced49642abcbd38aa170f13c48fb60218000400306721211d5404bd9da88e0204360a1a9ab8b87c66c1bc2fcdd37f3c2222cc200b0060b7986c88"]"#;
    match decode_extrinsics(data, "5DAAnrj7VHTznn2AWBemMuyBwZWs6FNFjdyVXUeYum3PTXFy") {
        Ok(res) => {
            for (index, hash) in &res {
                println!("index:{},tx hash is:{:?}", index, hash);
            }
        }
        Err(msg) => println!("error info {}", msg)
    }
}

#[test]
fn data_decode_test() {
    let encode_event = r#"0x0800000000000000102700000101000001000000000103050340420f00000100"#;
    // let encode_event = r#"0x140000000000000010270000010100000100000000038e50224acdcc7591c0e9a28ab7d91c099574f8d47f3a35a542dbaebf1ec5f30c00000100000002008e50224acdcc7591c0e9a28ab7d91c099574f8d47f3a35a542dbaebf1ec5f30c00c06e31d910010000000000000000000000010000000202ea990fcb7e9600adcf735a69d7bbcbaa62e52d2eb26125381379a91558fe083b8e50224acdcc7591c0e9a28ab7d91c099574f8d47f3a35a542dbaebf1ec5f30c00c06e31d91001000000000000000000000001000000000040420f00000100"#;
    let res = event_decode(encode_event, "", "");
    println!("res detail:{:?}", res);
}



