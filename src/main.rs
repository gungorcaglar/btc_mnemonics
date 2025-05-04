use tokio;
use reqwest;
use bitcoin::Network;
use serde_json::Value;
use std::io::prelude::*;
use fs4::fs_std::FileExt;
use std::{thread, time, process, fs::OpenOptions, fs::File};
use bitcoin_address_generator::{derive_bitcoin_address, generate_mnemonic};

struct Wallet {
    adresses: String,
    mnemonic: String,
}

impl Wallet {
    fn new() -> Self {
        Self {
            adresses : "".to_string(),
            mnemonic : "".to_string(),
        }
    }
}

fn main() {
    loop {
        let millis = time::Duration::from_millis(50);
        let kontrol = generate_wallets();
        check_balance(&kontrol);
        thread::sleep(millis);
    }
}

fn generate_wallets() -> Wallet {

    let mut kontrol = Wallet::new();

    // Generate a default 12-word mnemonic in English
    let mnemonic = generate_mnemonic(None, None).unwrap();
    println!("Generated mnemonic: {}", mnemonic);

    // Derive a Legacy (P2PKH) address
    let p2pkh_addr = derive_bitcoin_address(
        &mnemonic,
        Some("m/44'/0'/0'/0/0"),
        Some(Network::Bitcoin),
        None,
    )
    .unwrap();
    //println!("Legacy address: {}", p2pkh_addr.address);

    // Derive a Nested SegWit (P2SH-WPKH) address
    let p2sh_wpkh_addr = derive_bitcoin_address(
        &mnemonic,
        Some("m/49'/0'/0'/0/0"),
        Some(Network::Bitcoin),
        None,
    )
    .unwrap();
    //println!("Nested SegWit address: {}", p2sh_wpkh_addr.address);

    // Derive a Native SegWit (P2WPKH) address
    let p2wpkh_addr = derive_bitcoin_address(
        &mnemonic,
        Some("m/84'/0'/0'/0/0"),
        Some(Network::Bitcoin),
        None,
    )
    .unwrap();
    //println!("Native SegWit address: {}", p2wpkh_addr.address);

    // Derive a Taproot (P2TR) address
    let p2tr_addr = derive_bitcoin_address(
        &mnemonic,
        Some("m/86'/0'/0'/0/0"),
        Some(Network::Bitcoin),
        None,
    )
    .unwrap();
    //println!("Taproot address: {}", p2tr_addr.address);

    let all_adresses = p2pkh_addr.address.to_string()
        + "|"
        + &p2sh_wpkh_addr.address.to_string()
        + "|"
        + &p2wpkh_addr.address.to_string()
        + "|"
        + &p2tr_addr.address.to_string();

    kontrol.adresses = all_adresses;
    kontrol.mnemonic = mnemonic;
    kontrol
}

#[tokio::main]
async fn check_balance(kontrol: &Wallet) {
    let mut file = output_file();

    let base_url = "https://blockchain.info/balance?active=";
    let url = format!("{}{}", base_url, kontrol.adresses);

    match reqwest::get(url).await {
        Ok(response) => {
            if response.status().is_success() {
                // Yanıt gövdesini alıyoruz
                match response.text().await {
                    Ok(body) => {
                        match serde_json::from_str::<Value>(&body) {
                            Ok(json) => {
                                for (address, details) in json.as_object().unwrap() {
                                    println!(
                                        "Address: {}, Final Balance: {}",
                                        address, details["final_balance"]
                                    );
                                    if details["final_balance"].as_i64().unwrap() > 0 {
                                        file.lock_exclusive().expect("Couldn't lock file.");
                                        writeln!(file, "{}",kontrol.mnemonic).expect("Couldn't write to `win.txt` file.");
                                        //file.unlock().expect("Couldn't unlock file.");
                                        process::exit(1);
                                    }
                                }
                            }
                            Err(err) => eprintln!("Error parsing JSON: {}", err),
                        }
                    }
                    Err(err) => eprintln!("Error reading response body: {}", err),
                }
            } else {
                eprintln!("Request failed with status: {}", response.status());
            }
        }
        Err(err) => eprintln!("Request error: {}", err),
    }
}

#[track_caller]
fn output_file() -> File {
    OpenOptions::new()
        .append(true)
        .create(true)
        .read(true)
        .open("win.txt")
        .expect("Could not create or open `efficient_addresses.txt` file.")
}