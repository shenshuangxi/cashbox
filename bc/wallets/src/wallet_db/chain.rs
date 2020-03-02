use crate::wallet_db::db_helper::DataServiceProvider;
use crate::model::WalletObj;
use log::debug;

impl DataServiceProvider {

    pub fn get_available_chain(){
        //查询当前可用链类型，当前
        let sql = 	 "select id,type from detail.chain where selected=1 and status=1;";
    }

    pub fn display_eee_chain(&self) -> Result<Vec<WalletObj>, String> {

        let all_mn =   "select  e.wallet_id,e.fullname as wallet_name,e.chain_id,e.address,e.selected,e.is_visible as chain_is_visible,f.domain,f.type as chain_type,
			d.digit_id,d.contract_address,d.short_name,d.full_name,d.balance,d.is_visible as digit_is_visible,d.decimals,d.url_img
	 from (select * from Wallet a ,detail.Address b where a.wallet_id=b.wallet_id and b.chain_id in (5,6)) e,
	 ( select * from detail.DigitUseDetail,detail.DigitBase
         where digit_id = id and group_name !='ETH' and group_name !='BTC'
         ) as d,detail.Chain f where e.address_id = d.address_id and e.chain_id = f.id;";

        let mut cursor = self.db_hander.prepare(all_mn).unwrap().cursor();
        let mut tbwallets = Vec::new();

        while let Some(row) = cursor.next().unwrap() {
            let tbwallet = WalletObj {
                wallet_id: row[0].as_string().map(|str| String::from(str)),
                wallet_name: row[1].as_string().map(|str| String::from(str)),
                chain_id: row[2].as_integer(),
                address: row[3].as_string().map(|str| String::from(str)),
                selected: row[4].as_string().map(|value| Self::get_bool_value(value)),
                chain_is_visible: row[5].as_string().map(|value| Self::get_bool_value(value)),
                domain: row[6].as_string().map(|str| String::from(str)),
                chain_type: row[7].as_integer(),
                digit_id: row[8].as_integer(),
                contract_address: row[9].as_string().map(|str| String::from(str)),
                short_name: row[10].as_string().map(|str| String::from(str)),
                full_name: row[11].as_string().map(|str| String::from(str)),
                balance: row[12].as_string().map(|str| String::from(str)),
                digit_is_visible: row[13].as_string().map(|value| Self::get_bool_value(value)),
                decimals: row[14].as_integer(),
                url_img: row[15].as_string().map(|str| String::from(str)),

            };
            tbwallets.push(tbwallet);
        }
        Ok(tbwallets)
    }

    pub fn display_eth_chain(&self) -> Result<Vec<WalletObj>, String> {
      //这个sql语句关联表较多 关联逻辑： 查询出当前钱包有效钱包，这里是不区分主链或者测试链钱包 得到一个子表 e
        // 查询出当前地址使用详情跟代币的关联情况，得到子表 d
        // 最后和表f做自然连接，根据筛选条件找出符合要求的结果
        let all_mn =   "select  e.wallet_id,e.fullname as wallet_name,e.chain_id,e.address,e.selected,e.is_visible as chain_is_visible,f.domain,f.type as chain_type,
			d.digit_id,d.contract_address,d.short_name,d.full_name,d.balance,d.is_visible as digit_is_visible,d.decimals,d.url_img
	 from (select * from Wallet a ,detail.Address b where a.wallet_id=b.wallet_id and a.status=1 and b.status =1 and b.chain_id in (3,4)) e,
	 ( select * from detail.DigitUseDetail a,detail.DigitBase b where a.digit_id = b.id  and b.status = 1 and group_name !='EEE' and group_name !='BTC') as d,detail.Chain f
         where e.address_id = d.address_id and e.chain_id = f.id and f.status = 1;";


        let mut cursor = self.db_hander.prepare(all_mn).unwrap().cursor();
        let mut tbwallets = Vec::new();
        while let Some(row) = cursor.next().unwrap() {
            let tbwallet = WalletObj {
                wallet_id: row[0].as_string().map(|str| String::from(str)),
                wallet_name: row[1].as_string().map(|str| String::from(str)),
                chain_id: row[2].as_integer(),
                address: row[3].as_string().map(|str| String::from(str)),
                selected: row[4].as_string().map(|value| Self::get_bool_value(value)),
                chain_is_visible: row[5].as_string().map(|value| Self::get_bool_value(value)),
                domain: row[6].as_string().map(|str| String::from(str)),
                chain_type: row[7].as_integer(),
                digit_id: row[8].as_integer(),
                contract_address: row[9].as_string().map(|str| String::from(str)),
                short_name: row[10].as_string().map(|str| String::from(str)),
                full_name: row[11].as_string().map(|str| String::from(str)),
                balance: row[12].as_string().map(|str| String::from(str)),
                digit_is_visible: row[13].as_string().map(|value| Self::get_bool_value(value)),
                decimals: row[14].as_integer(),
                url_img: row[15].as_string().map(|str| String::from(str)),

            };
            tbwallets.push(tbwallet);
        }
        Ok(tbwallets)
    }

    pub fn display_btc_chain(&self) -> Result<Vec<WalletObj>, String> {

        let all_mn =   "select  e.wallet_id,e.fullname as wallet_name,e.chain_id,e.address,e.selected,e.is_visible as chain_is_visible,f.domain,f.type as chain_type,
			d.digit_id,d.contract_address,d.short_name,d.full_name,d.balance,d.is_visible as digit_is_visible,d.decimals,d.url_img
	 from (select * from Wallet a ,detail.Address b where a.wallet_id=b.wallet_id and b.chain_id in (1,2)) e,
	 ( select * from detail.DigitUseDetail,detail.DigitBase
         where digit_id = id and group_name !='EEE' and group_name !='ETH'
         ) as d,detail.Chain f where e.address_id = d.address_id and e.chain_id = f.id;";

        let mut cursor = self.db_hander.prepare(all_mn).unwrap().cursor();
        let mut tbwallets = Vec::new();
        debug!("get wallet item is:{}", tbwallets.len());
        while let Some(row) = cursor.next().unwrap() {
            let tbwallet = WalletObj {
                wallet_id: row[0].as_string().map(|str| String::from(str)),
                wallet_name: row[1].as_string().map(|str| String::from(str)),
                chain_id: row[2].as_integer(),
                address: row[3].as_string().map(|str| String::from(str)),
                selected: row[4].as_string().map(|value| Self::get_bool_value(value)),
                chain_is_visible: row[5].as_string().map(|value| Self::get_bool_value(value)),
                domain: row[6].as_string().map(|str| String::from(str)),
                chain_type: row[7].as_integer(),
                digit_id: row[8].as_integer(),
                contract_address: row[9].as_string().map(|str| String::from(str)),
                short_name: row[10].as_string().map(|str| String::from(str)),
                full_name: row[11].as_string().map(|str| String::from(str)),
                balance: row[12].as_string().map(|str| String::from(str)),
                digit_is_visible: row[13].as_string().map(|value| Self::get_bool_value(value)),
                decimals: row[14].as_integer(),
                url_img: row[15].as_string().map(|str| String::from(str)),

            };
            tbwallets.push(tbwallet);
        }
        Ok(tbwallets)
    }

    pub fn show_chain(&mut self, walletid: &str, wallet_type: i64) -> Result<(), String> {
        let sql = "UPDATE Address set is_visible = 1 WHERE wallet_id=? and chain_id=?;";
        match self.db_hander.prepare(sql) {
            Ok(mut stat) => {
                let bind_wallet_id = stat.bind(1, walletid).map_err(|e| format!("show_chain bind walletid,{}", e.to_string()));
                if bind_wallet_id.is_err() {
                    return Err(bind_wallet_id.unwrap_err());
                }

                let bind_wallet_type = stat.bind(2, wallet_type).map_err(|e| format!("show_chain bind wallet_type,{}", e.to_string()));
                if bind_wallet_type.is_err() {
                    return Err(bind_wallet_type.unwrap_err());
                }
                let exec = stat.next().map_err(|e| format!("exec show_chain,{}", e.to_string()));
                if exec.is_err() {
                    return Err(exec.unwrap_err());
                }
                Ok(())
            }
            Err(e) => Err(e.to_string())
        }
    }

    pub fn hide_chain(&mut self, walletid: &str, wallet_type: i64) -> Result<(), String> {
        let sql = "UPDATE Address set is_visible = 0 WHERE wallet_id=? and chain_id=?;";
        match self.db_hander.prepare(sql) {
            Ok(mut stat) => {
                let bind_wallet_id = stat.bind(1, walletid).map_err(|e| format!("hide_chain bind walletid,{}", e.to_string()));
                if bind_wallet_id.is_err() {
                    return Err(bind_wallet_id.unwrap_err());
                }

                let bind_wallet_type = stat.bind(2, wallet_type).map_err(|e| format!("hide_chain bind wallet_type,{}", e.to_string()));
                if bind_wallet_type.is_err() {
                    return Err(bind_wallet_type.unwrap_err());
                }
                let exec = stat.next().map_err(|e| format!("exec hide_chain,{}", e.to_string()));
                if exec.is_err() {
                    return Err(exec.unwrap_err());
                }
                Ok(())
            }
            Err(e) => Err(e.to_string())
        }
    }

    pub fn get_now_chain_type(&mut self, walletid: &str) -> Result<i64, String> {
        let query_sql = "select display_chain_id from Wallet where wallet_id = ?";

        let mut state = self.db_hander.prepare(query_sql).unwrap();
        state.bind(1,walletid).expect("get_now_chain_type is error!");

        match state.cursor().next() {
            Ok(value) => {
                let data = value.unwrap();
                Ok(data[0].as_integer().unwrap())
            }
            Err(e) => Err(e.to_string())
        }
    }

    pub fn set_now_chain_type(&mut self, walletid: &str, chain_type: i64) -> Result<(), String> {
        let sql = "UPDATE Wallet set display_chain_id = ? WHERE wallet_id=?;";

        match self.db_hander.prepare(sql) {
            Ok(mut stat) => {
                let bind_chain_type = stat.bind(1, chain_type).map_err(|e| format!("set_now_chain_type bind wallet_type,{}", e.to_string()));
                if bind_chain_type.is_err() {
                    return Err(bind_chain_type.unwrap_err());
                }

                let bind_wallet_id = stat.bind(2, walletid).map_err(|e| format!("set_now_chain_type bind walletid,{}", e.to_string()));
                if bind_wallet_id.is_err() {
                    return Err(bind_wallet_id.unwrap_err());
                }

                let exec = stat.next().map_err(|e| format!("exec set_now_chain_type,{}", e.to_string()));
                if exec.is_err() {
                    return Err(exec.unwrap_err());
                }
                Ok(())
            }
            Err(e) => Err(e.to_string())
        }
    }
}
