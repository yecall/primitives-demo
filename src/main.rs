fn main() {

}

#[cfg(test)]
mod tests {
    use substrate_primitives::sr25519;
    use substrate_primitives::crypto::{Ss58Codec, Pair as PairTrait};
    use substrate_primitives::sr25519::{Pair, Public};
    use hex;
    use sr_io::blake2_256;
    use parity_codec::{KeyedVec};

    pub type AccountId = sr25519::Public;

    #[test]
    fn generate_account_id() {
        let pair: Pair = sr25519::Pair::from_string(&format!("//{}", "Alice"), None).unwrap();

        let account_id = pair.public();

        // account_id is generate by default seed, it is 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY actually
        let account_id2 = Public::from_ss58check("5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY").unwrap();

        println!("{:?}", account_id);

        assert_eq!(account_id, account_id2);
    }

    #[test]
    fn display_public_hex() {
        let account_id = Public::from_ss58check("5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY").unwrap();

        let hex = hex::encode(account_id.0);

        assert_eq!("d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d", hex);
    }

    #[test]
    fn generate_account_id_from_public_hex() {
        let account_id = Public::from_slice(&hex::decode("d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d").unwrap());

        assert_eq!("5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY", format!("{}", account_id));
    }

    #[test]
    ///
    /// balance_storage_key = blake2_256("Balances FreeBalance" + public)
    /// you can get balance by rpc
    /// ```
    /// curl -X POST --data '{"jsonrpc":"2.0","id":"78","method":"state_getStorage","params":["0x7f864e18e3dd8b58386310d2fe0919eef27c6e558564b7f67f22d99d20f587bb", null]}' localhost:9933 -H 'Content-Type: application/json'
    /// ```
    ///
    fn key_for_balance() {
        let pair: Pair = sr25519::Pair::from_string(&format!("//{}", "Alice"), None).unwrap();

        let account_id: AccountId = pair.public();

        let balance_key = blake2_256(&account_id.to_keyed_vec(b"Balances FreeBalance"));

        assert_eq!("7f864e18e3dd8b58386310d2fe0919eef27c6e558564b7f67f22d99d20f587bb", hex::encode(balance_key));

        println!("balance_key={}", hex::encode(balance_key));
    }

    #[test]
    ///
    /// balance_nonce_key = blake2_256("System AccountNonce" + public)
    /// you can get nonce by rpc
    /// ```
    /// curl -X POST --data '{"jsonrpc":"2.0","id":"78","method":"state_getStorage","params":["0x5c54163a1c72509b5250f0a30b9001fdee9d9b48388b06921f1b210e81e3a1f0", null]}' localhost:9933 -H 'Content-Type: application/json'
    /// ```
    ///
    fn key_for_nonce() {
        let pair: Pair = sr25519::Pair::from_string(&format!("//{}", "Alice"), None).unwrap();

        let account_id: AccountId = pair.public();

        let nonce_key = blake2_256(&account_id.to_keyed_vec(b"System AccountNonce"));

        assert_eq!("5c54163a1c72509b5250f0a30b9001fdee9d9b48388b06921f1b210e81e3a1f0", hex::encode(nonce_key));

        println!("nonce_key={}", hex::encode(nonce_key));
    }

    #[test]
    fn key_for_indices_enumset() {

        let enum_set_key = blake2_256(&0u32.to_keyed_vec(b"Indices EnumSet"));

        assert_eq!("c98362e2ca21b342cc749022ed9b560e4d29ec9862a960c2538c314f1d279635", hex::encode(enum_set_key));

        println!("enum_set_key={}", hex::encode(enum_set_key));
    }

    #[test]
    fn test_extrinsic(){

    }
}


