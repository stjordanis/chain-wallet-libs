//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use wallet_js::*;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

const BLOCK0: &[u8] = include_bytes!("../../../test-vectors/block0");
const WALLET_VALUE: u64 = 1000000 + 10000 + 10000 + 1 + 100;

/// test to recover a yoroi style address in the test-vectors block0
///
#[wasm_bindgen_test]
fn yoroi1() {
    let mut wallet = Wallet::recover("neck bulb teach illegal soul cry monitor claw amount boring provide village rival draft stone", &[]).expect("couldn't recover wallet fully");
    let settings = wallet.retrieve_funds(BLOCK0).unwrap();
    assert_eq!(wallet.total_value(), WALLET_VALUE);

    let conversion = wallet.convert(&settings);

    assert_eq!(conversion.num_ignored(), 1);
    assert_eq!(conversion.total_value_ignored(), 1);
    assert_eq!(conversion.transactions_len(), 1);

    let _transaction_bytes = conversion
        .transactions_get(0)
        .expect("to get the only transaction present in the conversion");
}

#[wasm_bindgen_test]
fn gen_key() {
    // just test that the random generator works
    let _key = Ed25519ExtendedPrivate::generate();
}

#[wasm_bindgen_test]
fn encrypt_decrypt() {
    let data = [1u8; 64 * 2];
    let password = [1u8, 2, 3, 4];
    let encrypted = symmetric_encrypt(&password, &data).unwrap();
    assert_eq!(
        &symmetric_decrypt(&password, &encrypted).unwrap()[..],
        &data[..]
    );
}

#[wasm_bindgen_test]
fn sign_verify_extended() {
    let key = Ed25519ExtendedPrivate::generate();
    let msg = [1, 2, 3, 4u8];
    let signature = key.sign(&msg);

    assert!(key.public().verify(&signature, &msg));
}

#[wasm_bindgen_test]
fn sign_verify_bip32() {
    let key = Ed25519Bip32Private::generate();
    let msg = [1, 2, 3, 4u8];
    let signature = key.sign(&msg);

    assert!(key.public().verify(&signature, &msg));
}

#[wasm_bindgen_test]
fn sign_verify() {
    let key = Ed25519Private::generate();
    let msg = [1, 2, 3, 4u8];
    let signature = key.sign(&msg);

    assert!(key.public().verify(&signature, &msg));
}
