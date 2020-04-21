use super::*;
use sp_core::{sr25519,crypto::Ss58Codec};
use bip39::{Mnemonic, Language};
use substrate_bip39::mini_secret_from_entropy;

pub struct Sr25519;

impl Crypto for Sr25519 {
    type Seed = [u8; 32];
    type Pair = sr25519::Pair;
    type Public = sr25519::Public;

    fn seed_from_phrase(phrase: &str, password: Option<&str>) -> Result<Self::Seed,WalletError> {
        let mnemonic = Mnemonic::from_phrase(phrase, Language::English)?;
        Ok(mini_secret_from_entropy(mnemonic.entropy(),password.unwrap_or(""))?.to_bytes())
    }
    fn pair_from_seed(seed: &Self::Seed) -> Self::Pair {
        Pair::from_seed_slice(seed).expect("pair_from_seed")

    }
    fn pair_from_suri(suri: &str, password: Option<&str>) -> Result<Self::Pair,WalletError>{
        Ok(sr25519::Pair::from_string(suri, password)?)
    }
    fn ss58_from_pair(pair: &Self::Pair) -> String { pair.public().to_ss58check() }
    fn public_from_pair(pair: &Self::Pair) -> Vec<u8> { (&pair.public().0[..]).to_owned() }

    fn sign(phrase: &str,msg:&[u8])->Result<[u8;64],WalletError>{
        let seed = Self::seed_from_phrase(phrase,None)?;
        let pair = Self::pair_from_seed(&seed);
        Ok(pair.sign(msg).0)
    }
}
