use clap::{Parser, Subcommand};
use tfhe::prelude::*;
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
    Add {
        #[clap(short, long)]
        a: u8,
        #[clap(short, long)]
        b: u8,
    },
    Sub {
        #[clap(short, long)]
        a: u8,
        #[clap(short, long)]
        b: u8,
    },
    Mul {
        #[clap(short, long)]
        a: u8,
        #[clap(short, long)]
        b: u8,
    },
    Div {
        #[clap(short, long)]
        a: u8,
        #[clap(short, long)]
        b: u8,
    },
    Mod {
        #[clap(short, long)]
        a: u8,
        #[clap(short, long)]
        b: u8,
    },
}

fn main() {
    let config = ConfigBuilder::default().build();

    let (client_key, server_key) = generate_keys(config);

    let cli = Cli::parse();

    match &cli.command {
        Commands::Add { a, b } => {
            let clear_a = a;
            let clear_b = b;

            let a = FheUint8::try_encrypt(*clear_a, &client_key).unwrap();
            let b = FheUint8::try_encrypt(*clear_b, &client_key).unwrap();

            set_server_key(server_key);
            let sum = a + b;

            let decrypted_sum: u8 = sum.decrypt(&client_key);

            println!("Clear inputs: {} + {}", clear_a, clear_b);
            println!("Decrypted result: {}", decrypted_sum);
        }
        Commands::Sub { a, b } => {
            let clear_a = a;
            let clear_b = b;

            let a = FheUint8::try_encrypt(*clear_a, &client_key).unwrap();
            let b = FheUint8::try_encrypt(*clear_b, &client_key).unwrap();

            set_server_key(server_key);
            let sum = a - b;

            let decrypted_sum: u8 = sum.decrypt(&client_key);

            println!("Clear inputs: {} + {}", clear_a, clear_b);
            println!("Decrypted result: {}", decrypted_sum);
        }
        Commands::Mul { a, b } => {
            let clear_a = a;
            let clear_b = b;

            let a = FheUint8::try_encrypt(*clear_a, &client_key).unwrap();
            let b = FheUint8::try_encrypt(*clear_b, &client_key).unwrap();

            set_server_key(server_key);
            let sum = a * b;

            let decrypted_sum: u8 = sum.decrypt(&client_key);

            println!("Clear inputs: {} + {}", clear_a, clear_b);
            println!("Decrypted result: {}", decrypted_sum);
        }
        Commands::Div { a, b } => {
            assert_ne!(*b, 0, "Cannot divide by zero");

            let clear_a = a;
            let clear_b = b;

            let a = FheUint8::try_encrypt(*clear_a, &client_key).unwrap();
            let b = FheUint8::try_encrypt(*clear_b, &client_key).unwrap();

            set_server_key(server_key);
            let sum = a / b;

            let decrypted_sum: u8 = sum.decrypt(&client_key);

            println!("Clear inputs: {} + {}", clear_a, clear_b);
            println!("Decrypted result: {}", decrypted_sum);
        }
        Commands::Mod { a, b } => {
            assert_ne!(*b, 0, "Cannot divide by zero");

            let clear_a = a;
            let clear_b = b;

            let a = FheUint8::try_encrypt(*clear_a, &client_key).unwrap();
            let b = FheUint8::try_encrypt(*clear_b, &client_key).unwrap();

            set_server_key(server_key);
            let sum = a % b;

            let decrypted_sum: u8 = sum.decrypt(&client_key);

            println!("Clear inputs: {} + {}", clear_a, clear_b);
            println!("Decrypted result: {}", decrypted_sum);
        }
    }
}
