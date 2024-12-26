use clap::{Parser, Subcommand};
use std::fs::{create_dir_all, File};
use std::io::{Read, Write};
use tfhe::prelude::*;
use tfhe::ClientKey;
use tfhe::ServerKey;
use tfhe::{generate_keys, set_server_key, ConfigBuilder, FheUint8};
use bincode;

#[derive(Parser)]
#[command(
    name = "z-calculator",
    about = "Calculator with TFHE - Rust implementation"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    GenerateKeys {
        #[clap(short, long, default_value = "/tmp/z_calculator")]
        dir: String,
    },
    Add {
        #[clap(short, long)]
        a: u8,
        #[clap(short, long)]
        b: u8,
        #[clap(short, long, default_value = "/tmp/z_calculator/server_key.bin")]
        server_key_path: String,
        #[clap(short, long, default_value = "/tmp/z_calculator/client_key.bin")]
        client_key_path: String,
    },
    Sub {
        #[clap(short, long)]
        a: u8,
        #[clap(short, long)]
        b: u8,
        #[clap(short, long, default_value = "/tmp/z_calculator/server_key.bin")]
        server_key_path: String,
        #[clap(short, long, default_value = "/tmp/z_calculator/client_key.bin")]
        client_key_path: String,
    },
    Mul {
        #[clap(short, long)]
        a: u8,
        #[clap(short, long)]
        b: u8,
        #[clap(short, long, default_value = "/tmp/z_calculator/server_key.bin")]
        server_key_path: String,
        #[clap(short, long, default_value = "/tmp/z_calculator/client_key.bin")]
        client_key_path: String,
    },
    Div {
        #[clap(short, long)]
        a: u8,
        #[clap(short, long)]
        b: u8,
        #[clap(short, long, default_value = "/tmp/z_calculator/server_key.bin")]
        server_key_path: String,
        #[clap(short, long, default_value = "/tmp/z_calculator/client_key.bin")]
        client_key_path: String,
    },
    Mod {
        #[clap(short, long)]
        a: u8,
        #[clap(short, long)]
        b: u8,
        #[clap(short, long, default_value = "/tmp/z_calculator/server_key.bin")]
        server_key_path: String,
        #[clap(short, long, default_value = "/tmp/z_calculator/client_key.bin")]
        client_key_path: String,
    },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::GenerateKeys { dir } => {
            generate_and_save_keys(dir);
        }
        Commands::Add {
            a,
            b,
            server_key_path,
            client_key_path,
        } => {
            let server_key = load_server_key(server_key_path);
            let client_key = load_client_key(client_key_path);

            set_server_key(server_key);

            let clear_a = a;
            let clear_b = b;

            let a = FheUint8::try_encrypt(*clear_a, &client_key).unwrap();
            let b = FheUint8::try_encrypt(*clear_b, &client_key).unwrap();

            let sum = a + b;
            let decrypted_sum: u8 = sum.decrypt(&client_key);

            println!("Clear inputs: {} + {}", clear_a, clear_b);
            println!("Decrypted result: {}", decrypted_sum);
        }
        Commands::Sub {
            a,
            b,
            server_key_path,
            client_key_path,
        } => {
            let server_key = load_server_key(server_key_path);
            let client_key = load_client_key(client_key_path);

            set_server_key(server_key);
            let clear_a = a;
            let clear_b = b;

            let a = FheUint8::try_encrypt(*clear_a, &client_key).unwrap();
            let b = FheUint8::try_encrypt(*clear_b, &client_key).unwrap();

            let sum = a - b;

            let decrypted_sum: u8 = sum.decrypt(&client_key);

            println!("Clear inputs: {} + {}", clear_a, clear_b);
            println!("Decrypted result: {}", decrypted_sum);
        }
        Commands::Mul {
            a,
            b,
            server_key_path,
            client_key_path,
        } => {
            let server_key = load_server_key(server_key_path);
            let client_key = load_client_key(client_key_path);

            set_server_key(server_key);
            let clear_a = a;
            let clear_b = b;

            let a = FheUint8::try_encrypt(*clear_a, &client_key).unwrap();
            let b = FheUint8::try_encrypt(*clear_b, &client_key).unwrap();

            let sum = a * b;

            let decrypted_sum: u8 = sum.decrypt(&client_key);

            println!("Clear inputs: {} + {}", clear_a, clear_b);
            println!("Decrypted result: {}", decrypted_sum);
        }
        Commands::Div {
            a,
            b,
            server_key_path,
            client_key_path,
        } => {
            assert_ne!(*b, 0, "Cannot divide by zero");
            let server_key = load_server_key(server_key_path);
            let client_key = load_client_key(client_key_path);

            set_server_key(server_key);

            let clear_a = a;
            let clear_b = b;

            let a = FheUint8::try_encrypt(*clear_a, &client_key).unwrap();
            let b = FheUint8::try_encrypt(*clear_b, &client_key).unwrap();

            let sum = a / b;

            let decrypted_sum: u8 = sum.decrypt(&client_key);

            println!("Clear inputs: {} + {}", clear_a, clear_b);
            println!("Decrypted result: {}", decrypted_sum);
        }
        Commands::Mod {
            a,
            b,
            server_key_path,
            client_key_path,
        } => {
            assert_ne!(*b, 0, "Cannot divide by zero");
            let server_key = load_server_key(server_key_path);
            let client_key = load_client_key(client_key_path);

            set_server_key(server_key);

            let clear_a = a;
            let clear_b = b;

            let a = FheUint8::try_encrypt(*clear_a, &client_key).unwrap();
            let b = FheUint8::try_encrypt(*clear_b, &client_key).unwrap();

            let sum = a % b;

            let decrypted_sum: u8 = sum.decrypt(&client_key);

            println!("Clear inputs: {} + {}", clear_a, clear_b);
            println!("Decrypted result: {}", decrypted_sum);
        }
    }
}

/// Serialize the keys to bytes and save them to files
fn generate_and_save_keys(dir: &str) {
    // We generate a set of client/server keys, using the default parameters:
    let config = ConfigBuilder::default().build();

    let (client_key, server_key) = generate_keys(config);

    // We serialize the keys to bytes:
    let encoded_server_key: Vec<u8> = bincode::serialize(&server_key).unwrap();
    let encoded_client_key: Vec<u8> = bincode::serialize(&client_key).unwrap();

    create_dir_all(dir).expect("Failed to create directory");

    let server_key_file = &format!("{dir}/server_key.bin");
    let client_key_file = &format!("{dir}/client_key.bin");

    println!("server_key_file: {:?}", server_key_file);

    // We write the keys to files:
    let mut file = File::create(server_key_file).expect("failed to create server key file");
    file.write_all(encoded_server_key.as_slice())
        .expect("failed to write key to file");
    let mut file = File::create(client_key_file).expect("failed to create client key file");
    file.write_all(encoded_client_key.as_slice())
        .expect("failed to write key to file");
}

/// Load a server key from a file
fn load_server_key(path: &str) -> ServerKey {
    let mut file = File::open(path).expect("Failed to open server key file");
    let mut encoded_key: Vec<u8> = Vec::new();
    file.read_to_end(&mut encoded_key)
        .expect("Failed to read server key");
    bincode::deserialize(&encoded_key[..]).expect("Failed to deserialize server key")
}

/// Load a client key from a file
fn load_client_key(path: &str) -> ClientKey {
    let mut file = File::open(path).expect("Failed to open server key file");
    let mut encoded_key: Vec<u8> = Vec::new();
    file.read_to_end(&mut encoded_key)
        .expect("Failed to read server key");
    bincode::deserialize(&encoded_key[..]).expect("Failed to deserialize server key")
}
