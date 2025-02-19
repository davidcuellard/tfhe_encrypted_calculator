# TFHE Encrypted Calculator

## Overview

This project demonstrates the power of Zama's TFHE implementation through a simple calculator built using the tfhe-rs crate. tfhe-rs is a pure Rust implementation of TFHE (Fully Homomorphic Encryption) designed for performing Boolean and integer arithmetic operations over encrypted data.

The calculator encrypts input data using TFHE encryption, performs the operation, and then decrypts the result for display. It uses the bincode crate for serializing and deserializing the keys and the clap crate for parsing command-line arguments.

For more detailed information about the tfhe-rs library and how to use it, visit the official documentation at Zama TFHE Documentation.

## Prerequisites

Before running the tfhe_encrypted_calculator, ensure that you have the following:

- Rust installed: https://www.rust-lang.org/
- cargo package manager (comes with Rust)

The following dependencies are used in this project:

- tfhe: The library for performing TFHE operations.
- bincode: Used for serializing and deserializing the keys.
- clap: Used for parsing command-line arguments.

Run:

```bash
cargo build
```

## Commands

1. Generate Keys
   Generate and save the TFHE keys to a specified directory.

```bash
cargo run generate-keys --dir <directory_path>
```

2. Add
   Perform encrypted addition on two numbers.

```bash
cargo run add --a <number_a> --b <number_b> --server-key-path <server_key_file> --client-key-path <client_key_file>
```

3. Subtract
   Perform encrypted subtraction on two numbers.

```bash
cargo run sub --a <number_a> --b <number_b> --server-key-path <server_key_file> --client-key-path <client_key_file>
```

4. Multiply
   Perform encrypted multiplication on two numbers.

```bash
cargo run mul --a <number_a> --b <number_b> --server-key-path <server_key_file> --client-key-path <client_key_file>
```

5. Divide
   Perform encrypted division on two numbers.

```bash
cargo run div --a <number_a> --b <number_b> --server-key-path <server_key_file> --client-key-path <client_key_file>
```

6. Modulo
   Perform encrypted modulo operation on two numbers.

```bash
cargo run mod --a <number_a> --b <number_b> --server-key-path <server_key_file> --client-key-path <client_key_file>
```

## How It Works

- Key Generation: The generate_and_save_keys function generates a pair of TFHE keys and saves them to disk. You can generate the keys using the generate-keys command. The generated keys will be saved as server_key.bin and client_key.bin in the specified directory (default: /tmp/z_calculator).

- Key Loading: The server and client keys are loaded from files when performing an operation using the load_key function.

- Operation Handling: Each operation (addition, subtraction, etc.) loads the keys from disk, encrypts the input numbers using the FheUint8::try_encrypt method, performs the operation on the encrypted data, and then decrypts the result using the decrypt method.

- TFHE Library: The TFHE library is used to perform the homomorphic encryption operations. The keys are used to encrypt and decrypt data, and the operations are performed on the encrypted data.

## Example Usage

1. Generate Keys:

```bash
cargo run generate-keys --dir /tmp/z_calculator
```

2. Add Two Numbers:

```bash
cargo run add --a 5 --b 3 --server-key-path /tmp/z_calculator/server_key.bin --client-key-path /tmp/z_calculator/client_key.bin
```

This will encrypt the numbers 5 and 3, add them, and print the decrypted result.

## Docs

To run the docs, the nightly toolchain is needed to avoid errors. Here are the steps:

1. Install the nightly toolchain:

```bash
rustup toolchain install nightly
```

2. Use the nightly toolchain: You can either set the nightly toolchain as the default for your project or for the current terminal session.

- To set the nightly toolchain as the default for your project, run this command in your project directory:

```bash
  rustup override set nightly
```

- To use the nightly toolchain for the current terminal session, run:

```bash
rustup default nightly
```

3. Generate and open the documentation:

```bash
 cargo doc --open
```

## License

This project is licensed under the [MIT](https://choosealicense.com/licenses/mit/) License.
