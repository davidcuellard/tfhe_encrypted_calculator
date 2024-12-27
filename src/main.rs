/// This is a simple calculator that uses TFHE to perform operations on encrypted data.
/// The calculator supports addition, subtraction, multiplication, division, and modulo operations.
/// The calculator uses the Rust implementation of TFHE.
/// The calculator uses the default parameters for the TFHE keys.
/// The calculator saves the keys to files and loads them when performing operations.
/// The calculator uses the `bincode` crate to serialize and deserialize the keys.
/// The calculator uses the `clap` crate to parse command-line arguments.
/// The calculator uses the `tfhe` crate to perform the operations on encrypted data.
/// The calculator uses the `tfhe::prelude` module to simplify the code.
/// The calculator uses the `tfhe::ClientKey` and `tfhe::ServerKey` types to store the keys.
/// The calculator uses the `tfhe::generate_keys` function to generate the keys.
/// The calculator uses the `tfhe::set_server_key` function to set the server key.
/// The calculator uses the `tfhe::ConfigBuilder` type to build the configuration for the keys.
/// The calculator uses the `tfhe::FheUint8` type to perform operations on encrypted data.
use bincode;
use clap::{Parser, Subcommand};
use std::fs::{create_dir_all, File};
use std::io::{Read, Write};
use tfhe::prelude::*;
use tfhe::ClientKey;
use tfhe::ServerKey;
use tfhe::{generate_keys, set_server_key, ConfigBuilder, FheUint8};

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
            handle_operation(a, b, server_key_path, client_key_path, |a, b| a + b);
        }
        Commands::Sub {
            a,
            b,
            server_key_path,
            client_key_path,
        } => {
            handle_operation(a, b, server_key_path, client_key_path, |a, b| a - b);
        }
        Commands::Mul {
            a,
            b,
            server_key_path,
            client_key_path,
        } => {
            handle_operation(a, b, server_key_path, client_key_path, |a, b| a * b);
        }
        Commands::Div {
            a,
            b,
            server_key_path,
            client_key_path,
        } => {
            assert_ne!(*b, 0, "Cannot divide by zero");
            handle_operation(a, b, server_key_path, client_key_path, |a, b| a / b);
        }
        Commands::Mod {
            a,
            b,
            server_key_path,
            client_key_path,
        } => {
            assert_ne!(*b, 0, "Cannot divide by zero");
            handle_operation(a, b, server_key_path, client_key_path, |a, b| a % b);
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

    // We write the keys to files:
    let mut file = File::create(server_key_file).expect("failed to create server key file");
    file.write_all(encoded_server_key.as_slice())
        .expect("failed to write key to file");
    let mut file = File::create(client_key_file).expect("failed to create client key file");
    file.write_all(encoded_client_key.as_slice())
        .expect("failed to write key to file");
}

fn load_key<T>(path: &str) -> T
where
    T: serde::de::DeserializeOwned,
{
    let mut file = File::open(path).expect("Failed to open key file");
    let mut encoded_key: Vec<u8> = Vec::new();
    file.read_to_end(&mut encoded_key)
        .expect("Failed to read key file");
    bincode::deserialize(&encoded_key[..]).expect("Failed to deserialize key")
}

fn perform_operation(
    a: &u8,
    b: &u8,
    server_key: ServerKey,
    client_key: &ClientKey,
    operation: fn(FheUint8, FheUint8) -> FheUint8,
) -> u8 {
    set_server_key(server_key);

    let a = FheUint8::try_encrypt(*a, client_key).expect("Encryption of 'a' failed");
    let b = FheUint8::try_encrypt(*b, client_key).expect("Encryption of 'b' failed");

    let result = operation(a, b);

    result.decrypt(client_key)
}

fn handle_operation(
    a: &u8,
    b: &u8,
    server_key_path: &str,
    client_key_path: &str,
    operation: fn(FheUint8, FheUint8) -> FheUint8,
) {
    let server_key = load_key(server_key_path);
    let client_key = load_key(client_key_path);

    let result = perform_operation(a, b, server_key, &client_key, operation);

    println!("Result: {}", result);
}
