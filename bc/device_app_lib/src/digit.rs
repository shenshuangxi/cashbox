/// Expose the JNI interface for android below
#[cfg(target_os = "android")]
#[allow(non_snake_case)]
pub mod android {
    use jni::JNIEnv;
    use jni::objects::{JObject, JValue, JClass, JString};
    use jni::sys::{jobject, jint, jboolean};
    use wallets::{StatusCode, WalletError};
    use wallets::model::DigitList;

    #[no_mangle]
    #[allow(non_snake_case)]
    pub unsafe extern "C" fn Java_info_scry_wallet_1manager_NativeLib_showDigit(env: JNIEnv, _: JClass, walletId: JString, chainId: jint, digitId: JString) -> jobject {
        let wallet_id: String = env.get_string(walletId).unwrap().into();
        let digit_id: String = env.get_string(digitId).unwrap().into();

        let wallet_state_class = env.find_class("info/scry/wallet_manager/NativeLib$WalletState").expect("find NativeLib$WalletState");
        let state_obj = env.alloc_object(wallet_state_class).expect("create wallet_state_class instance ");
        let eth = wallets::module::Ethereum {};
        match eth.show_digit(wallet_id.as_str(), chainId as i64, digit_id.as_str()) {
            Ok(_code) => {
                env.set_field(state_obj, "status", "I", JValue::Int(StatusCode::OK as i32)).expect("find status type");
                env.set_field(state_obj, "isShowDigit", "Z", JValue::Bool(1 as u8)).expect("showDigit value");
            }
            Err(msg) => {
                env.set_field(state_obj, "isShowDigit", "Z", JValue::Bool(0 as u8)).expect("showDigit value");
                env.set_field(state_obj, "message", "Ljava/lang/String;", JValue::Object(JObject::from(env.new_string(msg.to_string()).unwrap()))).expect("set error msg value ");
            }
        }
        *state_obj
    }

    #[no_mangle]
    #[allow(non_snake_case)]
    pub unsafe extern "C" fn Java_info_scry_wallet_1manager_NativeLib_hideDigit(env: JNIEnv, _: JClass, walletId: JString, chainId: jint, digitId: JString) -> jobject {
        let wallet_id: String = env.get_string(walletId).unwrap().into();
        // let chain_id: String = env.get_string(chainId).unwrap().into();
        let digit_id: String = env.get_string(digitId).unwrap().into();

        let wallet_state_class = env.find_class("info/scry/wallet_manager/NativeLib$WalletState").expect("find wallet_state_class ");
        let state_obj = env.alloc_object(wallet_state_class).expect("create wallet_state_class instance ");
        let eth = wallets::module::Ethereum {};
        match eth.hide_digit(wallet_id.as_str(), chainId as i64, digit_id.as_str()) {
            Ok(_) => {
                env.set_field(state_obj, "status", "I", JValue::Int(StatusCode::OK as i32)).expect("find status type ");
                env.set_field(state_obj, "isHideDigit", "Z", JValue::Bool(1 as u8)).expect("hideDigit value ");
            }
            Err(msg) => {
                env.set_field(state_obj, "isHideDigit", "Z", JValue::Bool(0 as u8)).expect("hideDigit value ");
                env.set_field(state_obj, "message", "Ljava/lang/String;", JValue::Object(JObject::from(env.new_string(msg.to_string()).unwrap()))).expect("set error msg value ");
            }
        }
        *state_obj
    }

    #[no_mangle]
    #[allow(non_snake_case)]
    pub unsafe extern "C" fn Java_info_scry_wallet_1manager_NativeLib_addDigit(env: JNIEnv, _: JClass, walletId: JString, chainId: jint, digit_id: JString) -> jobject {
        let wallet_id: String = env.get_string(walletId).unwrap().into();
        let digit_id: String = env.get_string(digit_id).unwrap().into();

        let wallet_state_class = env.find_class("info/scry/wallet_manager/NativeLib$WalletState").expect("find wallet_state_class ");
        let state_obj = env.alloc_object(wallet_state_class).expect("create wallet_state_class instance ");
        let eth = wallets::module::Ethereum {};
        match eth.add_wallet_digit(&wallet_id, chainId as i64, &digit_id) {
            Ok(_) => {
                env.set_field(state_obj, "status", "I", JValue::Int(StatusCode::OK as i32)).expect("find status type ");
                env.set_field(state_obj, "isAddDigit", "Z", JValue::Bool(1 as u8)).expect("showDigit value ");
            }
            Err(msg) => {
                env.set_field(state_obj, "isAddDigit", "Z", JValue::Bool(0 as u8)).expect("showDigit value ");
                env.set_field(state_obj, "message", "Ljava/lang/String;", JValue::Object(JObject::from(env.new_string(msg.to_string()).unwrap()))).expect("set error msg value ");
            }
        }
        *state_obj
    }

    #[no_mangle]
    #[allow(non_snake_case)]
    pub unsafe extern "C" fn Java_info_scry_wallet_1manager_NativeLib_updateDigitBalance(env: JNIEnv, _: JClass, address: JString, digitId: JString, balance: JString) -> jobject {
        let address: String = env.get_string(address).unwrap().into();
        let digitId: String = env.get_string(digitId).unwrap().into();
        let balance: String = env.get_string(balance).unwrap().into();

        let wallet_state_class = env.find_class("info/scry/wallet_manager/NativeLib$WalletState").expect("find wallet_state_class ");
        let state_obj = env.alloc_object(wallet_state_class).expect("create wallet_state_class instance ");
        let eth = wallets::module::Ethereum {};
        match eth.update_balance(&address, &digitId, &balance) {
            Ok(_) => {
                env.set_field(state_obj, "status", "I", JValue::Int(StatusCode::OK as i32)).expect("set status value ");
                env.set_field(state_obj, "isUpdateDigitBalance", "Z", JValue::Bool(1 as u8)).expect("showDigit value ");
            }
            Err(msg) => {
                env.set_field(state_obj, "status", "I", JValue::Int(StatusCode::DylibError as i32)).expect("set status value ");
                env.set_field(state_obj, "message", "Ljava/lang/String;", JValue::Object(JObject::from(env.new_string(msg.to_string()).unwrap()))).expect("set error msg value ");
            }
        }
        *state_obj
    }

    #[no_mangle]
    #[allow(non_snake_case)]
    pub unsafe extern "C" fn Java_info_scry_wallet_1manager_NativeLib_updateDefaultDigitList(env: JNIEnv, _: JClass, digit_data: JString) -> jobject {
        let digit_data: String = env.get_string(digit_data).unwrap().into();
        let wallet_state_class = env.find_class("info/scry/wallet_manager/NativeLib$WalletState").expect("find wallet_state_class ");
        let state_obj = env.alloc_object(wallet_state_class).expect("create wallet_state_class instance ");


        let digits = serde_json::from_slice::<Vec<wallets::model::DefaultDigit>>(digit_data.as_bytes());
        if let Ok(digits) = digits {
            let eth = wallets::module::Ethereum {};
            match eth.update_default_digit(digits) {
                Ok(_) => {
                    env.set_field(state_obj, "status", "I", JValue::Int(StatusCode::OK as i32)).expect("set status value ");
                    env.set_field(state_obj, "isUpdateDefaultDigit", "Z", JValue::Bool(1 as u8)).expect("showDigit value ");
                }
                Err(msg) => {
                    env.set_field(state_obj, "status", "I", JValue::Int(StatusCode::DylibError as i32)).expect("set status value ");
                    env.set_field(state_obj, "message", "Ljava/lang/String;", JValue::Object(JObject::from(env.new_string(msg.to_string()).unwrap()))).expect("set error msg value ");
                }
            }
        } else {
            env.set_field(state_obj, "status", "I", JValue::Int(StatusCode::DylibError as i32)).expect("set status value ");
            env.set_field(state_obj, "message", "Ljava/lang/String;", JValue::Object(JObject::from(env.new_string(digits.unwrap_err().to_string()).unwrap()))).expect("set error msg value ");
        }

        *state_obj
    }

    #[no_mangle]
    #[allow(non_snake_case)]
    pub unsafe extern "C" fn Java_info_scry_wallet_1manager_NativeLib_updateAuthDigitList(env: JNIEnv, _: JClass, digit_data: JString) -> jobject {
        let digit_data: String = env.get_string(digit_data).unwrap().into();
        let wallet_state_class = env.find_class("info/scry/wallet_manager/NativeLib$WalletState").expect("find wallet_state_class ");
        let state_obj = env.alloc_object(wallet_state_class).expect("create wallet_state_class instance ");
        let _ = serde_json::from_slice::<Vec<wallets::model::EthToken>>(digit_data.as_bytes())
            .map_err(|err| WalletError::Serde(err))
            .and_then(|digits| {
                let eth = wallets::module::Ethereum {};
                eth.update_auth_digit(digits, true, None).map_err(|err| err.into())
            })
            .map(|_data| {
                env.set_field(state_obj, "status", "I", JValue::Int(StatusCode::OK as i32)).expect("set status value ");
                env.set_field(state_obj, "isUpdateAuthDigit", "Z", JValue::Bool(1 as u8)).expect("showDigit value ");
            })
            .map_err(|err| {
                env.set_field(state_obj, "status", "I", JValue::Int(StatusCode::DylibError as i32)).expect("set status value ");
                env.set_field(state_obj, "message", "Ljava/lang/String;", JValue::Object(JObject::from(env.new_string(err.to_string()).unwrap()))).expect("set error msg value ");
            });

        *state_obj
    }

    #[no_mangle]
    #[allow(non_snake_case)]
    pub unsafe extern "C" fn Java_info_scry_wallet_1manager_NativeLib_addNonAuthDigit(env: JNIEnv, _: JClass, digit_data: JString) -> jobject {
        let digit_data: String = env.get_string(digit_data).unwrap().into();
        let wallet_state_class = env.find_class("info/scry/wallet_manager/NativeLib$WalletState").expect("find wallet_state_class ");
        let state_obj = env.alloc_object(wallet_state_class).expect("create wallet_state_class instance ");

        let digits = serde_json::from_slice::<Vec<wallets::model::EthToken>>(digit_data.as_bytes());
        if let Ok(digits) = digits {
            let eth = wallets::module::Ethereum {};
            match eth.update_auth_digit(digits, false, None) {
                Ok(_) => {
                    env.set_field(state_obj, "status", "I", JValue::Int(StatusCode::OK as i32)).expect("set status value ");
                    env.set_field(state_obj, "isAddNonAuthDigit", "Z", JValue::Bool(1 as u8)).expect("showDigit value ");
                }
                Err(msg) => {
                    env.set_field(state_obj, "status", "I", JValue::Int(StatusCode::DylibError as i32)).expect("set status value ");
                    env.set_field(state_obj, "message", "Ljava/lang/String;", JValue::Object(JObject::from(env.new_string(msg.to_string()).unwrap()))).expect("set error msg value ");
                }
            }
        } else {
            env.set_field(state_obj, "status", "I", JValue::Int(StatusCode::DylibError as i32)).expect("set status value ");
            env.set_field(state_obj, "message", "Ljava/lang/String;", JValue::Object(JObject::from(env.new_string(digits.unwrap_err().to_string()).unwrap()))).expect("set error msg value ");
        }

        *state_obj
    }

    #[no_mangle]
    #[allow(non_snake_case)]
    pub unsafe extern "C" fn Java_info_scry_wallet_1manager_NativeLib_getDigitList(env: JNIEnv, _: JClass, chain_type: jint, is_auth: jboolean, start_item: jint, page_size: jint) -> jobject {
        let digit_list_class = env.find_class("info/scry/wallet_manager/NativeLib$DigitList").expect("find wallet_state_class ");
        let state_obj = env.alloc_object(digit_list_class).expect("create auth_list_class instance ");
        let eth = wallets::module::Ethereum {};
        match eth.query_auth_digit(chain_type as i64, is_auth != 0, start_item as i64, page_size as i64) {
            Ok(data) => get_jni_token_list(&env, state_obj, data),
            Err(msg) => {
                env.set_field(state_obj, "status", "I", JValue::Int(StatusCode::DylibError as i32)).expect("set status value ");
                env.set_field(state_obj, "message", "Ljava/lang/String;", JValue::Object(JObject::from(env.new_string(msg.to_string()).unwrap()))).expect("set error msg value ");
            }
        }
        *state_obj
    }

    #[no_mangle]
    #[allow(non_snake_case)]
    pub unsafe extern "C" fn Java_info_scry_wallet_1manager_NativeLib_queryDigit(env: JNIEnv, _: JClass, chain_type: jint, name: JString, contract_addr: JString) -> jobject {
        let name: String = env.get_string(name).unwrap().into();
        let contract_addr: String = env.get_string(contract_addr).unwrap().into();
        let query_name = if name.is_empty() { None } else { Some(name) };
        let query_contract = if contract_addr.is_empty() { None } else { Some(contract_addr) };

        let auth_list_class = env.find_class("info/scry/wallet_manager/NativeLib$DigitList").expect("find wallet_state_class ");
        let state_obj = env.alloc_object(auth_list_class).expect("create auth_list_class instance ");
        let eth = wallets::module::Ethereum {};
        match eth.query_digit(chain_type as i64, query_name, query_contract) {
            Ok(data) => get_jni_token_list(&env, state_obj, data),
            Err(msg) => {
                env.set_field(state_obj, "status", "I", JValue::Int(StatusCode::DylibError as i32)).expect("set status value ");
                env.set_field(state_obj, "message", "Ljava/lang/String;", JValue::Object(JObject::from(env.new_string(msg.to_string()).unwrap()))).expect("set error msg value ");
            }
        }
        *state_obj
    }

    fn get_jni_token_list(env: &JNIEnv, state_obj: JObject, data: DigitList) -> () {
        let array_list_class = env.find_class("java/util/ArrayList").expect("ArrayList");
        let array_list_obj = env.alloc_object(array_list_class).expect("array_list_class");
        env.call_method(array_list_obj, "<init>", "()V", &[]).expect("array_list_obj init method is exec");

        let eth_token_class = env.find_class("info/scry/wallet_manager/NativeLib$EthToken").expect("find NativeLib$EthToken class");
        for datum in data.eth_tokens {
            let eth_token_class_obj = env.alloc_object(eth_token_class).expect("alloc eth_token_class object");
            //Set digit property
            env.set_field(eth_token_class_obj, "id", "Ljava/lang/String;", JValue::Object(JObject::from(env.new_string(datum.id).unwrap()))).expect("eth_token_class_obj set id value");
            env.set_field(eth_token_class_obj, "symbol", "Ljava/lang/String;", JValue::Object(JObject::from(env.new_string(datum.symbol).unwrap()))).expect("eth_token_class_obj set symbol value");
            env.set_field(eth_token_class_obj, "name", "Ljava/lang/String;", JValue::Object(JObject::from(env.new_string(datum.name).unwrap()))).expect("eth_token_class_obj set name value");
            env.set_field(eth_token_class_obj, "logoUrl", "Ljava/lang/String;", JValue::Object(JObject::from(env.new_string(datum.logo_url).unwrap()))).expect("eth_token_class_obj set logoUrl value");
            env.set_field(eth_token_class_obj, "decimal", "I", JValue::Int(datum.decimal.as_str().parse::<i32>().unwrap())).expect("eth_token_class_obj set decimal value");
            env.set_field(eth_token_class_obj, "contract", "Ljava/lang/String;", JValue::Object(JObject::from(env.new_string(datum.contract).unwrap()))).expect("eth_token_class_obj set contract value");
            env.set_field(eth_token_class_obj, "chainType", "Ljava/lang/String;", JValue::Object(JObject::from(env.new_string(datum.chain_type).unwrap()))).expect("eth_token_class_obj set digitId value");
            env.call_method(array_list_obj, "add", "(Ljava/lang/Object;)Z", &[eth_token_class_obj.into()]).expect("array_list_obj add chain instance");
        }
        env.set_field(state_obj, "status", "I", JValue::Int(StatusCode::OK as i32)).expect("digit_class_obj set status value");
        env.set_field(state_obj, "count", "I", JValue::Int(data.count as i32)).expect("digit_class_obj set count value");
        env.set_field(state_obj, "startItem", "I", JValue::Int(0 as i32)).expect("digit_class_obj set startItem value");
        env.set_field(state_obj, "ethTokens", "Ljava/util/List;", JValue::Object(array_list_obj)).expect("set authDigit");
    }
}
