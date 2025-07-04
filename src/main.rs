use bitcoin::Network;
use bitcoin_address_generator::{derive_bitcoin_address, generate_mnemonic};
use fs4::fs_std::FileExt;
use reqwest;
use serde_json::Value;
use std::io::prelude::*;
use std::{fs::File, fs::OpenOptions, process, thread, time};
use std::time::Instant;
use tokio;

#[derive(Debug)]
struct Wallet {
    adresses: String,
    mnemonic: String,
}

impl Wallet {
    fn new() -> Self {
        Self {
            adresses: "".to_string(),
            mnemonic: "".to_string(),
        }
    }
}

fn main() {
    loop {
        print!("\x1Bc");//FOR CLEANING COMMAND SCREEN CLS OR CLEAR
        let now = Instant::now();
        let mut n = 0;
        let mut kontrol: Vec<Wallet> = Vec::new();
        while n < 50 {
            let wallet = generate_wallets();
            kontrol.push(wallet);
            n += 1;
        }
        check_balance(kontrol);
        let elapsed = now.elapsed();
        println!("50 Wallet Elapsed: {:.2?}", elapsed);
    }
}

fn generate_wallets() -> Wallet {
    let mut kontrol = Wallet::new();

    // Generate a default 12-word mnemonic in English
    let mnemonic = generate_mnemonic(None, None).unwrap();
    //println!("Generated mnemonic: {}", mnemonic);

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
        + &p2tr_addr.address.to_string()
        + "|";

    kontrol.adresses = all_adresses;
    kontrol.mnemonic = mnemonic;
    kontrol
}

#[allow(unused_variables)]
#[tokio::main]
async fn check_balance(kontrol: Vec<Wallet>) {
    let mut file = output_file();

    let base_url = "https://blockchain.info/balance?active=";
    let mut n = 0;
    let mut query: String = "".to_string();
    while n < kontrol.len() {
        query += &kontrol[n].adresses;
        n += 1;
    }

    let url = format!("{}{}", base_url, query);

    match reqwest::get(url).await {
        Ok(response) => {
            if response.status().is_success() {
                // Yanıt gövdesini alıyoruz
                match response.text().await {
                    Ok(body) => match serde_json::from_str::<Value>(&body) {
                        Ok(json) => {
                            for (address, details) in json.as_object().unwrap() {
                                /*println!(
                                    "Address: {}, Final Balance: {}",
                                    address, details["final_balance"]
                                );*/
                                if details["final_balance"].as_i64().unwrap() > 0 {
                                    let mut m = 0;
                                    file.lock_exclusive().expect("Couldn't lock file.");
                                    while m < kontrol.len() {
                                        writeln!(file, "{}", kontrol[m].mnemonic)
                                            .expect("Couldn't write to `win.txt` file.");
                                        m += 1;
                                    }
                                    process::exit(1);
                                }
                            }
                        }
                        Err(err) => eprintln!("Error parsing JSON: {}", err),
                    },
                    Err(err) => eprintln!("Error reading response body: {}", err),
                }
            } else {
                eprintln!("Request failed with status: {}", response.status());
            }
        }
        Err(err) => {
            eprintln!("Request error: {}", err);
            thread::sleep(time::Duration::from_secs(30));
        }
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